use std::sync::{Arc, Mutex};

use env_logger::Env;
use log::info;
use tonic::{Request, Response, Status, transport::Server};

use irondb::core::ClusterConfig;
use irondb::store::{InMemoryStore, Store};
use irondb::version::{VectorClock, Versioned};

use crate::irondbproto::{
    AreYouOkayReply, AreYouOkayRequest, GetReply, GetRequest, PutReply, PutRequest,
};
use crate::irondbproto::irondb_server::{Irondb, IrondbServer};

pub mod irondbproto {
    tonic::include_proto!("irondb");
}

#[derive(Debug)]
pub struct IrondbImpl<T: Store<Key=String, Value=String, Error=Box<dyn std::error::Error>>> {
    store: Arc<Mutex<T>>,
}

impl Default for IrondbImpl<InMemoryStore<String, String>> {
    fn default() -> Self {
        info!("Using InMemoryStore");

        IrondbImpl {
            store: Arc::new(Mutex::new(InMemoryStore::new())),
        }
    }
}

#[tonic::async_trait]
impl<T: Store<Key=String, Value=String, Error=Box<dyn std::error::Error>>> Irondb
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
            lock_res.unwrap().get(request.into_inner().key).unwrap_or_default();
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
        let old_values = store
            .put(
                inner.key.clone(),
                Versioned {
                    version: VectorClock::default(),
                    value: inner.value,
                },
            )
            .map_or_else(|_| Vec::new(), |v| v);

        let reply = PutReply {
            key: inner.key,
            previous: old_values
                .first()
                .map_or_else(|| "".to_string(), |ver_and_val| ver_and_val.value.clone()),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let addr = ClusterConfig::from("Cluster")
        .dflt_server_hostport()
        .parse()?;
    let irondb = IrondbImpl::default();

    Server::builder()
        .add_service(IrondbServer::new(irondb))
        .serve(addr)
        .await?;

    Ok(())
}
