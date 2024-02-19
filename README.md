# spin-router-middleware

> **WARNING**: This is a proof of concept implementation and not ready for use.

## Usage

```
use spin_sdk::http::{IntoResponse, Request, Response, ResponseBuilder, Router};
use spin_router_middleware::{MiddlewareBuilder, Middleware, Next};
use task_local_extensions::Extensions;
use spin_sdk::http_component;

struct TransparentMiddleware;

#[async_trait::async_trait(?Send)]
impl Middleware for TransparentMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Response {
        next.run(req, extensions).await
    }
}

#[http_component]
async fn handle_cors_tester(req: Request) -> Response {
    let mut router = Router::new();
    router.get_async("/", api::get);
    MiddlewareBuilder::new(router).with(TransparentMiddleware).run(req).await
}

mod api {
    use spin_sdk::http::{Request, Response, Params};

    // /
    pub async fn get(_req: Request, _params: Params) -> Response {
        Response::new(200, ())
    }
}j
```

## Inspiration

Most of the code is copy pasted from [reqwest-middleware](https://github.com/TrueLayer/reqwest-middleware) because it shows a middleware pattern that is easy to use and understand without any async runtime specific code.
