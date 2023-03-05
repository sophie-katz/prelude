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

// TODO: https://github.com/sophie-katz/prelude/issues/11

// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
// use oauth2::{
//     basic::BasicClient, AuthUrl, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl,
// };
// use serde::{Deserialize, Serialize};

// #[get("/login")]
// pub fn login() {
//     // Go to http://localhost:8080/realms/prelude/.well-known/openid-configuration to get configuration details

//     let client = BasicClient::new(
//         // client_id
//         ClientId::new("prelude".to_owned()),
//         // client_secret
//         None,
//         // auth_url
//         AuthUrl::new(
//             "http://localhost:8080/realms/prelude/protocol/openid-connect/auth".to_owned(),
//         )
//         .expect("todo"),
//         // token_url
//         Some(
//             TokenUrl::new(
//                 "http://auth:8080/realms/prelude/protocol/openid-connect/token".to_owned(),
//             )
//             .expect("todo"),
//         ),
//     )
//     .set_redirect_uri(
//         RedirectUrl::new("http://localhost:9000/#/authorization/token".to_owned()).expect("todo"),
//     );

//     let (pkce_challenge, _) = PkceCodeChallenge::new_random_sha256();

//     let (auth_url, _) = client
//         .authorize_url(CsrfToken::new_random)
//         // .add_scope(Scope::new("read".to_owned()))
//         // .add_scope(Scope::new("write".to_owned()))
//         .set_pkce_challenge(pkce_challenge)
//         .url();

//     println!(
//         "Open URL in browser to get authorized credentials: {:}",
//         auth_url
//     );

//     // let token_result = client
//     //     .exchange_code(AuthorizationCode::new("asdf".to_owned()))
//     //     .set_pkce_verifier(pkce_verifier)
//     //     .request_async(async_http_client)
//     //     .await
//     //     .expect("todo");

//     // println!("Token result: {:#?}", token_result);
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     aud: String,
//     sub: String,
// }

// #[get("/test?<token>")]
// pub async fn test(token: &str) {
//     let mut validation = Validation::new(Algorithm::RS256);

//     validation.sub = Some("sophie".to_owned());
//     validation.set_audience(&["me"]);

//     let token_data =
//         decode::<Claims>(token, &DecodingKey::from_secret(b"secret"), &validation).expect("todo");

//     println!(
//         "Claims {:?}, header {:?}",
//         token_data.claims, token_data.header
//     );
// }
