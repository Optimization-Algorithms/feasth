use std::convert;
use std::fmt;
use std::io::Error;
use std::num::ParseIntError;

use std::error;


#[derive(Debug)]
pub enum ParseError {
    IOError(Error),
    ParseError(ParseIntError),
    Generic(Box<dyn error::Error>)
}

impl convert::From<Error> for ParseError {
    fn from(e: Error) -> Self {
        Self::IOError(e)
    }
}

impl convert::From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseError(e)
    }
}

impl convert::From<Box<dyn error::Error>> for ParseError {
    fn from(e: Box<dyn error::Error>) -> Self {
        Self::Generic(e)
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "IO Error: {}", err),
            Self::ParseError(err) => write!(f, "Int Parse Error: {}", err),
            Self::Generic(err) => write!(f, "Error: {}", err)
        }
    }
}



#[derive(Debug)]
pub struct GetError {
    url: String,
    status: u16,
    reason: Option<&'static str>
}

impl GetError {
    pub fn new(url: String, status: reqwest::StatusCode) -> Self {
        Self {
            url,
            status: status.as_u16(),
            reason: status.canonical_reason()
        }
    }
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!("Error: GET {} ended with status {}", self.url, self.status);
        let msg = if let Some(reason) = self.reason {
            format!("{}\nReason: {}", msg, reason)
        } else {
            msg
        };
        write!(f, "{}", msg)
    }
}


impl error::Error for GetError {}

#[derive(Debug)]
pub enum AutoGetSizeError {
    WrongFormat(String),
    SizeNotFound(String)
}


impl fmt::Display for AutoGetSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WrongFormat(name) => write!(f, "cannot extract instance name from:`{}`\nUse a NAME-init.csv format instead", name),
            Self::SizeNotFound(name) => write!(f, "cannot find the size for given instance: `{}` on MipLib", name)
        }
    }
}

impl error::Error for AutoGetSizeError {}

#[derive(Debug)]
pub struct PathConversionError {}

impl fmt::Display for PathConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot convert given path to UTF-8 string")
    }
}

impl error::Error for PathConversionError {}
