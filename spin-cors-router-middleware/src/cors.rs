// all parts in here are based on https://github.com/ThorstenHans/spin-contrib-http
// I updated the code to work with the latest version of the spin-sdk and changed the
// builder_with_cors (which uses a http::builder) function to response_with_cors
// which is a function that takes a Response and adds the necessary headers to it.
// TODO: Add a PR to https://github.com/ThorstenHans/spin-contrib-http

use spin_sdk::http::{Request, Response, ResponseBuilder};

/// This struct is used to configure CORS support
pub struct Config {
    /// The origins to allow in CORS (separated by commas)
    pub allowed_origins: String,
    /// The HTTP methods to allow in CORS (separated by commas)
    pub allowed_methods: String,
    /// The HTTP headers to allow in CORS (separated by commas)
    pub allowed_headers: String,
    /// Whether or not to allow credentials in CORS
    pub allow_credentials: bool,
    /// The max age to allow in CORS
    pub max_age: Option<u32>,
}

impl Config {
    /// Checks if the provided origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        if self.allowed_origins.is_empty() || self.allowed_origins == NO_ORIGINS {
            return false;
        }
        if self.allowed_origins == ALL_ORIGINS {
            return true;
        }
        let allowed_origins: Vec<&str> = self.allowed_origins.split(',').collect();
        for allowed_origin in allowed_origins {
            if allowed_origin == origin {
                return true;
            }
        }
        false
    }

    /// Checks if the provided HTTP method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        if self.allowed_methods.is_empty() {
            return false;
        }
        if self.allowed_methods == ALL_METHODS {
            return true;
        }
        let allowed_methods: Vec<&str> = self.allowed_methods.split(',').collect();
        for allowed_method in allowed_methods {
            if allowed_method == method {
                return true;
            }
        }
        false
    }
}

/// Constant for allowing all HTTP methods in CORS
pub const ALL_METHODS: &str = "*";
/// Constant for allowing all HTTP headers in CORS
pub const ALL_HEADERS: &str = "*";
/// Constant for allowing all origins in CORS
pub const ALL_ORIGINS: &str = "*";
/// Constant for allowing no origins in CORS
pub const NO_ORIGINS: &str = "NULL";

pub fn response_with_cors<'a>(req: &'a mut Response, cors_config: &Config) -> &'a Response {
    let mut origin = cors_config.allowed_origins.as_str();
    if origin.is_empty() {
        origin = NO_ORIGINS;
    }
    req.set_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), origin);
    req.set_header(
        http::header::ACCESS_CONTROL_ALLOW_METHODS.as_str(),
        cors_config.allowed_methods.as_str(),
    );
    req.set_header(
        http::header::ACCESS_CONTROL_ALLOW_HEADERS.as_str(),
        cors_config.allowed_headers.as_str(),
    );
    req.set_header(
        http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
        format!("{}", cors_config.allow_credentials),
    );

    if cors_config.max_age.is_some() {
        req.set_header(
            http::header::ACCESS_CONTROL_MAX_AGE.as_str(),
            format!("{}", cors_config.max_age.unwrap()),
        );
    }
    req
}

pub fn handle_preflight(req: &Request, cors_config: &Config) -> Response {
    if req.header(http::header::ORIGIN.as_str()).is_none()
        || req
            .header(http::header::ACCESS_CONTROL_REQUEST_METHOD.as_str())
            .is_none()
    {
        return ResponseBuilder::new(204).build();
    }
    let Some(origin) = req.header(http::header::ORIGIN.as_str()).unwrap().as_str() else {
        return ResponseBuilder::new(204).build();
    };
    let Some(method) = req
        .header(http::header::ACCESS_CONTROL_REQUEST_METHOD.as_str())
        .unwrap()
        .as_str()
    else {
        return ResponseBuilder::new(204).build();
    };

    if origin.is_empty() || method.is_empty() {
        return ResponseBuilder::new(204).build();
    }
    if cors_config.is_origin_allowed(origin) && cors_config.is_method_allowed(method) {
        let mut builder = ResponseBuilder::new(204);
        builder
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), origin)
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS.as_str(), method)
            .header(
                http::header::ACCESS_CONTROL_ALLOW_HEADERS.as_str(),
                cors_config.allowed_headers.as_str(),
            )
            .header(
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
                format!("{}", cors_config.allow_credentials),
            );

        if cors_config.max_age.is_some() {
            builder.header(
                http::header::ACCESS_CONTROL_MAX_AGE.as_str(),
                format!("{}", cors_config.max_age.unwrap()),
            );
        }
        return builder.build();
    }
    ResponseBuilder::new(204).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_with_cors_sets_origins() {
        let origin = "http://localhost:3000";
        let cfg = Config {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let mut res = ResponseBuilder::new(200).build();
        let sut = response_with_cors(&mut res, &cfg);

        let actual = sut
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, origin);
    }

    #[test]
    fn request_with_cors_null_when_origins_is_empty() {
        let origin = "";
        let cfg = Config {
            allowed_origins: origin.to_string(),
            allowed_methods: ALL_METHODS.to_string(),
            allowed_headers: ALL_HEADERS.to_string(),
            allow_credentials: true,
            max_age: None,
        };
        let mut res = ResponseBuilder::new(200).build();
        let sut = response_with_cors(&mut res, &cfg);

        let actual = sut
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str())
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(actual, NO_ORIGINS);
    }
}
