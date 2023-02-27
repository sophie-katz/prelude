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

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    pub static ref CONFIGURATION_TYPE_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    pub static ref CONFIGURATION_KEY_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.]+$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationTypeResponse {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1))]
    #[validate(regex = "CONFIGURATION_TYPE_NAME_REGEX")]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
}

pub type ConfigurationTypeSetResponse = Vec<ConfigurationTypeResponse>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationKeyResponse {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1))]
    #[validate(regex = "CONFIGURATION_KEY_NAME_REGEX")]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[serde(rename = "type")]
    pub configuration_type: ConfigurationTypeResponse,
    pub optional: bool,
    #[serde(rename = "allowsMultiple")]
    pub allows_multiple: bool,
    #[serde(rename = "allowsUserOverride")]
    pub allows_user_override: bool,
}

pub type ConfigurationKeySetResponse = Vec<ConfigurationKeyResponse>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationValueResponse {
    #[serde(rename = "asBoolean")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asInteger")]
    pub as_integer: Option<i64>,
    #[serde(rename = "asFloat")]
    pub as_float: Option<f64>,
    #[serde(rename = "asString")]
    pub as_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationEntryItemResponse {
    #[validate(range(min = 1))]
    pub id: i32,
    pub value: ConfigurationValueResponse,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationEntryUserResponse {
    #[validate(length(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: String,
    #[validate(length(min = 1))]
    pub items: Vec<ConfigurationEntryItemResponse>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationEntryResponse {
    key: ConfigurationKeyResponse,
    #[validate(length(min = 1))]
    #[serde(rename = "itemsGlobal")]
    pub items_global: Vec<ConfigurationEntryItemResponse>,
    pub user: Option<ConfigurationEntryUserResponse>,
}

pub type ConfigurationEntrySetResponse = Vec<ConfigurationEntryResponse>;
