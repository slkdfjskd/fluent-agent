use std::sync::Arc;

use bytes::BufMut;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use tonic::codec::Codec;

use crate::common::error::Result;
use crate::domain::proto::entity;
use crate::domain::tonic::codec::decode::ToJsonDecoder;
use crate::domain::tonic::codec::encode::FromJsonEncoder;

#[derive(Debug, Clone)]
pub struct JsonCodec {
    resp_msg: Arc<entity::Message>,
}

impl JsonCodec {
    pub fn new(resp_msg: Arc<entity::Message>) -> Self {
        Self { resp_msg }
    }
}

impl Codec for JsonCodec {
    type Encode = Box<dyn MessageDyn>;
    type Decode = serde_json::Value;
    type Encoder = FromJsonEncoder;
    type Decoder = ToJsonDecoder;

    fn encoder(&mut self) -> Self::Encoder {
        FromJsonEncoder::default()
    }

    fn decoder(&mut self) -> Self::Decoder {
        ToJsonDecoder::new(self.resp_msg.clone())
    }
}
