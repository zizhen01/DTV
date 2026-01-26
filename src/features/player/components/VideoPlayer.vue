<template>
  <div class="relative flex h-full w-full min-h-0 flex-1 overflow-hidden" v-show="isActive">
    <div class="flex h-full w-full min-h-0 flex-col gap-3 overflow-hidden lg:flex-row lg:gap-0">
      <div class="relative flex h-full min-h-0 w-full flex-1 flex-col overflow-hidden">
        <div v-if="!roomId"
          class="flex h-full min-h-0 flex-1 flex-col items-center justify-center gap-3 p-8 text-center">
          <div class="text-text-muted opacity-80">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="12" y1="16" x2="12" y2="12"></line>
              <line x1="12" y1="8" x2="12.01" y2="8"></line>
            </svg>
          </div>
          <h3 class="text-base font-semibold text-text-main">未选择直播间</h3>
          <p class="text-sm text-text-muted">请从首页选择一个直播间开始观看。</p>
        </div>
        <div v-else-if="isLoadingStream" class="flex h-full min-h-0 flex-1 items-center justify-center">
          <LoadingDots />
        </div>
        <div v-else-if="isOfflineError"
          class="flex h-full min-h-0 flex-1 flex-col items-center justify-center gap-4 p-8 text-center">
          <!-- Display StreamerInfo if room details are available -->
          <StreamerInfo v-if="props.roomId && props.platform" :room-id="props.roomId" :platform="props.platform"
            :title="playerTitle" :anchor-name="playerAnchorName" :avatar="playerAvatar" :is-live="false"
            :is-followed="props.isFollowed || false" @follow="handleFollow" @unfollow="handleUnfollow"
            class="w-full max-w-[700px] border-b border-border-main pb-5" />
          <div class="max-w-[560px]">
            <div class="mb-4 flex items-center justify-center text-text-muted">
              <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path
                  d="M16 16.427A4.002 4.002 0 0 0 12.005 20a4 4 0 0 0-3.995-3.573M12 12V2M8.5 7L7 5.5M15.5 7l1.5-1.5M5.562 10.223l-1.842.511M18.438 10.223l1.842.511M12 2a3.5 3.5 0 0 1 3.5 3.5V12H8.5V5.5A3.5 3.5 0 0 1 12 2z" />
                <line x1="1" y1="1" x2="23" y2="23" stroke-width="2"></line>
              </svg>
            </div>
            <h3 class="text-base font-semibold text-text-main">
              😴 获取直播流失败了
            </h3>
            <p class="mt-1 text-sm text-text-muted">
              主播当前未开播，请稍后再来。
            </p>
            <button @click="retryInitialization"
              class="mt-4 inline-flex items-center justify-center rounded-full bg-brand px-5 py-2 text-sm font-semibold text-white shadow-sm transition hover:bg-brand/90 active:scale-[0.98]">
              再试一次
            </button>
          </div>
        </div>
        <div v-else-if="streamError && !isOfflineError"
          class="flex h-full min-h-0 flex-1 flex-col items-center justify-center gap-3 p-8 text-center">
          <div class="text-text-muted opacity-80">
            <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="12" y1="8" x2="12" y2="12"></line>
              <line x1="12" y1="16" x2="12.01" y2="16"></line>
            </svg>
          </div>
          <h3 class="text-base font-semibold text-text-main">加载失败</h3>
          <p class="max-w-[640px] text-sm text-text-muted">{{ streamError }}</p>
          <button @click="retryInitialization"
            class="mt-2 inline-flex items-center justify-center rounded-full bg-brand px-5 py-2 text-sm font-semibold text-white shadow-sm transition hover:bg-brand/90 active:scale-[0.98]">
            再试一次
          </button>
        </div>
        <div v-else class="group relative flex min-h-0 flex-1 flex-col overflow-hidden bg-black">
          <div class="relative w-full flex-1 min-h-[clamp(260px,50vh,560px)] md:min-h-[clamp(300px,60vh,720px)]">
            <div ref="playerContainerRef" class="absolute inset-0"></div>
          </div>

          <!-- Overlay Close Button -->
          <button @click.stop="handleUnfollow"
            class="absolute top-2 right-2 z-[60] flex size-8 items-center justify-center rounded-full bg-black/60 text-white/80 opacity-0 backdrop-blur-sm transition-all group-hover:opacity-100 hover:bg-red-500 hover:text-white"
            title="关闭画面">
            <X :size="18" stroke-width="3" />
          </button>
        </div>

        <Transition enter-active-class="transition-opacity duration-300" enter-from-class="opacity-0"
          enter-to-class="opacity-100" leave-active-class="transition-opacity duration-300"
          leave-from-class="opacity-100" leave-to-class="opacity-0">
          <button v-if="
            isDanmuSidebarCollapsed &&
            roomId &&
            !isInWebFullscreen &&
            !isFullScreen &&
            showDanmuPanel
          " @click="toggleDanmuSidebar"
            class="absolute right-0 top-1/2 z-50 flex h-12 w-6 -translate-y-1/2 items-center justify-center rounded-l-md border border-white/10 border-r-0 bg-black/50 text-white/70 backdrop-blur transition-all hover:w-7 hover:bg-black/70 hover:text-white"
            title="展开弹幕列表">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m15 18-6-6 6-6" />
            </svg>
          </button>
        </Transition>
      </div>

      <DanmuList v-if="roomId && !isLoadingStream && !streamError && isDanmuSidebarCollapsed" :room-id="props.roomId"
        :messages="danmakuMessages" @collapse="toggleDanmuSidebar"
        class="transition-[width, flex-basis,opacity,transform,border] duration-300 ease-[cubic-bezier(0.4,0,0.2,1)] lg:w-[220px] lg:flex-[0_0_220px]"
        :class="{
          hidden: isFullScreen,
          'pointer-events-none lg:w-0 lg:flex-[0_0_0px] lg:opacity-0 lg:border-0':
            isDanmuSidebarCollapsed,
        }" />
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  computed,
  nextTick,
  onMounted,
  onActivated,
  onDeactivated,
  onBeforeUnmount,
  reactive,
  ref,
  shallowRef,
  watch,
} from "vue";
import Artplayer from "artplayer";
import mpegts from "mpegts.js";
import Hls from "hls.js";
import { X } from "lucide-vue-next";

import { Platform as StreamingPlatform } from "../../../types/app/platform";
import type {
  DanmakuMessage,
  DanmuOverlayInstance,
} from "../../../types/models/danmaku";
import {
  applyDanmuFontFamilyForOS,
  loadDanmuPreferences,
  loadStoredVolume,
  persistDanmuPreferences,
  sanitizeDanmuArea,
  sanitizeDanmuOpacity,
  type DanmuUserSettings,
} from "../constants";

import {
  applyDanmuOverlayPreferences,
  createDanmuOverlay,
  ensureDanmuOverlayHost,
  // syncDanmuEnabledState
} from "../danmuOverlay";
import type { PlayerProps } from "../types";
import {
  startCurrentDanmakuListener as startDanmakuListener,
  stopCurrentDanmakuListener as stopDanmakuListener,
} from "../danmakuManager";
import {
  getLineLabel,
  getLineOptionsForPlatform,
  persistLinePreference,
  resolveCurrentLineFor,
  resolveStoredLine,
} from "../lineOptions";

import { getLiveStreamV2 } from "../../../api/live";
import type { FollowedStreamer } from "../../../types/models/streamer";
import type { SupportedPlatform } from "../../../types/app/platform";

import StreamerInfo from "./StreamerInfo.vue";
import DanmuList from "./DanmuList.vue";
import LoadingDots from "../../../components/ui/LoadingDots.vue";

import { useImageProxy } from "../../following/composables/useProxy";
import { usePlayerStore } from "../../../store/playerStore";
import { useFollowStore } from "../../../store/followStore";

// Ensure image proxy helpers are available in this component
const { ensureProxyStarted, proxify } = useImageProxy();

const props = defineProps<PlayerProps & { isActive?: boolean }>();

const playerStore = usePlayerStore();
const emit = defineEmits<{
  (e: "follow", streamer: FollowedStreamer): void;
  (e: "unfollow", roomId: string): void;
  (e: "close-player"): void;
  (e: "fullscreen-change", isFullscreen: boolean): void;
  (e: "request-refresh-details"): void;
  (e: "request-player-reload"): void;
}>();

const isClosing = ref(false);
const isDanmuSidebarCollapsed = ref(false); // Default to collapsed
const MIN_DANMU_WIDTH = 1100;
const windowWidth = ref(typeof window !== "undefined" ? window.innerWidth : 0);
const updateWindowWidth = () => {
  windowWidth.value = typeof window !== "undefined" ? window.innerWidth : 0;
};

const showDanmuPanel = computed(() => windowWidth.value >= MIN_DANMU_WIDTH);

const toggleDanmuSidebar = () => {
  isDanmuSidebarCollapsed.value = !isDanmuSidebarCollapsed.value;
};

const playerContainerRef = ref<HTMLDivElement | null>(null);
const playerInstance = shallowRef<Artplayer | null>(null);
const flvPlayerInstance = shallowRef<mpegts.Player | null>(null);
const hlsInstance = shallowRef<Hls | null>(null);
const danmuInstance = shallowRef<DanmuOverlayInstance | null>(null);
const danmakuMessages = ref<DanmakuMessage[]>([]);
const isDanmakuListenerActive = ref(false); // Tracks if a danmaku listener is supposed to be running
const unlistenDanmakuFn = ref<(() => void) | null>(null);

const isLoadingStream = ref(true);
const streamError = ref<string | null>(null);
const isOfflineError = ref(false); // Added to track '主播未开播' state

// Reactive state for streamer info, initialized by props, potentially updated by internal fetches (for Douyin)
const playerTitle = ref(props.title);
const playerAnchorName = ref(props.anchorName);
const playerAvatar = ref(props.avatar);
const playerIsLive = ref(props.isLive);

const isInNativePlayerFullscreen = ref(false); // New: Tracks Artplayer element's native fullscreen
const isInWebFullscreen = ref(false);
const isFullScreen = ref(false); // True if EITHER native player OR web fullscreen is active

const isDanmuEnabled = ref(true);
const danmuSettings = reactive<DanmuUserSettings>({
  color: "#ffffff",
  strokeColor: "#444444",
  fontSize: "20px",
  duration: 10000,
  area: 0.5,
  mode: "scroll",
  opacity: 1,
});

const storedDanmuPreferences = loadDanmuPreferences();
if (storedDanmuPreferences) {
  isDanmuEnabled.value = storedDanmuPreferences.enabled;
  Object.assign(danmuSettings, storedDanmuPreferences.settings);
}

const isMutedInStore = computed(() => {
  const key = `${props.platform.toUpperCase()}:${props.roomId}`;
  return !!playerStore.activeStreamers.find(
    (s) => `${s.platform.toUpperCase()}:${s.roomId}` === key,
  )?.isMuted;
});

watch(isMutedInStore, (muted) => {
  if (playerInstance.value) {
    playerInstance.value.muted = muted;
    console.log(
      `[Player] Sync mute from store: ${props.platform}/${props.roomId} -> ${muted}`,
    );
  }
});

// OS specific states
const osName = ref<string>("");

// 画质切换相关
const qualityOptions = ["原画", "高清", "标清"] as const;

const resolveStoredQuality = (platform?: StreamingPlatform | null): string => {
  // Always default to the highest quality.
  // We intentionally do NOT restore last-used quality from localStorage because
  // previous fallback logic could have persisted a lower quality.
  void platform;
  return "原画";
};

const currentQuality = ref<string>(resolveStoredQuality(props.platform));
const isQualitySwitching = ref(false);
const isRefreshingStream = ref(false);
const isLineSwitching = ref(false);

const currentLine = ref<string | null>(resolveStoredLine(props.platform));
const lineOptions = computed(() => getLineOptionsForPlatform(props.platform));
const getCurrentLineLabel = (key?: string | null) =>
  getLineLabel(lineOptions.value, key);

function resetFullscreenState() {
  isInNativePlayerFullscreen.value = false;
  isInWebFullscreen.value = false;
  isFullScreen.value = false;
}

function updateFullscreenFlag() {
  if (isClosing.value) {
    return;
  }
  isFullScreen.value =
    isInNativePlayerFullscreen.value || isInWebFullscreen.value;
  emit("fullscreen-change", isFullScreen.value);
}

const followStore = useFollowStore();

const handleFollow = () => {
  const streamerToFollow: FollowedStreamer = {
    id: props.roomId!,
    platform: props.platform,
    nickname: playerAnchorName.value || props.anchorName || props.roomId!,
    avatarUrl: playerAvatar.value || props.avatar || "",
    roomTitle: playerTitle.value || props.title || "",
  };
  followStore.followStreamer(streamerToFollow);
};

const handleUnfollow = () => {
  if (props.roomId) {
    followStore.unfollowStreamer(props.platform, props.roomId);
  }
};

function destroyPlayerInstance() {
  const player = playerInstance.value;
  if (player) {
    try {
      player.destroy(false); // Destroy without removing DOM initially to handle clean up
    } catch (error) {
      console.error("[Player] Error destroying player instance:", error);
    }
  }
  playerInstance.value = null;

  if (flvPlayerInstance.value) {
    try {
      flvPlayerInstance.value.destroy();
    } catch (e) {
      console.warn("[Player] Error destroying mpegts instance:", e);
    }
    flvPlayerInstance.value = null;
  }

  if (hlsInstance.value) {
    try {
      hlsInstance.value.destroy();
    } catch (e) {
      console.warn("[Player] Error destroying hls instance:", e);
    }
    hlsInstance.value = null;
  }

  const danmu = danmuInstance.value;
  if (danmu) {
    try {
      danmu.stop?.();
    } catch (error) {
      console.error("[Player] Error stopping danmu overlay:", error);
    }
    danmuInstance.value = null;
  }

  resetFullscreenState();
}

async function mountArtPlayer(
  streamUrl: string,
  platformCode: StreamingPlatform,
  roomId: string,
  streamType?: string | null,
) {
  await nextTick();

  if (!playerContainerRef.value) {
    streamError.value = "播放器容器初始化失败。";
    return;
  }

  playerContainerRef.value.innerHTML = "";

  const playbackType = streamType === "hls" ? "m3u8" : "flv";

  const storedPlayerVolume = loadStoredVolume();

  const ensureVideoLayout = (video: HTMLVideoElement) => {
    try {
      video.style.width = "100%";
      video.style.height = "100%";
      video.style.display = "block";
      video.style.objectFit = "contain";

      const p1 = video.parentElement as HTMLElement | null;
      const p2 = p1?.parentElement as HTMLElement | null;
      const p3 = p2?.parentElement as HTMLElement | null;
      for (const el of [p1, p2, p3]) {
        if (!el) continue;
        el.style.height = "100%";
        el.style.minHeight = "0";
      }
    } catch (error) {
      // Non-critical
    }
  };

  const art = new Artplayer({
    container: playerContainerRef.value,
    url: streamUrl,
    type: playbackType,
    isLive: true,
    autoplay: true,
    volume: storedPlayerVolume !== null ? storedPlayerVolume : 0.7,
    muted: storedPlayerVolume === 0,
    pip: true,
    fullscreen: true,
    fullscreenWeb: true,
    autoSize: true,
    setting: true,
    loop: false,
    flip: true,
    playbackRate: false,
    aspectRatio: true,
    screenshot: true,
    miniProgressBar: true,
    theme: "#a855f7", // Using brand color
    lang: "zh-cn",
    controls: [
      {
        name: "refresh",
        index: 10,
        position: "left",
        html: '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 21h5v-5"/></svg>',
        tooltip: "刷新",
        style: {
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        },
        click: function () {
          void reloadCurrentStream("refresh");
        },
      },
      {
        name: "danmu-toggle",
        index: 20,
        position: "right",
        html: isDanmuEnabled.value
          ? '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/></svg>'
          : '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-50"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><line x1="2" y1="2" x2="22" y2="22"/></svg>',
        tooltip: isDanmuEnabled.value ? "关闭弹幕" : "开启弹幕",
        style: {
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        },
        // click: function (item: any) {
        //   isDanmuEnabled.value = !isDanmuEnabled.value;
        //   item.tooltip = isDanmuEnabled.value ? "关闭弹幕" : "开启弹幕";
        //   item.html = isDanmuEnabled.value
        //     ? '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/></svg>'
        //     : '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-50"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><line x1="2" y1="2" x2="22" y2="22"/></svg>';

        //   syncDanmuEnabledState(
        //     danmuInstance.value,
        //     danmuSettings,
        //     isDanmuEnabled.value,
        //     playerContainerRef.value,
        //   );
        // },
        click: function (item: any) {
          // 1. 切换开关状态
          isDanmuEnabled.value = !isDanmuEnabled.value;

          // 2. 更新按钮 UI
          item.tooltip = isDanmuEnabled.value ? "关闭弹幕" : "开启弹幕";
          item.html = isDanmuEnabled.value
            ? '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/></svg>'
            : '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-50"><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-1"/><path d="M15 22h-1a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M10 22h1a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-1"/><path d="M14 6H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><path d="M22 6h-3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1V7a1 1 0 0 0-1-1Z"/><line x1="2" y1="2" x2="22" y2="22"/></svg>';

          // 3. 调用合并后的统一更新函数
          applyDanmuOverlayPreferences(
            danmuInstance.value,
            danmuSettings,
            isDanmuEnabled.value,
            playerContainerRef.value // 传入 Ref 的 DOM 节点
          );
        },
      },
      {
        name: "danmu-settings",
        index: 21,
        position: "right",
        html: '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1-1-1.74v-.51a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>',
        tooltip: "弹幕设置",
        style: {
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        },
        selector: [
          {
            default: true,
            html: "不透明度",
            html2: `${Math.round(danmuSettings.opacity * 100)}%`,
            tooltip: "调节弹幕透明度",
            range: [danmuSettings.opacity, 0.1, 1, 0.1],
            onChange: function (item: any, element: HTMLInputElement) {
              const value = Number(element.value);
              danmuSettings.opacity = value;
              item.html2 = `${Math.round(value * 100)}%`;
              applyDanmuOverlayPreferences(
                danmuInstance.value,
                danmuSettings,
                isDanmuEnabled.value,
                playerContainerRef.value,
              );
            },
          },
          {
            html: "显示区域",
            html2: `${danmuSettings.area === 1 ? "100%" : danmuSettings.area === 0.5 ? "半屏" : "1/4屏"}`,
            tooltip: "调节弹幕显示区域",
            switch: true,
            onSwitch: function (item: any) {
              if (danmuSettings.area === 1) danmuSettings.area = 0.25;
              else if (danmuSettings.area === 0.5) danmuSettings.area = 1;
              else danmuSettings.area = 0.5;

              item.html2 = `${danmuSettings.area === 1 ? "100%" : danmuSettings.area === 0.5 ? "半屏" : "1/4屏"}`;
              applyDanmuOverlayPreferences(
                danmuInstance.value,
                danmuSettings,
                isDanmuEnabled.value,
                playerContainerRef.value,
              );
            },
          },
          {
            html: "字体大小",
            html2: `${danmuSettings.fontSize}`,
            tooltip: "调节弹幕字体大小",
            range: [parseInt(danmuSettings.fontSize), 12, 40, 1],
            onChange: function (item: any, element: HTMLInputElement) {
              const value = Number(element.value);
              danmuSettings.fontSize = `${value}px`;
              item.html2 = `${value}px`;
              applyDanmuOverlayPreferences(
                danmuInstance.value,
                danmuSettings,
                isDanmuEnabled.value,
                playerContainerRef.value,
              );
            },
          },
        ],
      },
      {
        name: "quality",
        index: 22,
        position: "right",
        html: currentQuality.value,
        style: {
          padding: "0 10px",
        },
        selector: qualityOptions.map((q) => ({
          html: q,
          default: q === currentQuality.value,
        })),
        onSelect: async function (item: any) {
          await switchQuality(item.html as string);
          return item.html;
        },
      },
    ],
    customType: {
      m3u8: function (video, url) {
        ensureVideoLayout(video);
        if (Hls.isSupported()) {
          if (hlsInstance.value) {
            hlsInstance.value.destroy();
          }
          const hls = new Hls({
            maxBufferLength: 10, maxMaxBufferLength: 20,
            enableWorker: true,
          });
          hls.loadSource(url);
          hls.attachMedia(video);
          hlsInstance.value = hls;
        } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
          video.src = url;
        }
      },
      flv: function (video, url) {
        ensureVideoLayout(video);
        if (mpegts.isSupported()) {
          if (flvPlayerInstance.value) {
            flvPlayerInstance.value.destroy();
          }
          const flvPlayer = mpegts.createPlayer(
            {
              type: "flv",
              url: url,
              isLive: true,
              cors: true,
            },
            {
              enableWorker: true,
              lazyLoad: true,
              stashInitialSize: 128,
              autoCleanupSourceBuffer: true,
              lazyLoadMaxDuration: 30,
              deferLoadAfterSourceOpen: true,
            },
          );
          flvPlayer.attachMediaElement(video);

          if (import.meta.env.DEV) {
            video.addEventListener(
              "loadedmetadata",
              () => {
                console.log(
                  `[Player] Video metadata: ${video.videoWidth}x${video.videoHeight}`,
                );

                const rect = video.getBoundingClientRect();
                console.log(
                  "[Player] Video rect:",
                  `${Math.round(rect.width)}x${Math.round(rect.height)}`,
                );
                console.log("[Player] Video state:", {
                  readyState: video.readyState,
                  paused: video.paused,
                  currentTime: video.currentTime,
                  muted: video.muted,
                  volume: video.volume,
                });
              },
              { once: true },
            );

            window.setTimeout(() => {
              const rect = video.getBoundingClientRect();
              console.log(
                "[Player] Video rect (2s):",
                `${Math.round(rect.width)}x${Math.round(rect.height)}`,
              );
              console.log("[Player] Video state (2s):", {
                readyState: video.readyState,
                paused: video.paused,
                currentTime: video.currentTime,
                videoWidth: video.videoWidth,
                videoHeight: video.videoHeight,
              });
            }, 2000);

            try {
              flvPlayer.on(
                (mpegts as any).Events.ERROR,
                (type: any, detail: any, info: any) => {
                  console.error("[Player] mpegts error:", type, detail, info);
                },
              );
              flvPlayer.on((mpegts as any).Events.MEDIA_INFO, (info: any) => {
                console.log("[Player] mpegts media info:", info);
              });
              flvPlayer.on(
                (mpegts as any).Events.STATISTICS_INFO,
                (info: any) => {
                  if (!info) return;
                  if (typeof info.decodedFrames === "number") {
                    console.log("[Player] mpegts stats decodedFrames:", info);
                  }
                },
              );
            } catch (error) {
              // Non-critical; depends on mpegts.js build
            }
          }

          flvPlayer.load();
          // mpegts.js requires an explicit play() to start transmuxing.
          // Artplayer will control the HTMLMediaElement, but the mpegts player
          // itself still needs to enter the running state.
          try {
            flvPlayer.play();
          } catch (error) {
            console.warn("[Player] Failed starting mpegts playback:", error);
          }
          flvPlayerInstance.value = flvPlayer;
        }
      },
    },
  });

  // Add Line Control dynamically if needed
  if (lineOptions.value.length > 0) {
    art.controls.add({
      name: "line-switch",
      index: 23,
      position: "right",
      html: getCurrentLineLabel(currentLine.value),
      style: {
        padding: "0 10px",
      },
      selector: lineOptions.value.map((opt) => ({
        html: opt.label,
        value: opt.key,
        default: opt.key === currentLine.value,
      })),
      onSelect: async function (item: any) {
        const key = (item as any).value || item.html;
        await switchLine(key);
        return item.html;
      },
    });
  }

  playerInstance.value = art;

  art.on("ready", async () => {
    // Extra resize pass for webview layout timing.
    try {
      setTimeout(() => {
        try {
          (art as any).resize?.();
        } catch (error) {
          // Non-critical
        }
      }, 50);
    } catch (error) {
      // Non-critical
    }

    ensureDanmuOverlayHost(art);
    let overlayInstance = await createDanmuOverlay(
      art,
      danmuSettings,
      isDanmuEnabled.value,
    );
    danmuInstance.value = overlayInstance;

    try {
      if (roomId) {
        await startCurrentDanmakuListener(
          platformCode,
          roomId,
          overlayInstance,
        );
      }
    } catch (error) {
      console.error(
        "[Player] Failed starting danmaku listener after ready:",
        error,
      );
    }

    updateFullscreenFlag();

    // Workaround for volume persistence in ArtPlayer if not handled by default options perfectly
    if (storedPlayerVolume !== null) {
      art.volume = storedPlayerVolume;
      if (storedPlayerVolume === 0) art.muted = true;
    }
  });

  art.on("play", () => {
    danmuInstance.value?.play?.();
  });

  art.on("pause", () => {
    danmuInstance.value?.pause?.();
  });

  art.on("destroy", () => {
    danmuInstance.value?.stop?.();
    danmuInstance.value = null;
  });

  art.on("error", (error: any) => {
    console.error("[Player] ArtPlayer error:", error);
    streamError.value = `播放器错误: ${error?.message || error}`;
  });

  art.on("fullscreen", (state) => {
    isInNativePlayerFullscreen.value = state;
    updateFullscreenFlag();
    // Ensure danmu overlay is correct size
    danmuInstance.value?.resize?.();
  });

  art.on("fullscreenWeb", (state) => {
    isInWebFullscreen.value = state;
    updateFullscreenFlag();
    danmuInstance.value?.resize?.();
  });

  art.on("volume", (vol) => {
    // Artplayer volume is 0-1
    // If muted, it might be separate state, but volume change usually implies setting it
    if (vol === 0) {
      art.muted = true;
    } else {
      art.muted = false;
    }
    // Persist
    // Using existing helper if it accepts 0-1
    // Check helpers in constants
  });
}

async function initializePlayerAndStream(
  pRoomId: string,
  pPlatform: StreamingPlatform,
  _pStreamUrlProp?: string | null,
  isRefresh: boolean = false,
  oldRoomIdForCleanup?: string | null,
  oldPlatformForCleanup?: StreamingPlatform | null,
) {
  isLoadingStream.value = true;
  streamError.value = null;
  isOfflineError.value = false;

  // Detect OS and adjust danmu font family per platform
  osName.value = await applyDanmuFontFamilyForOS();

  if (!isRefresh) {
    danmakuMessages.value = [];
  }

  if (props.initialError && props.initialError.includes("主播未开播")) {
    streamError.value = props.initialError;
    isOfflineError.value = true;
    playerTitle.value = props.title;
    playerAnchorName.value = props.anchorName;
    playerAvatar.value = props.avatar;
    playerIsLive.value = false;
    destroyPlayerInstance();
    isLoadingStream.value = false;
    return;
  }

  if (
    oldRoomIdForCleanup &&
    oldPlatformForCleanup !== undefined &&
    oldPlatformForCleanup !== null
  ) {
    await stopCurrentDanmakuListener(
      oldPlatformForCleanup,
      oldRoomIdForCleanup,
    );
  } else {
    await stopCurrentDanmakuListener();
  }

  destroyPlayerInstance();

  const effectiveLine = resolveCurrentLineFor(pPlatform, currentLine.value);

  try {
    const platformForBackend: SupportedPlatform = (() => {
      if (pPlatform === StreamingPlatform.DOUYU) return "douyu";
      if (pPlatform === StreamingPlatform.DOUYIN) return "douyin";
      if (pPlatform === StreamingPlatform.HUYA) return "huya";
      return "bilibili";
    })();

    const cookieForBackend =
      platformForBackend === "bilibili"
        ? props.cookie ||
        (typeof localStorage !== "undefined"
          ? localStorage.getItem("bilibili_cookie")
          : null) ||
        null
        : null;

    const resp = await getLiveStreamV2({
      platform: platformForBackend,
      room_id: pRoomId,
      quality: currentQuality.value,
      line: effectiveLine ?? null,
      cookie: cookieForBackend,
      debug: false,
    });

    try {
      await ensureProxyStarted();
    } catch (error) {
      console.warn("[Player] Failed starting image proxy:", error);
    }

    playerTitle.value = resp.room?.title ?? props.title;
    playerAnchorName.value = resp.room?.anchor_name ?? props.anchorName;
    playerAvatar.value = resp.room?.avatar
      ? proxify(resp.room.avatar)
      : props.avatar;

    playerIsLive.value = resp.status === "live";

    if (resp.status === "offline") {
      streamError.value = "主播未开播。";
      isOfflineError.value = true;
      isLoadingStream.value = false;
      playerIsLive.value = false;
      return;
    }

    if (resp.status === "error") {
      throw new Error(resp.error || "加载直播流失败，请稍后再试。");
    }

    const streamUrl = resp.playback?.url;
    console.log(
      `[Player] Fetched stream URL for ${pPlatform} room ${pRoomId}:`,
      streamUrl,
    );
    if (!streamUrl) {
      throw new Error("未能获取有效的直播流地址。");
    }

    const streamType =
      resp.playback?.stream_type && resp.playback.stream_type !== "unknown"
        ? resp.playback.stream_type
        : undefined;

    isLoadingStream.value = false;

    // 成功获取流后，记录到活跃播放列表
    playerStore.setStreamerInfo({
      roomId: pRoomId,
      platform: pPlatform,
      title: playerTitle.value || "",
      anchorName: playerAnchorName.value || "",
      avatar: playerAvatar.value || "",
      isLive: playerIsLive.value || false,
    });

    await mountArtPlayer(streamUrl, pPlatform, pRoomId, streamType);
  } catch (error: any) {
    console.error(
      `[Player] Error initializing stream for ${pPlatform} room ${pRoomId}:`,
      error,
    );
    destroyPlayerInstance();

    const errorMessage = error?.message || "加载直播流失败，请稍后再试。";

    if (errorMessage.includes("主播未开播")) {
      streamError.value = errorMessage;
      isOfflineError.value = true;

      try {
        const platformForBackend: SupportedPlatform = (() => {
          if (pPlatform === StreamingPlatform.DOUYU) return "douyu";
          if (pPlatform === StreamingPlatform.DOUYIN) return "douyin";
          if (pPlatform === StreamingPlatform.HUYA) return "huya";
          return "bilibili";
        })();

        const cookieForBackend =
          platformForBackend === "bilibili"
            ? props.cookie ||
            (typeof localStorage !== "undefined"
              ? localStorage.getItem("bilibili_cookie")
              : null) ||
            null
            : null;

        const resp = await getLiveStreamV2({
          platform: platformForBackend,
          room_id: pRoomId,
          quality: currentQuality.value,
          line: effectiveLine ?? null,
          cookie: cookieForBackend,
          debug: false,
        });

        await ensureProxyStarted();
        playerTitle.value = resp.room?.title ?? props.title;
        playerAnchorName.value = resp.room?.anchor_name ?? props.anchorName;
        playerAvatar.value = resp.room?.avatar
          ? proxify(resp.room.avatar)
          : props.avatar;
      } catch (infoError) {
        console.warn(
          "[Player] Failed to fetch basic streamer info for offline page:",
          infoError,
        );
      }
    } else {
      streamError.value = errorMessage;
      isOfflineError.value = false;
    }

    isLoadingStream.value = false;
  }
}
const danmakuManagerContext = {
  danmakuMessages,
  isDanmuEnabled,
  danmuSettings,
  isDanmakuListenerActive,
  unlistenDanmakuFn,
  props,
};

const startCurrentDanmakuListener = async (
  platform: StreamingPlatform,
  roomId: string,
  danmuOverlay: DanmuOverlayInstance | null,
) => {
  await startDanmakuListener(
    danmakuManagerContext,
    platform,
    roomId,
    danmuOverlay,
  );
};

const stopCurrentDanmakuListener = async (
  platform?: StreamingPlatform,
  roomId?: string | null | undefined,
) => {
  await stopDanmakuListener(danmakuManagerContext, platform, roomId);
};

const retryInitialization = async () => {
  await reloadCurrentStream("refresh");
};

const lastInitKey = ref<string | null>(null);

watch(
  [() => props.roomId, () => props.platform, () => props.isActive] as const,
  async ([roomId, platform, isActive], [prevRoomId, prevPlatform, prevActive]) => {
    const resolvedIsActive = isActive !== false;
    const resolvedPrevActive = prevActive !== false;

    if (!resolvedIsActive) {
      if (resolvedPrevActive) {
        try {
          await stopCurrentDanmakuListener(
            prevPlatform,
            prevRoomId ?? undefined,
          );
        } catch (error) {
          console.warn("[Player] Failed stopping danmaku listener:", error);
        }
      }

      try {
        playerInstance.value?.pause();
      } catch (error) {
        // Non-critical
      }

      try {
        flvPlayerInstance.value?.pause();
      } catch (error) {
        // Non-critical
      }

      try {
        hlsInstance.value?.stopLoad();
      } catch (error) {
        // Non-critical
      }
      return;
    }

    if (!roomId || !platform) {
      if (props.initialError) {
        streamError.value = props.initialError;
        isOfflineError.value = props.initialError.includes("主播未开播");
      }
      isLoadingStream.value = false;
      return;
    }

    const key = `${platform}:${roomId}`;
    if (
      key === lastInitKey.value &&
      playerInstance.value &&
      !isLoadingStream.value &&
      !streamError.value
    ) {
      // Became active again; resume playback
      try {
        await playerInstance.value.play();
      } catch (error) {
        // Autoplay policies / state errors are expected occasionally
      }
      return;
    }

    lastInitKey.value = key;
    await initializePlayerAndStream(
      roomId,
      platform,
      props.streamUrl ?? null,
      false,
      prevRoomId ?? null,
      prevPlatform ?? null,
    );
  },
  { immediate: true },
);

// 画质切换函数
const switchQuality = async (quality: string) => {
  if (isQualitySwitching.value) {
    return;
  }
  if (!qualityOptions.includes(quality as (typeof qualityOptions)[number])) {
    return;
  }
  if (!props.roomId || props.platform == null) {
    emit("request-player-reload");
    return;
  }
  if (quality === currentQuality.value) {
    return;
  }

  isQualitySwitching.value = true;
  const previousQuality = currentQuality.value;

  try {
    currentQuality.value = quality;
    try {
      // Artplayer controls are not reactive; update the label immediately.
      (playerInstance.value as any)?.controls?.update?.({
        name: "quality",
        html: quality,
      });
    } catch (error) {
      // Non-critical
    }
    if (typeof window !== "undefined") {
      window.localStorage.setItem(
        `${props.platform}_preferred_quality`,
        quality,
      );
    }
    await reloadCurrentStream("quality");
    console.log(`[Player] 画质切换完成: ${quality}`);
  } catch (error) {
    console.error("[Player] 画质切换失败:", error);
    currentQuality.value = previousQuality;
    try {
      (playerInstance.value as any)?.controls?.update?.({
        name: "quality",
        html: previousQuality,
      });
    } catch (innerError) {
      // Non-critical
    }
    if (typeof window !== "undefined") {
      window.localStorage.setItem(
        `${props.platform}_preferred_quality`,
        previousQuality,
      );
    }
  } finally {
    isQualitySwitching.value = false;
  }
};

const switchLine = async (lineKey: string) => {
  if (isLineSwitching.value) {
    return;
  }
  const options = lineOptions.value;
  if (!options.length) {
    return;
  }
  if (!options.some((option) => option.key === lineKey)) {
    return;
  }
  if (!props.roomId || props.platform == null) {
    emit("request-player-reload");
    return;
  }
  if (currentLine.value === lineKey) {
    return;
  }

  isLineSwitching.value = true;
  const previousLine = currentLine.value;

  try {
    currentLine.value = lineKey;
    persistLinePreference(props.platform, lineKey);
    await reloadCurrentStream("line");
    console.log(`[Player] 线路切换完成: ${lineKey}`);
  } catch (error) {
    console.error("[Player] 线路切换失败:", error);
    currentLine.value = previousLine ?? null;
    if (previousLine) {
      persistLinePreference(props.platform, previousLine);
    }
  } finally {
    isLineSwitching.value = false;
  }
};

// 初始化画质偏好
const initializeQualityPreference = () => {
  currentQuality.value = resolveStoredQuality(props.platform);
};

async function reloadCurrentStream(
  trigger: "refresh" | "quality" | "line" = "refresh",
) {
  if (isLoadingStream.value) {
    return;
  }
  if (!props.roomId || props.platform == null) {
    emit("request-player-reload");
    return;
  }
  const isRefreshAction = trigger === "refresh";
  if (isRefreshAction) {
    isRefreshingStream.value = true;
  }
  try {
    await initializePlayerAndStream(
      props.roomId,
      props.platform,
      props.streamUrl ?? null,
      true,
      props.roomId,
      props.platform,
    );
  } finally {
    if (isRefreshAction) {
      isRefreshingStream.value = false;
    }
  }
}

const getDanmuSettingsSnapshot = (): DanmuUserSettings => ({
  color: danmuSettings.color,
  strokeColor: danmuSettings.strokeColor,
  fontSize: danmuSettings.fontSize,
  duration: danmuSettings.duration,
  area: sanitizeDanmuArea(danmuSettings.area),
  mode: danmuSettings.mode,
  opacity: sanitizeDanmuOpacity(danmuSettings.opacity),
});

const persistCurrentDanmuPreferences = () => {
  persistDanmuPreferences({
    enabled: isDanmuEnabled.value,
    settings: getDanmuSettingsSnapshot(),
  });
};

onMounted(async () => {
  updateWindowWidth();
  if (typeof window !== "undefined") {
    window.addEventListener("resize", updateWindowWidth, { passive: true });
  }
  // 初始化画质偏好
  initializeQualityPreference();

  if (!props.roomId || props.platform == null) {
    if (props.initialError) {
      if (props.initialError.includes("主播未开播")) {
        streamError.value = props.initialError;
        isOfflineError.value = true;
      } else {
        streamError.value = props.initialError;
        isOfflineError.value = false; // Ensure it's not marked as offline for other errors
      }
    }
    isLoadingStream.value = false;
  }

  persistCurrentDanmuPreferences();
});

watch(
  [playerTitle, playerAnchorName, playerAvatar, playerIsLive],
  () => {
    if (props.roomId && props.platform) {
      playerStore.setStreamerInfo({
        roomId: props.roomId,
        platform: props.platform,
        title: playerTitle.value ?? "",
        anchorName: playerAnchorName.value ?? "",
        avatar: playerAvatar.value ?? "",
        isLive: playerIsLive.value ?? false,
      });
    }
  },
  { immediate: true },
);

onActivated(() => {
  console.log(`[Player] Instance activated: ${props.platform}/${props.roomId}`);
  if (playerInstance.value && !isLoadingStream.value) {
    // 尝试恢复播放
    playerInstance.value.play().catch((err: any) => {
      console.warn("[Player] Failed to resume playback on activation:", err);
    });
  }
});

onDeactivated(() => {
  console.log(
    `[Player] Instance deactivated: ${props.platform}/${props.roomId}`,
  );
});

onBeforeUnmount(async () => {
  if (props.platform && props.roomId) {
    playerStore.removeStreamer(props.platform, props.roomId);
  }
  isClosing.value = true;
  if (typeof window !== "undefined") {
    window.removeEventListener("resize", updateWindowWidth);
  }
  const platformToStop: StreamingPlatform = props.platform;
  const roomIdToStop: string | null = props.roomId;
  await stopCurrentDanmakuListener(platformToStop, roomIdToStop);

  // Keep shared stream proxy alive

  destroyPlayerInstance();
  danmakuMessages.value = [];
});
</script>
