use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio_util::codec::{BytesCodec, Decoder};
use std::error::Error;
use serde::{Serialize, Deserialize};
use bincode;
use futures::sink::SinkExt;
use futures::StreamExt;
use bytes::Bytes;
use crate::http_frame::{Body, Header, HttpFrame};

mod http_frame;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub ticker: String,
    pub amount: f32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream);

    let message = HttpFrame {
        header: Header { method: "POST".to_string(), uri: "www.my_test_stock_app.com/stock/purchase".to_string() },
        body: Body { ticker: "BYND".to_string(), amount: 3.2 },
    };

    let message_bin = bincode::serialize(&message).unwrap();
    let sending_message = Bytes::from(message_bin);
    framed.send(sending_message).await.unwrap();

    let message = framed.next().await.unwrap().unwrap();
    let message = bincode::deserialize::<HttpFrame>(&message).unwrap();
    println!("{:?}", message);

    Ok(())
}
