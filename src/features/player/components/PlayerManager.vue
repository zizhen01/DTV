<template>
  <div
    class="fixed inset-0 z-10"
    :class="{
      'bg-black': isPlayerRoute,
      'pointer-events-none -z-10 opacity-0': !isPlayerRoute,
    }"
    :style="{
      top: isPlayerRoute || isPlayerFullscreen ? '0' : 'var(--navbar-height)',
    }"
  >
    <!-- Unified Container -->
    <div class="block h-full w-full">
      <!-- Active Players -->
      <div
        v-for="s in playerStore.activeStreamers"
        :key="`${s.platform}:${s.roomId}`"
        class="relative overflow-hidden bg-neutral-900 transition-all duration-300"
        :class="getWrapperClass(s)"
        v-show="shouldShowWrapper(s)"
      >
        <VideoPlayer
          :room-id="s.roomId"
          :platform="s.platform"
          :is-followed="followStore.isFollowed(s.platform, s.roomId)"
          :title="s.title"
          :anchor-name="s.anchorName"
          :avatar="s.avatar"
          :is-live="s.isLive"
          :is-active="isCurrentPlayer(s)"
          :is-visible="shouldShowWrapper(s)"
          @follow="handleFollow"
          @unfollow="handleUnfollow"
          @fullscreen-change="handleFullscreenChange"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useRoute } from "vue-router";
import { usePlayerStore } from "../../../store/playerStore";
import { useFollowStore } from "../../../store/followStore";
import VideoPlayer from "./VideoPlayer.vue";
import type { FollowedStreamer } from "../../../types/models/streamer";

const emit = defineEmits<{
  (e: "follow", streamer: FollowedStreamer): void;
  (e: "unfollow", roomId: string): void;
  (e: "fullscreen-change", isFullscreen: boolean): void;
}>();

const playerStore = usePlayerStore();
const followStore = useFollowStore();
const route = useRoute();

const isFullscreenInternal = ref(false);

const isPlayerFullscreen = computed(() => {
  return isFullscreenInternal.value;
});

const handleFollow = (streamer: FollowedStreamer) => {
  emit("follow", streamer);
};

const handleUnfollow = (roomId: string) => {
  emit("unfollow", roomId);
};

const handleFullscreenChange = (isFullscreen: boolean) => {
  isFullscreenInternal.value = isFullscreen;
  emit("fullscreen-change", isFullscreen);
};

const isPlayerRoute = computed(() => route.name === "StreamRoom");

const isCurrentPlayer = (s: any) => {
  // Active state depends only on store, enabling background playback
  const current = playerStore.currentStreamer;
  if (!current) return false;
  return s.platform === current.platform && s.roomId === current.roomId;
};

const shouldShowWrapper = (s: any) => {
  // Only show visible player when in StreamRoom and it matches route params
  if (isPlayerRoute.value) {
    const routePlatform = route.params.platform?.toString().toLowerCase();
    const routeRoomId = route.params.roomId?.toString();
    return routePlatform === s.platform.toLowerCase() && routeRoomId === s.roomId;
  }
  return false;
};

const getWrapperClass = (_s: any) => {
  return "single-view h-full w-full min-h-0";
};
</script>
