use crate::common::error;
use crate::common::error::Code::OK;
use crate::common::error::{invalid_argument_error_with_str, Result};
use crate::domain::proto::entity;
use bytes::BufMut;
use chrono::DateTime;
use protobuf::reflect::{MessageDescriptor, ReflectValueBox, RuntimeFieldType, RuntimeType};
use protobuf::MessageDyn;
use std::sync::Arc;
use tonic::codec::{EncodeBuf, Encoder};
use tonic::Status;

#[derive(Default)]
pub struct FromJsonEncoder {}

impl Encoder for FromJsonEncoder {
    type Item = Box<dyn MessageDyn>;
    type Error = Status;

    fn encode(
        &mut self,
        item: Self::Item,
        dst: &mut EncodeBuf<'_>,
    ) -> std::result::Result<(), Status> {
        let vec = item.write_to_bytes_dyn().unwrap();
        if vec.len() <= dst.remaining_mut() {
            for b in vec {
                dst.put_u8(b)
            }
        }
        Ok(())
    }
}

pub fn build_req_message_dyn(
    req_json: &serde_json::Value,
    req_msg: Arc<entity::Message>,
) -> Result<ReflectValueBox> {
    let req_md = req_msg
        .dyn_fd
        .message_by_package_relative_name(&req_msg.name)
        .ok_or(error::proto_parse_error(format!(
            "input message not found name:{}",
            &req_msg.name
        )))?;

    build_req_message_dyn0("root", &req_md, &req_json)
}

fn build_req_message_dyn0(
    parent_name: &str,
    req_md: &MessageDescriptor,
    json: &serde_json::Value,
) -> Result<ReflectValueBox> {
    if json.is_null() {
        return Ok(ReflectValueBox::Message(req_md.new_instance()));
    }
    let obj = json
        .as_object()
        .ok_or(error::invalid_argument_error(format!(
            "json field [{}] must be a object or null",
            parent_name
        )))?;
    let mut message_dyn = req_md.new_instance();
    let fields = req_md.fields();
    for field in fields {
        let name = field.json_name();
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(rt) => {
                let ref_val = parse_req_field_base_type(name, &rt, obj.get(name))?;
                if let Some(val) = ref_val {
                    field.set_singular_field(&mut *message_dyn, val)
                }
            }
            RuntimeFieldType::Repeated(rt) => {
                let val = obj.get(name);
                if let Some(v) = val {
                    let arr = v.as_array().ok_or(error::invalid_argument_error(format!(
                        "field [{}] must be a array",
                        name
                    )))?;
                    for v in arr {
                        let ref_val = parse_req_field_base_type(name, &rt, Some(v))?;
                        if let Some(vv) = ref_val {
                            field.mut_repeated(&mut *message_dyn).push(vv)
                        }
                    }
                }
            }
            RuntimeFieldType::Map(_rt_k, rt_v) => {
                // FIXME map 目前只支持String 类型的key

                let val = obj.get(name);
                if let Some(v) = val {
                    let map = v.as_object().ok_or(error::invalid_argument_error(format!(
                        "field [{}] must be a object",
                        name
                    )))?;
                    for (k, v) in map {
                        let ref_v = parse_req_field_base_type(name, &rt_v, Some(v))?;
                        if let Some(vv) = ref_v {
                            field
                                .mut_map(&mut *message_dyn)
                                .insert(ReflectValueBox::String(k.clone()), vv)
                        }
                    }
                }
            }
        }
    }
    return Ok(ReflectValueBox::Message(message_dyn));
}

fn parse_req_field_base_type(
    name: &str,
    rt_type: &RuntimeType,
    json_opt: Option<&serde_json::Value>,
) -> Result<Option<ReflectValueBox>> {
    return match rt_type {
        RuntimeType::I32 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_i64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a integer",
                name
            )))?;
            Ok(Some(ReflectValueBox::I32(v as i32)))
        }),
        RuntimeType::I64 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_i64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a integer",
                name
            )))?;
            Ok(Some(ReflectValueBox::I64(v)))
        }),
        RuntimeType::U32 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_u64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a unsigned integer",
                name
            )))?;
            Ok(Some(ReflectValueBox::U32(v as u32)))
        }),
        RuntimeType::U64 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_u64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a unsigned integer",
                name
            )))?;
            Ok(Some(ReflectValueBox::U64(v)))
        }),
        RuntimeType::F32 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_f64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a float",
                name
            )))?;
            Ok(Some(ReflectValueBox::F32(v as f32)))
        }),
        RuntimeType::F64 => json_opt.map_or(Ok(None), |json| {
            let v = json.as_f64().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a float",
                name
            )))?;
            Ok(Some(ReflectValueBox::F64(v)))
        }),
        RuntimeType::Bool => json_opt.map_or(Ok(None), |json| {
            let v = json.as_bool().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a boolean",
                name
            )))?;
            Ok(Some(ReflectValueBox::Bool(v)))
        }),
        RuntimeType::String => json_opt.map_or(Ok(None), |json| {
            let v = json
                .as_str()
                .ok_or(error::invalid_argument_error(format!(
                    "field [{}] must be a string",
                    name
                )))?
                .to_string();
            Ok(Some(ReflectValueBox::String(v)))
        }),
        RuntimeType::VecU8 => json_opt.map_or(Ok(None), |json| {
            let s = json.as_str().ok_or(error::invalid_argument_error(format!(
                "field [{}] must be a hex encoded string",
                name
            )))?;
            Ok(Some(ReflectValueBox::Bytes(hex::decode(s)?)))
        }),
        RuntimeType::Enum(e) => json_opt.map_or_else(
            || {
                let mut enum_i = -1;
                for v in e.values() {
                    enum_i = v.value();
                    break;
                }
                Ok(Some(ReflectValueBox::Enum(e.clone(), enum_i)))
            },
            |json| {
                let mut enum_i = -1;
                let e_value = json.as_str().ok_or(error::invalid_argument_error(format!(
                    "field [{}] must be a string",
                    name
                )))?;
                for v in e.values() {
                    let vname = v.name();
                    if v.name().eq(e_value) {
                        enum_i = v.value();
                        break;
                    }
                }
                if enum_i == -1 {
                    return Err(error::invalid_argument_error(format!(
                        "enum [{}] does not have value named {}",
                        name, e_value
                    )));
                }
                Ok(Some(ReflectValueBox::Enum(e.clone(), enum_i)))
            },
        ),
        RuntimeType::Message(m) => {
            json_opt.map_or(Ok(None), |value| {
                if value.is_null() {
                    return Ok(None);
                }
                // 时间特殊处理
                if m.full_name().eq("google.protobuf.Timestamp") {
                    json_opt.map_or(Ok(Some(ReflectValueBox::String("".to_string()))), |json| {
                        let v = json
                            .as_str()
                            .ok_or(error::invalid_argument_error(format!(
                                "field [{}] must be a string",
                                name
                            )))?
                            .to_string();
                        let date_time = DateTime::parse_from_rfc3339(&v)?;
                        let mut map = serde_json::Map::new();
                        let seconds = serde_json::Number::from(date_time.timestamp());
                        map.insert("seconds".to_string(), serde_json::Value::Number(seconds));
                        let nanos = serde_json::Number::from(date_time.timestamp_subsec_nanos());
                        map.insert("nanos".to_string(), serde_json::Value::Number(nanos));
                        let json = serde_json::Value::Object(map);
                        let r = build_req_message_dyn0(name, m, &json)?;
                        Ok(Some(r))
                    })
                } else {
                    let r = build_req_message_dyn0(name, m, value)?;
                    Ok(Some(r))
                }
            })
        }
    };
}
