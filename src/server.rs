mod hello;

use hello::say_server::{Say, SayServer};
use hello::{SayRequest, SayResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default)]
struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        let SayRequest { name } = request.into_inner();

        Ok(Response::new(SayResponse {
            message: format!("hello {}", name),
        }))
    }

    // specify the output of rpc call
    type SendStreamStream = mpsc::Receiver<Result<SayResponse, Status>>;

    async fn send_stream(
        &self,
        request: Request<SayRequest>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        let SayRequest { name } = request.into_inner();

        let (mut tx, rx) = mpsc::channel(4);

        // creating a new task
        tokio::spawn(async move {
            // looping and sending our response using stream
            for _ in 0..4 {
                // sending response to our channel
                if let Err(e) = tx
                    .send(Ok(SayResponse {
                        message: format!("hello {}", name),
                    }))
                    .await
                {
                    println!("{}", e);
                };
            }
        });

        // returning our receiver so that tonic can listen on receiver and send the response to client
        Ok(Response::new(rx))
    }

    async fn receive_stream(
        &self,
        request: Request<Streaming<SayRequest>>,
    ) -> Result<Response<SayResponse>, Status> {
        let mut stream = request.into_inner();
        let mut message = String::new();

        while let Some(req) = stream.message().await? {
            message.push_str(&format!("Hello {}\n", req.name))
        }

        Ok(Response::new(SayResponse { message }))
    }

    //defining return stream
    type BidirectionalStream = mpsc::Receiver<Result<SayResponse, Status>>;

    async fn bidirectional(
        &self,
        request: Request<Streaming<SayRequest>>,
    ) -> Result<Response<Self::BidirectionalStream>, Status> {
        // converting request in stream
        let mut stream = request.into_inner();

        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(req) = stream.message().await.unwrap() {
                // sending data as soon it is available
                if let Err(e) = tx
                    .send(Ok(SayResponse {
                        message: format!("hello {}", req.name),
                    }))
                    .await
                {
                    println!("{}", e);
                };
            }
        });

        Ok(Response::new(rx))
    }
}

fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(token) => token.to_str(),
        None => return Err(Status::unauthenticated("Token not found")),
    };

    // do some validation with token here ...
    println!("TOKEN: {:?}", token);

    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let say = MySay::default();

    println!("Server listening on {}", addr);

    let svc = SayServer::new(say);

    // Token-based authentication
    // let svc = SayServer::with_interceptor(say, interceptor);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
