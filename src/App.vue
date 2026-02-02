<template>
  <div class="flex h-screen flex-col overflow-hidden bg-app-bg text-text-main">
    <div
      class="fixed left-0 right-0 top-0 z-40 h-6"
      data-tauri-drag-region
    ></div>
    <!-- Global Dynamic Island -->
    <DynamicIsland @open-search="showSearchModal = true" />

    <!-- Global Search Modal -->
    <SearchModal v-model="showSearchModal" @search="handleSearch" />

    <PlayerManager @fullscreen-change="handleFullscreenChange" />

    <div class="flex min-h-0 flex-1 overflow-hidden">
      <main
        class="relative min-h-0 flex-1 overflow-y-auto"
        :class="{ 'p-0': isPlayerRoute }"
      >
        <router-view
          v-slot="{ Component, route }"
          @follow="handleFollowStore"
          @unfollow="handleUnfollowStore"
          @fullscreen-change="handleFullscreenChange"
        >
          <keep-alive :include="['ChannelList', 'StreamRoom']">
            <component
              :is="Component"
              :key="route.name === 'StreamRoom' ? route.path : 'home'"
            />
          </keep-alive>
        </router-view>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import PlayerManager from "./features/player/components/PlayerManager.vue";
import DynamicIsland from "./components/layout/DynamicIsland.vue";
import SearchModal from "./components/ui/SearchModal.vue";
import type { FollowedStreamer } from "./types/models/streamer";
import { useFollowStore } from "./store/followStore";
import "./styles/global.css";

const router = useRouter();
const route = useRoute();
const followStore = useFollowStore();

const showSearchModal = ref(false);
const isPlayerFullscreen = ref(false);

const isPlayerRoute = computed(() => route.name === "StreamRoom");

const handleFollowStore = (streamer: FollowedStreamer) => {
  followStore.followStreamer(streamer);
};

const handleUnfollowStore = (roomId: string) => {
  const match = followStore.followedStreamers.find((s) => s.id === roomId);
  if (match) {
    followStore.unfollowStreamer(match.platform, match.id);
  }
};

const handleFullscreenChange = (isFullscreen: boolean) => {
  isPlayerFullscreen.value = isFullscreen;
};

const handleSearch = (query: string) => {
  // Implement actual search navigation logic here
};

// NOTE: Window resize / maximize listeners are handled elsewhere if needed.
</script>
