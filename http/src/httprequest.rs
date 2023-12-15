use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Resource {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Identifying_resources_on_the_Web
    Path(String),
}

// impl fmt::Display for Resource {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Resource::Path(path) => write!(f, "{}", path),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    V3_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2" => Version::V2_0,
            "HTTP/3" => Version::V3_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let mut items = line.split_whitespace();
                parsed_method = items.next().unwrap().into();
                parsed_resource = Resource::Path(items.next().unwrap().to_string());
                parsed_version = items.next().unwrap().into();
            } else if line.contains(":") {
                let mut header_elems = line.split(":");
                let key = header_elems.next().unwrap().to_string();
                let value = header_elems.next().unwrap().to_string();
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_body = line
            }
        }
        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers: parsed_headers,
            body: parsed_body.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
        let m: Method = "biubiubiu".into();
        assert_eq!(m, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/2".into();
        assert_eq!(v, Version::V2_0);
        let v: Version = "HTTP/4".into();
        assert_eq!(v, Version::Uninitialized);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /whitepaper/ HTTP/2\r\nHost: blog.tfei.moe\r\nUser-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64;) Gecko/20100101 Firefox/120.0\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\nAccept-Language: en-US,en;q=0.5\r\nAccept-Encoding: gzip, deflate, br\r\n\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " blog.tfei.moe".into());
        headers_expected.insert(
            "User-Agent".into(),
            " Mozilla/5.0 (Windows NT 10.0; Win64; x64;) Gecko/20100101 Firefox/120.0".into(),
        );
        headers_expected.insert("Accept".into(), " text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".into()
        );
        headers_expected.insert("Accept-Language".into(), " en-US,en;q=0.5".into());
        headers_expected.insert("Accept-Encoding".into(), " gzip, deflate, br".into());

        let request: HttpRequest = s.into();

        assert_eq!(Method::GET, request.method);
        assert_eq!(Resource::Path("/whitepaper/".to_string()), request.resource);
        assert_eq!(Version::V2_0, request.version);
        assert_eq!(headers_expected, request.headers);
    }
}
