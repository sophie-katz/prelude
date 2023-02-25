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
use std::collections::HashMap;
use validator::Validate;

lazy_static! {
    pub static ref CONFIGURATION_TYPE_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    pub static ref CONFIGURATION_KEY_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.]+$").unwrap();
}

/// A configuration type object
///
/// Documentation can be found in `core/api-spec/openapi.yml`.
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
    pub configuration_type: ConfigurationTypeResponse,
    pub optional: bool,
    pub allows_multiple: bool,
    pub allows_user_override: bool,
}

pub type ConfigurationKeySetResponse = Vec<ConfigurationKeyResponse>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationValueResponse {
    pub as_boolean: Option<bool>,
    pub as_integer: Option<i64>,
    pub as_float: Option<f64>,
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
    pub userId: String,
    #[validate(length(min = 1))]
    pub items: Vec<ConfigurationEntryItemResponse>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Validate)]
pub struct ConfigurationEntryResponse {
    key: ConfigurationKeyResponse,
    #[validate(length(min = 1))]
    pub items_global: Vec<ConfigurationEntryItemResponse>,
    pub user: Option<ConfigurationEntryUserResponse>,
}

pub type ConfigurationEntrySetResponse = Vec<ConfigurationEntryResponse>;

// #[cfg(test)]
// mod tests {
//     use validator::Validate;

//     use super::UserResponse;

//     #[test]
//     fn validate_good_no_icon() {
//         assert_eq!(
//             UserResponse {
//                 id: 1,
//                 username: "admin".to_owned(),
//                 icon: None,
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_good_username_special_characters() {
//         assert_eq!(
//             UserResponse {
//                 id: 1,
//                 username: ".-_admin.-_".to_owned(),
//                 icon: None,
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_good_with_icon() {
//         assert_eq!(
//             UserResponse {
//                 id: 1,
//                 username: "admin".to_owned(),
//                 icon: Some("data:image/jpeg;base64, asdf".to_owned()),
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_id_0() {
//         assert_ne!(
//             UserResponse {
//                 id: 0,
//                 username: "admin".to_owned(),
//                 icon: None
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_id_negative() {
//         assert_ne!(
//             UserResponse {
//                 id: -1,
//                 username: "admin".to_owned(),
//                 icon: None
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_username_empty() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "".to_owned(),
//                 icon: None
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_username_invalid_character() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "admin?".to_owned(),
//                 icon: None
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_username_all_special_characters() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "-._".to_owned(),
//                 icon: None
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_icon_empty() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "admin".to_owned(),
//                 icon: Some("".to_owned())
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_icon_not_base64_0() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "admin".to_owned(),
//                 icon: Some("data:image/jpeg;base64, ".to_owned())
//             }
//             .validate(),
//             Ok(())
//         );
//     }

//     #[test]
//     fn validate_bad_icon_not_base64_1() {
//         assert_ne!(
//             UserResponse {
//                 id: 1,
//                 username: "admin".to_owned(),
//                 icon: Some("ata:image/jpeg;base64, asdf".to_owned())
//             }
//             .validate(),
//             Ok(())
//         );
//     }
// }
