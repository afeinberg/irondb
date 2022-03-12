use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

use crate::irondbproto::irondb_server::{Irondb, IrondbServer};
use crate::irondbproto::{
    AreYouOkayReply, AreYouOkayRequest, ClockEntry, GetReply, GetRequest, PutReply, PutRequest,
};
use irondb::store::{InMemoryStore, Store};
use irondb::version::{VectorClock, Versioned};

pub mod irondbproto {
    tonic::include_proto!("irondb");
}

#[derive(Debug)]
pub struct IrondbImpl<T: Store<Key = String, Value = String, Error = Box<dyn std::error::Error>>> {
    store: Arc<Mutex<T>>,
}

impl Default for IrondbImpl<InMemoryStore<String, String>> {
    fn default() -> Self {
        IrondbImpl {
            store: Arc::new(Mutex::new(InMemoryStore::new())),
        }
    }
}

#[tonic::async_trait]
impl<T: Store<Key = String, Value = String, Error = Box<dyn std::error::Error>>> Irondb
    for IrondbImpl<T>
where
    T: Sync + Send + 'static,
{
    async fn are_you_okay(
        &self,
        request: Request<AreYouOkayRequest>,
    ) -> Result<Response<AreYouOkayReply>, Status> {
        let reply = AreYouOkayReply {
            message: format!("AreYouOkay {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
        let lock_res = self.store.lock();
        let results: Vec<Versioned<String>> =
            lock_res.unwrap().get(request.into_inner().key).unwrap();
        let results = results
            .into_iter()
            .map(|ver_and_val| irondbproto::get_reply::Versioned {
                value: ver_and_val.value,
                version: Vec::new(),
            });
        let reply = GetReply {
            results: results.collect(),
        };
        Ok(Response::new(reply))
    }

    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutReply>, Status> {
        let lock_res = self.store.lock();
        let mut store = lock_res.unwrap();
        let inner = request.into_inner();
        store
            .put(
                inner.key.clone(),
                Versioned {
                    version: VectorClock::default(),
                    value: inner.value,
                },
            )
            .unwrap();
        let reply = PutReply {
            key: inner.key,
            previous: "".to_string(),
        };
        Ok(Response::new(reply))
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
