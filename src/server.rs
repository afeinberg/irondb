use tonic::{Request, Response, Status, transport::Server};

use crate::irondb::{AreYouOkayReply, AreYouOkayRequest};
use crate::irondb::irondb_server::{Irondb, IrondbServer};

pub mod irondb {
    tonic::include_proto!("irondb");
}

#[derive(Debug, Default)]
pub struct MyIrondb {}

#[tonic::async_trait]
impl Irondb for MyIrondb {
    async fn are_you_okay(
        &self,
        request: Request<AreYouOkayRequest>,
    ) -> Result<Response<AreYouOkayReply>, Status> {
        let reply = irondb::AreYouOkayReply {
            message: format!("AreYouOkay {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetReply>, Status> {
        todo!()
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
