use tonic::{transport::Server, Request, Response, Status};

use protos::irondb_server::{Irondb, IrondbServer};
use protos::{AreYouOkayReply, AreYouOkayRequest};

#[derive(Debug, Default)]
pub struct MyIrondb {}

#[tonic::async_trait]
impl Irondb for MyIrondb {
    async fn are_you_okay(
        &self,
        request: Request<AreYouOkayRequest>,
    ) -> Result<Response<AreYouOkayReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = protos::AreYouOkayReply {
            message: format!("AreYouOkay {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyIrondb::default();

    Server::builder()
        .add_service(IrondbServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
