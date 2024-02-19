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

