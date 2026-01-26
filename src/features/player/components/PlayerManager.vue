<template>
  <div 
    class="fixed inset-0 z-10"
    :class="{
      'bg-black': isPlayerRoute,
      'pointer-events-none opacity-0 -z-10': !isPlayerRoute
    }"
    :style="{ 
      top: isPlayerFullscreen ? '0' : 'var(--navbar-height)',
    }"
  >
    <!-- Unified Container -->
    <div 
      class="w-full h-full block"
    >
      <!-- Active Players -->
      <div 
        v-for="s in playerStore.activeStreamers" 
        :key="`${s.platform}:${s.roomId}`"
        class="bg-neutral-900 overflow-hidden relative transition-all duration-300"
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

const playerStore = usePlayerStore();
const followStore = useFollowStore();
const route = useRoute();

const isFullscreenInternal = ref(false);

const isPlayerFullscreen = computed(() => {
  return isFullscreenInternal.value; 
});

const isPlayerRoute = computed(() => route.name === "StreamRoom");

const isCurrentPlayer = (s: any) => {
  if (!isPlayerRoute.value) return false;
  const routePlatform = route.params.platform?.toString().toLowerCase();
  const routeRoomId = route.params.roomId?.toString();
  return (
    routePlatform === s.platform.toLowerCase() &&
    routeRoomId === s.roomId
  );
};

const shouldShowWrapper = (s: any) => {
  if (isPlayerRoute.value) return isCurrentPlayer(s);
  return false;
};

const getWrapperClass = (_s: any) => {
  return 'single-view h-full w-full min-h-0';
};


const handleFollow = (streamer: FollowedStreamer) => {
  followStore.followStreamer(streamer);
};

const handleUnfollow = (roomId: string) => {
  const streamer = playerStore.activeStreamers.find(s => s.roomId === roomId);
  if (streamer) {
    followStore.unfollowStreamer(streamer.platform, streamer.roomId);
  }
};

const emit = defineEmits(["fullscreen-change"]);
const handleFullscreenChange = (v: boolean) => {
  isFullscreenInternal.value = v;
  emit("fullscreen-change", v);
};
</script>

<style scoped>
.global-player-manager {
  /* Ensure it doesn't block interactions when hidden */
}
</style>