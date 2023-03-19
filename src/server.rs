use tonic::{transport::Server, Request, Response, Status};

use agent::agent_server::{Agent, AgentServer};
use agent::{
    EvaluateAlertRequest, EvaluateAlertResponse, EvaluateBlockRequest, EvaluateBlockResponse,
    EvaluateTxRequest, EvaluateTxResponse, InitializeRequest, InitializeResponse,
};

pub mod agent {
    tonic::include_proto!("network.forta");
}

#[derive(Debug, Default)]
pub struct AgentService {}

#[tonic::async_trait]
impl Agent for AgentService {
    async fn initialize(
        &self,
        request: Request<InitializeRequest>,
    ) -> Result<Response<InitializeResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        dbg!(req);

        // FIXME remove hardcoded values
        let reply = InitializeResponse {
            status: 0,
            errors: vec![agent::Error {
                message: String::default(),
            }],
            addresses: vec![String::default()],
            alert_config: Some(agent::AlertConfig::default()),
        };

        Ok(Response::new(reply))
    }
    async fn evaluate_tx(
        &self,
        request: Request<EvaluateTxRequest>,
    ) -> Result<Response<EvaluateTxResponse>, Status> {
        unimplemented!()
    }
    async fn evaluate_block(
        &self,
        request: Request<EvaluateBlockRequest>,
    ) -> Result<Response<EvaluateBlockResponse>, Status> {
        unimplemented!()
    }
    async fn evaluate_alert(
        &self,
        request: Request<EvaluateAlertRequest>,
    ) -> Result<Response<EvaluateAlertResponse>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let agent_service = AgentService::default();

    Server::builder()
        .add_service(AgentServer::new(agent_service))
        .serve(addr)
        .await?;
    Ok(())
}
