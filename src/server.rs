use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status, transport::Server};

use crate::irondbproto::{AreYouOkayReply, AreYouOkayRequest, GetReply, GetRequest, PutReply, PutRequest, ClockEntry};
use crate::irondbproto::irondb_server::{Irondb, IrondbServer};
use irondb::version::Versioned;
use irondb::store::{Store, InMemoryStore};

pub mod irondbproto {
    tonic::include_proto!("irondb");
}

#[derive(Debug)]
pub struct IrondbImpl {
   store: Arc<Mutex<InMemoryStore<String, String>>>
}

impl Default for IrondbImpl {
    fn default() -> Self {
        IrondbImpl {
            store: Arc::new(Mutex::new(InMemoryStore::new())),
        }
    }
}

#[tonic::async_trait]
impl Irondb for IrondbImpl {
    async fn are_you_okay(
        &self,
        request: Request<AreYouOkayRequest>,
    ) -> Result<Response<AreYouOkayReply>, Status> {
        let reply = AreYouOkayReply {
            message: format!("AreYouOkay {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetReply>, Status> {
        let lock_res = self.store.lock();
        let results: Vec<Versioned<String>> = lock_res.unwrap().get(request.into_inner().key).unwrap();
        let results = results.into_iter().map(|ver_and_val| {
            irondbproto::get_reply::Versioned {
                value: ver_and_val.value,
                version: Vec::new(),
            }
        });
        let reply = GetReply {results: results.collect() };
        Ok(Response::new(reply))
    }

    fn put< 'life0, 'async_trait>(& 'life0 self,request:tonic::Request<PutRequest> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<tonic::Response<PutReply> ,tonic::Status> > + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let irondb = IrondbImpl::default();

    Server::builder()
        .add_service(IrondbServer::new(irondb))
        .serve(addr)
        .await?;

    Ok(())
}
