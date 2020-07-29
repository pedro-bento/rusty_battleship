use bytes::Bytes;
use mini_redis::{client, Result};
// use std::io;

pub struct Chat {
    client: client::Client,
    receive_channel: client::Subscriber,
    send_channel_key: String,
}

impl Chat {
    pub async fn new(
        addr: String,
        receive_channel_key: String,
        send_channel_key: String,
    ) -> Result<Chat> {
        let client = client::connect(addr.as_str()).await?;
        let subscriber = client.subscribe(vec![receive_channel_key.into()]).await?;

        Ok(Chat {
            client: client::connect(addr.as_str()).await?,
            receive_channel: subscriber,
            send_channel_key: send_channel_key,
        })
    }

    pub async fn receive(&mut self) -> Result<Option<client::Message>> {
        Ok(self.receive_channel.next_message().await?)
    }

    pub async fn send(&mut self, msg: Bytes) -> Result<()> {
        self.client
            .publish(self.send_channel_key.as_str(), msg)
            .await?;
        Ok(())
    }
}
