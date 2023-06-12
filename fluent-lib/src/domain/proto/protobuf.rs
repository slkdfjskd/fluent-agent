use chrono::Local;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::{FileDescriptor, MessageDescriptor, RuntimeFieldType, RuntimeType};

use crate::common::error;
use crate::common::error::Result;
use crate::domain::proto::entity::Message as EntityMessage;
use crate::domain::proto::entity::{Method, ProtoProject, Service};

pub fn parse(proto_files: Vec<PathBuf>, import_path: Vec<PathBuf>) -> Result<ProtoProject> {
    let r = protobuf_parse::Parser::new()
        .pure()
        .includes(&import_path)
        .inputs(&proto_files)
        .parse_and_typecheck();
    if let Err(ref e) = r {
        info!("protobuf parse err :{}", e)
    }

    let fd_protos = r?.file_descriptors;

    let dyn_fds = dyn_fd_link(fd_protos)?;
    let mut fd_services = Vec::new();
    // FIXME 多个proto 包名相同，所以map 的 value 得是个 vec
    let mut fd_pkg_map: HashMap<String, Vec<Arc<FileDescriptor>>> = HashMap::new();
    let mut fd_pkg_none = Vec::new();
    for rc_fd in dyn_fds {
        if rc_fd.proto().service.len() > 0 {
            fd_services.push(rc_fd.clone());
        }

        match rc_fd.proto().package.as_ref() {
            Some(pkg) => {
                let r = rc_fd.clone();
                let v = fd_pkg_map.get_mut(pkg);
                match v {
                    Some(v) => v.push(r),
                    None => {
                        let v = vec![r];
                        fd_pkg_map.insert(pkg.clone(), v);
                    }
                }
            }
            None => {
                let r = rc_fd.clone();
                fd_pkg_none.push(r)
            }
        }
    }
    let mut services = Vec::new();
    let mut service_map = HashMap::new();
    for rc_fd in fd_services {
        let proto = &rc_fd.proto();
        let package_name = proto
            .package
            .as_ref()
            .ok_or(error::proto_parse_error_with_str("package name is none"))?
            .clone();
        for service_dp in &proto.service {
            let service_name = service_dp
                .name
                .as_ref()
                .ok_or(error::proto_parse_error_with_str("service name is none"))?;
            let service_name = build_full_service_name(&package_name, service_name);
            let mut methods = Vec::new();
            let mut method_map = HashMap::new();
            for method_dp in &service_dp.method {
                let method_name = method_dp
                    .name
                    .as_ref()
                    .ok_or(error::proto_parse_error_with_str("method name is none"))?
                    .clone();
                let input_type = method_dp
                    .input_type
                    .as_ref()
                    .ok_or(error::proto_parse_error_with_str("input type is none"))?
                    .clone();
                let output_type = method_dp
                    .output_type
                    .as_ref()
                    .ok_or(error::proto_parse_error_with_str("input type is none"))?
                    .clone();
                let (input_msg_pkg, input_msg_name) = split_message_type(input_type)?;
                let (output_msg_pkg, output_msg_name) = split_message_type(output_type)?;
                let req_fd = find_fd_by_msg_pkg_name(
                    &input_msg_pkg,
                    &input_msg_name,
                    &fd_pkg_map,
                    &fd_pkg_none,
                )?;
                let resp_fd = find_fd_by_msg_pkg_name(
                    &output_msg_pkg,
                    &output_msg_name,
                    &fd_pkg_map,
                    &fd_pkg_none,
                )?;
                let req_fields = fields(&input_msg_name, req_fd.clone())?;
                let resp_fields = fields(&output_msg_name, resp_fd.clone())?;

                debug!("json:{}", serde_json::to_string(&req_fields)?);
                debug!("json resp:{}", serde_json::to_string(&resp_fields)?);

                let method = Arc::new(Method {
                    name: method_name,
                    request: Arc::new(EntityMessage {
                        name: input_msg_name,
                        dyn_fd: req_fd.clone(),
                        json: req_fields,
                    }),
                    response: Arc::new(EntityMessage {
                        name: output_msg_name,
                        dyn_fd: resp_fd.clone(),
                        json: resp_fields,
                    }),
                });

                methods.push(method.clone());
                method_map.insert(method.name.clone(), method.clone());
            }
            let service = Arc::new(Service {
                name: service_name.clone(),
                methods,
                method_map,
            });
            services.push(service.clone());
            service_map.insert(service.name.clone(), service.clone());
        }
    }
    // FIXME 考虑只返回services 去掉Arc
    Ok(ProtoProject {
        services,
        service_map,
    })
}

fn build_full_service_name(package_name: &str, service_name: &str) -> String {
    let mut result = "".to_string();
    result.push_str(package_name);
    result.push('.');
    result.push_str(service_name);
    return result;
}

fn fields(message_name: &String, fd: Arc<FileDescriptor>) -> Result<serde_json::Value> {
    let md = fd
        .message_by_package_relative_name(message_name)
        .ok_or_else(|| {
            error::proto_parse_error(format!("message name:{} not found", message_name))
        })?;
    Ok(handle_message_type(md)?)
}

fn handle_message_type(md: MessageDescriptor) -> Result<serde_json::Value> {
    let fields = md.fields();
    let mut result = serde_json::Map::new();
    for field in fields {
        let name = field.json_name().to_string();
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(t) => {
                let value = handle_base_type(t)?;
                result.insert(name, value);
            }
            RuntimeFieldType::Repeated(t) => {
                // let name_clone = name.as_ref().unwrap().clone();
                // new_message_field_with_array(name, FieldType::Array, handle_base_type(None, t)?)
                let value = handle_base_type(t)?;
                result.insert(name, serde_json::Value::Array(vec![value]));
            }
            RuntimeFieldType::Map(_k, t) => {
                let value = handle_base_type(t)?;
                let mut map = serde_json::Map::new();
                map.insert("key1".to_string(), value);
                result.insert(name, serde_json::Value::Object(map));
            }
        };
    }
    return Ok(serde_json::Value::Object(result));
}

fn handle_base_type(rt_type: RuntimeType) -> Result<serde_json::Value> {
    let result = match rt_type {
        RuntimeType::I32
        | RuntimeType::I64
        | RuntimeType::U32
        | RuntimeType::U64
        | RuntimeType::F32
        | RuntimeType::F64 => serde_json::Value::Number(serde_json::Number::from(0)),
        RuntimeType::Bool => serde_json::Value::Bool(false),
        RuntimeType::String => serde_json::Value::String("".to_string()),
        RuntimeType::VecU8 => serde_json::Value::String("hex encode".to_string()),
        RuntimeType::Enum(e) => {
            let value = e
                .values()
                .next()
                .ok_or(error::proto_parse_error_with_str("enum value not found"))?
                .name()
                .to_string();
            serde_json::Value::String(value)
        }
        RuntimeType::Message(m) => {
            if m.full_name().eq("google.protobuf.Timestamp") {
                serde_json::Value::String(Local::now().to_rfc3339())
            } else {
                handle_message_type(m)?
            }
        }
    };
    return Ok(result);
}

/// FIXME clone 过多需要优化
fn dyn_fd_link(fd_protos: Vec<FileDescriptorProto>) -> Result<Vec<Arc<FileDescriptor>>> {
    let mut fd_name_map = HashMap::new();
    for fd_proto in &fd_protos {
        let name = fd_proto
            .name
            .as_ref()
            .ok_or(error::proto_parse_error_with_str("proto name is none"))?
            .clone();
        fd_name_map.insert(name, Arc::new(fd_proto.clone()));
    }
    let mut dyn_fd_map = HashMap::new();
    let mut result = Vec::new();
    for fd in fd_protos {
        let dyn_fd = new_dyn_fd(fd, &fd_name_map, &mut dyn_fd_map)?;
        result.push(dyn_fd.clone());
    }
    return Ok(result);
}

fn new_dyn_fd(
    fd_proto: FileDescriptorProto,
    fd_name_map: &HashMap<String, Arc<FileDescriptorProto>>,
    dyn_fd_map: &mut HashMap<String, Arc<FileDescriptor>>,
) -> Result<Arc<FileDescriptor>> {
    let name = fd_proto
        .name
        .as_ref()
        .ok_or(error::proto_parse_error_with_str("fd_proto name is none"))?
        .clone();
    let dyn_fd = dyn_fd_map.get(&name);
    return match dyn_fd {
        Some(dyn_fd) => Ok(dyn_fd.clone()),
        None => {
            let mut deps = Vec::new();
            for dep in &fd_proto.dependency {
                let dep_fd = fd_name_map.get(dep).ok_or_else(|| {
                    error::proto_parse_error(format!("dep name:{} not found", dep))
                })?;
                let dyn_dep = new_dyn_fd(dep_fd.deref().clone(), fd_name_map, dyn_fd_map)?;
                deps.push(dyn_dep.deref().clone());
            }
            let key = fd_proto
                .name
                .as_ref()
                .ok_or(error::proto_parse_error_with_str("fd_proto name is none"))?
                .clone();
            let dyn_fd = FileDescriptor::new_dynamic(fd_proto, &deps)?;

            let rc_dyn_fd = Arc::new(dyn_fd);
            dyn_fd_map.insert(key, rc_dyn_fd.clone());
            Ok(rc_dyn_fd)
        }
    };
}

fn find_fd_by_msg_pkg_name(
    msg_pkg_name: &String,
    msg_name: &String,
    fd_pkg_map: &HashMap<String, Vec<Arc<FileDescriptor>>>,
    fd_pkg_none: &Vec<Arc<FileDescriptor>>,
) -> Result<Arc<FileDescriptor>> {
    // FIXME 包名相同 在比较 message_type
    let fd = fd_pkg_map.get(msg_pkg_name);
    match fd {
        Some(fd_vec) => {
            if fd_vec.is_empty() {
                return Err(error::proto_parse_error(format!(
                    "MessagePkgName: {} not found",
                    msg_pkg_name
                )));
            } else if fd_vec.len() == 1 {
                return Ok(fd_vec.get(0).unwrap().clone());
            } else {
                find_fd_by_msg_name(msg_name, fd_vec)
            }
        }
        None => find_fd_by_msg_name(msg_name, fd_pkg_none),
    }
}

fn find_fd_by_msg_name(
    msg_name: &String,
    fd_vec: &Vec<Arc<FileDescriptor>>,
) -> Result<Arc<FileDescriptor>> {
    for fd in fd_vec {
        for t in &fd.proto().message_type {
            let name = t
                .name
                .as_ref()
                .ok_or(error::proto_parse_error_with_str(
                    "message type name is none",
                ))?
                .clone();
            if name.eq(msg_name) {
                return Ok(fd.clone());
            }
        }
    }
    return Err(error::proto_parse_error(format!(
        "msg_name: {} not found",
        msg_name
    )));
}

fn split_message_type(message_type: String) -> Result<(String, String)> {
    let mut strs: Vec<&str> = message_type.split(".").collect();
    if strs.is_empty() {
        return Err(error::proto_parse_error_with_str(
            "message_type split length is zero",
        ));
    }
    let message_name = strs.remove(strs.len() - 1);
    let mut message_pkg = strs.join(".");
    if let Some(i) = message_pkg.find('.') {
        if i == 0 {
            message_pkg.remove(i);
        }
    }
    return Ok((message_pkg, message_name.to_string()));
}

#[cfg(test)]
mod tests {

    use crate::domain::proto::protobuf::parse;
    use std::path::Path;

    #[test]
    fn test_init() {
        let includes = vec![
            Path::new("./proto").to_path_buf(),
            Path::new("./proto/vendor").to_path_buf(),
        ];
        let protos = vec![Path::new("./proto/service.proto").to_path_buf()];
        let result = parse(protos, includes);
        result.unwrap();
    }
}
