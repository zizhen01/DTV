<template>
  <aside class="app-sidebar" data-tauri-drag-region>
    <div class="sidebar-header" data-tauri-drag-region>
      <!-- Logo removed for density -->
    </div>

    <!-- 平台入口已移至顶部导航栏，侧边栏仅保留关注列表 -->
    
    <div class="follow-list-wrapper">
      <FollowList 
        :followedAnchors="sortedFollowedAnchors"
        @selectAnchor="handleSelectAnchor"
        @unfollow="handleUnfollow"
        @reorderList="handleReorderList"
        class="follow-list-component"
      />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { FollowedStreamer } from '../platforms/common/types';
import FollowList from '../components/FollowsList/index.vue';

const emit = defineEmits(['selectAnchor', 'unfollow', 'reorderList']);

const props = withDefaults(defineProps<{ followedAnchors?: FollowedStreamer[] }>(), {
  followedAnchors: () => []
});

const customSortedAnchors = ref<FollowedStreamer[]>([]);

const sortedFollowedAnchors = computed(() => {
  if (!props.followedAnchors?.length) return [];
  const toKey = (a: FollowedStreamer) => `${a.platform}:${a.id}`;
  const currentKeys = new Set(props.followedAnchors.map(toKey));
  const customSortedIsValid = customSortedAnchors.value.length > 0 && 
    customSortedAnchors.value.length === props.followedAnchors.length && 
    customSortedAnchors.value.every(customAnchor => currentKeys.has(toKey(customAnchor)));
  let baseOrder: FollowedStreamer[];
  if (customSortedIsValid) {
    const propsMap = new Map(props.followedAnchors.map(anchor => [toKey(anchor), anchor]));
    baseOrder = customSortedAnchors.value
      .map(customAnchor => propsMap.get(toKey(customAnchor)))
      .filter(Boolean) as FollowedStreamer[];
  } else {
    baseOrder = [...props.followedAnchors];
  }
  const live: FollowedStreamer[] = [];
  const looping: FollowedStreamer[] = [];
  const rest: FollowedStreamer[] = [];
  baseOrder.forEach(anchor => {
    if (anchor.liveStatus === 'LIVE') live.push(anchor);
    else if (anchor.liveStatus === 'REPLAY') looping.push(anchor);
    else rest.push(anchor);
  });
  return [...live, ...looping, ...rest];
});

const handleSelectAnchor = (anchor: FollowedStreamer) => emit('selectAnchor', anchor);
const handleUnfollow = (payload: any) => emit('unfollow', payload);
const handleReorderList = (reorderedList: FollowedStreamer[]) => {
  customSortedAnchors.value = [...reorderedList];
  emit('reorderList', reorderedList);
};
</script>

<style scoped>
.app-sidebar {
  width: 230px;
  background: transparent;
  display: flex;
  flex-direction: column;
  padding: 0 10px;
  gap: 24px;
  z-index: 100;
}

.sidebar-header {
  height: 0;
  margin: 0;
  padding: 0;
}

/* Redesign Nav Items */
.navigation {
  display: flex;
  flex-direction: column;
  gap: 10px;
  position: relative;
  margin-top: 0;
}

.nav-shared-highlight {
  position: absolute;
  left: 0;
  right: 0;
  background: transparent;
  border: none;
  border-radius: var(--radius-md);
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  pointer-events: none;
  z-index: 0;
}

.nav-item {
  --nav-accent-rgb: 0, 218, 198;
  position: relative;
  display: flex;
  align-items: center;
  padding: 10px 16px;
  border-radius: var(--radius-md);
  color: var(--secondary-text);
  text-decoration: none;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.92), rgba(245, 248, 252, 0.9));
  z-index: 1;
  overflow: hidden;
  box-shadow: none;
}

.nav-item:hover {
  color: var(--primary-text);
  background: var(--hover-bg);
}

.nav-item.is-active {
  color: var(--primary-text);
  font-weight: 700;
  background: linear-gradient(155deg, rgba(var(--nav-accent-rgb), 0.22), rgba(255, 255, 255, 0.92));
  box-shadow: none;
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.nav-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.nav-icon {
  width: 38px;
  height: 38px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  padding: 6px;
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.85);
  border: none;
  box-shadow: none;
}

.nav-icon img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  transition: all 0.3s ease;
}

.nav-item:hover .nav-icon {
  background: rgba(255, 255, 255, 0.95);
}

.nav-item.is-active .nav-icon {
  background: rgba(255, 255, 255, 0.96);
  box-shadow: none;
}

.nav-item.is-active .nav-icon img,
.nav-item:hover .nav-icon img {
  filter: brightness(1.05) saturate(1.1);
}

/* In light mode, revert invert if needed, but for accent color text, usually dark text on bright green */
html[data-theme="light"] .nav-item.is-active .nav-icon img {
  filter: drop-shadow(0 4px 10px rgba(0, 128, 128, 0.25));
}

html[data-theme="light"] .nav-icon {
  background: rgba(15, 23, 42, 0.06);
}

html[data-theme="light"] .nav-item:hover .nav-icon {
  background: rgba(var(--nav-accent-rgb), 0.14);
}

html[data-theme="light"] .nav-item.is-active .nav-icon {
  background: rgba(var(--nav-accent-rgb), 0.18);
}

/* Per-platform accent colors (deeper tones) */
.nav-item.nav-douyu { --nav-accent-rgb: 234, 120, 30; }
.nav-item.nav-douyin { --nav-accent-rgb: 110, 64, 200; }
.nav-item.nav-huya { --nav-accent-rgb: 215, 140, 20; }
.nav-item.nav-bilibili { --nav-accent-rgb: 60, 130, 240; }

.active-dot {
  position: absolute;
  right: 14px;
  top: 50%;
  transform: translateY(-50%);
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: rgba(var(--nav-accent-rgb), 0.9);
  display: none;
}

.nav-item.is-active .active-dot {
  display: block;
}

@media (prefers-color-scheme: dark) {
  .nav-item {
    background: rgba(255, 255, 255, 0.06);
    box-shadow: none;
  }
  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
  }
  .nav-item.is-active {
    background: linear-gradient(155deg, rgba(var(--nav-accent-rgb), 0.28), rgba(255, 255, 255, 0.08));
  }
  .nav-icon {
    background: rgba(255, 255, 255, 0.12);
  }
  .nav-item.is-active .nav-icon {
    background: rgba(255, 255, 255, 0.16);
  }
}

.nav-name {
  font-size: 16px;
  letter-spacing: 0.3px;
}

.active-dot {
  display: none; /* Removed dot in favor of full pill active state */
}

.follow-list-wrapper {
  flex: 1;
  overflow: hidden;
  margin-top: 0;
  padding: 0;
  background: transparent;
}

.follow-list-component {
  height: 100%;
  padding: 0;
}

:global(:root:not([data-theme="light"])) .app-sidebar {
  background: rgba(255, 255, 255, 0.035);
}

:deep(.follow-list-component::-webkit-scrollbar) {
  width: 4px;
}
:deep(.follow-list-component::-webkit-scrollbar-thumb) {
  background: var(--glass-border);
  border-radius: 10px;
}
</style>
```
