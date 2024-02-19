/// A [`MiddlewareBuilder`] is used to attach one or more [`Middleware`]s to
/// a [`Router`].
/// When attached to a [`Router`] (generally using [`with`]), middlewares are run
/// whenever the middleware chain is run with [`run`]. The middlware that was
/// added last will receive the incoming request first and the response last.
///
/// # Example
///
/// ```ignore
/// use spin_sdk::http::{IntoResponse, Request, Response, ResponseBuilder, Router};
/// use spin_router_middleware::{MiddlewareBuilder, Middleware, Next};
/// use task_local_extensions::Extensions;
/// use spin_sdk::http_component;
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
///
///
/// #[http_component]
/// async fn handle_cors_tester(req: Request) -> Response {
///     let mut router = Router::new();
///     router.get_async("/", api::get);
///     MiddlewareBuilder::new(router).with(TransparentMiddleware).run(req).await;
/// }
///
/// mod api {
///     use spin_sdk::http::Params;
///
///     use super::*;
///
///     // /goodbye/:planet
///     pub async fn get(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
///         Ok(Response::new(200, ()))
///     }
/// }

/// ```
///
/// [`Router`]: https://docs.rs/spin-sdk/latest/spin_sdk/http/struct.Router.html
/// [`Middleware`]: super::middleware::Middleware
/// [`MiddlewareBuilder`]: crate::MiddlewareBuilder
/// [`with`]: crate::MiddlewareBuilder::with
/// [`run`]: crate::MiddlewareBuilder::run
mod builder;
mod middleware;

pub use builder::MiddlewareBuilder;
pub use middleware::{Middleware, Next};
