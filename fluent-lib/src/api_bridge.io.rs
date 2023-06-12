use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init(port_: i64) {
    wire_init_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_import_proto(
    port_: i64,
    project_id: i64,
    proto_files: *mut wire_StringList,
    import_paths: *mut wire_StringList,
) {
    wire_import_proto_impl(port_, project_id, proto_files, import_paths)
}

#[no_mangle]
pub extern "C" fn wire_get_proto_file(port_: i64, project_id: i64) {
    wire_get_proto_file_impl(port_, project_id)
}

#[no_mangle]
pub extern "C" fn wire_list_nav_project(port_: i64) {
    wire_list_nav_project_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_project(port_: i64, name: *mut wire_uint_8_list, req_type: i32) {
    wire_create_project_impl(port_, name, req_type)
}

#[no_mangle]
pub extern "C" fn wire_get_config(port_: i64, key: *mut wire_uint_8_list) {
    wire_get_config_impl(port_, key)
}

#[no_mangle]
pub extern "C" fn wire_get_batch_config(port_: i64, keys: *mut wire_StringList) {
    wire_get_batch_config_impl(port_, keys)
}

#[no_mangle]
pub extern "C" fn wire_put_config(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_put_config_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_delete_configs(port_: i64, keys: *mut wire_StringList) {
    wire_delete_configs_impl(port_, keys)
}

#[no_mangle]
pub extern "C" fn wire_delete_project(port_: i64, project_id: i64) {
    wire_delete_project_impl(port_, project_id)
}

#[no_mangle]
pub extern "C" fn wire_update_project_name(
    port_: i64,
    project_id: i64,
    new_name: *mut wire_uint_8_list,
) {
    wire_update_project_name_impl(port_, project_id, new_name)
}

#[no_mangle]
pub extern "C" fn wire_get_request(port_: i64, request_id: i64) {
    wire_get_request_impl(port_, request_id)
}

#[no_mangle]
pub extern "C" fn wire_update_request(port_: i64, request: *mut wire_UpdateRequest) {
    wire_update_request_impl(port_, request)
}

#[no_mangle]
pub extern "C" fn wire_send_request(port_: i64, param: *mut wire_SendRequest) {
    wire_send_request_impl(port_, param)
}

#[no_mangle]
pub extern "C" fn wire_create_env(port_: i64, param: *mut wire_CreateEnvironment) {
    wire_create_env_impl(port_, param)
}

#[no_mangle]
pub extern "C" fn wire_update_env_variable(port_: i64, param: *mut wire_UpdateEnvironment) {
    wire_update_env_variable_impl(port_, param)
}

#[no_mangle]
pub extern "C" fn wire_update_env_name(
    port_: i64,
    new_name: *mut wire_uint_8_list,
    old_name: *mut wire_uint_8_list,
) {
    wire_update_env_name_impl(port_, new_name, old_name)
}

#[no_mangle]
pub extern "C" fn wire_delete_env_variable(port_: i64, id: i64) {
    wire_delete_env_variable_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_delete_env(port_: i64, env_name: *mut wire_uint_8_list) {
    wire_delete_env_impl(port_, env_name)
}

#[no_mangle]
pub extern "C" fn wire_list_env(port_: i64) {
    wire_list_env_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_unique_id(port_: i64) {
    wire_unique_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_lib_info(port_: i64) {
    wire_lib_info_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_list_next_request_log(
    port_: i64,
    last_id: i64,
    keyword: *mut wire_uint_8_list,
    page_size: u16,
) {
    wire_list_next_request_log_impl(port_, last_id, keyword, page_size)
}

#[no_mangle]
pub extern "C" fn wire_list_pre_request_log(
    port_: i64,
    first_id: i64,
    keyword: *mut wire_uint_8_list,
    page_size: u16,
) {
    wire_list_pre_request_log_impl(port_, first_id, keyword, page_size)
}

#[no_mangle]
pub extern "C" fn wire_get_latest_request_log(port_: i64, request_id: i64) {
    wire_get_latest_request_log_impl(port_, request_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_create_environment_0() -> *mut wire_CreateEnvironment {
    support::new_leak_box_ptr(wire_CreateEnvironment::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_send_request_0() -> *mut wire_SendRequest {
    support::new_leak_box_ptr(wire_SendRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_update_environment_0() -> *mut wire_UpdateEnvironment {
    support::new_leak_box_ptr(wire_UpdateEnvironment::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_update_request_0() -> *mut wire_UpdateRequest {
    support::new_leak_box_ptr(wire_UpdateRequest::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_entry_dto_0(len: i32) -> *mut wire_list_entry_dto {
    let wrap = wire_list_entry_dto {
        ptr: support::new_leak_vec_ptr(<wire_EntryDTO>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<CreateEnvironment> for *mut wire_CreateEnvironment {
    fn wire2api(self) -> CreateEnvironment {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CreateEnvironment>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SendRequest> for *mut wire_SendRequest {
    fn wire2api(self) -> SendRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SendRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<UpdateEnvironment> for *mut wire_UpdateEnvironment {
    fn wire2api(self) -> UpdateEnvironment {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UpdateEnvironment>::wire2api(*wrap).into()
    }
}
impl Wire2Api<UpdateRequest> for *mut wire_UpdateRequest {
    fn wire2api(self) -> UpdateRequest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UpdateRequest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CreateEnvironment> for wire_CreateEnvironment {
    fn wire2api(self) -> CreateEnvironment {
        CreateEnvironment {
            env_name: self.env_name.wire2api(),
            name: self.name.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<EntryDTO> for wire_EntryDTO {
    fn wire2api(self) -> EntryDTO {
        EntryDTO {
            name: self.name.wire2api(),
            value: self.value.wire2api(),
        }
    }
}

impl Wire2Api<Vec<EntryDTO>> for *mut wire_list_entry_dto {
    fn wire2api(self) -> Vec<EntryDTO> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<SendRequest> for wire_SendRequest {
    fn wire2api(self) -> SendRequest {
        SendRequest {
            request_id: self.request_id.wire2api(),
            url: self.url.wire2api(),
            req_type: self.req_type.wire2api(),
            headers: self.headers.wire2api(),
            params: self.params.wire2api(),
            req_json: self.req_json.wire2api(),
            env_name: self.env_name.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<UpdateEnvironment> for wire_UpdateEnvironment {
    fn wire2api(self) -> UpdateEnvironment {
        UpdateEnvironment {
            id: self.id.wire2api(),
            name: self.name.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<UpdateRequest> for wire_UpdateRequest {
    fn wire2api(self) -> UpdateRequest {
        UpdateRequest {
            id: self.id.wire2api(),
            name: self.name.wire2api(),
            url: self.url.wire2api(),
            method: self.method.wire2api(),
            headers: self.headers.wire2api(),
            params: self.params.wire2api(),
            req_json: self.req_json.wire2api(),
            resp_json: self.resp_json.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CreateEnvironment {
    env_name: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EntryDTO {
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_entry_dto {
    ptr: *mut wire_EntryDTO,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SendRequest {
    request_id: i64,
    url: *mut wire_uint_8_list,
    req_type: i32,
    headers: *mut wire_list_entry_dto,
    params: *mut wire_list_entry_dto,
    req_json: *mut wire_uint_8_list,
    env_name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UpdateEnvironment {
    id: i64,
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UpdateRequest {
    id: i64,
    name: *mut wire_uint_8_list,
    url: *mut wire_uint_8_list,
    method: *mut wire_uint_8_list,
    headers: *mut wire_list_entry_dto,
    params: *mut wire_list_entry_dto,
    req_json: *mut wire_uint_8_list,
    resp_json: *mut wire_uint_8_list,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_CreateEnvironment {
    fn new_with_null_ptr() -> Self {
        Self {
            env_name: core::ptr::null_mut(),
            name: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CreateEnvironment {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EntryDTO {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_EntryDTO {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SendRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            request_id: Default::default(),
            url: core::ptr::null_mut(),
            req_type: Default::default(),
            headers: core::ptr::null_mut(),
            params: core::ptr::null_mut(),
            req_json: core::ptr::null_mut(),
            env_name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SendRequest {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_UpdateEnvironment {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            name: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_UpdateEnvironment {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_UpdateRequest {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            name: core::ptr::null_mut(),
            url: core::ptr::null_mut(),
            method: core::ptr::null_mut(),
            headers: core::ptr::null_mut(),
            params: core::ptr::null_mut(),
            req_json: core::ptr::null_mut(),
            resp_json: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_UpdateRequest {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
