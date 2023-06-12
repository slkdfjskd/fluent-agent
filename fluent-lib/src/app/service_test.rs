#[cfg(test)]
mod tests {
    use crate::app::service::{import_proto, list_nav_project};
    use crate::app::{dto, param, service};
    use crate::common::types::ReqType;
    use crate::infra;
    use crate::infra::persistence;

    #[test]
    fn test_import_proto() {
        persistence::init_db().unwrap();
        let includes = vec![
            "../fluent-test".to_string(),
            "../fluent-test/vendor".to_string(),
        ];
        let protos = vec!["../fluent-test/proto/service.proto".to_string()];
        let r = import_proto(10001, protos, includes);
        println!("importProtoResult:{:?}", r);
    }

    #[test]
    fn test_list_nav_project() {
        let nav_projects = list_nav_project();
        println!("nav_project:{:?}", nav_projects);
    }

    #[test]
    fn test_send_request() {
        persistence::init_db().unwrap();
        let json = r#"{
                "attrs":  {
                    "key1":  {
                        "id":  0
                    }
                },
                "commonMessage": null, 
                "haha":  0,
                "image":  "FFFF",
                "list":  [
                   
                ],
                "resultCode":  "OK",
                "resultCodeList":  [
                    "OK"
                ],
                "startTime":  "2023-03-26T15:45:19.723709+08:00",
                "subMchId":  {
                    "value":  ""
                },
                "targetTime":  "2023-03-26T15:45:19.723780+08:00",
                "test":  "",
                "test6":  ""
            }
            "#
        .to_string();
        let param = param::SendRequest {
            request_id: 10010,
            url: "127.0.0.1:25000".to_string(),
            req_type: ReqType::GRPC,
            headers: vec![dto::EntryDTO {
                name: "test-name".to_string(),
                value: "test-value".to_string(),
            }],
            params: vec![],
            req_json: json,
            env_name: "test".to_string(),
        };
        let result = service::send_request(param);
        let result = result.unwrap();
        println!("result:{:?}", result);
    }

    #[test]
    fn test_send_request2() {
        let json = r#"{
            "targetTime": {
                "nanos":  0,
                "seconds":  16577571600
            },
            "mealplanIds": [
                78683289307158528
                    ],
            "storeIds": [65189733539708928],
            "cafeteriaIds": [

            ]
        }"#
        .to_string();

        let param = param::SendRequest {
            request_id: 10044,
            url: "".to_string(),
            // url: "http://127.0.0.1:60000".to_string(),
            req_type: ReqType::GRPC,
            headers: vec![],
            params: vec![],
            req_json: json,
            env_name: "Dev".to_string(),
        };
        let result = service::send_request(param);
        let result = result.unwrap();
        println!("result:{:?}", result)
    }

    #[test]
    fn test_list_next_request_log() {
        persistence::init_db().unwrap();
        let result = service::list_next_request_log(1, "".to_string(), 10);
        result.data.unwrap();
        let result = service::list_next_request_log(0, "".to_string(), 10);
        result.data.unwrap();
    }

    #[test]
    fn test_list_pre_request_log() {
        persistence::init_db().unwrap();
        let result = service::list_pre_request_log(1, "".to_string(), 10);
        let data = result.data.unwrap();
        let result = service::list_pre_request_log(data.first_id, "".to_string(), 10);
        result.data.unwrap();
    }

    #[test]
    fn test_create_env() {
        infra::persistence::init_db().unwrap();
        let param = param::CreateEnvironment {
            env_name: "test".to_string(),
            name: "base_url".to_string(),
            value: "127.0.0.1:25000".to_string(),
        };
        let result = service::create_env(param);
        println!("result: {:?}", result)
    }

    #[test]
    fn test_update_env() {
        infra::persistence::init_db().unwrap();
        let param = param::UpdateEnvironment {
            id: 10280,
            name: "base_url3".to_string(),
            value: "https://www.baidu.com123".to_string(),
        };
        let result = service::update_env_variable(param);
        println!("result:{:?}", result)
    }

    #[test]
    fn delete_variable() {
        infra::persistence::init_db().unwrap();
        let result = service::delete_env_variable(10280);
        println!("result:{:?}", result)
    }

    #[test]
    fn delete_env() {
        infra::persistence::init_db().unwrap();
        let result = service::delete_env("test".to_string());
        println!("result:{:?}", result)
    }

    #[test]
    fn list_env() {
        infra::persistence::init_db().unwrap();
        let result = service::list_env();
        println!("result:{:?}", result)
    }

    #[test]
    fn test_get_latest_request_log() {
        persistence::init_db().unwrap();
        let result = service::get_latest_request_log(10220);
        println!("result:{:?}", result);
    }

    #[test]
    fn test_put_config() {
        persistence::init_db().unwrap();
        let result = service::put_config("test".to_string(), "test".to_string());
        println!("result:{:?}", result);
        let result = service::put_config("test2".to_string(), "test2".to_string());
        println!("result:{:?}", result);
    }

    #[test]
    fn test_get_batch_config() {
        persistence::init_db().unwrap();
        let keys = vec!["test".to_string(), "test2".to_string()];
        let result = service::get_batch_config(keys);
        println!("result:{:?}", result)
    }

    #[test]
    fn test_delete_configs() {
        persistence::init_db().unwrap();
        let keys = vec!["10001".to_string()];
        let result = service::delete_configs(keys);
        println!("result:{:?}", result)
    }
}
