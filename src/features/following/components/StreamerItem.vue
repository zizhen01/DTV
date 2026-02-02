<template>
  <div
    class="group relative flex w-full items-center justify-start overflow-hidden bg-transparent select-none"
    :class="{ 'justify-center': collapsed }"
  >
    <div
      class="flex min-w-0 flex-1 items-center gap-3.5 px-3 py-2"
      :class="{
        'flex-none justify-center px-0': collapsed,
        'gap-2.5 px-2.5 py-1.5': isCompact && !collapsed,
      }"
      @click="onClick"
    >
      <div
        ref="avatarRef"
        class="relative shrink-0"
        :class="isCompact && !collapsed ? 'h-9 w-9' : big ? 'h-12 w-12' : 'h-10 w-10'"
      >
        <img
          v-if="shouldLoadAvatar"
          :src="avatarSrc"
          :alt="streamer.nickname"
          loading="lazy"
          decoding="async"
          fetchpriority="low"
          @error="handleImgError($event, streamer)"
          class="h-full w-full rounded-2xl border border-white/10 object-cover transition-all duration-300 group-hover:scale-105 group-hover:rounded-xl group-hover:shadow-lg"
        />
        <div
          v-else-if="canLoadAvatar"
          class="h-full w-full rounded-2xl border border-white/10"
          aria-hidden="true"
        ></div>
        <div
          v-else
          class="flex h-full w-full items-center justify-center rounded-2xl bg-surface-high text-[14px] font-black"
        >
          {{ streamer.nickname[0] }}
        </div>

        <!-- Collapsed Status Indicator (Overlay) -->
        <div
          v-if="collapsed"
          class="absolute -right-1 -bottom-1 h-3.5 w-3.5 rounded-full border-2 border-app-bg"
          :class="{
            'bg-[#10b981]':
              !streamer.lastUpdateFailed &&
              getLiveIndicatorClass(streamer) === 'is-live',
            'bg-amber-500':
              !streamer.lastUpdateFailed &&
              getLiveIndicatorClass(streamer) === 'is-replay',
            'bg-gray-400':
              streamer.lastUpdateFailed ||
              getLiveIndicatorClass(streamer) === 'is-offline',
          }"
        ></div>
      </div>

      <div
        v-if="!collapsed"
        class="flex min-w-0 flex-1 flex-col gap-0.5"
        :class="{ 'gap-0': isCompact }"
      >
        <div class="flex items-center gap-2" :class="{ 'gap-1.5': isCompact }">
          <span
            class="truncate text-[13.5px] font-black tracking-tight text-text-main group-hover:text-brand transition-colors"
            :class="{ 'text-[12.5px]': isCompact }"
            :title="streamer.nickname"
            >{{ streamer.nickname }}</span
          >
          <div
            class="h-1.5 w-1.5 rounded-full"
            :class="[
              isOnline ? 'bg-emerald-400' : 'bg-amber-400',
              isOnline ? 'animate-pulse' : '',
            ]"
          ></div>
        </div>
        <div
          class="truncate text-[11px] font-bold text-text-muted transition-colors group-hover:text-text-dim"
          :class="{ 'text-[10px]': isCompact }"
          :title="streamer.roomTitle"
        >
          {{ streamer.roomTitle || "未在直播" }}
        </div>
      </div>
    </div>

    <div
      v-if="!collapsed"
      class="ml-auto flex flex-shrink-0 items-center gap-1.5 pr-3.5"
      :class="{ 'gap-1 pr-2.5': isCompact }"
    >
      <button
        v-if="streamer.lastUpdateFailed"
        class="text-red-500 transition-transform hover:scale-110 active:scale-95"
        @click.stop="onRetry"
        :title="`${streamer.lastError || '刷新失败'} - 点击重试`"
      >
        <AlertCircle :size="14" />
      </button>
      <template v-if="isCompact">
        <div
          class="inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-[10px] font-black"
          :class="statusMeta.className"
        >
          <span class="h-1.5 w-1.5 rounded-full" :class="statusMeta.dotClass"></span>
          <span>{{ statusMeta.label }}</span>
        </div>
        <div
          v-if="showPlatform"
          class="inline-flex items-center rounded-full border border-border-main/70 bg-surface-high/50 px-2 py-0.5 text-[10px] font-extrabold text-text-muted"
        >
          {{ platformLabel(streamer.platform) }}
        </div>
      </template>
      <template v-else>
        <div
          v-if="showPlatform"
          class="flex items-center gap-1.5 rounded-full border px-2.5 py-0.5 text-[10px] font-extrabold"
          :class="{ 'border-red-500/50': streamer.lastUpdateFailed }"
        >
          <span
            class="h-2 w-2 rounded-full"
            :class="{
              'bg-[#10b981]':
                !streamer.lastUpdateFailed &&
                getLiveIndicatorClass(streamer) === 'is-live',
              'bg-red-700 shadow-[0_0_8px_#f59e0b]':
                !streamer.lastUpdateFailed &&
                getLiveIndicatorClass(streamer) === 'is-replay',
              'bg-[var(--border-color)]':
                streamer.lastUpdateFailed ||
                getLiveIndicatorClass(streamer) === 'is-offline',
            }"
          ></span>
          <span>{{ platformLabel(streamer.platform) }}</span>
        </div>
        <div
          v-else
          class="h-2 w-2 rounded-full"
          :class="{
            'bg-[#10b981]':
              !streamer.lastUpdateFailed &&
              getLiveIndicatorClass(streamer) === 'is-live',
            'bg-red-700 shadow-[0_0_8px_#f59e0b]':
              !streamer.lastUpdateFailed &&
              getLiveIndicatorClass(streamer) === 'is-replay',
            'bg-[var(--border-color)]':
              streamer.lastUpdateFailed ||
              getLiveIndicatorClass(streamer) === 'is-offline',
          }"
        ></div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Platform } from "../../../types/app/platform";
import type { FollowedStreamer } from "../../../types/models/streamer";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { AlertCircle } from "lucide-vue-next";
import { useFollowStore } from "../../../store/followStore";

const props = defineProps<{
  streamer: FollowedStreamer;
  getAvatarSrc: (s: FollowedStreamer) => string;
  handleImgError: (ev: Event, s: FollowedStreamer) => void;
  getLiveIndicatorClass: (s: FollowedStreamer) => string;
  proxyBase?: string;
  big?: boolean;
  showPlatform?: boolean;
  collapsed?: boolean;
  density?: "default" | "compact";
}>();

const emit = defineEmits<{ (e: "clickItem", s: FollowedStreamer): void }>();

const followStore = useFollowStore();

const onClick = () => emit("clickItem", props.streamer);
const onRetry = () =>
  followStore.retryStreamer(props.streamer.platform, props.streamer.id);

const canLoadAvatar = computed(() => {
  return (
    !!props.streamer.avatarUrl &&
    (props.streamer.platform !== Platform.BILIBILI || !!props.proxyBase)
  );
});

const isAvatarVisible = ref(false);
const avatarRef = ref<HTMLElement | null>(null);
let avatarObserver: IntersectionObserver | null = null;

const setupAvatarObserver = () => {
  if (!canLoadAvatar.value) {
    isAvatarVisible.value = false;
    return;
  }
  if (typeof window === "undefined" || !("IntersectionObserver" in window)) {
    isAvatarVisible.value = true;
    return;
  }
  if (avatarObserver) {
    avatarObserver.disconnect();
    avatarObserver = null;
  }
  avatarObserver = new IntersectionObserver(
    (entries) => {
      const entry = entries[0];
      if (entry?.isIntersecting) {
        isAvatarVisible.value = true;
        avatarObserver?.disconnect();
        avatarObserver = null;
      }
    },
    { rootMargin: "200px" },
  );
  if (avatarRef.value) {
    avatarObserver.observe(avatarRef.value);
  } else {
    isAvatarVisible.value = true;
  }
};

onMounted(setupAvatarObserver);
watch(canLoadAvatar, setupAvatarObserver);
onUnmounted(() => avatarObserver?.disconnect());

const platformLabel = (p: Platform): string => {
  switch (p) {
    case Platform.DOUYU:
      return "斗鱼";
    case Platform.DOUYIN:
      return "抖音";
    case Platform.HUYA:
      return "虎牙";
    case Platform.BILIBILI:
      return "B站";
    default:
      return "未知";
  }
};

const showPlatform = computed(() => !!props.showPlatform);
const isCompact = computed(() => props.density === "compact");

const statusMeta = computed(() => {
  const status = props.getLiveIndicatorClass(props.streamer);
  if (status === "is-live") {
    return {
      label: "LIVE",
      className: "border-emerald-500/40 bg-emerald-500/10 text-emerald-400",
      dotClass: "bg-emerald-400",
    };
  }
  return {
    label: props.streamer.lastUpdateFailed ? "ERR" : "OFF",
    className: "border-amber-500/40 bg-amber-500/10 text-amber-400",
    dotClass: "bg-amber-400",
  };
});

const isOnline = computed(() => {
  const status = props.getLiveIndicatorClass(props.streamer);
  return status === "is-live";
});

const shouldLoadAvatar = computed(
  () => canLoadAvatar.value && isAvatarVisible.value,
);
const avatarSrc = computed(() =>
  shouldLoadAvatar.value ? props.getAvatarSrc(props.streamer) : "",
);
</script>
