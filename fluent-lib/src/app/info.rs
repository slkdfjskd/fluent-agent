use super::dto;
use crate::common::error::Code;
use crate::common::types::ReqType;

include!(concat!("../common/", "version.rs"));

pub fn lib_info() -> dto::LibInfo {
    log::info!("lib_info");
    dto::LibInfo {
        version: String::from(VERSION),
        build_num: BUILD_NUM,
        build_at: String::from(BUILD_AT),
        commit_hash: String::from(COMMIT_HASH),
        code: Code::OK,
        req_type: Some(ReqType::HTTP),
    }
}

#[cfg(test)]
mod tests {
    use crate::app::info::lib_info;

    #[test]
    fn test_lib_info() {
        let info = lib_info();
        assert_eq!("0.0.1", info.version);
    }

    #[test]
    fn test_request_test() {}
}
