import VueKeyCloak from 'keycloak-js';
import { boot } from 'quasar/wrappers';

// "async" is optional;
// more info on params: https://v2.quasar.dev/quasar-cli/boot-files
export default boot(async ({app, store}) => {
  const keycloak = new VueKeyCloak({
    url: "http://localhost:8080",
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
      console.log("Authenticated");
      app.config.globalProperties.$keycloak = keycloak;
    }
  }).catch((error) => {
    console.error(`Failed to initialize Keycloak.JS: ${JSON.stringify(error)}`);
  });

  setInterval(() => {
    keycloak.updateToken(70).then((refreshed) => {
      if (refreshed) {
        console.log(`Token refreshued: ${refreshed}`);
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
