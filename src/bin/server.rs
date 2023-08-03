use forta_rs::agent::InitializeResponse;
use forta_rs::types::GetAgentHandlers;
use forta_rs::{
    Agent, AgentServer, AlertConfig, CombinerBotSubscription, Error, EvaluateAlertRequest,
    EvaluateAlertResponse, EvaluateBlockRequest, EvaluateBlockResponse, EvaluateTxRequest,
    EvaluateTxResponse, InitializeRequest, ResponseStatus,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct AgentService {
    get_agent_handlers: Option<Arc<Mutex<GetAgentHandlers>>>,
    is_initialized: Arc<Mutex<bool>>,
    initialize_response: Option<Arc<Mutex<InitializeResponse>>>,
}

// constructor(getAgentHandlers) {
//   assertExists(getAgentHandlers, "getAgentHandlers");
//   this.getAgentHandlers = getAgentHandlers;
//   this.initializeAgentHandlers();
//   this.isInitialized = false; // makes sure agent initialize handler only called once
//   this.initializeResponse = {};
// }

impl AgentService {
    async fn new(mut get_agent_handlers: Option<Arc<Mutex<GetAgentHandlers>>>) -> Self {
        assert!(
            get_agent_handlers.is_some(),
            "get_agent_handlers must exist"
        );

        // let mutex = get_agent_handlers.as_mut().unwrap().lock();
        // let

        let mut agent = AgentService {
            get_agent_handlers,
            is_initialized: Arc::new(Mutex::new(false)),
            initialize_response: Some(Arc::new(Mutex::new(InitializeResponse::default()))),
        };

        agent
    }

    // async fn get_agent_handler(&mut self) {
    //     // let () = self.get_agent_handler().await;
    //     let () = Box::pin(self.get_agent_handler()).await;
    // }
}
impl Default for AgentService {
    fn default() -> Self {
        AgentService {
            is_initialized: Arc::new(Mutex::new(false)),
            get_agent_handlers: None,
            initialize_response: None,
        }
    }
}
#[tonic::async_trait]
impl Agent for AgentService {
    async fn initialize(
        &self,
        request: Request<InitializeRequest>,
    ) -> Result<Response<InitializeResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut is_initialized = self.is_initialized.lock().await;
        *is_initialized = true;

        let req = request.into_inner();

        let reply = InitializeResponse {
            status: ResponseStatus::Success as i32,
            errors: vec![Error {
                message: String::default(),
            }],
            addresses: vec![String::default()],
            alert_config: Some(AlertConfig {
                subscriptions: vec![CombinerBotSubscription {
                    bot_id: req.agent_id,
                    alert_id: "".to_owned(),
                    alert_ids: vec!["".to_owned()],
                    chain_id: 1,
                }],
            }),
        };

        Ok(Response::new(reply))
    }
    async fn evaluate_tx(
        &self,
        request: Request<EvaluateTxRequest>,
    ) -> Result<Response<EvaluateTxResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        dbg!(req);

        let reply = EvaluateTxResponse {
            status: ResponseStatus::Success as i32,
            errors: vec![Error {
                message: String::default(),
            }],
            findings: vec![],
            metadata: HashMap::default(),
            timestamp: String::new(),
            latency_ms: u32::default(),
            private: false,
        };

        Ok(Response::new(reply))
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
