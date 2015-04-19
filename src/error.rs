use hyper::HttpError;
use std::string::FromUtf8Error;

use protocol::*;

#[derive(Debug)]
pub enum GameError {
    GameFinished,
    GameNotStarted,
    GuessFailed,
    GuessComplete,
    CurrentNoWord,
    ServerError(String),
    HttpError(String),
    ParseJsonError(String),
    OtherError(String),
}

impl From<HttpError> for GameError {
    fn from(e: HttpError) -> GameError {
        GameError::HttpError(format!("{}", e))
    }
}
impl From<()> for GameError {
    fn from(_: ()) -> GameError {
        GameError::OtherError("unknown error".to_string())
    }
}
impl From<FromUtf8Error> for GameError {
    fn from(e: FromUtf8Error) -> GameError {
        GameError::OtherError(format!("{}", e))
    }
}
impl From<::serde::json::error::Error> for GameError {
    fn from(e: ::serde::json::error::Error) -> GameError {
        GameError::ParseJsonError(format!("{}", e))
    }
}
impl From<ServerError> for GameError {
    fn from(e: ServerError) -> GameError {
        GameError::ServerError(e.message.to_string())
    }
}
