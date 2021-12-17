use irondb::irondb_client::IrondbClient;
use irondb::AreYouOkayRequest;

pub mod irondb {
    tonic::include_proto!("irondb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = IrondbClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AreYouOkayRequest {
        name: "Tonic".into(),
    });

    let response = client.are_you_okay(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
