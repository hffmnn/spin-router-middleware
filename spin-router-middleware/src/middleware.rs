use spin_sdk::http::{Request, Response, Router};
use std::sync::Arc;
use task_local_extensions::Extensions;

/// When attached to a [`ClientWithMiddleware`] (generally using [`with`]), middleware is run
/// whenever the client issues a request, in the order it was attached.
///
/// # Example
///
/// ```
/// use spin_sdk::http::{IntoResponse, Request, Response, ResponseBuilder, Router};
/// use spin_router_middleware::{MiddlewareBuilder, Middleware, Next};
/// use task_local_extensions::Extensions;
///
/// struct TransparentMiddleware;
///
/// #[async_trait::async_trait(?Send)]
/// impl Middleware for TransparentMiddleware {
///     async fn handle(
///         &self,
///         req: Request,
///         extensions: &mut Extensions,
///         next: Next<'_>,
///     ) -> Response {
///         next.run(req, extensions).await
///     }
/// }
/// ```
///
/// [`ClientWithMiddleware`]: crate::ClientWithMiddleware
/// [`with`]: crate::ClientBuilder::with
#[async_trait::async_trait(?Send)]
pub trait Middleware: 'static + Send + Sync {
    /// Invoked with a request before sending it. If you want to continue processing the request,
    /// you should explicitly call `next.run(req, extensions)`.
    ///
    /// If you need to forward data down the middleware stack, you can use the `extensions`
    /// argument.
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Response;
}

#[async_trait::async_trait(?Send)]
impl<F> Middleware for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Request, &'a mut Extensions, Next<'a>) -> BoxFuture<'a, Response>,
{
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Response {
        (self)(req, extensions, next).await
    }
}

/// Next encapsulates the remaining middleware chain to run in [`Middleware::handle`]. You can
/// forward the request down the chain with [`run`].
///
/// [`Middleware::handle`]: Middleware::handle
/// [`run`]: Self::run
#[derive(Clone)]
pub struct Next<'a> {
    router: &'a Router,
    middlewares: &'a [Arc<dyn Middleware>],
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

impl<'a> Next<'a> {
    pub(crate) fn new(router: &'a Router, middlewares: &'a [Arc<dyn Middleware>]) -> Self {
        Next {
            router,
            middlewares,
        }
    }

    pub fn run(mut self, req: Request, extensions: &'a mut Extensions) -> BoxFuture<'a, Response> {
        if let Some((current, rest)) = self.middlewares.split_first() {
            self.middlewares = rest;
            Box::pin(current.handle(req, extensions, self))
        } else {
            Box::pin(async move { self.router.handle(req) })
        }
    }
}
