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
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

import Keycloak from 'keycloak-js';
import { boot } from 'quasar/wrappers';
import { useAuthenticationStore } from '../stores/authentication';

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(async ({app, store}) => {
  const authenticationStore = useAuthenticationStore();

  const keycloak = new Keycloak({
    url: "http://localhost:9003",
    realm: "portobello",
    clientId: "portobello"
  });

  console.log("Initializing Keycloak.JS");

  keycloak.init({
    onLoad: "login-required",
    responseMode: "query"
  }).then((auth) => {
    if (!auth) {
      console.error("Could not authenticate");
      window.location.reload();
    } else {
      console.log(`Authenticated - token: ${JSON.stringify(keycloak.token)}`);
      authenticationStore.keycloakInstance = keycloak;
    }
  }).catch((error) => {
    console.error(`Failed to initialize Keycloak.JS: ${JSON.stringify(error)}`);
  });

  setInterval(() => {
    keycloak.updateToken(70).then((refreshed) => {
      if (refreshed) {
        console.log(`Token refreshed: ${refreshed}`);
      } else if (keycloak.tokenParsed && keycloak.tokenParsed.exp && keycloak.timeSkew) {
        console.warn(`Token not refreshed, valid for ${Math.round(keycloak.tokenParsed.exp + keycloak.timeSkew - new Date().getTime() / 1000)} seconds`);
      } else {
        console.error("Token parsing isn't working");
      }
    }).catch(() => {
      console.error("Failed to refresh token");
    });
  }, 6000);
});
