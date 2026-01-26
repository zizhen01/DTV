import { createRouter, createWebHistory } from "vue-router";
import type { Platform } from "../types/app/platform";

const ChannelList = () => import("../pages/ChannelList.vue");
const StreamRoom = () => import("../pages/StreamRoom.vue");

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/channels/douyu",
    },
    {
      path: "/channels/:platform",
      name: "ChannelList",
      component: ChannelList,
    },
    {
      path: "/room/:platform/:roomId",
      name: "StreamRoom",
      component: StreamRoom,
      props: (route) => ({
        roomId: String(route.params.roomId ?? ""),
        platform: String(route.params.platform ?? "").toUpperCase() as Platform,
      }),
    },
  ],
});

export default router;
