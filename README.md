# spin-router-middleware

> **WARNING**: This is a proof of concept implementation and not ready for use.

## Usage

```
use spin_sdk::http::{IntoResponse, Request, Response, ResponseBuilder, Router};
use spin_router_middleware::{MiddlewareBuilder, Middleware, Next};
use task_local_extensions::Extensions;

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

let mut router = Router::new();
router.get_async("/", get);
let builder = MiddlewareBuilder::new(router).with(TransparentMiddleware);
```

## Inspiration

Most of the code is copy pasted from [reqwest-middleware](https://github.com/TrueLayer/reqwest-middleware) because it shows a middleware pattern that is easy to use and understand without any async runtime specific code.
