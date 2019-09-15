use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    NoDirection,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        match self {
            MyError::NoDirection => "no direction found in level file",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
