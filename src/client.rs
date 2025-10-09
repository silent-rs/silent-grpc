use tokio_stream::iter;
use tonic::Status;
use tonic::transport::{Channel, Error as TransportError};

use crate::proto::helloworld::{
    HelloReply, HelloRequest, HelloSummary, greeter_client::GreeterClient,
};

#[allow(dead_code)]
pub async fn unary_example() -> Result<HelloReply, Status> {
    let mut client = build_client()
        .await
        .map_err(|err| Status::unavailable(err.to_string()))?;
    let response = client
        .say_hello(HelloRequest {
            name: "Silent".to_owned(),
        })
        .await?;
    Ok(response.into_inner())
}

#[allow(dead_code)]
pub async fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = build_client().await?;

    let response = client
        .say_hello(HelloRequest {
            name: "Silent".to_owned(),
        })
        .await?;
    println!("Unary: {:?}", response.into_inner());

    let mut stream = client
        .lots_of_replies(HelloRequest {
            name: "Streaming".to_owned(),
        })
        .await?
        .into_inner();
    while let Some(reply) = stream.message().await? {
        println!("Server stream item: {:?}", reply);
    }

    let client_stream = iter([
        HelloRequest {
            name: "Alice".to_owned(),
        },
        HelloRequest {
            name: "Bob".to_owned(),
        },
        HelloRequest {
            name: "Charlie".to_owned(),
        },
    ]);
    let summary = client.lots_of_greetings(client_stream).await?.into_inner();
    println!("Client stream summary: {:?}", summary);

    let outbound = iter([
        HelloRequest {
            name: "Bidirectional".to_owned(),
        },
        HelloRequest {
            name: "Streaming".to_owned(),
        },
    ]);
    let mut bidi = client.bidi_hello(outbound).await?.into_inner();
    while let Some(reply) = bidi.message().await? {
        println!("Bidi reply: {:?}", reply);
    }

    Ok(())
}

async fn build_client() -> Result<GreeterClient<Channel>, TransportError> {
    GreeterClient::connect("http://127.0.0.1:50051").await
}

#[allow(dead_code)]
pub async fn aggregate_names() -> Result<HelloSummary, Status> {
    let mut client = build_client()
        .await
        .map_err(|err| Status::unavailable(err.to_string()))?;
    let names = iter([
        HelloRequest {
            name: "Dave".to_owned(),
        },
        HelloRequest {
            name: "Eve".to_owned(),
        },
    ]);
    let response = client.lots_of_greetings(names).await?;
    Ok(response.into_inner())
}
