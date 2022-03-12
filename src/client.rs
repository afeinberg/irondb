use ::irondb::core::ClusterConfig;

use crate::irondb::{GetRequest, PutRequest};
use crate::irondb::irondb_client::IrondbClient;

pub(crate) mod irondb {
    tonic::include_proto!("irondb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = ClusterConfig::from("Cluster").dflt_server_hostport();
    let mut client = IrondbClient::connect(format!("http://{}/", addr)).await?;

    let put_request = tonic::Request::new(PutRequest {
        key: "harry".to_string(),
        value: "potter".to_string(),
        version: vec![],
    });
    let get_request = tonic::Request::new(GetRequest {
        key: "harry".into(),
    });

    let response = client.get(get_request).await?;

    println!(
        "put request({:?}), get response({:?})",
        put_request, response
    );

    Ok(())
}
