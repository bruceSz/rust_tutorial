use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let resp = client.say_hello(request).await?;
    println!("Hello resp from={:?}", resp);
    Ok(())
}