<template>
  <div class="flex items-center gap-2.5 px-1.5">
    <button 
      class="rounded-[12px] border border-[var(--glass-border)] bg-[var(--glass-bg)] px-3.5 py-1.5 text-[12px] font-bold tracking-[0.02em] text-[var(--secondary-text)] transition-all duration-300 ease-[cubic-bezier(0.23,1,0.32,1)] hover:-translate-y-0.5 hover:border-[var(--accent-color)] hover:bg-[var(--hover-bg)] hover:text-[var(--accent-color)]"
      :class="activeFilter === 'ALL' ? 'bg-[var(--bg-secondary)] border-[var(--border)] text-[var(--text-primary)] shadow-[var(--shadow-low)] -translate-y-0.5' : ''"
      @click="$emit('update:activeFilter', 'ALL')"
    >全部</button>
    <button 
      v-for="p in visiblePlatforms" 
      :key="p" 
      class="rounded-[12px] border border-[var(--glass-border)] bg-[var(--glass-bg)] px-3.5 py-1.5 text-[12px] font-bold tracking-[0.02em] text-[var(--secondary-text)] transition-all duration-300 ease-[cubic-bezier(0.23,1,0.32,1)] hover:-translate-y-0.5 hover:border-[var(--accent-color)] hover:bg-[var(--hover-bg)] hover:text-[var(--accent-color)]"
      :class="activeFilter === p ? 'bg-[var(--bg-secondary)] border-[var(--border)] text-[var(--text-primary)] shadow-[var(--shadow-low)] -translate-y-0.5' : ''"
      @click="$emit('update:activeFilter', p)"
    >{{ platformLabel(p) }}</button>
  </div>
</template>

<script setup lang="ts">
import { Platform } from '../../platforms/common/types';

defineProps<{
  visiblePlatforms: Platform[],
  activeFilter: 'ALL' | Platform,
}>();

const platformLabel = (p: Platform): string => {
  switch (p) {
    case Platform.DOUYU: return '斗鱼';
    case Platform.DOUYIN: return '抖音';
    case Platform.HUYA: return '虎牙';
    case Platform.BILIBILI: return 'B站';
    default: return '未知';
  }
};
</script>

