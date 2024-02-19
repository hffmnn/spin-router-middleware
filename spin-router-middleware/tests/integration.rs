use spin_router_middleware::{Middleware, MiddlewareBuilder, Next};
use spin_sdk::http::{Method, Params, Request, RequestBuilder, Response, ResponseBuilder, Router};
use task_local_extensions::Extensions;

static REQUEST_HEADER: &str = "request-header";
static RESPONSE_HEADER: &str = "response-header";

struct HeaderMiddleware {
    header_value: String,
}

// Middleware that adds its header_value to the request and response headers
#[async_trait::async_trait(?Send)]
impl Middleware for HeaderMiddleware {
    async fn handle(&self, req: Request, extensions: &mut Extensions, next: Next<'_>) -> Response {
        let mut value = self.header_value.clone();
        if let Some(header) = req.header(REQUEST_HEADER) {
            if let Some(header_value) = header.as_str() {
                value = format!("{},{}", header_value, &self.header_value)
            }
        };

        let mut inner_req = RequestBuilder::new(req.method().clone(), req.uri()).build();
        inner_req.set_header(REQUEST_HEADER, value);

        let mut res = next.run(inner_req, extensions).await;

        if let Some(header) = res.header(RESPONSE_HEADER) {
            if let Some(header_value) = header.as_str() {
                let value = format!("{},{}", header_value, &self.header_value);
                res.set_header(RESPONSE_HEADER, &value);
            }
        } else {
            res.set_header(RESPONSE_HEADER, &self.header_value);
        }
        res
    }
}

pub async fn get(req: Request, _params: Params) -> Response {
    ResponseBuilder::new(200)
        .header(
            REQUEST_HEADER,
            req.header(REQUEST_HEADER).unwrap().as_str().unwrap(),
        )
        .build()
}

macro_rules! block_on {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
fn test_middleware_order() {
    let mut router = Router::new();
    router.get_async("/", get);

    let req = Request::new(Method::Get, "/");
    let builder = MiddlewareBuilder::new(router)
        // this gets called second
        .with(HeaderMiddleware {
            header_value: "inner".to_string(),
        })
        // this gets called first
        .with(HeaderMiddleware {
            header_value: "outer".to_string(),
        });
    let res = block_on!(builder.run(req));
    assert_eq!(
        res.header(REQUEST_HEADER).unwrap().as_str(),
        Some("outer,inner")
    );
    assert_eq!(
        res.header(RESPONSE_HEADER).unwrap().as_str(),
        Some("inner,outer")
    );
}
