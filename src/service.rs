use std::{pin::Pin, time::Duration};

use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request as TonicRequest, Response, Status, Streaming};

use crate::{
    handler::{
        DEFAULT_STREAM_COUNT, STREAM_INTERVAL_MS, SummaryRecord, create_greeting, create_summary,
        format_timestamp, validate_name,
    },
    proto::helloworld::{HelloReply, HelloRequest, HelloSummary, greeter_server::Greeter},
};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send>>;

#[derive(Debug, Clone)]
pub struct GreeterService;

impl GreeterService {
    pub const fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: TonicRequest<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let payload = request.into_inner();
        let name = validate_name(&payload.name).map_err(Status::invalid_argument)?;

        let greeting = create_greeting(&name);
        Ok(Response::new(HelloReply {
            id: greeting.id,
            message: greeting.message,
            created_at: format_timestamp(greeting.created_at),
        }))
    }

    type LotsOfRepliesStream = ResponseStream;

    async fn lots_of_replies(
        &self,
        request: TonicRequest<HelloRequest>,
    ) -> Result<Response<Self::LotsOfRepliesStream>, Status> {
        let payload = request.into_inner();
        let name = validate_name(&payload.name).map_err(Status::invalid_argument)?;

        let (tx, rx) = mpsc::channel::<Result<HelloReply, Status>>(DEFAULT_STREAM_COUNT);
        tokio::spawn(async move {
            for _ in 0..DEFAULT_STREAM_COUNT {
                let greeting = create_greeting(&name);
                let reply = HelloReply {
                    id: greeting.id,
                    message: greeting.message,
                    created_at: format_timestamp(greeting.created_at),
                };
                if tx.send(Ok(reply)).await.is_err() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(STREAM_INTERVAL_MS)).await;
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as ResponseStream
        ))
    }

    async fn lots_of_greetings(
        &self,
        request: TonicRequest<Streaming<HelloRequest>>,
    ) -> Result<Response<HelloSummary>, Status> {
        let mut stream = request.into_inner();
        let mut collected = Vec::new();

        while let Some(message) = stream.next().await {
            let message = message?;
            let name = validate_name(&message.name).map_err(Status::invalid_argument)?;
            collected.push(name);
        }

        if collected.is_empty() {
            return Err(Status::invalid_argument("至少需要一个有效的名称"));
        }

        let summary = create_summary(collected);
        let SummaryRecord {
            id,
            total,
            names,
            created_at,
        } = summary;
        Ok(Response::new(HelloSummary {
            id,
            total,
            names,
            created_at: format_timestamp(created_at),
        }))
    }

    type BidiHelloStream = ResponseStream;

    async fn bidi_hello(
        &self,
        request: TonicRequest<Streaming<HelloRequest>>,
    ) -> Result<Response<Self::BidiHelloStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = mpsc::channel::<Result<HelloReply, Status>>(DEFAULT_STREAM_COUNT);

        tokio::spawn(async move {
            while let Some(message) = inbound.next().await {
                match message {
                    Ok(msg) => match validate_name(&msg.name) {
                        Ok(name) => {
                            let greeting = create_greeting(&name);
                            let reply = HelloReply {
                                id: greeting.id,
                                message: greeting.message,
                                created_at: format_timestamp(greeting.created_at),
                            };
                            if tx.send(Ok(reply)).await.is_err() {
                                break;
                            }
                        }
                        Err(err_msg) => {
                            if tx
                                .send(Err(Status::invalid_argument(err_msg)))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    },
                    Err(status) => {
                        if tx.send(Err(status)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as ResponseStream
        ))
    }
}
