mod logger;

pub use logger::init_logger;

pub use tracing;
pub use tracing::{debug, error, info, warn};
