use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Object {
    Pair(Box<Object>, Box<Object>),
    Symbol(String),
    Integer(i32),
    Character(char),
}

impl Object {
    fn parse_atom(string: &str) -> Self {
        if let Ok(i) = str::parse::<i32>(string) {
            Self::Integer(i)
        } else if string.len() == 2 || string.starts_with('\\') {
            Self::Character(string.chars().nth(1).unwrap())
        } else if !string.is_empty() {
            Self::Symbol(string.to_string())
        } else {
            panic!("Unparsable atom");
        }
    }

    fn append_to_pair_list(&mut self, appende: Object) {
        let mut cur_object: &mut Self = match self {
            Self::Pair(_, b) => b,
            _ => panic!("Not a list"),
        };

        loop {
            match cur_object {
                Self::Pair(_, b) => cur_object = b,
                Self::Symbol(s) => {
                    if s == "nil" {
                        break 
                    } else {
                        panic!("Not a list")
                    }
                },
                _ => panic!("Not a list"),
            }
        }

        *cur_object = appende;
    }

    fn new_nil() -> Self {
        Self::Symbol("nil".to_string())
    }

    fn array_to_pair_list(array: Vec<Object>) -> Self {
        let mut ret = Self::Pair(Box::new(array[0].clone()), Box::new(Self::new_nil()));

        let mut iter = array.into_iter();
        iter.next();

        for i in iter {
            ret.append_to_pair_list(Self::Pair(Box::new(i), Box::new(Self::new_nil())));
        }

        ret
    }

    fn iter_to_objects(strings: &mut dyn Iterator<Item = String>) -> Self {
        let mut list = Vec::new();
        let mut dot_occured = false;

        loop {
            match strings.next() {
                Some(s) => match s.as_str() {
                    "(" => {
                        if dot_occured {
                            let mut list = Self::array_to_pair_list(list);
 
                            list.append_to_pair_list(Self::iter_to_objects(strings));

                            if strings.next() != Some(")".to_string()) {
                                panic!("Unexpected object after dotted list end")
                            }

                            break list;
                        } else {
                            list.push(Self::iter_to_objects(strings))
                        }
                    },
                    ")" => {
                        break if list.is_empty() {
                            Self::new_nil()
                        } else {
                            Self::array_to_pair_list(list)
                        }
                    }
                    "." => {
                        if !list.is_empty() {
                            dot_occured = true
                        } else {
                            panic!("Expected object before '.'")
                        }
                    },
                    s => {
                        if dot_occured {
                            if strings.next() == Some(")".to_string()) {
                                let mut list = Self::array_to_pair_list(list);
 
                                list.append_to_pair_list(Self::parse_atom(s));

                                break list;
                            } else {
                                panic!("Unexpected object after dotted list end")
                            }
                        } else {
                            list.push(Self::parse_atom(s));
                        }
                    },
                },
                None => panic!("Unmatched '('"),
            }
        }
    }

    fn eval(strings: Vec<String>) -> Self {
        let mut iter = strings.into_iter();

        match iter.next() {
            Some(s) => match s.as_str() {
                "(" => Self::iter_to_objects(&mut iter),
                ")" => panic!("Unmatched ')'"),
                s => Object::parse_atom(s),
            },

            None => Object::Symbol(String::new())
        }
    }
}

pub struct Lisp {
    variables: HashMap<String, Rc<Object>>,
}

impl Lisp {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, input: &str) -> String {
        let strings = Self::split_into_strings(input); // Split code into seperate tokens
        let object = Object::eval(strings); // Evaluate tokens into objects
        //TODO excecute objects

        format!("{:?}", object)
    }
   
    fn split_into_strings(input: &str) -> Vec<String> {
        let regex = Regex::new(r"(?m)\(|\)|[^\s()]*").unwrap();
 
        regex.captures_iter(input)
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
}
