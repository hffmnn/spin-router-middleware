use spin_sdk::http::{Response, Router};
use std::sync::Arc;
use task_local_extensions::Extensions;

use super::middleware::{Middleware, Next};

pub struct MiddlewareBuilder {
    router: Router,
    middleware_stack: Vec<Arc<dyn Middleware>>,
}

impl MiddlewareBuilder {
    pub fn new(router: Router) -> Self {
        MiddlewareBuilder {
            router,
            middleware_stack: Vec::new(),
        }
    }

    /// Convenience method to attach middleware.
    ///
    /// If you need to keep a reference to the middleware after attaching, use [`with_arc`].
    ///
    /// [`with_arc`]: Self::with_arc
    pub fn with<M>(self, middleware: M) -> Self
    where
        M: Middleware,
    {
        self.with_arc(Arc::new(middleware))
    }

    /// Add middleware to the chain. [`with`] is more ergonomic if you don't need the `Arc`.
    ///
    /// [`with`]: Self::with
    pub fn with_arc(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware_stack.push(middleware);
        self
    }

    /// Run the middleware stack.
    pub async fn run(mut self, req: spin_sdk::http::Request) -> Response {
        // Because we want to run the middleware in the reversed order they were
        // attached (the middleware that is attached first should run last), we
        // need to reverse the middleware stack.
        self.middleware_stack.reverse();
        let next = Next::new(&self.router, &self.middleware_stack);
        let mut ext = Extensions::new();
        next.run(req, &mut ext).await
    }
}
