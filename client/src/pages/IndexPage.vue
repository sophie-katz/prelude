<template>
  <q-page>
    <under-construction-large />
  </q-page>
</template>

<script lang="ts">
// import { Todo, Meta } from 'components/models';
import UnderConstructionLarge from '../components/UnderConstructionLarge.vue';
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'IndexPage',
  components: { UnderConstructionLarge }
});
</script>
