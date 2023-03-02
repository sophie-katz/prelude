<!--
MIT License

Copyright (c) 2023 Sophie Katz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
-->

<template>
  <q-layout view="hHh Lpr lFf">
    <q-header>
      <q-toolbar class="items-stretch bg-grey-3 text-primary">
        <q-btn
          flat
          icon="apps">
          <q-menu anchor="bottom left" style="width: 500px;" class="bg-grey-1">
            <div class="column">
              <div class="row col q-pa-md">
                <q-btn
                  class="q-mr-md col-5"
                  color="primary"
                  icon="computer"
                  label="Code"
                  size="md"
                  to="/code"
                  />

                <q-btn
                  class="col-5"
                  color="primary"
                  icon="bar_chart"
                  label="Dashboard"
                  size="md"
                  to="/dashboard"
                  />
              </div>

              <div class="row col q-pa-md">
                <q-btn
                  class="q-mr-md col-5"
                  color="primary"
                  icon="sync"
                  label="Deploy"
                  to="/deploy"
                  />

                <q-btn
                  class="col-5"
                  color="primary"
                  icon="description"
                  label="Document"
                  to="/document"
                  />
              </div>

              <div class="row col q-pa-md">
                <q-btn
                  class="col-5 q-pr-md"
                  color="primary"
                  icon="view_kanban"
                  label="Ticket"
                  to="/ticket"
                  />
              </div>
            </div>
          </q-menu>
        </q-btn>

        <q-toolbar-title
          class="column justify-center q-ml-lg">
          <q-breadcrumbs
            class="col-auto"
            active-color="primary">
            <q-breadcrumbs-el icon="home" />
            <q-breadcrumbs-el label="Tickets" />
            <q-breadcrumbs-el label="PBLO-1" />
          </q-breadcrumbs>
        </q-toolbar-title>

        <div class="column justify-center">
          <q-btn
            flat
            round>
            <q-avatar
                color="secondary"
                text-color="white">
                SK
            </q-avatar>

            <q-menu anchor="bottom left" class="bg-grey-1" style="width: 200px;" :offset="[0, 4]">
              <div class="column q-ma-md items-center">
                <div class="col">
                  {{ currentUserName() }}
                </div>

                <q-btn class="col-auto q-mt-md" color="primary" label="Sign out" @click="signOut" />
              </div>
            </q-menu>
          </q-btn>
        </div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <div class="column full-height">
        <div class="col">
          <under-construction-large />
        </div>
        <div class="col-auto text-center" style="opacity: 0.5;">
          Press [ to open/close
        </div>
      </div>
    </q-drawer>

    <q-page-container>
      <q-page>
        <div class="row absolute-left items-end">
          <div class="col column" style="margin-left: -12px; margin-bottom: 24px;">
            <q-btn
              class="rotate-90"
              square
              unelevated
              :ripple="false"
              color="grey-3"
              text-color="black"
              size="xs"
              :icon="leftDrawerOpen ? 'expand_more' : 'expand_less'"
              @click="toggleLeftDrawer" />
          </div>
        </div>
        <router-view />
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import UnderConstructionLarge from '../components/UnderConstructionLarge.vue';
import { useAuthenticationStore } from "../stores/authentication";
import { useAPIStore } from "../stores/api";

const authenticationStore = useAuthenticationStore();
const apiStore = useAPIStore();

export default defineComponent({
    name: "MainLayout",
    setup() {
        const leftDrawerOpen = ref(false);

        function toggleLeftDrawer() {
            leftDrawerOpen.value = !leftDrawerOpen.value;
        };

        document.addEventListener("keydown", function (event) {
          if (event.key == "[") {
            toggleLeftDrawer();
          }
        });

        apiStore.api.getConfigurationTypes().then((response) => {
          console.log(response);
        });

        return {
            leftDrawerOpen,
            toggleLeftDrawer,
            signOut: () => {
              authenticationStore.keycloakInstance?.logout({
                redirectUri: "http://localhost:9000/#/"
              });
            },
            currentUserName: () => authenticationStore.keycloakInstance?.idTokenParsed?.preferred_username,
        };
    },
    components: { UnderConstructionLarge }
});
</script>
