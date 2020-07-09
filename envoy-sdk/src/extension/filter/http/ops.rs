// Copyright 2020 Tetrate
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
    RequestBodyOps, RequestFlowOps, RequestHeadersOps, RequestTrailersOps, ResponseBodyOps,
    ResponseFlowOps, ResponseHeadersOps, ResponseTrailersOps,
};
use crate::abi::proxy_wasm_ext::hostcalls;
use crate::abi::proxy_wasm_ext::types::{BufferType, Bytes, MapType};
use crate::host;

pub(super) struct Host;

impl RequestHeadersOps for Host {
    fn get_request_headers(&self) -> host::Result<Vec<(String, String)>> {
        hostcalls::get_map(MapType::HttpRequestHeaders)
    }

    fn set_request_headers(&self, headers: Vec<(&str, &str)>) -> host::Result<()> {
        hostcalls::set_map(MapType::HttpRequestHeaders, headers)
    }

    fn get_request_header(&self, name: &str) -> host::Result<Option<String>> {
        hostcalls::get_map_value(MapType::HttpRequestHeaders, name)
    }

    fn set_request_header(&self, name: &str, value: Option<&str>) -> host::Result<()> {
        hostcalls::set_map_value(MapType::HttpRequestHeaders, name, value)
    }

    fn add_request_header(&self, name: &str, value: &str) -> host::Result<()> {
        hostcalls::add_map_value(MapType::HttpRequestHeaders, name, value)
    }
}

impl RequestBodyOps for Host {
    fn get_request_body(&self, start: usize, max_size: usize) -> host::Result<Option<Bytes>> {
        hostcalls::get_buffer(BufferType::HttpRequestBody, start, max_size)
    }
}

impl RequestTrailersOps for Host {
    fn get_request_trailers(&self) -> host::Result<Vec<(String, String)>> {
        hostcalls::get_map(MapType::HttpRequestTrailers)
    }

    fn set_request_trailers(&self, trailers: Vec<(&str, &str)>) -> host::Result<()> {
        hostcalls::set_map(MapType::HttpRequestTrailers, trailers)
    }

    fn get_request_trailer(&self, name: &str) -> host::Result<Option<String>> {
        hostcalls::get_map_value(MapType::HttpRequestTrailers, name)
    }

    fn set_request_trailer(&self, name: &str, value: Option<&str>) -> host::Result<()> {
        hostcalls::set_map_value(MapType::HttpRequestTrailers, name, value)
    }

    fn add_request_trailer(&self, name: &str, value: &str) -> host::Result<()> {
        hostcalls::add_map_value(MapType::HttpRequestTrailers, name, value)
    }
}

impl ResponseHeadersOps for Host {
    fn get_response_headers(&self) -> host::Result<Vec<(String, String)>> {
        hostcalls::get_map(MapType::HttpResponseHeaders)
    }

    fn set_response_headers(&self, headers: Vec<(&str, &str)>) -> host::Result<()> {
        hostcalls::set_map(MapType::HttpResponseHeaders, headers)
    }

    fn get_response_header(&self, name: &str) -> host::Result<Option<String>> {
        hostcalls::get_map_value(MapType::HttpResponseHeaders, name)
    }

    fn set_response_header(&self, name: &str, value: Option<&str>) -> host::Result<()> {
        hostcalls::set_map_value(MapType::HttpResponseHeaders, name, value)
    }

    fn add_response_header(&self, name: &str, value: &str) -> host::Result<()> {
        hostcalls::add_map_value(MapType::HttpResponseHeaders, name, value)
    }
}

impl ResponseBodyOps for Host {
    fn get_response_body(&self, start: usize, max_size: usize) -> host::Result<Option<Bytes>> {
        hostcalls::get_buffer(BufferType::HttpResponseBody, start, max_size)
    }
}

impl ResponseTrailersOps for Host {
    fn get_response_trailers(&self) -> host::Result<Vec<(String, String)>> {
        hostcalls::get_map(MapType::HttpResponseTrailers)
    }

    fn set_response_trailers(&self, headers: Vec<(&str, &str)>) -> host::Result<()> {
        hostcalls::set_map(MapType::HttpResponseTrailers, headers)
    }

    fn get_response_trailer(&self, name: &str) -> host::Result<Option<String>> {
        hostcalls::get_map_value(MapType::HttpResponseTrailers, name)
    }

    fn set_response_trailer(&self, name: &str, value: Option<&str>) -> host::Result<()> {
        hostcalls::set_map_value(MapType::HttpResponseTrailers, name, value)
    }

    fn add_response_trailer(&self, name: &str, value: &str) -> host::Result<()> {
        hostcalls::add_map_value(MapType::HttpResponseTrailers, name, value)
    }
}

impl RequestFlowOps for Host {
    fn resume_request(&self) -> host::Result<()> {
        hostcalls::resume_http_request()
    }

    fn send_response(
        &self,
        status_code: u32,
        headers: Vec<(&str, &str)>,
        body: Option<&[u8]>,
    ) -> host::Result<()> {
        hostcalls::send_http_response(status_code, headers, body)
    }

    fn clear_route_cache(&self) -> host::Result<()> {
        hostcalls::clear_http_route_cache()
    }
}

impl ResponseFlowOps for Host {
    fn resume_response(&self) -> host::Result<()> {
        hostcalls::resume_http_response()
    }
}
