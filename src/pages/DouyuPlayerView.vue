<template>
  <div class="flex h-full w-full flex-1 flex-col items-stretch justify-center bg-transparent text-center text-[var(--primary-text)]">
    <MainPlayer 
      v-if="roomId && !isLoadingDetails"
      :room-id="roomId" 
      :platform="Platform.DOUYU" 
      :is-followed="isFollowed"
      :title="streamerDetails?.roomTitle ?? undefined"
      :anchor-name="streamerDetails?.nickname ?? undefined"
      :avatar="streamerDetails?.avatarUrl ?? undefined"
      :is-live="streamerDetails?.isLive ?? undefined"
      :initial-error="detailsError" 
      @follow="handleFollow" 
      @unfollow="handleUnfollow" 
      @close-player="handleClosePlayer" 
      @fullscreen-change="handlePlayerFullscreenChange"
      @request-refresh-details="handleRefreshDetails"
      @request-player-reload="handlePlayerReload" />
    <div v-else-if="roomId && isLoadingDetails" class="flex flex-1 items-center justify-center">
      <LoadingDots />
    </div>
    <div v-else-if="detailsError" class="flex flex-1 flex-col items-center justify-center gap-3">
      <p class="px-5 text-[1.1em]">错误: {{ detailsError }}</p>
      <button class="mt-2 rounded-md bg-[var(--accent)] px-5 py-2.5 text-base text-white" @click="router.back()">返回</button>
    </div>
    <div v-else class="flex flex-1 flex-col items-center justify-center gap-3">
      <p class="px-5 text-[1.1em]">无效的斗鱼房间ID。</p>
      <button class="mt-2 rounded-md bg-[var(--accent)] px-5 py-2.5 text-base text-white" @click="router.back()">返回</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import MainPlayer from '../components/player/index.vue';
import LoadingDots from '../components/Common/LoadingDots.vue';
import { useFollowStore } from '../store/followStore';
import type { FollowedStreamer } from '../platforms/common/types';
import { Platform } from '../platforms/common/types';
import { fetchDouyuStreamerDetails } from '../platforms/douyu/streamerInfoParser';
import type { StreamerDetails } from '../platforms/common/types';

const props = defineProps<{
  roomId: string;
}>();

const emit = defineEmits(['fullscreen-change']);

const router = useRouter();
const followStore = useFollowStore();

const streamerDetails = ref<StreamerDetails | null>(null);
const isLoadingDetails = ref(false);
const detailsError = ref<string | null>(null);
const playerKey = ref(0);
let hasLoadedDetailsForCurrentRoom = false; // Flag to prevent re-fetching for the same room ID

const loadStreamerDetails = async (currentRoomId: string) => {
  if (!currentRoomId) {
    streamerDetails.value = null;
    detailsError.value = 'Room ID is invalid.';
    isLoadingDetails.value = false;
    hasLoadedDetailsForCurrentRoom = false; // Reset flag
    return;
  }

  if (hasLoadedDetailsForCurrentRoom && streamerDetails.value?.roomId === currentRoomId) {
    if(isLoadingDetails.value) isLoadingDetails.value = false;
    return;
  }
  
  isLoadingDetails.value = true;
  detailsError.value = null;
  if (streamerDetails.value?.roomId !== currentRoomId) {
      streamerDetails.value = null;
  }

  try {
    const result = await fetchDouyuStreamerDetails(currentRoomId);
    if (result?.errorMessage) {
      detailsError.value = result.errorMessage;
      streamerDetails.value = null; 
      console.warn(`[DouyuPlayerView] Error from fetchDouyuStreamerDetails: ${result.errorMessage}`);
    } else if (!result || !result.nickname) { 
      detailsError.value = '获取到的主播信息无效或不完整。';
      streamerDetails.value = null; 
      console.warn('[DouyuPlayerView] Invalid or incomplete data from backend.', result);
    } else {
      streamerDetails.value = result; 
      detailsError.value = null; 
    }
  } catch (e: any) {
    console.error(`[DouyuPlayerView] Exception while loading streamer details for ${currentRoomId}:`, e);
    detailsError.value = e.message || '加载主播详情时发生未知错误。';
    streamerDetails.value = null;
  } finally {
    isLoadingDetails.value = false;
    hasLoadedDetailsForCurrentRoom = true; 
  }
};

const isFollowed = computed(() => {
  return followStore.isFollowed(Platform.DOUYU, props.roomId);
});

interface MainPlayerFollowEventData {
  nickname: string;
  avatarUrl: string;
  roomTitle?: string; 
}

const handleFollow = (streamerDataFromPlayer: MainPlayerFollowEventData) => {
  const streamerToFollow: FollowedStreamer = {
    id: props.roomId,
    platform: Platform.DOUYU,
    nickname: streamerDataFromPlayer.nickname, 
    avatarUrl: streamerDataFromPlayer.avatarUrl,
    roomTitle: streamerDataFromPlayer.roomTitle, 
  };
  followStore.followStreamer(streamerToFollow);
};

const handleUnfollow = () => {
  followStore.unfollowStreamer(Platform.DOUYU, props.roomId);
  if (streamerDetails.value) {
  }
};

const handleClosePlayer = () => {
  console.log('[DouyuPlayerView] Close button clicked. Navigating to Douyu home.');
  router.replace('/'); // Navigate to Douyu home page
};

const handlePlayerFullscreenChange = (isFullscreen: boolean) => {
  emit('fullscreen-change', isFullscreen);
};

const handleRefreshDetails = () => {
  if (props.roomId) {
    hasLoadedDetailsForCurrentRoom = false; // Reset flag to allow re-fetch
    streamerDetails.value = null; // Optionally clear current details to ensure UI updates to loading
    detailsError.value = null;    // Clear previous errors
    loadStreamerDetails(props.roomId);
  } else {
    console.warn('[DouyuPlayerView] request-refresh-details received but no roomId available.');
  }
};

const handlePlayerReload = () => {
  playerKey.value++;
  if (props.roomId) {
    hasLoadedDetailsForCurrentRoom = false; 
    streamerDetails.value = null; 
    detailsError.value = null;
    loadStreamerDetails(props.roomId);
  }
};

watch(() => props.roomId, (newRoomId, oldRoomId) => {
  if (newRoomId) {
    if (newRoomId !== oldRoomId) {
      hasLoadedDetailsForCurrentRoom = false; // Reset flag for the new room ID
      loadStreamerDetails(newRoomId);
    } else { // roomId is the same, or it's the initial immediate:true call
      if (!hasLoadedDetailsForCurrentRoom) {
         loadStreamerDetails(newRoomId);
      }
    }
  } else {
    streamerDetails.value = null;
    detailsError.value = null;
    isLoadingDetails.value = false;
    hasLoadedDetailsForCurrentRoom = false;
  }
}, { immediate: true });

onMounted(() => {
  if (props.roomId && hasLoadedDetailsForCurrentRoom && isLoadingDetails.value) {
     isLoadingDetails.value = false; 
  } else if (!props.roomId && isLoadingDetails.value) {
     isLoadingDetails.value = false;
  }
});

</script>

