use agent::agent_client::AgentClient;
use agent::InitializeRequest;

pub mod agent {
    tonic::include_proto!("network.forta");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(InitializeRequest {
        agent_id: "0xagentid".to_owned(),
        proxy_host: "".to_owned(),
    });

    let response = client.initialize(request).await?;

    dbg!(response);

    Ok(())
}
