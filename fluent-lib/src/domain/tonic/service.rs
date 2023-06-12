use crate::common::error;
use crate::common::error::Result;
use crate::domain::config::repo::ConfigRepo;
use crate::domain::config::CONFIG_REPO_WRAP;
use crate::domain::proto;
use crate::domain::tonic::{client, codec, entity};
use hex::ToHex;
use hyper::http::uri::{Parts, PathAndQuery};
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use regex::Regex;
use std::borrow::BorrowMut;
use std::str::FromStr;
use std::sync::Arc;
use tonic::codegen::http;
use tonic::codegen::http::uri::Scheme;
use tonic::metadata::KeyAndValueRef;

pub fn request(req: &entity::Request) -> Result<entity::Response> {
    let (uri, tls) = parse_url(&req.url)?;
    println!("uri:{}", uri);
    let req_msg_dyn = build_req_message_dyn(&req.req_json, req.req_msg.clone())?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let resp;
    if tls {
        resp = runtime.block_on(client::request_with_tls(
            uri,
            &req.path,
            &req.metadata,
            req_msg_dyn,
            req.resp_msg.clone(),
        ))?;
    } else {
        resp = runtime.block_on(client::request(
            uri,
            &req.path,
            &req.metadata,
            req_msg_dyn,
            req.resp_msg.clone(),
        ))?;
    }

    let mut metadata = vec![];
    let metadata_map_iter = resp.metadata().iter();
    for entry in metadata_map_iter {
        match entry {
            KeyAndValueRef::Ascii(k, v) => {
                let key = k.to_string();
                let value = v.to_str()?;
                let entry = proto::entity::Entry {
                    name: key,
                    value: value.to_string(),
                };
                metadata.push(entry);
            }
            KeyAndValueRef::Binary(k, v) => {
                let key = k.encode_hex();
                let value = v.encode_hex();
                let entry = proto::entity::Entry { name: key, value };
                metadata.push(entry);
            }
        }
    }

    let result = entity::Response {
        metadata,
        body: resp.into_inner(),
    };
    return Ok(result);
}

fn parse_url(url: &str) -> Result<(http::uri::Uri, bool)> {
    let uri = http::uri::Uri::from_str(url)?;
    let scheme = uri.scheme();
    return match scheme {
        Some(s) => {
            if "https".eq(s.as_str()) {
                return Ok((uri, true));
            }
            Ok((uri, false))
        }
        None => {
            let port = uri.port_u16();
            let scheme = match port {
                Some(p) => {
                    if p == 443 {
                        "https"
                    } else {
                        "http"
                    }
                }
                None => "http",
            };
            let mut part = Parts::from(uri);
            part.scheme = Some(Scheme::from_str(scheme)?);
            part.path_and_query = Some(PathAndQuery::from_str("/")?);
            Ok((http::uri::Uri::from_parts(part)?, "https".eq(scheme)))
        }
    };
}

fn build_req_message_dyn(
    req_json: &serde_json::Value,
    req_msg: Arc<proto::entity::Message>,
) -> Result<Box<dyn MessageDyn>> {
    let ref_value_box = codec::encode::build_req_message_dyn(req_json, req_msg)?;
    match ref_value_box {
        ReflectValueBox::Message(msg_dyn) => Ok(msg_dyn),
        _ => Err(error::invalid_argument_error(format!(
            "build req_message_dyn is not message type"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_request() {}
}
