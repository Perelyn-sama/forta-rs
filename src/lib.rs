pub mod agent {
    tonic::include_proto!("network.forta");
}
pub mod types;

pub use agent::agent_client::AgentClient;
pub use agent::agent_server::{Agent, AgentServer};
pub use agent::{
    AlertConfig, CombinerBotSubscription, Error, EvaluateAlertRequest, EvaluateAlertResponse,
    EvaluateBlockRequest, EvaluateBlockResponse, EvaluateTxRequest, EvaluateTxResponse,
    InitializeRequest, ResponseStatus,
};
