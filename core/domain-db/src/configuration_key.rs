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

use crate::configuration_type::ConfigurationType;
use domain_common::CONFIGURATION_KEY_NAME_REGEX;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationKey<'configuration_type> {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1))]
    #[validate(regex = "CONFIGURATION_KEY_NAME_REGEX")]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(range(min = 1))]
    pub type_id: i32,
    #[serde(skip_serializing, skip_deserializing)]
    pub type_value: Option<&'configuration_type ConfigurationType>,
    pub optional: bool,
    pub allows_multiple: bool,
    pub allows_user_override: bool,
}

pub type ConfigurationKeySet<'type_set> = HashMap<i32, ConfigurationKey<'type_set>>;
