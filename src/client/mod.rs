mod http;

pub use http::HttpClient;

/// Trait for protocol clients - allows future extension for GraphQL, gRPC, etc
pub trait ProtocolClient: Send + Sync {
    // Placeholder for future protocol implementations
}