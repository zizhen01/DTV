<template>
  <div class="flex h-screen overflow-hidden">
    <Sidebar
      v-show="!shouldHidePlayerChrome"
      :is-collapsed="isSidebarCollapsed"
      :followed-anchors="followedStreamers"
      @toggle="toggleSidebar"
      @select-anchor="handleSelectAnchor"
      @unfollow="handleUnfollowStore"
      @reorder-list="handleReorderListStore"
    />

    <div class="flex min-w-0 flex-1 flex-col bg-[var(--bg-primary)] transition-none">
      <Navbar
        v-show="!shouldHidePlayerChrome"
        :theme="theme"
        :active-platform="activePlatform"
        @theme-toggle="toggleTheme"
        @platform-change="handlePlatformChange"
        @select-anchor="handleSelectAnchorFromSearch"
      />

      <main class="relative flex-1 overflow-y-auto px-4 pb-3 pt-4" :class="{ 'p-0': isPlayerRoute }">
        <router-view
          v-slot="{ Component, route }"
          @follow="handleFollowStore"
          @unfollow="handleUnfollowStore"
          @fullscreen-change="handleFullscreenChange"
        >
          <transition name="fade" mode="out-in">
            <keep-alive :include="['DouyuHomeView', 'DouyinHomeView', 'HuyaHomeView', 'BilibiliHomeView']">
              <component :is="Component" :key="route.path" />
            </keep-alive>
          </transition>
        </router-view>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import Navbar from './layout/Navbar.vue';
import Sidebar from './layout/Sidebar.vue';
import type { Platform as UiPlatform } from './layout/types';
import { useThemeStore } from './stores/theme';
import { useFollowStore } from './store/followStore';
import { Platform } from './platforms/common/types';
import type { FollowedStreamer } from './platforms/common/types';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { UnlistenFn } from '@tauri-apps/api/event';
import './styles/global.css';

const router = useRouter();
const route = useRoute();
const followStore = useFollowStore();

const isSidebarCollapsed = ref(false);
const isPlayerFullscreen = ref(false);
const isWindowMaximized = ref(false);
const currentWindow = typeof window !== 'undefined' ? getCurrentWindow() : null;
let unlistenResize: UnlistenFn | null = null;

const themeStore = useThemeStore();
const theme = computed(() => themeStore.getEffectiveTheme());

const routePlatform = computed<UiPlatform>(() => {
  const name = route.name as string | undefined;
  const path = route.path;
  if (name === 'douyinPlayer' || name === 'DouyinHome' || path.startsWith('/douyin')) return 'douyin';
  if (name === 'huyaPlayer' || name === 'HuyaHome' || path.startsWith('/huya')) return 'huya';
  if (name === 'bilibiliPlayer' || name === 'BilibiliHome' || path.startsWith('/bilibili')) return 'bilibili';
  return 'douyu';
});

const activePlatform = computed<UiPlatform>(() => routePlatform.value);

const followedStreamers = computed<FollowedStreamer[]>(() => followStore.getFollowedStreamers);

const syncWindowMaximizedState = async () => {
  if (!currentWindow) {
    return;
  }
  try {
    isWindowMaximized.value = await currentWindow.isMaximized();
  } catch (error) {
    console.error('[App] Failed to query maximized state', error);
  }
};

onMounted(async () => {
  if (!currentWindow) {
    return;
  }
  await syncWindowMaximizedState();
  try {
    unlistenResize = await currentWindow.onResized(syncWindowMaximizedState);
  } catch (error) {
    console.error('[App] Failed to listen for resize events', error);
  }
});

onBeforeUnmount(async () => {
  if (unlistenResize) {
    await unlistenResize();
    unlistenResize = null;
  }
});

const isPlayerRoute = computed(() => {
  const name = route.name as string | undefined;
  return (
    name === 'douyuPlayer' ||
    name === 'douyinPlayer' ||
    name === 'huyaPlayer' ||
    name === 'bilibiliPlayer'
  );
});

const shouldHidePlayerChrome = computed(() => (
  isPlayerRoute.value && (isPlayerFullscreen.value || isWindowMaximized.value)
));

const toggleSidebar = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

const toggleTheme = () => {
  themeStore.setUserPreference(theme.value === 'light' ? 'dark' : 'light');
};

const handlePlatformChange = (platform: UiPlatform | 'all') => {
  if (platform === 'douyin') {
    router.push({ name: 'DouyinHome' });
  } else if (platform === 'huya') {
    router.push({ name: 'HuyaHome' });
  } else if (platform === 'bilibili') {
    router.push({ name: 'BilibiliHome' });
  } else {
    router.push({ name: 'DouyuHome' });
  }
};

const handleSelectAnchor = (streamer: FollowedStreamer) => {
  if (streamer.platform === Platform.DOUYIN) {
    router.push({ name: 'douyinPlayer', params: { roomId: streamer.id } });
  } else if (streamer.platform === Platform.HUYA) {
    router.push({ name: 'huyaPlayer', params: { roomId: streamer.id } });
  } else if (streamer.platform === Platform.BILIBILI) {
    router.push({ name: 'bilibiliPlayer', params: { roomId: streamer.id } });
  } else {
    router.push({ name: 'douyuPlayer', params: { roomId: streamer.id } });
  }
};

const handleSelectAnchorFromSearch = (payload: { id: string; platform: Platform; nickname?: string; avatarUrl?: string | null }) => {
  handleSelectAnchor({
    id: payload.id,
    platform: payload.platform,
    nickname: payload.nickname ?? payload.id,
    avatarUrl: payload.avatarUrl ?? '',
    currentRoomId: payload.id,
    liveStatus: 'UNKNOWN',
  });
};

const handleFollowStore = (streamer: FollowedStreamer) => {
  followStore.followStreamer(streamer);
};

const handleUnfollowStore = (payload: { platform: Platform; id: string } | string) => {
  if (typeof payload === 'string') {
    followStore.unfollowStreamer(Platform.DOUYU, payload);
  } else {
    followStore.unfollowStreamer(payload.platform, payload.id);
  }
};

const handleReorderListStore = (reorderedList: FollowedStreamer[]) => {
  followStore.updateOrder(reorderedList);
};

const handleFullscreenChange = (isFullscreen: boolean) => {
  isPlayerFullscreen.value = isFullscreen;
};
</script>

