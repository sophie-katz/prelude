// MIT License
//
// Copyright (c) 2023 Sophie Katz
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    configuration_entry_item::ConfigurationEntryItem,
    configuration_entry_user::ConfigurationEntryUser, configuration_key::ConfigurationKey,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationEntry<'configuration_type, 'key> {
    #[validate(range(min = 1))]
    pub key_id: i32,
    #[serde(skip_serializing, skip_deserializing)]
    pub key_value: Option<&'key ConfigurationKey<'configuration_type>>,
    #[validate(length(min = 1))]
    pub items_global: Vec<ConfigurationEntryItem>,
    pub user: Option<ConfigurationEntryUser>,
}

pub type ConfigurationEntrySet<'type_set, 'key_set> =
    HashMap<i32, ConfigurationEntry<'type_set, 'key_set>>;
