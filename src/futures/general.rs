use crate::futures::model::*;
use crate::client::*;
use crate::errors::*;
use crate::api::API;
use crate::api::Futures;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

#[cfg(feature = "blocking")]
impl FuturesGeneral {
    // Test connectivity
    pub fn ping(&self) -> Result<String> {
        self.client.get(API::Futures(Futures::Ping), None)?;
        Ok("pong".into())
    }

    // Check server time
    pub fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Futures(Futures::Time), None)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get(API::Futures(Futures::ExchangeInfo), None)
    }

    // Get Symbol information
    pub fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info() {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(not(feature = "blocking"))]
impl FuturesGeneral {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get(API::Futures(Futures::Ping), None).await?;
        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get(API::Futures(Futures::Time), None).await
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client
            .get(API::Futures(Futures::ExchangeInfo), None)
            .await
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }
}
