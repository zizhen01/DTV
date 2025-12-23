<template>
  <div class="app">
    <div class="sidebar-container" v-show="!shouldHidePlayerChrome">
      <div class="sidebar-glass-panel">
        <Sidebar 
          :followedAnchors="followedStreamersFromStore" 
          @selectAnchor="handleStreamerSelect"
          @unfollow="handleUnfollowStore"
          @reorderList="handleReorderListStore"
        />
      </div>
    </div>
    
    <div class="main-content">
      <div class="content-glass-panel">
        <Header 
          v-show="!shouldHidePlayerChrome"
          @select-anchor="handleStreamerSelect"
          @follow="handleFollowStore"
          @unfollow="handleUnfollowStore"
        />
        <div class="page-container">
          <router-view 
            v-slot="{ Component, route }" 
            @follow="handleFollowStore"
            @unfollow="handleUnfollowStore"
            @fullscreen-change="handleFullscreenChange"
          >
            <transition name="fade" mode="out-in">
              <keep-alive :include="['HomeView', 'DouyinHomeView', 'HuyaHomeView', 'BilibiliHomeView']">
                <component :is="Component" :key="route.path" />
              </keep-alive>
            </transition>
          </router-view>
        </div>
      </div>
    </div>

    <!-- SVG Squircle Definition -->
    <svg width="0" height="0" style="position: absolute; pointer-events: none;">
      <defs>
        <clipPath id="squircle-clip" clipPathUnits="objectBoundingBox">
          <path d="M0.5 0 C0.9995 0 1 0.0005 1 0.5 C1 0.9995 0.9995 1 0.5 1 C0.0005 1 0 0.9995 0 0.5 C0 0.0005 0.0005 0 0.5 0 Z" />
        </clipPath>
        <mask id="squircle-mask" maskUnits="objectBoundingBox" maskContentUnits="objectBoundingBox">
          <rect width="1" height="1" fill="black" />
          <path d="M0.5 0 C0.9995 0 1 0.0005 1 0.5 C1 0.9995 0.9995 1 0.5 1 C0.0005 1 0 0.9995 0 0.5 C0 0.0005 0.0005 0 0.5 0 Z" fill="white" />
        </mask>
      </defs>
    </svg>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { platform as detectPlatform } from '@tauri-apps/plugin-os'
import type { UnlistenFn } from '@tauri-apps/api/event'
import Sidebar from './Sidebar.vue'
import Header from './Header.vue'
import { useFollowStore } from '../store/followStore'
import type { FollowedStreamer } from '../platforms/common/types'
import { Platform } from '../platforms/common/types'

const router = useRouter()
const followStore = useFollowStore()

const followedStreamersFromStore = computed(() => followStore.getFollowedStreamers)

const isPlayerFullscreen = ref(false)
const isWindowMaximized = ref(false)
const currentWindow = typeof window !== 'undefined' ? getCurrentWindow() : null
let unlistenResize: UnlistenFn | null = null

const syncWindowMaximizedState = async () => {
  if (!currentWindow) return
  try {
    isWindowMaximized.value = await currentWindow.isMaximized()
  } catch (error) {
    console.error('[MainLayout] Failed to query maximized state', error)
  }
}

onMounted(async () => {
  if (!currentWindow) return
  await syncWindowMaximizedState()
  try {
    unlistenResize = await currentWindow.onResized(syncWindowMaximizedState)
  } catch (error) {
    console.error('[MainLayout] Failed to listen for resize events', error)
  }
  try {
    const osName = await detectPlatform()
    document.documentElement.setAttribute('data-platform', osName)
  } catch (error) {
    console.error('[MainLayout] Failed to detect platform', error)
  }
})

onBeforeUnmount(async () => {
  if (unlistenResize) {
    await unlistenResize()
    unlistenResize = null
  }
})

const isPlayerRoute = computed(() => {
  const name = router.currentRoute.value.name
  return (
    name === 'douyuPlayer' ||
    name === 'douyinPlayer' ||
    name === 'huyaPlayer' ||
    name === 'bilibiliPlayer'
  )
})

const shouldHidePlayerChrome = computed(() => {
  return isPlayerRoute.value && (isPlayerFullscreen.value || isWindowMaximized.value)
})

const handleStreamerSelect = (streamer: FollowedStreamer) => {
  let routeName = '';
  if (streamer.platform === Platform.DOUYU) {
    routeName = 'douyuPlayer';
  } else if (streamer.platform === Platform.DOUYIN) {
    routeName = 'douyinPlayer';
  } else if (streamer.platform === Platform.HUYA) {
    routeName = 'huyaPlayer';
  } else if (streamer.platform === Platform.BILIBILI) {
    routeName = 'bilibiliPlayer';
  } else {
    console.error('Unsupported platform for player:', streamer.platform);
    return;
  }

  router.push({
    name: routeName,
    params: {
      roomId: streamer.id,
    },
  });
}

const handleFollowStore = (streamer: FollowedStreamer) => {
  followStore.followStreamer(streamer)
}

const handleUnfollowStore = (payload: {platform: Platform, id: string} | string) => {
  if (typeof payload === 'string') {
    followStore.unfollowStreamer(Platform.DOUYU, payload)
  } else {
    followStore.unfollowStreamer(payload.platform, payload.id)
  }
}

const handleReorderListStore = (reorderedList: FollowedStreamer[]) => {
  followStore.updateOrder(reorderedList)
}

const handleFullscreenChange = (isFullscreen: boolean) => {
  isPlayerFullscreen.value = isFullscreen
}
</script>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  background: transparent;
  background-image: var(--page-mesh);
  background-attachment: fixed;
  color: var(--primary-text);
  overflow: hidden;
  font-family: 'Outfit', 'Inter', system-ui, -apple-system, sans-serif;
  transition: background-color 0.5s ease;
}

.sidebar-container {
  flex-shrink: 0;
  height: 100%;
}

.sidebar-glass-panel {
  height: 100%;
  display: flex;
  background: transparent;
  position: relative;
  overflow: hidden;
}

.sidebar-glass-panel::before {
  content: '';
  position: absolute;
  inset: 0;
  background-image: var(--sidebar-overlay);
  background-size: cover;
  background-position: center;
  opacity: var(--sidebar-overlay-opacity);
  pointer-events: none;
  z-index: 0;
}

.sidebar-glass-panel > * {
  position: relative;
  z-index: 1;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  padding: 0;
  background: transparent;
}

.content-glass-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--content-panel-bg, var(--glass-bg));
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: none;
  border-radius: 0;
  box-shadow: var(--glass-shadow);
  overflow: hidden;
  position: relative;
}

.page-container {
  flex: 1;
  overflow: hidden;
  position: relative;
}

/* Forest Glass Background Enhancements */
.app::before {
  content: '';
  position: absolute;
  inset: 0;
  background-image: var(--ambient-bg-image), var(--page-mesh);
  background-size: cover, cover;
  background-position: center, center;
  filter: var(--ambient-bg-filter, blur(18px) saturate(1.1) brightness(0.9));
  opacity: var(--ambient-bg-opacity);
  z-index: -1;
  pointer-events: none;
}

/* Fullscreen mode handling */
.app.hide-ui .main-content {
  padding: 0;
}

.app.hide-ui .content-glass-panel {
  border-radius: 0;
  border: none;
}

.fade-enter-active,
.fade-leave-active {
  transition: all 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}

.fade-enter-from {
  opacity: 0;
  transform: scale(0.98) translateY(10px);
}

.fade-leave-to {
  opacity: 0;
  transform: scale(1.02) translateY(-10px);
}
</style>
