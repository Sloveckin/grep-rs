use std::{
    error::{self},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub enum ErrorType {
    IOError(Rc<dyn error::Error>),
    NotFound,
}

impl ErrorType {
    pub fn display(&self) {
        match self {
            Self::IOError(err) => println!("{err}"),
            Self::NotFound => println!("Nothing was founded"),
        }
    }
}

pub type GrepResult<T> = Result<T, ErrorType>;
