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

//! `Envoy` `Network Filter API`.

use crate::abi::proxy_wasm_ext::types::{Action, Bytes, PeerType};
use crate::extension::Result;
use crate::host;
use crate::host::http::client as http_client;

pub use context::FilterContext;

pub mod context;
pub mod ops;

pub type FilterStatus = Action;

pub trait Filter {
    fn on_new_connection(&mut self) -> Result<FilterStatus> {
        Ok(FilterStatus::Continue)
    }

    fn on_downstream_data(
        &mut self,
        _data_size: usize,
        _end_of_stream: bool,
        _ops: &dyn DownstreamDataOps,
    ) -> Result<FilterStatus> {
        Ok(FilterStatus::Continue)
    }

    fn on_downstream_close(&mut self, _peer_type: PeerType) -> Result<()> {
        Ok(())
    }

    fn on_upstream_data(
        &mut self,
        _data_size: usize,
        _end_of_stream: bool,
        _ops: &dyn UpstreamDataOps,
    ) -> Result<FilterStatus> {
        Ok(FilterStatus::Continue)
    }

    fn on_upstream_close(&mut self, _peer_type: PeerType) -> Result<()> {
        Ok(())
    }

    fn on_connection_complete(&mut self) -> Result<()> {
        Ok(())
    }

    // Http Client callbacks

    fn on_http_call_response(
        &mut self,
        _request: http_client::RequestHandle,
        _num_headers: usize,
        _body_size: usize,
        _num_trailers: usize,
        _filter_ops: &dyn Ops,
        _http_client_ops: &dyn http_client::ResponseOps,
    ) -> Result<()> {
        Ok(())
    }
}

pub trait DownstreamDataOps {
    fn get_downstream_data(&self, start: usize, max_size: usize) -> host::Result<Option<Bytes>>;
}

pub trait UpstreamDataOps {
    fn get_upstream_data(&self, start: usize, max_size: usize) -> host::Result<Option<Bytes>>;
}

pub trait Ops: DownstreamDataOps + UpstreamDataOps {
    fn as_downstream_data_ops(&self) -> &dyn DownstreamDataOps;

    fn as_upstream_data_ops(&self) -> &dyn UpstreamDataOps;
}

impl<T> Ops for T
where
    T: DownstreamDataOps + UpstreamDataOps,
{
    fn as_downstream_data_ops(&self) -> &dyn DownstreamDataOps {
        self
    }

    fn as_upstream_data_ops(&self) -> &dyn UpstreamDataOps {
        self
    }
}

impl dyn Ops {
    pub fn default() -> &'static dyn Ops {
        &ops::Host
    }
}
