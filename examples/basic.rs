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

use peakrequests::{get, post, post_json, PeakRequests};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    match get("https://httpbin.org/get") {
        Ok(response) => {
            println!("status: {}", response.status_code);
            println!("body: {}", response.text);
        }
        Err(e) => eprintln!("error: {}", e),
    }

    let mut form_data = HashMap::new();
    form_data.insert("key1", "value1");
    form_data.insert("key2", "value2");

    match post("https://httpbin.org/post", form_data) {
        Ok(response) => println!("post response: {}", response.text),
        Err(e) => eprintln!("error: {}", e),
    }

    let json_data = json!({
        "name": "greg",
        "age": 30,
        "is_swag": true
    });

    match post_json("https://httpbin.org/post", json_data) {
        Ok(response) => println!("json post response: {}", response.text),
        Err(e) => eprintln!("error: {}", e),
    }

    let mut client = PeakRequests::new()
        .headers({
            let mut headers = HashMap::new();
            headers.insert("User-Agent".to_string(), "PeakRequests/0.1".to_string());
            headers.insert("Accept".to_string(), "application/json".to_string());
            headers
        })
        .timeout(10)
        .allow_redirects(true)
        .max_redirects(5);

    match client.get("https://httpbin.org/headers") {
        Ok(response) => println!("client response: {}", response.text),
        Err(e) => eprintln!("error: {}", e),
    }
}
