#[derive(Debug, Clone)]
pub enum ReqType {
    HTTP,
    GRPC,
}

impl From<&String> for ReqType {
    fn from(s: &String) -> Self {
        let ref_s: &str = s;
        match ref_s {
            "GRPC" => ReqType::GRPC,
            "HTTP" => ReqType::HTTP,
            _ => ReqType::HTTP,
        }
    }
}

impl From<String> for ReqType {
    fn from(s: String) -> Self {
        let ref_s: &str = &s;
        match ref_s {
            "GRPC" => ReqType::GRPC,
            "HTTP" => ReqType::HTTP,
            _ => ReqType::HTTP,
        }
    }
}

impl From<&ReqType> for String {
    fn from(r: &ReqType) -> Self {
        match r {
            ReqType::GRPC => "GRPC".to_string(),
            ReqType::HTTP => "HTTP".to_string(),
        }
    }
}
