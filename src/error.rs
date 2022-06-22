use miette::{Diagnostic, SourceSpan};
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Diagnostic, Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Parser(#[from] ParserError),
    #[error(transparent)]
    Interpreter(#[from] InterpreterError),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Diagnostic, Error, Debug)]
pub enum SetupError {
    #[error("Usage {} [script]", std::env::args().nth(0).unwrap())]
    Usage,
    #[error("IO Error: ")]
    Io(#[from] io::Error),
}

#[derive(Diagnostic, Error, Debug)]
#[error("{message}")]
#[diagnostic(help("good luck"))]
pub struct ParserError {
    #[source_code]
    pub src: String,
    #[label("here")]
    pub span: SourceSpan,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum InterpreterError {}
