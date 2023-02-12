<template>
    <div class="row items-center">
        <q-img
            src="../assets/under-construction-64.png"
            spinner-color="black"
            class="col-1 q-ma-xs"
            style="max-width: 1rem; max-height: 1rem;" />

        <div class="col-auto">
            Under construction
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "UnderConstructionSmall"
});
</script>
