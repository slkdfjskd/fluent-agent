use crate::common::error::Code::{InvalidArgument, ProtoParseError, RespInvalidArgument};
use chrono::ParseError;
use hex::FromHexError;
use log::SetLoggerError;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::result::Result as StdResult;
use tonic::codegen::http::uri::{InvalidUri, InvalidUriParts};
use tonic::metadata::errors::{InvalidMetadataKey, InvalidMetadataValue, ToStrError};
use tonic::Status;

pub type Result<T = ()> = StdResult<T, Error>;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Code {
    OK,
    ProtoParseError,
    LibError,
    InvalidArgument,
    RespInvalidArgument,
    TonicError,
    HexError,
    SqliteError,
    SerdeJsonError,
    ProtoBufError,
    ChronError,
    AnyhowError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: Code,
    pub msg: String,
}

pub fn error(code: Code, msg: String) -> Error {
    return Error { code, msg };
}

pub fn error_with_msg(msg: String) -> Error {
    return Error {
        code: Code::LibError,
        msg,
    };
}

pub fn proto_parse_error(msg: String) -> Error {
    return error(ProtoParseError, msg);
}

pub fn proto_parse_error_with_str(msg: &str) -> Error {
    return error(ProtoParseError, msg.to_string());
}

pub fn invalid_argument_error(msg: String) -> Error {
    return error(InvalidArgument, msg);
}

pub fn invalid_argument_error_with_str(msg: &str) -> Error {
    return error(InvalidArgument, msg.to_string());
}

pub fn resp_invalid_argument_error(msg: String) -> Error {
    return error(RespInvalidArgument, msg);
}

#[allow(dead_code)]
pub fn error_with_str(msg: &str) -> Error {
    return error_with_msg(msg.to_string());
}

impl std::error::Error for Error {}

impl From<Error> for Status {
    fn from(e: Error) -> Self {
        Status::new(tonic::Code::Internal, e.msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {:?} , msg: {}", self.code, self.msg)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        return error_with_msg(s);
    }
}

impl From<&dyn std::error::Error> for Error {
    fn from(e: &dyn std::error::Error) -> Self {
        return error_with_msg(e.to_string());
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return error(Code::SerdeJsonError, e.to_string());
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        return error(Code::AnyhowError, e.to_string());
    }
}

impl From<protobuf::Error> for Error {
    fn from(e: protobuf::Error) -> Self {
        return error(Code::ProtoBufError, e.to_string());
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        return error(Code::SqliteError, e.to_string());
    }
}

impl From<Error> for rusqlite::Error {
    fn from(e: Error) -> Self {
        return rusqlite::Error::InvalidParameterName(e.msg);
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        return error_with_msg(e.to_string());
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        return error(Code::ChronError, e.to_string());
    }
}

impl From<SetLoggerError> for Error {
    fn from(e: SetLoggerError) -> Self {
        return error_with_msg(e.to_string());
    }
}

impl From<Status> for Error {
    fn from(s: Status) -> Self {
        let mut msg = "".to_string();
        msg.push_str("code:");
        msg.push_str(&s.code().to_string());
        msg.push_str(",");
        msg.push_str("msg:");
        msg.push_str(&s.message().to_string());
        return error(Code::TonicError, msg);
    }
}

impl From<InvalidUri> for Error {
    fn from(e: InvalidUri) -> Self {
        return invalid_argument_error(e.to_string());
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        return error(Code::TonicError, e.to_string());
    }
}

impl From<InvalidMetadataKey> for Error {
    fn from(e: InvalidMetadataKey) -> Self {
        return error(Code::TonicError, e.to_string());
    }
}

impl From<InvalidMetadataValue> for Error {
    fn from(e: InvalidMetadataValue) -> Self {
        return error(Code::TonicError, e.to_string());
    }
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        return error(Code::HexError, e.to_string());
    }
}

impl From<ToStrError> for Error {
    fn from(e: ToStrError) -> Self {
        return error(Code::TonicError, e.to_string());
    }
}

impl From<InvalidUriParts> for Error {
    fn from(e: InvalidUriParts) -> Self {
        return error(Code::TonicError, e.to_string());
    }
}
