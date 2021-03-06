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

use super::{ConfigureOps, ContextOps, DrainOps};
use crate::abi::proxy_wasm::hostcalls;
use crate::host::{self, ByteString};

pub(super) struct Host;

impl ContextOps for Host {
    fn configuration(&self, start: usize, max_size: usize) -> host::Result<ByteString> {
        hostcalls::get_plugin_configuration(start, max_size)
    }
}

impl ConfigureOps for Host {}

impl DrainOps for Host {
    fn done(&self) -> host::Result<()> {
        hostcalls::done()
    }
}
