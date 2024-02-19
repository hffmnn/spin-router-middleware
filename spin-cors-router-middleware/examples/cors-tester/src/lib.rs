use spin_cors_router_middleware::{
    Config, CorsMiddleware, MiddlewareBuilder, ALL_HEADERS, ALL_METHODS, ALL_ORIGINS,
};
use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
async fn handle_cors_tester(req: Request) -> Response {
    let mut router = Router::new();
    router.get_async("/", api::get);
    let builder = MiddlewareBuilder::new(router).with(CorsMiddleware::new(Config {
        allowed_origins: ALL_ORIGINS.into(),
        allowed_methods: ALL_METHODS.into(),
        allowed_headers: ALL_HEADERS.into(),
        allow_credentials: true,
        max_age: None,
    }));
    builder.run(req).await
}

mod api {
    use spin_sdk::http::Params;

    use super::*;

    // /
    pub async fn get(_req: Request, _params: Params) -> Response {
        Response::new(200, ())
    }
}
