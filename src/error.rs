use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Scanner(#[from] ScannerError),
    #[error(transparent)]
    Parser(#[from] ParserError),
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Usage {} [script]", std::env::args().nth(0).unwrap())]
    Usage,
    #[error("IO Error: ")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum ScannerError {
    // #[error(transparent)]
    // Nom(#[from] nom::error::Error<Span>),
    #[error("Unexpected `EoF`")]
    UnexpectedEof,
    #[error("[line {line}] Error at `{token}`: {message}.")]
    At {
        line: usize,
        token: String,
        message: String,
    },
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("[line {line}] Error at `{token}`: {message}.")]
    At {
        line: usize,
        token: String,
        message: String,
    },
    #[error("{0}")]
    Tmp(String),
    #[error("{0}")]
    Consume(String),
    #[error(transparent)]
    Scanner(#[from] ScannerError),
}
