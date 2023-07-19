use regex::Regex;

use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::any::Any;

use crate::{errors::*, lisp::Lisp};

//#[derive(Clone)]
pub enum Object {
    Nil,
    True,
    Pair(Rc<Object>, Rc<Object>),
    Symbol(String),
    Number(f32),
    Character(char),
    Quoted(Rc<Object>),
    LispFunc(Vec<String>, Vec<Rc<Object>>),
    RustFunc(fn(&mut Lisp, Rc<Object>) -> RustFuncResult),
    RustType(Box<dyn Any>),
}

impl Object {
    fn parse_atom(string: &str) -> Result<Self, LispError> {
        if let Ok(i) = str::parse::<f32>(string) {
            Ok(Self::Number(i))
        } else if string.len() == 2 && string.starts_with('\\') {
            Ok(Self::Character(string.chars().nth(1).unwrap()))
        } else if string.starts_with('"') && string.ends_with('"') {
            let mut string = string.to_string();
            string.pop();

            let string = string
                .replace("\\\\", "\\")
                .replace("\\\"", "\"")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\n", "\n")
                .replace("\\0", "\0");

            let objects = string
                .chars()
                .skip(1)
                .map(Self::Character)
                .map(Rc::new)
                .collect();

            Ok(Self::array_to_pair_list(objects))
        } else if !string.is_empty() {
            Ok(Self::Symbol(string.to_string()))
        } else {
            Err(LispError::new(
                LispErrorKind::Parser,
                ParserError::UnparsableAtom(string.to_string()),
            ))
        }
    }

    pub fn string_to_lisp_string(string: &str) -> Self {
        let objects = string.chars().map(Self::Character).map(Rc::new).collect();

        Self::array_to_pair_list(objects)
    }

    #[allow(clippy::result_unit_err)]
    pub fn pair_list_to_string(&self) -> Result<String, ()> {
        let mut string = String::new();
        let mut cur_object = self;

        loop {
            match cur_object {
                Object::Pair(a, b) => {
                    if let Self::Character(c) = **a {
                        string.push(c);

                        cur_object = b;
                    } else {
                        return Err(());
                    }
                }
                Object::Nil => break,
                _ => return Err(()),
            }
        }

        Ok(string)
    }

    fn append_to_pair_list(&mut self, appende: Object) {
        let mut cur_object: &mut Self = match self {
            Self::Pair(_, b) => Rc::get_mut(b).unwrap(),
            _ => panic!("Not a list"),
        };

        loop {
            match cur_object {
                Self::Pair(_, b) => cur_object = Rc::get_mut(b).unwrap(),
                Self::Nil => break,
                _ => panic!("Not a list"),
            }
        }

        *cur_object = appende;
    }

    fn array_to_pair_list(array: Vec<Rc<Object>>) -> Self {
        let mut ret = Self::Pair(array[0].clone(), Rc::new(Self::Nil));

        let mut iter = array.into_iter();
        iter.next();

        for i in iter {
            ret.append_to_pair_list(Self::Pair(i.clone(), Rc::new(Self::Nil)));
        }

        ret
    }

    fn iter_to_object(strings: &mut dyn Iterator<Item = String>) -> Result<Object, LispError> {
        let mut list = Vec::new();
        let mut dot_occured = false;

        Ok(loop {
            match strings.next() {
                Some(s) => match s.as_str() {
                    "(" => {
                        if dot_occured {
                            let mut list = Self::array_to_pair_list(list);

                            list.append_to_pair_list(Self::iter_to_object(strings)?);

                            let next = strings.next().unwrap();
                            if next != *")" {
                                return Err(LispError::new(
                                    LispErrorKind::Parser,
                                    ParserError::InvalidToken(next),
                                ));
                            }

                            break list;
                        } else {
                            list.push(Rc::new(Self::iter_to_object(strings)?))
                        }
                    }
                    ")" => {
                        break if list.is_empty() {
                            Self::Nil
                        } else {
                            Self::array_to_pair_list(list)
                        }
                    }
                    "." => {
                        if !list.is_empty() {
                            dot_occured = true
                        } else {
                            return Err(LispError::new(
                                LispErrorKind::Parser,
                                ParserError::InvalidToken(".".to_string()),
                            ));
                        }
                    }
                    s => {
                        if dot_occured {
                            if strings.next() == Some(")".to_string()) {
                                let mut list = Self::array_to_pair_list(list);

                                // Ignore comments
                                if !s.starts_with(';') {
                                    list.append_to_pair_list(Object::parse_atom(s)?);
                                }

                                break list;
                            } else {
                                return Err(LispError::new(
                                    LispErrorKind::Parser,
                                    ParserError::InvalidToken(s.to_string()),
                                ));
                            }
                        } else if let Some(o) = Self::parse_string(s, strings)? {
                            list.push(Rc::new(o))
                        }
                    }
                },
                None => {
                    return Err(LispError::new(
                        LispErrorKind::Parser,
                        ParserError::UnmatchedToken('('),
                    ))
                }
            }
        })
    }

    fn parse_string(
        string: &str,
        iter: &mut dyn Iterator<Item = String>,
    ) -> Result<Option<Object>, LispError> {
        match string {
            "(" => Ok(Some(Self::iter_to_object(iter)?)),
            ")" => Err(LispError::new(
                LispErrorKind::Parser,
                ParserError::UnmatchedToken(')'),
            )),
            "." => Err(LispError::new(
                LispErrorKind::Parser,
                ParserError::InvalidToken(".".to_string()),
            )),
            "\'" => {
                if let Some(next) = iter.next() {
                    if let Some(next) = Self::parse_string(&next, iter)? {
                        Ok(Some(Object::Quoted(Rc::new(next))))
                    } else {
                        Err(LispError::new(
                            LispErrorKind::Parser,
                            ParserError::EmptyQuote,
                        ))
                    }
                } else {
                    Err(LispError::new(
                        LispErrorKind::Parser,
                        ParserError::EmptyQuote,
                    ))
                }
            }
            s => {
                // Ignore comments
                if !s.starts_with(';') {
                    Ok(Some(Object::parse_atom(s)?))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn eval_strings(strings: Vec<String>) -> Result<Vec<Object>, LispError> {
        let mut iter = strings.into_iter();
        let mut ret = Vec::new();

        loop {
            match iter.next() {
                Some(s) => {
                    if let Some(o) = Self::parse_string(&s, &mut iter)? {
                        ret.push(o)
                    }
                }
                None => break Ok(ret),
            }
        }
    }

    fn split_into_strings(input: &str) -> Vec<String> {
        let regex = Regex::new(r#"(?m);[^\n]*|"(?:\\.|[^"\\])*"|'|\(|\)|[^\s()]*"#).unwrap();

        regex
            .captures_iter(input)
            .filter_map(|x| {
                let s = x.get(0).unwrap().as_str().to_string();

                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .collect()
    }

    pub fn eval(input: &str) -> Result<Vec<Object>, LispError> {
        let strings = Self::split_into_strings(input);
        Self::eval_strings(strings)
    }
}

use core::fmt;
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(a, b) => {
                let mut string = true;
                let mut objects = Vec::new();
                let mut cur_object = self;

                loop {
                    match cur_object {
                        Self::Pair(a, b) => {
                            objects.push(a);

                            match **a {
                                Self::Character(_) => (),
                                _ => string = false,
                            }

                            cur_object = b;
                        }
                        Self::Nil => break,
                        c => {
                            return if objects.is_empty() {
                                write!(f, "({:?} . {:?})", a, b)
                            } else {
                                write!(f, "(")?;

                                let mut objects = objects.iter();

                                if let Some(o) = objects.next() {
                                    write!(f, "{}", o)?;

                                    for o in objects {
                                        write!(f, " {}", o)?;
                                    }
                                }

                                write!(f, " . {})", c)
                            }
                        }
                    }
                }

                if string {
                    let mut string = String::new();

                    for o in objects {
                        if let Self::Character(c) = **o {
                            match c {
                                '\\' => string.push_str("\\\\"),
                                '\"' => string.push_str("\\\n"),
                                '\t' => string.push_str("\\t"),
                                '\r' => string.push_str("\\r"),
                                '\n' => string.push_str("\\n"),
                                '\0' => string.push_str("\\0"),
                                c => string.push(c),
                            }
                        } else {
                            panic!("Unexpected type in string");
                        }
                    }

                    write!(f, "\"{}\"", string)
                } else {
                    write!(f, "(")?;

                    let mut objects = objects.iter();

                    if let Some(o) = objects.next() {
                        write!(f, "{}", o)?;

                        for o in objects {
                            write!(f, " {}", o)?;
                        }
                    }

                    write!(f, ")")
                }
            }
            Self::Number(i) => write!(f, "{}", i),
            Self::Character(c) => write!(f, "\\{}", c),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::Quoted(o) => write!(f, "'{}", o),
            Self::Nil => write!(f, "()"),
            Self::True => write!(f, "t"),
            Self::RustFunc(x) => write!(f, "{:p}", x),
            Self::LispFunc(a, _) => write!(f, "({})", a.join(" ")),
            Self::RustType(t) => write!(f, "{:?}", t),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Pair(a, b) => match other {
                Self::Pair(c, d) => a == c && b == d,
                _ => false,
            },
            Self::Number(i) => match other {
                Self::Number(o) => i == o,
                _ => false,
            },
            Self::Character(c) => match other {
                Self::Character(o) => c == o,
                _ => false,
            },
            Self::Symbol(s) => match other {
                Self::Symbol(o) => s == o,
                _ => false,
            },
            Self::Quoted(s) => match other {
                Self::Quoted(o) => s == o,
                _ => false,
            },
            Self::RustFunc(_) => false,
            Self::LispFunc(_, _) => false,
            Self::Nil => matches!(other, Self::Nil),
            Self::True => matches!(other, Self::True),
            Self::RustType(_) => false,
        }
    }
}
