/*
 * MIT License
 *
 * Copyright (c) 2025 Squirrel
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use reqwest::{blocking::Client, header, redirect::Policy};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub text: String,
    pub headers: HashMap<String, String>,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct PeakRequests {
    client: Option<Client>,
    headers: HashMap<String, String>,
    timeout: Option<u64>,
    allow_redirects: bool,
    max_redirects: usize,
}

impl PeakRequests {
    pub fn new() -> Self {
        PeakRequests {
            client: None,
            headers: HashMap::new(),
            timeout: None,
            allow_redirects: true,
            max_redirects: 10,
        }
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn allow_redirects(mut self, allow: bool) -> Self {
        self.allow_redirects = allow;
        self
    }

    pub fn max_redirects(mut self, max: usize) -> Self {
        self.max_redirects = max;
        self
    }

    fn init_client(&mut self) -> Result<(), String> {
        let mut client_builder = Client::builder();

        if !self.headers.is_empty() {
            let mut headers = header::HeaderMap::new();
            for (key, value) in &self.headers {
                headers.insert(
                    header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?,
                    header::HeaderValue::from_str(value).map_err(|e| e.to_string())?,
                );
            }
            client_builder = client_builder.default_headers(headers);
        }

        if let Some(timeout_secs) = self.timeout {
            client_builder = client_builder.timeout(Duration::from_secs(timeout_secs));
        }

        client_builder = if self.allow_redirects {
            client_builder.redirect(Policy::limited(self.max_redirects))
        } else {
            client_builder.redirect(Policy::none())
        };

        self.client = Some(client_builder.build().map_err(|e| e.to_string())?);
        Ok(())
    }

    pub fn get(&mut self, url: &str) -> Result<Response, String> {
        self._request("GET", url, None, None)
    }

    pub fn post(
        &mut self,
        url: &str,
        data: Option<HashMap<&str, &str>>,
        json: Option<Value>,
    ) -> Result<Response, String> {
        self._request("POST", url, data, json)
    }

    pub fn put(
        &mut self,
        url: &str,
        data: Option<HashMap<&str, &str>>,
        json: Option<Value>,
    ) -> Result<Response, String> {
        self._request("PUT", url, data, json)
    }

    pub fn delete(&mut self, url: &str) -> Result<Response, String> {
        self._request("DELETE", url, None, None)
    }

    fn _request(
        &mut self,
        method: &str,
        url: &str,
        data: Option<HashMap<&str, &str>>,
        json: Option<Value>,
    ) -> Result<Response, String> {
        if self.client.is_none() {
            self.init_client()?;
        }

        let client = self.client.as_ref().unwrap();
        let mut request_builder = match method {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return Err(format!("unsupported HTTP method: {}", method)),
        };

        if let Some(form_data) = data {
            request_builder = request_builder.form(&form_data);
        }

        if let Some(json_data) = json {
            request_builder = request_builder.json(&json_data);
        }

        let response = request_builder.send().map_err(|e| e.to_string())?;
        let status_code = response.status().as_u16();
        let response_url = response.url().to_string();

        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
        }

        let text = response.text().map_err(|e| e.to_string())?;

        Ok(Response {
            status_code,
            text,
            headers,
            url: response_url,
        })
    }
}

pub fn get(url: &str) -> Result<Response, String> {
    PeakRequests::new().get(url)
}

pub fn post(url: &str, data: HashMap<&str, &str>) -> Result<Response, String> {
    PeakRequests::new().post(url, Some(data), None)
}

pub fn post_json(url: &str, json: Value) -> Result<Response, String> {
    PeakRequests::new().post(url, None, Some(json))
}

pub fn put(url: &str, data: HashMap<&str, &str>) -> Result<Response, String> {
    PeakRequests::new().put(url, Some(data), None)
}

pub fn put_json(url: &str, json: Value) -> Result<Response, String> {
    PeakRequests::new().put(url, None, Some(json))
}

pub fn delete(url: &str) -> Result<Response, String> {
    PeakRequests::new().delete(url)
}
