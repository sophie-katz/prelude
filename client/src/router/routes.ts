import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/IndexPage.vue') },
      { path: 'dashboard', component: () => import('pages/DashboardPage.vue') },
      { path: 'deploy', component: () => import('pages/DeployPage.vue') },
      { path: 'document', component: () => import('pages/DocumentPage.vue') },
      { path: 'search', component: () => import('pages/SearchPage.vue') },
      { path: 'code', component: () => import('pages/CodePage.vue') },
      { path: 'ticket', component: () => import('pages/TicketPage.vue') }
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
