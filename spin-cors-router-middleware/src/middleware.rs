use spin_router_middleware::{Middleware, Next};
use spin_sdk::http::{Method, Request, Response};
use task_local_extensions::Extensions;

use crate::{handle_preflight, response_with_cors, Config};

pub struct CorsMiddleware {
    cors_config: Config,
}

impl CorsMiddleware {
    pub fn new(cors_config: Config) -> Self {
        Self { cors_config }
    }
}

#[async_trait::async_trait(?Send)]
impl Middleware for CorsMiddleware {
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Response {
        if req.method() == &Method::Options {
            return handle_preflight(&req, &self.cors_config);
        }

        let mut res = next.run(req, extensions).await;
        response_with_cors(&mut res, &self.cors_config);
        res
    }
}
