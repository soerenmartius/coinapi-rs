use reqwest;
use reqwest::{Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING};
use std::io::Read;

static API_HOST: &'static str = "https://rest.coinapi.io";

#[derive(Clone)]
pub struct Client {
    api_key: String
}

impl Client {
    // returns a new instance of Client
    pub fn new(api_key: String) -> Self {
        Client { api_key }
    }

    // GET
    pub fn get(&self, endpoint: &str, parameter: &str) -> Result<String, reqwest::Error> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);

        if !parameter.is_empty() {
            url.push_str(format!("?{}", parameter).as_str());
        }

        let client = reqwest::Client::new();

        let response = client
            .get(url.as_str())
            .headers(self.build_headers(String::from("application/json"), true).unwrap())
            .send().unwrap();

        self.handle_response(response)
    }

    // set headers
    fn build_headers(&self, content_type: String, compress: bool) -> Result<HeaderMap, reqwest::header::InvalidHeaderValue> {
        let mut headers = HeaderMap::new();

        headers.insert(HeaderName::from_static("x-coinapi-key"), HeaderValue::from_str(self.api_key.as_str())?);
        headers.insert(ACCEPT, HeaderValue::from_str(content_type.as_str())?);

        if compress {
            headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("deflate, gzip"));
        }

        Ok(headers)
    }

    // handle response
    fn handle_response(&self, mut response: Response) -> Result<String, reqwest::Error> {
        match response.status() {
            StatusCode::OK => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();
                Ok(body)
            }
            StatusCode::BAD_REQUEST => {panic!() }
            StatusCode::UNAUTHORIZED => {panic!() }
            StatusCode::FORBIDDEN => {panic!() }
            StatusCode::TOO_MANY_REQUESTS => { panic!()}
            _ => (panic!())
        }
    }
}
