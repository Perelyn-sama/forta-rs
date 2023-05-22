pub mod types;
use std::future::Future;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use types::{
    agent::{
        self,
        agent_server::{Agent, AgentServer},
        EvaluateAlertRequest, EvaluateAlertResponse, EvaluateBlockRequest, EvaluateBlockResponse,
        EvaluateTxRequest, EvaluateTxResponse, InitializeRequest, InitializeResponse,
        ResponseStatus,
    },
    AgentHandlers, GetAgentHandlers, GetAgentHandlersOptions,
};

#[derive(Debug)]
pub struct AgentService {
    get_agent_handlers: Option<Arc<Mutex<GetAgentHandlers>>>,
    is_initialized: Arc<Mutex<bool>>,
    initialize_response: Option<Arc<Mutex<InitializeResponse>>>,
}
impl AgentService {
    fn new(get_agent_handlers: Option<Arc<Mutex<GetAgentHandlers>>>) -> Self {
        assert!(
            get_agent_handlers.is_some(),
            "get_agent_handlers must exist"
        );
        let mut agent = AgentService {
            get_agent_handlers,
            is_initialized: Arc::new(Mutex::new(false)),
            initialize_response: Some(Arc::new(Mutex::new(InitializeResponse::default()))),
        };

        Self::initialize_agent_handlers(&mut agent);

        agent
    }

    fn initialize_agent_handlers(&mut self) {
        let options = GetAgentHandlersOptions {
            should_run_initialize: Some(false),
        };
        let agent_handlers = Self::get_agent_handlers(Some(options));
        unimplemented!()
    }

    // Implement the function with the specified signature
    fn get_agent_handlers(
        options: Option<GetAgentHandlersOptions>,
    ) -> Box<dyn Future<Output = AgentHandlers> + 'static> {
        // Function implementation goes here
        // You can create and return a future that resolves to an AgentHandlers instance
        // For simplicity, we'll just return an empty Boxed future here
        // Box::pin(async move { AgentHandlers })
        unimplemented!()
    }
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
            errors: vec![agent::Error {
                message: String::default(),
            }],
            addresses: vec![String::default()],
            alert_config: Some(agent::AlertConfig {
                subscriptions: vec![agent::CombinerBotSubscription {
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
            errors: vec![agent::Error {
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
    // let agent_service = AgentService::default();
    let agent_service = AgentService::default();

    Server::builder()
        .add_service(AgentServer::new(agent_service))
        .serve(addr)
        .await?;
    Ok(())
}
