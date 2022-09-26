use std::net::SocketAddr;
use tokio::net::ToSocketAddrs;

use crate::{convert::UnitValue, Client, ClientOptions, KlickhouseError};

#[derive(Clone)]
pub struct ConnectionManager {
    destination: Vec<SocketAddr>,
    options: ClientOptions,
}

impl ConnectionManager {
    pub async fn new<A: ToSocketAddrs>(
        destination: A,
        options: ClientOptions,
    ) -> std::io::Result<Self> {
        Ok(Self {
            destination: tokio::net::lookup_host(destination).await?.collect(),
            options,
        })
    }
}

#[async_trait::async_trait]
impl bb8::ManageConnection for ConnectionManager {
    type Connection = Client;
    type Error = KlickhouseError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Client::connect(&self.destination[..], self.options.clone()).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _ = conn.query_one::<UnitValue<String>>("select '';").await?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}