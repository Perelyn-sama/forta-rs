use forta_rs::AgentClient;
use forta_rs::{InitializeRequest, ResponseStatus};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentClient::connect("http://[::1]:50051").await?;

    let initiaze_request = tonic::Request::new(InitializeRequest {
        agent_id: "0xagentid".to_owned(),
        proxy_host: "".to_owned(),
    });
    {

        // let event = TransactionEvent::default();
        // let event1 = TransactionEvent {
        //     r#type: EventType::Block as i32,
        //     transaction: None,
        //     receipt: None,
        //     network: None,
        //     traces: vec![],
        //     addresses: HashMap::default(),
        //     block: None,
        //     logs: vec![],
        //     is_contract_deployment: false,
        //     contract_address: String::default(),
        //     timestamps: None,
        //     tx_addresses: HashMap::default(),
        // };

        // let evaluatetx_request = tonic::Request::new(EvaluateTxRequest {
        //     request_id: String::default(),
        //     event: Some(event),
        // });
        //
    }

    let initialize_response = client.initialize(initiaze_request).await?;

    dbg!(initialize_response);

    Ok(())
}

#[tokio::test]
async fn test_initialize() {
    let mut client = AgentClient::connect("http://[::1]:50051").await.unwrap();

    let initiaze_request = tonic::Request::new(InitializeRequest {
        agent_id: "0xagentid".to_owned(),
        proxy_host: "".to_owned(),
    });

    let initialize_response = client.initialize(initiaze_request).await.unwrap();

    assert_eq!(
        initialize_response.into_inner().status(),
        ResponseStatus::Success
    );
}
