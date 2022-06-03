use crate::object::Object;
use std::error::Error;
use std::fmt;

pub type LispResult = Result<Box<Object>, LispError>;
pub type RustFuncResult = Result<Box<Object>, RustFuncError>;

#[derive(Debug, Clone)]
pub enum LispErrorKind {
    Parser,
    Eval,
    RustFunc,
}

#[derive(Debug)]
pub struct LispError {
    kind: LispErrorKind,
    error: Box<dyn Error>,
}

impl LispError {
    pub fn new<E>(kind: LispErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn Error>>,
    {
        let error = error.into();

        Self { kind, error }
    }
}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LispErrorKind::Parser => write!(f, "Error parsing code: {}", self.error),
            LispErrorKind::Eval => write!(f, "Error evaluating object: {}", self.error),
            LispErrorKind::RustFunc => write!(f, "{}", self.error),
        }
    }
}

impl Error for LispError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    UnmatchedToken(char),
    InvalidToken(String),
    UnparsableAtom(String),
    EmptyQuote,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnmatchedToken(c) => write!(f, "Unmatched token: '{}'", c),
            Self::UnparsableAtom(a) => write!(f, "Unparsable atom: {}", a),
            Self::InvalidToken(c) => write!(f, "Invalid token: '{}'", c),
            Self::EmptyQuote => write!(f, "Empty quote"),
        }
    }
}

impl Error for ParserError {}

#[derive(Debug, Clone)]
pub enum EvalError {
    UnknownSymbol(String),
    GlobalExists(String),
    NonFunction(Object),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSymbol(s) => write!(f, "Unknown symbol: {}", s),
            Self::GlobalExists(s) => write!(f, "Global already exists: {}", s),
            Self::NonFunction(o) => write!(f, "Attempt to call non-function: {}", o),
        }
    }
}

impl Error for EvalError {}

#[derive(Debug)]
pub enum RustFuncError {
    InvalidArguments(ArgumentsError),
    LispError(LispError),
}

impl RustFuncError {
    pub fn new_args_error(error: ArgumentsError) -> Self {
        Self::InvalidArguments(error)
    }
}

impl fmt::Display for RustFuncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArguments(e) => {
                write!(f, "Error running function: Invalid arguments: {}", e)
            }
            Self::LispError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for RustFuncError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidArguments(e) => Some(e),
            Self::LispError(e) => Some(e),
        }
    }
}

impl From<LispError> for RustFuncError {
    fn from(error: LispError) -> Self {
        Self::LispError(error)
    }
}

#[derive(Debug, Clone)]
pub enum ArgumentsError {
    TooMany,
    NotEnough,
    WrongType,
    DottedPair,
}

impl fmt::Display for ArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooMany => write!(f, "Too many arguments"),
            Self::NotEnough => write!(f, "Not enough arguments"),
            Self::WrongType => write!(f, "Arguments of wrong type"),
            Self::DottedPair => write!(f, "Dotted-pair arguments not accepted"),
        }
    }
}

impl Error for ArgumentsError {}
