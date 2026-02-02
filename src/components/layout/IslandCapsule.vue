<template>
  <div
    class="relative flex items-center overflow-hidden rounded-full transition-all duration-500 ease-[cubic-bezier(0.16,1,0.3,1)]"
    :class="[
      isExpanded ? 'pr-2 bg-surface-high/60 shadow-lg' : 'pr-0 hover:bg-surface-high/30',
      isActive ? 'bg-surface-high ring-1 ring-brand/30' : ''
    ]"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @click="handleClick"
  >
    <!-- Avatar Section -->
    <div class="relative flex h-9 w-9 shrink-0 items-center justify-center p-0.5">
      <img
        :src="streamer.avatar || ''"
        class="h-full w-full rounded-full object-cover transition-transform duration-500 ease-[cubic-bezier(0.16,1,0.3,1)]"
        :class="{ 'rotate-12 scale-110': isActive }"
      />
      <div
        class="absolute -right-0.5 -bottom-0.5 h-2.5 w-2.5 rounded-full border-2 border-app-bg"
        :class="streamer.isLive ? 'bg-emerald-400' : 'bg-amber-400'"
        title="在线状态"
      ></div>
      <!-- Active Indicator (Waveform simulation) -->
      <div v-if="isActive" class="absolute inset-0 flex items-center justify-center rounded-full bg-black/10">
         <div class="flex gap-0.5 items-end h-3">
            <div class="w-0.5 bg-brand animate-[bounce_1s_infinite] h-2"></div>
            <div class="w-0.5 bg-brand animate-[bounce_1.2s_infinite] h-3"></div>
            <div class="w-0.5 bg-brand animate-[bounce_0.8s_infinite] h-1.5"></div>
         </div>
      </div>
    </div>

    <!-- Info & Controls Section -->
    <div
      class="flex items-center gap-0 overflow-hidden transition-all duration-500 ease-[cubic-bezier(0.16,1,0.3,1)]"
      :class="isExpanded ? 'max-w-[400px] opacity-100 ml-2' : 'max-w-0 opacity-0 ml-0'"
    >
      <!-- Text Info -->
      <div class="flex flex-col whitespace-nowrap pr-3">
        <span class="max-w-[80px] truncate text-[11px] font-black leading-tight">{{ streamer.anchorName }}</span>
        <span class="text-[9px] font-bold text-text-muted leading-none mt-0.5">{{ isActive ? '正在播放' : '待播放' }}</span>
      </div>

      <!-- Control Buttons -->
      <div class="flex items-center gap-1 pr-1 border-l border-border-main/30 pl-2">
        <!-- Fav -->
        <button
          @click.stop="$emit('fav')"
          class="flex h-6 w-6 items-center justify-center rounded-full hover:bg-surface-high text-text-muted hover:text-brand transition-colors"
          :title="isFollowed ? '取消关注' : '关注'"
        >
          <Star class="size-3" :fill="isFollowed ? 'currentColor' : 'none'" />
        </button>
        
        <!-- Mute (Toggle) -->
        <button
          @click.stop="$emit('mute')"
          class="flex h-6 w-6 items-center justify-center rounded-full hover:bg-surface-high text-text-muted hover:text-text-main transition-colors"
          :title="streamer.isMuted ? '取消静音' : '静音'"
        >
          <VolumeX v-if="streamer.isMuted" class="size-3" />
          <Volume2 v-else class="size-3" />
        </button>

        <!-- Close -->
        <button
          @click.stop="$emit('close')"
          class="flex h-6 w-6 items-center justify-center rounded-full hover:bg-red-500/20 text-text-muted hover:text-red-500 transition-colors"
          title="关闭"
        >
          <X class="size-3" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { Star, X, Volume2, VolumeX } from "lucide-vue-next";
import type { StreamerInfoState } from "../../store/playerStore";

const props = defineProps<{
  streamer: StreamerInfoState;
  isActive: boolean;
  isFollowed: boolean;
}>();

const emit = defineEmits<{
  (e: "click"): void;
  (e: "fav"): void;
  (e: "mute"): void;
  (e: "close"): void;
}>();

const isHovered = ref(false);

// Expand if Active OR Hovered
const isExpanded = computed(() => props.isActive || isHovered.value);

const handleClick = () => {
  // If not active, activate it
  if (!props.isActive) {
    emit("click");
  } else {
    // If active, maybe jump to room? Handled by parent
    emit("click");
  }
};
</script>
