extern crate url;

use std::collections::HashMap;

/// Generic URI builder that handles all URI-related stuff.
pub struct UriBuilder<'a> {
    _api_ver: &'a str,
    _method: &'a str,
    _params: HashMap<&'a str, String>,
}

/// Implemented by basically every query builder in the crate.
pub trait HasBuilder<'a> {
    fn builder(&mut self) -> &mut UriBuilder<'a>;
}

impl<'a> UriBuilder<'a> {
    pub fn new() -> Self {
        UriBuilder {
            _api_ver: "2.5",
            _method: "",
            _params: HashMap::with_capacity(10),
        }
    }

    /// Set the endpoint method.
    pub fn method(&mut self, method: &'a str) -> &mut Self {
        self._method = method;
        self
    }

    /// Add param to the URI.
    pub fn param(&mut self, key: &'a str, val: String) -> &mut Self {
        self._params.insert(key, val);
        self
    }

    /// Consumes the builder and returns the corresponding URI.
    pub fn build(self) -> String {
        let base = format!("http://api.openweathermap.org/data/{api}/{method}",
                           api = self._api_ver,
                           method = self._method);
        let mut ser = url::form_urlencoded::Serializer::new(String::new());

        match self._params.len() {
            0 => base,
            _ => {
                for (k, v) in self._params {
                    ser.append_pair(k, v.as_str());
                }
                base + "?" + ser.finish().as_str()
            }            
        }
    }
}