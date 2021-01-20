mod hello;

fn main() {}

#[cfg(test)]
mod test {
    use crate::hello::say_client::SayClient;
    use crate::hello::SayRequest;
    use futures::stream::iter;
    use tonic::Request;

    #[tokio::main]
    #[test]
    async fn test_send() -> Result<(), Box<dyn std::error::Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await?;

        let mut client = SayClient::new(channel);

        // Token-based authentication
        // let token = "token";
        // let mut client = SayClient::with_interceptor(channel, move |mut req: Request<()>| {
        //     req.metadata_mut().insert(
        //         "authorization",
        //         tonic::metadata::MetadataValue::from_str(token).unwrap(),
        //     );
        //     Ok(req)
        // });

        let request = tonic::Request::new(SayRequest {
            name: String::from("jack"),
        });

        let response = client.send(request).await?.into_inner();
        println!("RESPONSE SEND={:?}", response);

        Ok(())
    }

    #[tokio::main]
    #[test]
    async fn test_send_stream() -> Result<(), Box<dyn std::error::Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await?;

        let mut client = SayClient::new(channel);

        let request = tonic::Request::new(SayRequest {
            name: String::from("jack"),
        });

        let mut response = client.send_stream(request).await?.into_inner();
        while let Some(res) = response.message().await? {
            println!("RESPONSE SEND STREAM={:?}", res);
        }

        Ok(())
    }

    #[tokio::main]
    #[test]
    async fn test_receive_stream() -> Result<(), Box<dyn std::error::Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await?;

        let mut client = SayClient::new(channel);

        // creating a stream
        // iter Converts an `Iterator` into a `Stream`
        let request = tonic::Request::new(iter(vec![
            SayRequest {
                name: String::from("jack"),
            },
            SayRequest {
                name: String::from("tom"),
            },
            SayRequest {
                name: String::from("jerry"),
            },
        ]));

        // sending stream
        let response = client.receive_stream(request).await?.into_inner();
        println!("RESPONSE RECEIVE STREAM={:?}", response.message);

        Ok(())
    }

    #[tokio::main]
    #[test]
    async fn test_bidirectional() -> Result<(), Box<dyn std::error::Error>> {
        let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
            .connect()
            .await?;

        let mut client = SayClient::new(channel);

        // creating a stream
        // iter Converts an `Iterator` into a `Stream`
        let request = tonic::Request::new(iter(vec![
            SayRequest {
                name: String::from("jack"),
            },
            SayRequest {
                name: String::from("tom"),
            },
            SayRequest {
                name: String::from("jerry"),
            },
        ]));

        // sending stream
        // listening on the response stream
        let mut response = client.bidirectional(request).await?.into_inner();
        while let Some(res) = response.message().await? {
            println!("RESPONSE bidirectional={:?}", res);
        }

        Ok(())
    }
}
