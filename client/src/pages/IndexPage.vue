<template>
  <q-page>
    <under-construction-small />
  </q-page>
</template>

<script lang="ts">
// import { Todo, Meta } from 'components/models';
import UnderConstructionSmall from '../components/UnderConstructionSmall.vue';
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'IndexPage',
  components: { UnderConstructionSmall }
});
</script>
