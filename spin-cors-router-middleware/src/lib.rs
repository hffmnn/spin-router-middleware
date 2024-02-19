mod cors;
mod middleware;

pub(crate) use cors::{handle_preflight, response_with_cors};

pub use cors::{Config, ALL_HEADERS, ALL_METHODS, ALL_ORIGINS};
pub use middleware::CorsMiddleware;
pub use spin_router_middleware::MiddlewareBuilder;
