use hyper::client::HttpConnector;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;
use rustls::client::ServerCertVerified;
use rustls::client::ServerCertVerifier;
use rustls::Connection::Server;
use rustls::{Certificate, ClientConfig, Error, ServerName};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tonic::codegen::http;
use tonic::metadata::{MetadataKey, MetadataValue};
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{IntoRequest, Response};
use tower::ServiceBuilder;

use crate::common::error;
use crate::common::error::Result;
use crate::domain::proto;
use crate::domain::tonic::codec;

pub async fn request(
    uri: Uri,
    path: &str,
    metadata: &Vec<proto::entity::Entry>,
    body: Box<dyn MessageDyn>,
    resp_msg: Arc<proto::entity::Message>,
) -> Result<Response<serde_json::Value>> {
    let mut req = tonic::Request::new(body);
    for entry in metadata {
        req.metadata_mut().insert(
            MetadataKey::from_str(&entry.name)?,
            MetadataValue::try_from(&entry.value)?,
        );
    }

    let endpoint = tonic::transport::Endpoint::new(uri)?
        .connect_timeout(Duration::from_secs(60))
        .timeout(Duration::from_secs(60));
    let conn = endpoint.connect().await?;
    let mut inner = tonic::client::Grpc::new(conn);
    inner.ready().await.map_err(|e| {
        tonic::Status::new(
            tonic::Code::Internal,
            format!("Service was not ready: {}", e),
        )
    })?;

    let codec = codec::codec::JsonCodec::new(resp_msg);
    let path = http::uri::PathAndQuery::from_str(path)?;
    let resp: Response<serde_json::Value> = inner.unary(req, path, codec).await?;
    Ok(resp)
}

struct InsecureSkipVerification {}

impl ServerCertVerifier for InsecureSkipVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> std::result::Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }
}

pub async fn request_with_tls(
    uri: Uri,
    path: &str,
    metadata: &Vec<proto::entity::Entry>,
    body: Box<dyn MessageDyn>,
    resp_msg: Arc<proto::entity::Message>,
) -> Result<Response<serde_json::Value>> {
    let mut req = tonic::Request::new(body);
    for entry in metadata {
        req.metadata_mut().insert(
            MetadataKey::from_str(&entry.name)?,
            MetadataValue::try_from(&entry.value)?,
        );
    }

    let tls = ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(InsecureSkipVerification {}))
        .with_no_client_auth();

    let mut http = HttpConnector::new();
    http.set_connect_timeout(Some(Duration::from_secs(60)));
    http.enforce_http(false);

    let connector = tower::ServiceBuilder::new()
        .layer_fn(move |s| {
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_tls_config(tls.clone())
                .https_or_http()
                .enable_http2()
                .wrap_connector(s)
        })
        .service(http);

    let client = hyper::Client::builder().build(connector);
    let svc = ServiceBuilder::new()
        .map_request(move |mut req: http::Request<tonic::body::BoxBody>| {
            let uri = Uri::builder()
                .scheme(uri.scheme().unwrap().clone())
                .authority(uri.authority().unwrap().clone())
                .path_and_query(req.uri().path_and_query().unwrap().clone())
                .build()
                .unwrap();
            *req.uri_mut() = uri;
            req
        })
        .service(client);

    let mut inner = tonic::client::Grpc::new(svc);
    inner.ready().await.map_err(|e| {
        tonic::Status::new(
            tonic::Code::Internal,
            format!("Service was not ready: {}", e),
        )
    })?;

    let codec = codec::codec::JsonCodec::new(resp_msg);
    let path = http::uri::PathAndQuery::from_str(path)?;
    let resp: Response<serde_json::Value> = inner.unary(req, path, codec).await?;
    Ok(resp)
}
