
use agent::{
    AlertEvent, BlockEvent, Finding, 
    InitializeResponse, TransactionEvent,
};
// use agent
use std::future::Future;

pub mod agent {
    tonic::include_proto!("network.forta");
}

#[derive(Debug)]
pub(crate) struct GetAgentHandlersOptions {
    should_run_initialize: Option<bool>,
}
pub(crate) type GetAgentHandlers =
    fn(options: Option<GetAgentHandlersOptions>) -> Box<dyn Future<Output = AgentHandlers> + 'static>;
pub(crate) type Initialize = fn() -> Box<dyn Future<Output = Option<InitializeResponse>> + 'static>;
pub(crate) type HandleBlock = fn(blockEvent: BlockEvent) -> Box<dyn Future<Output = Vec<Finding>> + 'static>;
pub(crate) type HandleAlert = fn(alertEvent: AlertEvent) -> Box<dyn Future<Output = Vec<Finding>> + 'static>;
pub(crate) type HandleTransaction = fn(txEvent: TransactionEvent) -> Box<dyn Future<Output = Vec<Finding>> + 'static>;

#[derive(Debug)]
pub(crate) struct AgentHandlers {
    initialize: Initialize,
    initialize_response: InitializeResponse,
    handle_transaction: HandleTransaction,
    handle_block: HandleBlock,
    handle_alert: HandleAlert,
}