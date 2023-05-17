#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingTimestamps {
    #[prost(string, tag = "1")]
    pub block: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub feed: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bot_request: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bot_response: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub source_alert: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentInfo {
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub image_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_test: bool,
    #[prost(string, tag = "5")]
    pub manifest: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScannerInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertResponse {
    #[prost(message, repeated, tag = "1")]
    pub alerts: ::prost::alloc::vec::Vec<SignedAlert>,
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(string, tag = "1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub algorithm: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BloomFilter {
    #[prost(uint64, tag = "1")]
    pub k: u64,
    #[prost(uint64, tag = "2")]
    pub m: u64,
    #[prost(string, tag = "3")]
    pub bitset: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "AlertType", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub finding: ::core::option::Option<Finding>,
    #[prost(string, tag = "4")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "5")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "6")]
    pub agent: ::core::option::Option<AgentInfo>,
    #[prost(map = "string, string", tag = "7")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "8")]
    pub scanner: ::core::option::Option<ScannerInfo>,
    #[prost(message, optional, tag = "9")]
    pub timestamps: ::core::option::Option<TrackingTimestamps>,
    #[prost(bool, tag = "10")]
    pub truncated: bool,
    #[prost(message, optional, tag = "11")]
    pub address_bloom_filter: ::core::option::Option<BloomFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedAlert {
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<Alert>,
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<Signature>,
    #[prost(string, tag = "3")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub published_with_tx: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    #[prost(enumeration = "label::EntityType", tag = "1")]
    pub entity_type: i32,
    #[prost(string, tag = "2")]
    pub entity: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub confidence: f32,
    #[prost(bool, tag = "6")]
    pub remove: bool,
    #[prost(string, tag = "7")]
    pub label: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Label`.
pub mod label {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EntityType {
        UnknownEntityType = 0,
        Address = 1,
        Transaction = 2,
        Block = 3,
        Url = 4,
    }
    impl EntityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EntityType::UnknownEntityType => "UNKNOWN_ENTITY_TYPE",
                EntityType::Address => "ADDRESS",
                EntityType::Transaction => "TRANSACTION",
                EntityType::Block => "BLOCK",
                EntityType::Url => "URL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_ENTITY_TYPE" => Some(Self::UnknownEntityType),
                "ADDRESS" => Some(Self::Address),
                "TRANSACTION" => Some(Self::Transaction),
                "BLOCK" => Some(Self::Block),
                "URL" => Some(Self::Url),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    #[prost(string, tag = "1")]
    pub protocol: ::prost::alloc::string::String,
    #[prost(enumeration = "finding::Severity", tag = "2")]
    pub severity: i32,
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(enumeration = "finding::FindingType", tag = "4")]
    pub r#type: i32,
    #[prost(string, tag = "5")]
    pub alert_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub everest_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub private: bool,
    #[prost(string, repeated, tag = "10")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, double", tag = "11")]
    pub indicators: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(message, repeated, tag = "12")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(string, repeated, tag = "13")]
    pub related_alerts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Severity {
        Unknown = 0,
        Info = 1,
        Low = 2,
        Medium = 3,
        High = 4,
        Critical = 5,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unknown => "UNKNOWN",
                Severity::Info => "INFO",
                Severity::Low => "LOW",
                Severity::Medium => "MEDIUM",
                Severity::High => "HIGH",
                Severity::Critical => "CRITICAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "INFO" => Some(Self::Info),
                "LOW" => Some(Self::Low),
                "MEDIUM" => Some(Self::Medium),
                "HIGH" => Some(Self::High),
                "CRITICAL" => Some(Self::Critical),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FindingType {
        UnknownType = 0,
        Exploit = 1,
        Suspicious = 2,
        Degraded = 3,
        Information = 4,
    }
    impl FindingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FindingType::UnknownType => "UNKNOWN_TYPE",
                FindingType::Exploit => "EXPLOIT",
                FindingType::Suspicious => "SUSPICIOUS",
                FindingType::Degraded => "DEGRADED",
                FindingType::Information => "INFORMATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN_TYPE" => Some(Self::UnknownType),
                "EXPLOIT" => Some(Self::Exploit),
                "SUSPICIOUS" => Some(Self::Suspicious),
                "DEGRADED" => Some(Self::Degraded),
                "INFORMATION" => Some(Self::Information),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlertType {
    UnknownAlertType = 0,
    Transaction = 1,
    Block = 2,
    Private = 3,
    Combination = 4,
}
impl AlertType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AlertType::UnknownAlertType => "UNKNOWN_ALERT_TYPE",
            AlertType::Transaction => "TRANSACTION",
            AlertType::Block => "BLOCK",
            AlertType::Private => "PRIVATE",
            AlertType::Combination => "COMBINATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_ALERT_TYPE" => Some(Self::UnknownAlertType),
            "TRANSACTION" => Some(Self::Transaction),
            "BLOCK" => Some(Self::Block),
            "PRIVATE" => Some(Self::Private),
            "COMBINATION" => Some(Self::Combination),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRequest {
    #[prost(string, tag = "1")]
    pub agent_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub proxy_host: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeResponse {
    #[prost(enumeration = "ResponseStatus", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    #[prost(string, repeated, tag = "3")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub alert_config: ::core::option::Option<AlertConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertConfig {
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<CombinerBotSubscription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinerBotSubscription {
    #[prost(string, tag = "1")]
    pub bot_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub alert_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub chain_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateTxRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<TransactionEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateBlockRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<BlockEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateAlertRequest {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<AlertEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateTxResponse {
    #[prost(enumeration = "ResponseStatus", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    #[prost(message, repeated, tag = "3")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub latency_ms: u32,
    #[prost(bool, tag = "7")]
    pub private: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateBlockResponse {
    #[prost(enumeration = "ResponseStatus", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    #[prost(message, repeated, tag = "3")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub latency_ms: u32,
    #[prost(bool, tag = "7")]
    pub private: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluateAlertResponse {
    #[prost(enumeration = "ResponseStatus", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<Error>,
    #[prost(message, repeated, tag = "3")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub latency_ms: u32,
    #[prost(bool, tag = "7")]
    pub private: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEvent {
    #[prost(enumeration = "block_event::EventType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<block_event::Network>,
    #[prost(message, optional, tag = "5")]
    pub block: ::core::option::Option<block_event::EthBlock>,
    #[prost(message, optional, tag = "6")]
    pub timestamps: ::core::option::Option<TrackingTimestamps>,
}
/// Nested message and enum types in `BlockEvent`.
pub mod block_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Network {
        #[prost(string, tag = "1")]
        pub chain_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EthBlock {
        #[prost(string, tag = "1")]
        pub difficulty: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub extra_data: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub gas_limit: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub gas_used: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub hash: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub logs_bloom: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub miner: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub mix_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "9")]
        pub nonce: ::prost::alloc::string::String,
        #[prost(string, tag = "10")]
        pub number: ::prost::alloc::string::String,
        #[prost(string, tag = "11")]
        pub parent_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "12")]
        pub receipts_root: ::prost::alloc::string::String,
        #[prost(string, tag = "13")]
        pub sha3_uncles: ::prost::alloc::string::String,
        #[prost(string, tag = "14")]
        pub size: ::prost::alloc::string::String,
        #[prost(string, tag = "15")]
        pub state_root: ::prost::alloc::string::String,
        #[prost(string, tag = "16")]
        pub timestamp: ::prost::alloc::string::String,
        #[prost(string, tag = "17")]
        pub total_difficulty: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "18")]
        pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "19")]
        pub transactions_root: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "20")]
        pub uncles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventType {
        Block = 0,
        Reorg = 1,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Block => "BLOCK",
                EventType::Reorg => "REORG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCK" => Some(Self::Block),
                "REORG" => Some(Self::Reorg),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    #[prost(enumeration = "transaction_event::EventType", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub transaction: ::core::option::Option<transaction_event::EthTransaction>,
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub receipt: ::core::option::Option<transaction_event::EthReceipt>,
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<transaction_event::Network>,
    #[prost(message, repeated, tag = "5")]
    pub traces: ::prost::alloc::vec::Vec<transaction_event::Trace>,
    #[prost(map = "string, bool", tag = "6")]
    pub addresses: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
    #[prost(message, optional, tag = "7")]
    pub block: ::core::option::Option<transaction_event::EthBlock>,
    #[prost(message, repeated, tag = "8")]
    pub logs: ::prost::alloc::vec::Vec<transaction_event::Log>,
    #[prost(bool, tag = "9")]
    pub is_contract_deployment: bool,
    #[prost(string, tag = "10")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub timestamps: ::core::option::Option<TrackingTimestamps>,
    #[prost(map = "string, bool", tag = "12")]
    pub tx_addresses: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
}
/// Nested message and enum types in `TransactionEvent`.
pub mod transaction_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Network {
        #[prost(string, tag = "1")]
        pub chain_id: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EthBlock {
        #[prost(string, tag = "1")]
        pub block_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub block_number: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub block_timestamp: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EthTransaction {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub nonce: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub gas_price: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub gas: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub input: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub v: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub r: ::prost::alloc::string::String,
        #[prost(string, tag = "9")]
        pub s: ::prost::alloc::string::String,
        #[prost(string, tag = "10")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "11")]
        pub hash: ::prost::alloc::string::String,
        #[prost(string, tag = "12")]
        pub from: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Log {
        #[prost(string, tag = "1")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "3")]
        pub data: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub block_number: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub transaction_index: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub block_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub log_index: ::prost::alloc::string::String,
        #[prost(bool, tag = "9")]
        pub removed: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EthReceipt {
        #[prost(string, tag = "1")]
        pub root: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub status: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub cumulative_gas_used: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub logs_bloom: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub logs: ::prost::alloc::vec::Vec<Log>,
        #[prost(string, tag = "6")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub gas_used: ::prost::alloc::string::String,
        #[prost(string, tag = "9")]
        pub block_hash: ::prost::alloc::string::String,
        #[prost(string, tag = "10")]
        pub block_number: ::prost::alloc::string::String,
        #[prost(string, tag = "11")]
        pub transaction_index: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TraceAction {
        #[prost(string, tag = "1")]
        pub call_type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub input: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub init: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub balance: ::prost::alloc::string::String,
        #[prost(string, tag = "9")]
        pub refund_address: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TraceResult {
        #[prost(string, tag = "1")]
        pub gas_used: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub code: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub output: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Trace {
        #[prost(message, optional, tag = "1")]
        pub action: ::core::option::Option<TraceAction>,
        #[prost(string, tag = "2")]
        pub block_hash: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub block_number: i64,
        #[prost(message, optional, tag = "4")]
        pub result: ::core::option::Option<TraceResult>,
        #[prost(int64, tag = "5")]
        pub subtraces: i64,
        #[prost(int64, repeated, tag = "6")]
        pub trace_address: ::prost::alloc::vec::Vec<i64>,
        #[prost(string, tag = "7")]
        pub transaction_hash: ::prost::alloc::string::String,
        #[prost(int64, tag = "8")]
        pub transaction_position: i64,
        #[prost(string, tag = "9")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag = "10")]
        pub error: ::prost::alloc::string::String,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventType {
        Block = 0,
        Reorg = 1,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Block => "BLOCK",
                EventType::Reorg => "REORG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BLOCK" => Some(Self::Block),
                "REORG" => Some(Self::Reorg),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertEvent {
    #[prost(message, optional, tag = "1")]
    pub alert: ::core::option::Option<alert_event::Alert>,
    #[prost(message, optional, tag = "2")]
    pub timestamps: ::core::option::Option<TrackingTimestamps>,
}
/// Nested message and enum types in `AlertEvent`.
pub mod alert_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Alert {
        /// Unique string to identify this class of finding,
        /// primarily used to group similar findings for the end user
        #[prost(string, tag = "1")]
        pub alert_id: ::prost::alloc::string::String,
        /// List of addresses involved in the alert
        #[prost(string, repeated, tag = "2")]
        pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// List of contracts related to the alert
        #[prost(message, repeated, tag = "3")]
        pub contracts: ::prost::alloc::vec::Vec<alert::Contract>,
        /// Timestamp when the alert was published
        #[prost(string, tag = "4")]
        pub created_at: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub description: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub hash: ::prost::alloc::string::String,
        #[prost(map = "string, string", tag = "7")]
        pub metadata: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        #[prost(string, tag = "8")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "9")]
        pub projects: ::prost::alloc::vec::Vec<alert::Project>,
        #[prost(int32, tag = "10")]
        pub scan_node_count: i32,
        #[prost(string, tag = "11")]
        pub severity: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "12")]
        pub source: ::core::option::Option<alert::Source>,
        #[prost(string, tag = "13")]
        pub finding_type: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "14")]
        pub related_alerts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(uint64, tag = "15")]
        pub chain_id: u64,
        #[prost(message, repeated, tag = "16")]
        pub labels: ::prost::alloc::vec::Vec<alert::Label>,
        #[prost(bool, tag = "17")]
        pub truncated: bool,
        #[prost(message, optional, tag = "18")]
        pub address_bloom_filter: ::core::option::Option<super::BloomFilter>,
    }
    /// Nested message and enum types in `Alert`.
    pub mod alert {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Contract {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub project_id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Project {
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Block {
            #[prost(uint64, tag = "1")]
            pub number: u64,
            #[prost(string, tag = "2")]
            pub hash: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub timestamp: ::prost::alloc::string::String,
            #[prost(uint64, tag = "4")]
            pub chain_id: u64,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Bot {
            #[prost(string, repeated, tag = "1")]
            pub chain_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(string, tag = "2")]
            pub created_at: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub description: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub developer: ::prost::alloc::string::String,
            #[prost(string, tag = "5")]
            pub doc_reference: ::prost::alloc::string::String,
            #[prost(bool, tag = "6")]
            pub enabled: bool,
            #[prost(string, tag = "7")]
            pub id: ::prost::alloc::string::String,
            #[prost(string, tag = "8")]
            pub image: ::prost::alloc::string::String,
            #[prost(string, tag = "9")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag = "10")]
            pub reference: ::prost::alloc::string::String,
            #[prost(string, tag = "11")]
            pub repository: ::prost::alloc::string::String,
            #[prost(string, repeated, tag = "12")]
            pub projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(string, repeated, tag = "13")]
            pub scan_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(string, tag = "14")]
            pub version: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SourceAlertEvent {
            #[prost(string, tag = "1")]
            pub bot_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub alert_hash: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub timestamp: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub chain_id: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Source {
            #[prost(string, tag = "1")]
            pub transaction_hash: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "2")]
            pub bot: ::core::option::Option<Bot>,
            #[prost(message, optional, tag = "3")]
            pub block: ::core::option::Option<Block>,
            #[prost(message, optional, tag = "4")]
            pub source_event: ::core::option::Option<SourceAlertEvent>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Label {
            #[prost(string, tag = "1")]
            pub label: ::prost::alloc::string::String,
            #[prost(float, tag = "2")]
            pub confidence: f32,
            #[prost(string, tag = "3")]
            pub entity: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub entity_type: ::prost::alloc::string::String,
            #[prost(bool, tag = "5")]
            pub remove: bool,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseStatus {
    Unknown = 0,
    Error = 1,
    Success = 2,
}
impl ResponseStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseStatus::Unknown => "UNKNOWN",
            ResponseStatus::Error => "ERROR",
            ResponseStatus::Success => "SUCCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "ERROR" => Some(Self::Error),
            "SUCCESS" => Some(Self::Success),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod agent_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AgentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AgentClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AgentClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AgentClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AgentClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn initialize(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeRequest>,
        ) -> Result<tonic::Response<super::InitializeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.forta.Agent/Initialize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn evaluate_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluateTxRequest>,
        ) -> Result<tonic::Response<super::EvaluateTxResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.forta.Agent/EvaluateTx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn evaluate_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluateBlockRequest>,
        ) -> Result<tonic::Response<super::EvaluateBlockResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.forta.Agent/EvaluateBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn evaluate_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::EvaluateAlertRequest>,
        ) -> Result<tonic::Response<super::EvaluateAlertResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/network.forta.Agent/EvaluateAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod agent_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AgentServer.
    #[async_trait]
    pub trait Agent: Send + Sync + 'static {
        async fn initialize(
            &self,
            request: tonic::Request<super::InitializeRequest>,
        ) -> Result<tonic::Response<super::InitializeResponse>, tonic::Status>;
        async fn evaluate_tx(
            &self,
            request: tonic::Request<super::EvaluateTxRequest>,
        ) -> Result<tonic::Response<super::EvaluateTxResponse>, tonic::Status>;
        async fn evaluate_block(
            &self,
            request: tonic::Request<super::EvaluateBlockRequest>,
        ) -> Result<tonic::Response<super::EvaluateBlockResponse>, tonic::Status>;
        async fn evaluate_alert(
            &self,
            request: tonic::Request<super::EvaluateAlertRequest>,
        ) -> Result<tonic::Response<super::EvaluateAlertResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AgentServer<T: Agent> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Agent> AgentServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AgentServer<T>
    where
        T: Agent,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/network.forta.Agent/Initialize" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeSvc<T: Agent>(pub Arc<T>);
                    impl<T: Agent> tonic::server::UnaryService<super::InitializeRequest>
                    for InitializeSvc<T> {
                        type Response = super::InitializeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).initialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitializeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/network.forta.Agent/EvaluateTx" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluateTxSvc<T: Agent>(pub Arc<T>);
                    impl<T: Agent> tonic::server::UnaryService<super::EvaluateTxRequest>
                    for EvaluateTxSvc<T> {
                        type Response = super::EvaluateTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvaluateTxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).evaluate_tx(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EvaluateTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/network.forta.Agent/EvaluateBlock" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluateBlockSvc<T: Agent>(pub Arc<T>);
                    impl<
                        T: Agent,
                    > tonic::server::UnaryService<super::EvaluateBlockRequest>
                    for EvaluateBlockSvc<T> {
                        type Response = super::EvaluateBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvaluateBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).evaluate_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EvaluateBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/network.forta.Agent/EvaluateAlert" => {
                    #[allow(non_camel_case_types)]
                    struct EvaluateAlertSvc<T: Agent>(pub Arc<T>);
                    impl<
                        T: Agent,
                    > tonic::server::UnaryService<super::EvaluateAlertRequest>
                    for EvaluateAlertSvc<T> {
                        type Response = super::EvaluateAlertResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvaluateAlertRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).evaluate_alert(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EvaluateAlertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Agent> Clone for AgentServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Agent> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Agent> tonic::server::NamedService for AgentServer<T> {
        const NAME: &'static str = "network.forta.Agent";
    }
}
