use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Clone, Default)]
struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        version: &'a str,
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        HttpResponse::<'a> {
            version,
            status_code,
            status_text: match status_code {
                "200" => "OK",
                "400" => "Bad Request",
                "404" => "Not Found",
                "500" => "Internal Server Error",
                _ => "Not Found",
            },
            headers: match &headers {
                Some(_h) => headers.clone(),
                None => {
                    let mut h = HashMap::new();
                    h.insert("Content-Type", "text/html");
                    Some(h)
                }
            },
            body,
        }
    }

    pub fn send_response(&self, write_stream: &mut impl Write) {
        let response = self.clone();
        let _ = write!(write_stream, "{}", String::from(response));
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut headers_string = "".into();
        for (k, v) in map.iter() {
            headers_string = format!("{}{}:{}\r\n", headers_string, k, v);
        }
        headers_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let response = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &response.version(),
            &response.status_code(),
            &response.status_text(),
            &response.headers(),
            &value.body.unwrap().len(),
            &response.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual =
            HttpResponse::new("HTTP/2.0", "200", None, Some("Biu Biu Biu!!!".into()));
        let response_expected = HttpResponse {
            version: "HTTP/2.0",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Biu Biu Biu!!!".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual =
            HttpResponse::new("HTTP/2.0", "404", None, Some("No Rice, No Life".into()));
        let response_expected = HttpResponse {
            version: "HTTP/2.0",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("No Rice, No Life".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_response_struct_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Rust~".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 5\r\n\r\nRust~";
        assert_eq!(http_string, response_actual);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/2.0",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("BiuBiuBiu".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual =
            "HTTP/2.0 200 OK\r\nContent-Type:text/html\r\nContent-Length: 9\r\n\r\nBiuBiuBiu";
        assert_eq!(http_string, response_actual);
    }
}
