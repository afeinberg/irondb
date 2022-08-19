use anyhow::{Result, Error};
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};
use derive_more::Constructor;

use crate::memstore;

#[derive(Debug, Clone)]
pub struct Store {
    tx: mpsc::Sender<StoreRequest>,
}

#[derive(Debug)]
pub(crate) enum StoreRequest {
    Get(GetRequest),
}

#[derive(Debug)]
pub(crate) struct GetRequest {
    pub(crate) key: Vec<u8>,
    pub(crate) response: oneshot::Sender<Result<GetResponse>>,
}

#[derive(Constructor, Debug)]
pub struct GetResponse {
    value: Option<Bytes>,
}

impl Store {

    /// New memory store.
    pub fn memory_store() -> Self {
        let (tx, rx): (mpsc::Sender<StoreRequest>, mpsc::Receiver<StoreRequest>) = mpsc::channel(1);
        let _handle = std::thread::spawn(move || memstore::handle(rx));
        Self { tx }
    }

    /// Get `key`.
    pub async fn request_get(&self, key: Vec<u8>) -> Result<GetResponse> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(StoreRequest::Get(GetRequest { key, response: tx}))
            .await
            .map_err(Error::from)?;
        rx.await
           .map_err(Error::from)?
    }
}
