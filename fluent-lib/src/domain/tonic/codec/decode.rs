use std::fmt::format;
use std::ops::Deref;
use std::sync::Arc;

use bytes::Buf;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use protobuf::reflect::{MessageDescriptor, ReflectValueRef, RuntimeFieldType, RuntimeType};
use protobuf::MessageDyn;
use tonic::codec::{DecodeBuf, Decoder};
use tonic::Status;

use crate::common::error;
use crate::common::error::Result;
use crate::domain::proto::entity;

pub struct ToJsonDecoder {
    resp_msg: Arc<entity::Message>,
}

impl ToJsonDecoder {
    pub(crate) fn new(resp_msg: Arc<entity::Message>) -> Self {
        Self { resp_msg }
    }
}

impl Decoder for ToJsonDecoder {
    type Item = serde_json::Value;
    type Error = Status;

    fn decode(
        &mut self,
        src: &mut DecodeBuf<'_>,
    ) -> std::result::Result<Option<Self::Item>, Status> {
        let name = &self.resp_msg.name;
        let resp_md = self
            .resp_msg
            .dyn_fd
            .message_by_package_relative_name(name)
            .ok_or(error::resp_invalid_argument_error(format!(
                "{} resp messageDescriptor not found",
                name
            )))?;
        let mut resp_msg_dyn = resp_md.new_instance();
        let data = src.chunk();
        merge_from_bytes_dyn(&mut resp_msg_dyn, data)?;
        src.advance(data.len());
        let result = parse_resp_message_dyn(&resp_md, &resp_msg_dyn)?;
        return Ok(Some(result));
    }
}

pub fn merge_from_bytes_dyn(msg_dyn: &mut Box<dyn MessageDyn>, data: &[u8]) -> Result<()> {
    let result = msg_dyn.merge_from_bytes_dyn(data)?;
    return Ok(result);
}

pub fn parse_resp_message_dyn(
    resp_md: &MessageDescriptor,
    resp_msg_dyn: &Box<dyn MessageDyn>,
) -> Result<serde_json::Value> {
    let mut result = serde_json::Map::new();
    let fields = resp_md.fields();
    for field_fd in fields {
        let name = field_fd.json_name();
        match field_fd.runtime_field_type() {
            RuntimeFieldType::Singular(rt) => {
                let ref_value = field_fd.get_singular(&**resp_msg_dyn);
                let val = parse_resp_field_base_type(&rt, ref_value)?;
                result.insert(name.to_string(), val);
            }
            RuntimeFieldType::Repeated(rt) => {
                let mut arr = vec![];
                let ref_value_arr = field_fd.get_repeated(&**resp_msg_dyn);
                for ref_value in ref_value_arr {
                    let val = parse_resp_field_base_type(&rt, Some(ref_value))?;
                    arr.push(val)
                }
                result.insert(name.to_string(), serde_json::Value::Array(arr));
            }
            RuntimeFieldType::Map(rt_k, rt_v) => {
                let mut map = serde_json::Map::new();
                let field_map = field_fd.get_map(&**resp_msg_dyn);
                for (k, v) in &field_map {
                    let k = k.to_string();
                    let v = parse_resp_field_base_type(&rt_v, Some(v))?;
                    map.insert(k, v);
                }
                result.insert(name.to_string(), serde_json::Value::Object(map));
            }
        }
    }

    return Ok(serde_json::Value::Object(result));
}

fn parse_resp_field_base_type(
    rt_type: &RuntimeType,
    ref_value: Option<ReflectValueRef>,
) -> Result<serde_json::Value> {
    let result = match rt_type {
        RuntimeType::I32 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(serde_json::Number::from(ref_value.to_i32().unwrap()))
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::I64 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(serde_json::Number::from(ref_value.to_i64().unwrap()))
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::U32 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(serde_json::Number::from(ref_value.to_u32().unwrap()))
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::U64 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(serde_json::Number::from(ref_value.to_u64().unwrap()))
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::F32 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(ref_value.to_f32().unwrap() as f64).unwrap(),
                )
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::F64 => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(ref_value.to_f64().unwrap()).unwrap(),
                )
            } else {
                serde_json::Value::Number(serde_json::Number::from(0))
            }
        }
        RuntimeType::Bool => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::Bool(ref_value.to_bool().unwrap())
            } else {
                serde_json::Value::Bool(false)
            }
        }
        RuntimeType::String => {
            if let Some(ref_value) = ref_value {
                serde_json::Value::String(ref_value.to_string())
            } else {
                serde_json::Value::String("".to_string())
            }
        }
        RuntimeType::VecU8 => {
            if let Some(ref_value) = ref_value {
                let byte_arr = ref_value.to_bytes().unwrap();
                serde_json::Value::String(hex::encode(byte_arr))
            } else {
                serde_json::Value::String("".to_string())
            }
        }
        RuntimeType::Enum(e) => {
            let mut enum_i = 0;
            if let Some(ref_value) = ref_value {
                enum_i = ref_value.to_enum_value().unwrap();
            }
            let mut value = "".to_string();
            for v in e.values() {
                if v.value() == enum_i {
                    value = v.name().to_string();
                    break;
                }
            }
            serde_json::Value::String(value)
        }
        RuntimeType::Message(m) => {
            if let Some(ref_value) = ref_value {
                let msg_dyn = ref_value.to_message().unwrap().clone_box();
                let mut json_msg = parse_resp_message_dyn(&m, &msg_dyn)?;
                if m.full_name().eq("google.protobuf.Timestamp") {
                    let map_msg = json_msg.as_object().ok_or(error::error_with_str(
                        "google.protobuf.Timestamp parse error",
                    ))?;
                    let mut seconds: i64 = 0;
                    let mut nanos: u64 = 0;
                    for (k, v) in map_msg {
                        if k.eq("seconds") {
                            seconds = v.as_i64().ok_or(error::error_with_str(
                                "google.protobuf.Timestamp parse seconds error",
                            ))?;
                        } else if k.eq("nanos") {
                            nanos = v.as_u64().ok_or(error::error_with_str(
                                "google.protobuf.Timestamp parse nanos error",
                            ))?;
                        }
                    }

                    let naive = NaiveDateTime::from_timestamp_opt(seconds, nanos as u32).ok_or(
                        error::error_with_str(
                            "google.protobuf.Timestamp parse NaiveDateTime error",
                        ),
                    )?;
                    let date_time = Local.timestamp_opt(seconds, nanos as u32).unwrap();
                    json_msg = serde_json::Value::String(date_time.to_rfc3339());
                }
                json_msg
            } else {
                serde_json::Value::Null
            }
        }
    };

    return Ok(result);
}
