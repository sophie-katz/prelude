import { defineStore } from 'pinia';
import { DefaultApi } from "@core/api-bindings-client-typescript-fetch";

export const useAPIStore = defineStore('api', {
  state: () => ({
    api: new DefaultApi({
      basePath: "http://localhost:9000/api",
      middleware: []
    })
  })
});
