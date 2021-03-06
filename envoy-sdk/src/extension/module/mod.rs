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

use std::collections::HashMap;

use crate::abi::proxy_wasm::traits::RootContext;
use crate::extension::Result;

pub use self::config::Module;
pub use self::start::install;

mod config;
mod dispatcher;
mod start;

type ContextFactory = dyn FnMut(u32) -> Result<Box<dyn RootContext>>;
type ContextFactoryHashMap = HashMap<String, Box<ContextFactory>>;
