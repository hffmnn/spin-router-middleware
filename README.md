# spin-router-middleware

> **⚠️WARNING⚠️**: This is a proof of concept and not ready for any kind of production use. Also it comes with a lot of "I have no idea what I am doing here" code.

## Usage

```
[dependencies]
# only needed when `CorsMiddleware` gets used
spin-cors-router-middleware = { git = "https://github.com/hffmnn/spin-router-middleware.git", branch = "main" }
# only needed when writing own middlewares
async-trait = "0.1.77"
spin-router-middleware = { git = "https://github.com/hffmnn/spin-router-middleware.git", branch = "main" }
task-local-extensions = "0.1.4"
```

```
use spin_cors_router_middleware::{CorsMiddleware, ALL_HEADERS, ALL_METHODS, ALL_ORIGINS};
use spin_router_middleware::{Middleware, MiddlewareBuilder, Next};
use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;
use task_local_extensions::Extensions;

struct TransparentMiddleware;

#[async_trait::async_trait(?Send)]
impl Middleware for TransparentMiddleware {
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Response {
        next.run(req, extensions).await
    }
}

#[http_component]
async fn handle_cors_tester(req: Request) -> Response {
    let mut router = Router::new();
    router.get_async("/", api::get);
    // Note: `CorsMiddleware` gets hit first by the incoming request, `TransparentMiddleware` second.
    // The response chain is in reverse order.
    MiddlewareBuilder::new(router)
        .with(TransparentMiddleware)
        .with(CorsMiddleware::new(spin_cors_router_middleware::Config {
            allowed_origins: ALL_ORIGINS.into(),
            allowed_methods: ALL_METHODS.into(),
            allowed_headers: ALL_HEADERS.into(),
            allow_credentials: false,
            max_age: None,
        }))
        .run(req)
        .await
}

mod api {
    use spin_sdk::http::{Params, Request, Response};

    // /
    pub async fn get(_req: Request, _params: Params) -> Response {
        Response::new(200, ())
    }
}
```

## Inspiration

Most of the code is copy pasted from [reqwest-middleware](https://github.com/TrueLayer/reqwest-middleware) because it shows a middleware pattern that is easy to use and understand without any async runtime specific code. The cors stuff comes from [spin-contrib-http](https://github.com/ThorstenHans/spin-contrib-http).

## Open Questions

- How to forward state to the http handler? For example an auth middleware that checks for a token and forwards the user to the handler.
    - One option could be some kind of global state which would be okay (I guess) because the component dies after the request anyways.

