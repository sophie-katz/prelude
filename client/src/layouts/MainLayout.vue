<template>
  <q-layout view="hHh Lpr lFf">
    <q-header elevated>
      <q-toolbar class="items-stretch">
        <q-btn
          flat
          icon="apps">
          <q-menu anchor="bottom left" style="width: 500px;">
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

        <!-- <q-toolbar-title>
          Quasar App
        </q-toolbar-title> -->

        <div class="column justify-center q-ml-lg">
          <q-breadcrumbs
            class="col-auto"
            active-color="white">
            <q-breadcrumbs-el icon="home" />
            <q-breadcrumbs-el label="Tickets" />
            <q-breadcrumbs-el label="PBLO-1" />
          </q-breadcrumbs>
        </div>

        <!-- <div>Quasar v{{ $q.version }}</div> -->
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
        <div class="row absolute-left items-center">
          <div class="col column" style="margin-left: -12px;">
            <q-btn
              class="rotate-90"
              square
              unelevated
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

        return {
            leftDrawerOpen,
            toggleLeftDrawer
        };
    },
    components: { UnderConstructionLarge }
});
</script>
