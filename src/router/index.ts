import { createRouter, createWebHistory } from "vue-router";
import PlatformHomeView from "../pages/PlatformHomeView.vue";
import UniversalPlayerView from "../pages/UniversalPlayerView.vue";
import { Platform } from "../platforms/common/types";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/:platform?",
      name: "PlatformHome",
      component: PlatformHomeView,
    },
    {
      path: "/player/:platform/:roomId",
      name: "UniversalPlayer",
      component: UniversalPlayerView,
      props: (route) => ({
        roomId: route.params.roomId,
        platform: (route.params.platform as string).toUpperCase() as Platform,
      }),
    },
  ],
});

export default router;
