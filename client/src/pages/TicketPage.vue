<template>
    <q-page>
      <under-construction-large />
    </q-page>
  </template>

  <script lang="ts">
  import UnderConstructionLarge from '../components/UnderConstructionLarge.vue';
  import { defineComponent, ref } from 'vue';

  export default defineComponent({
    name: 'TicketPage',
    components: { UnderConstructionLarge }
  });
  </script>
