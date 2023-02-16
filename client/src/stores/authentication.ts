import { defineStore } from 'pinia';

export const useAuthenticationStore = defineStore('authentication', {
  state: (): { [keycloakInstance: string]: object | null} => ({
    keycloakInstance: null
  }),
});
