<template>
    <openapi-explorer spec-url="http://localhost:9000/openapi.yml" />
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import 'openapi-explorer';
  
  export default defineComponent({
    name: 'OpenAPIExplorerPage',
  });
  </script>
  