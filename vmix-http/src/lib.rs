pub mod client;
pub mod traits;

// Re-export commonly used types
pub use client::HttpVmixClient;
pub use traits::VmixApiClient;

// Re-export vmix-core for convenience
pub use vmix_core;

// Re-export types from vmix-tcp for convenience (InputNumber, TallyData)
// These are needed by the HTTP client trait
pub use vmix_tcp::{InputNumber, TallyData};
