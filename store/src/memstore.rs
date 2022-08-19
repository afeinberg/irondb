use std::collections::BTreeMap;

use bytes::Bytes;
use tokio::sync::mpsc;

use crate::{
    store::{StoreRequest, GetResponse}
};

pub(crate) fn handle(mut rx: mpsc::Receiver<StoreRequest>) {
    let mut storage = BTreeMap::<Vec<u8>, Bytes>::new();

    while let Some(req) = rx.blocking_recv() {
        match req {
            StoreRequest::Get(req) => {
                let value = storage.get(&req.key).map(Bytes::clone);
                let _ = req.response.send(Ok(GetResponse::new(value)));
            }
        }
    }
}
