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
