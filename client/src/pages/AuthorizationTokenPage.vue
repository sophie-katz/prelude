<template>
    <div class="q-ma-md" style="white-space: pre-wrap; overflow-wrap: break-word;">
        <h5 class="q-mb-none">Authentication token <q-btn flat round icon="content_copy" @click="copyToClipboard"></q-btn></h5>

        {{ token() }}
    </div>
</template>

<script lang="ts">
// TODO: https://github.com/sophie-katz/portobello/issues/11

import { defineComponent } from 'vue';
import { useAuthenticationStore } from "../stores/authentication";

export default defineComponent({
    name: 'AuthorizationTokenPage',
    setup() {
        const authenticationStore = useAuthenticationStore();

        const token = () => authenticationStore.keycloakInstance?.token;

        return {
            token,
            copyToClipboard: () => {
                navigator.clipboard.writeText(token());
            }
        };
    },
});
</script>
