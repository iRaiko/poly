use std::error::Error;
use std::convert::From;
use std::io::Error as IoError;
use std::fmt;

#[derive(Debug)]
pub enum CustomError{
    Io(IoError),
    File,
}

impl Error for CustomError{
    fn cause(&self) -> Option<&dyn Error>{
        match *self{
            CustomError::Io(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl fmt::Display for CustomError
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            CustomError::Io(ref cause) => write!(formatter, "IO Error: {}", cause),
            CustomError::File => write!(formatter, "Something went wrong haha yes"),
        }
    }
}

impl From<std::io::Error> for CustomError
{
    fn from(cause: IoError) -> CustomError{
        CustomError::Io(cause)
    }
}