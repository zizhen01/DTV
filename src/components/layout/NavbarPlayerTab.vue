<template>
  <div
    class="group relative flex cursor-pointer items-center gap-2 overflow-hidden rounded-full border border-border-main py-1 pr-2 pl-1 shadow-sm transition-all duration-300 ease-out select-none"
    :class="
      isActive
        ? 'w-auto border-brand/30 bg-brand/10 ring-1 ring-brand/20'
        : 'w-8 border-transparent bg-surface-high/40 hover:w-40 hover:bg-surface-high'
    "
    draggable="true"
    @dragstart="$emit('dragstart', $event)"
    @dragover.prevent
    @click="$emit('select')"
  >
    <!-- Avatar -->
    <div class="relative h-6 w-6 flex-shrink-0">
      <img
        v-if="avatar"
        :src="avatar"
        :alt="anchorName"
        class="h-full w-full rounded-full object-cover"
      />
      <div
        v-else
        class="flex h-full w-full items-center justify-center rounded-full bg-surface-mid text-[10px] font-bold text-text-muted"
      >
        {{ anchorName?.[0] || "?" }}
      </div>
      <!-- Live Indicator -->
      <span v-if="isLive" class="absolute -right-0.5 -bottom-0.5 flex size-2">
        <span
          class="relative inline-flex h-2 w-2 rounded-full bg-green-500 ring-1 ring-surface-high"
        ></span>
      </span>
    </div>

    <!-- Info (Hidden when inactive, shown when active or hover) -->
    <div
      class="flex max-w-[100px] flex-col justify-center overflow-hidden opacity-0 transition-opacity delay-75 duration-300 group-hover:opacity-100"
      :class="{ '!opacity-100': isActive }"
    >
      <div
        class="truncate text-[11px] font-bold text-text-main"
        :class="{ 'text-brand': isActive }"
      >
        {{ anchorName }}
      </div>
    </div>

    <!-- Actions -->
    <div
      class="ml-auto flex items-center opacity-0 transition-all group-hover:opacity-100"
    >
      <!-- Close Button -->
      <button
        @click.stop="$emit('close')"
        class="flex size-5 items-center justify-center rounded-full text-text-muted hover:bg-red-500/10 hover:text-red-500"
        title="关闭直播间"
      >
        <X :size="12" stroke-width="3" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { X } from "lucide-vue-next";
import type { Platform } from "../../types/app/platform";

defineProps<{
  roomId: string;
  platform: Platform;
  anchorName: string;
  avatar: string;
  isLive: boolean;
  isActive: boolean;
}>();

defineEmits<{
  (e: "select"): void;
  (e: "close"): void;
  (e: "dragstart", event: DragEvent): void;
}>();
</script>
