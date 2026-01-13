<template>
  <aside
    class="mica z-[100] flex h-full flex-col border-r border-[var(--border)] transition-none"
    :style="{
      width: isCollapsed ? 'var(--sidebar-collapsed-width)' : 'var(--sidebar-width)',
    }"
  >
    <div v-show="!isCollapsed" class="flex-1 min-h-0 overflow-hidden px-3 pb-3">
      <FollowList
        :followedAnchors="followedAnchors"
        @selectAnchor="emit('select-anchor', $event)"
        @unfollow="emit('unfollow', $event)"
        @reorderList="emit('reorder-list', $event)"
      />
    </div>
  </aside>
</template>

<script setup lang="ts">
import type { FollowedStreamer, Platform } from '../platforms/common/types';
import FollowList from '../components/FollowsList/index.vue';

defineProps<{
  isCollapsed: boolean;
  followedAnchors: FollowedStreamer[];
}>();

const emit = defineEmits<{
  (event: 'select-anchor', streamer: FollowedStreamer): void;
  (event: 'unfollow', payload: { platform: Platform; id: string } | string): void;
  (event: 'reorder-list', newList: FollowedStreamer[]): void;
}>();
</script>

