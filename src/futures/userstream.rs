use crate::model::*;
use crate::client::*;
use crate::errors::*;
use crate::api::API;
use crate::api::Futures;

#[derive(Clone)]
pub struct FuturesUserStream {
    pub client: Client,
    pub recv_window: u64,
}

#[cfg(feature = "blocking")]
impl FuturesUserStream {
    // User Stream
    pub fn start(&self) -> Result<UserDataStream> {
        self.client.post(API::Futures(Futures::UserDataStream))
    }

    // Current open orders on a symbol
    pub fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(API::Futures(Futures::UserDataStream), listen_key)
    }

    pub fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Futures(Futures::UserDataStream), listen_key)
    }
}

#[cfg(not(feature = "blocking"))]
impl FuturesUserStream {
    // User Stream
    pub async fn start(&self) -> Result<UserDataStream> {
        self.client.post(API::Futures(Futures::UserDataStream)).await
    }

    // Current open orders on a symbol
    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client
            .put(API::Futures(Futures::UserDataStream), listen_key)
            .await
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Futures(Futures::UserDataStream), listen_key)
            .await
    }
}