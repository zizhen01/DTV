<template>
  <header class="app-header" data-tauri-drag-region>
    <nav class="platforms" data-tauri-drag-region>
          <div class="platforms-wrapper" ref="platformsRef">
            <div v-if="highlight.visible" class="platforms-highlight" :style="highlightStyles"></div>
      <router-link
        v-for="item in platformItems"
        :key="item.key"
        :to="item.path"
        custom
        v-slot="{ href, navigate, isActive }"
      >
        <a
          :href="href"
          :class="['platform-pill', { 'is-active': isActive }]"
          @click="(e) => navigate(e)"
          @mouseenter="handlePlatformHover(item.key)"
          @mouseleave="clearPlatformHover"
          :ref="(el) => setPlatformItemRef(item.key, el)"
          data-tauri-drag-region="none"
        >
          <img :src="item.logo" :alt="item.key" />
        </a>
      </router-link>
      </div>
    </nav>

    <div class="spacer" />

    <div class="header-actions" :class="{ 'header-actions--windows': shouldShowWindowsControls }">
      <div class="search-container" ref="searchContainerRef" data-tauri-drag-region>
        <div class="search-box">
          <input
            v-model="searchQuery"
            :placeholder="placeholderText"
            @input="handleSearch"
            @focus="showResults = true"
            @blur="handleBlur"
            class="search-input"
          />
          <button class="search-button" data-tauri-drag-region="none" @click="doSearch" :disabled="isLoadingSearch">
            <svg v-if="!isLoadingSearch" width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path d="M7.333 12.667A5.333 5.333 0 1 0 7.333 2a5.333 5.333 0 0 0 0 10.667zM14 14l-4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
            </svg>
            <div v-else class="mini-spinner"></div>
          </button>
        </div>

        <div v-show="showResults" class="search-results-wrapper">
          <div v-if="isLoadingSearch" class="search-loading">搜索中...</div>
          <div v-else-if="searchError" class="search-error-message">{{ searchError }}</div>
          <div v-else-if="searchResults.length > 0" class="search-results-list">
            <div v-for="anchor in searchResults" :key="anchor.platform + '-' + anchor.roomId" class="search-result-item" @mousedown="selectAnchor(anchor)">
              <div class="result-avatar">
                <img v-if="anchor.avatar" :src="anchor.avatar" :alt="anchor.userName" class="avatar-img" />
                <div v-else class="avatar-placeholder">{{ anchor.userName[0] }}</div>
              </div>

              <div class="result-main-content">
                <div class="result-line-1-main">
                  <span class="result-name" :title="anchor.userName">{{ anchor.userName }}</span>
                  <span class="live-status-badge styled-badge" :class="{ 'is-live': anchor.liveStatus }">{{ anchor.liveStatus ? '直播中' : '未开播' }}</span>
                </div>
                <div class="result-line-2-main">
                  <span class="result-room-title" :title="anchor.roomTitle || '无标题'">{{ anchor.roomTitle || '无直播标题' }}</span>
                  <span class="result-roomid styled-badge">{{ anchor.webId || anchor.roomId }}</span>
                </div>
              </div>

              <div class="result-meta-right">
                <span class="platform-tag styled-badge" :class="[anchor.platform.toLowerCase(), { 'douyu': anchor.platform === Platform.DOUYU, 'douyin': anchor.platform === Platform.DOUYIN, 'huya': anchor.platform === Platform.HUYA }]">
                  {{ anchor.platform === Platform.DOUYU ? '斗鱼' : (anchor.platform === Platform.DOUYIN ? '抖音' : (anchor.platform === Platform.HUYA ? '虎牙' : anchor.platform)) }}
                </span>
              </div>
            </div>
          </div>

          <div v-else-if="trimmedQuery && !isLoadingSearch && !searchError" class="search-no-results">
            无匹配结果。
            <button v-if="isPureNumeric(trimmedQuery)" class="search-fallback-btn" @mousedown.prevent="tryEnterRoom(trimmedQuery)" @click.prevent="tryEnterRoom(trimmedQuery)">尝试进入直播间 {{ trimmedQuery }}</button>
          </div>
        </div>
      </div>

      <button class="github-btn" title="打开 GitHub" data-tauri-drag-region="none" @click="openGithub">
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path fill="currentColor" d="M12 2C6.477 2 2 6.485 2 12.02c0 4.424 2.865 8.176 6.839 9.5.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.342-3.369-1.342-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.004.071 1.532 1.034 1.532 1.034.892 1.53 2.341 1.088 2.91.833.091-.647.35-1.089.636-1.34-2.22-.253-4.555-1.113-4.555-4.951 0-1.094.39-1.988 1.029-2.688-.103-.253-.446-1.27.098-2.646 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.54 9.54 0 0 1 2.505.337c1.909-1.296 2.748-1.026 2.748-1.026.545 1.376.202 2.393.1 2.646.64.7 1.028 1.594 1.028 2.688 0 3.848-2.338 4.695-4.566 4.944.36.31.68.924.68 1.861 0 1.343-.012 2.427-.012 2.757 0 .269.18.58.688.481A10.02 10.02 0 0 0 22 12.02C22 6.485 17.523 2 12 2z"/>
        </svg>
      </button>
      <button @click="toggleTheme" class="theme-btn" :class="{ 'theme-btn--windows': shouldShowWindowsControls }" :title="effectiveTheme === 'dark' ? '切换到日间模式' : '切换到夜间模式'" data-tauri-drag-region="none">
        <Transition name="theme-icon" mode="out-in">
          <Sun v-if="effectiveTheme === 'dark'" key="sun" class="theme-icon" :stroke-width="1.8" />
          <Moon v-else key="moon" class="theme-icon" :stroke-width="1.8" />
        </Transition>
      </button>

      <WindowsWindowControls v-if="shouldShowWindowsControls" />
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, reactive, nextTick, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { platform as detectPlatform } from '@tauri-apps/plugin-os';
import { openUrl } from '@tauri-apps/plugin-opener';
import { Platform } from '../platforms/common/types';
import { useThemeStore } from '../stores/theme';
import { useRoute } from 'vue-router';
import { Sun, Moon } from 'lucide-vue-next';
import WindowsWindowControls from '../components/window-controls/WindowsWindowControls.vue';
import douyuLogo from '../assets/douyu.webp';
import douyinLogo from '../assets/douyin.webp';
import huyaLogo from '../assets/huya.webp';
import bilibiliLogo from '../assets/bilibili.webp';

interface DouyinApiStreamInfo {
  title?: string | null;
  anchor_name?: string | null;
  avatar?: string | null;
  status?: number | null;
  error_message?: string | null;
  web_rid?: string | null;
}

interface HuyaAnchorItem {
  room_id: string;
  avatar: string;
  user_name: string;
  live_status: boolean;
  title: string;
}

interface SearchResultItem {
  platform: Platform;
  roomId: string;
  webId?: string | null;
  userName: string;
  roomTitle?: string | null;
  avatar: string | null;
  liveStatus: boolean;
  fansCount?: string;
  category?: string;
  rawStatus?: number | null;
}

const searchQuery = ref('');
const trimmedQuery = computed(() => searchQuery.value.trim());
const searchResults = ref<SearchResultItem[]>([]);
const showResults = ref(false);
const searchError = ref<string | null>(null);
const isLoadingSearch = ref(false);

const emit = defineEmits(['selectAnchor']);

const themeStore = useThemeStore();
const route = useRoute();

// Proxy support for Bilibili avatar images in search results
const proxyBase = ref<string | null>(null);
const ensureProxyStarted = async () => {
  if (!proxyBase.value) {
    try {
      const base = await invoke<string>('start_static_proxy_server');
      proxyBase.value = base;
    } catch (e) {
      console.error('[Header] Failed to start static proxy server', e);
    }
  }
};
const proxify = (url?: string | null): string | null => {
  if (!url) return null;
  if (proxyBase.value) {
    return `${proxyBase.value}/image?url=${encodeURIComponent(url)}`;
  }
  return url;
};

const effectiveTheme = computed(() => themeStore.getEffectiveTheme());

const detectedPlatform = ref<string | null>(null);
const searchContainerRef = ref<HTMLElement | null>(null);
const isWindowsPlatform = computed(() => {
  const name = detectedPlatform.value?.toLowerCase() ?? '';
  return name.startsWith('win');
});
const shouldShowWindowsControls = computed(() => isWindowsPlatform.value);

onMounted(async () => {
  try {
    detectedPlatform.value = await detectPlatform();
  } catch (error) {
    console.error('[Header] Failed to detect platform', error);
    if (typeof navigator !== 'undefined') {
      const ua = navigator.userAgent.toLowerCase();
      if (ua.includes('windows')) {
        detectedPlatform.value = 'windows';
      }
    }
  }
});

const handleDocumentMouseDown = (event: MouseEvent) => {
  const container = searchContainerRef.value;
  if (!container) return;
  if (event.target instanceof Node && !container.contains(event.target)) {
    showResults.value = false;
  }
};

const currentPlatform = computed<Platform>(() => {
  const name = route.name as string | undefined;
  const path = route.path;

  // Prefer route name for accuracy
  if (name) {
    if (name === 'douyinPlayer' || name === 'DouyinHome') return Platform.DOUYIN;
    if (name === 'huyaPlayer' || name === 'HuyaHome') return Platform.HUYA;
    if (name === 'bilibiliPlayer' || name === 'BilibiliHome') return Platform.BILIBILI;
    if (name === 'douyuPlayer' || name === 'DouyuHome') return Platform.DOUYU;
  }

  // Fallback to path matching (covers both home and player routes)
  if (path.startsWith('/player/douyin') || path.startsWith('/douyin')) return Platform.DOUYIN;
  if (path.startsWith('/player/huya') || path.startsWith('/huya')) return Platform.HUYA;
  if (path.startsWith('/player/bilibili') || path.startsWith('/bilibili')) return Platform.BILIBILI;
  if (path.startsWith('/player/douyu') || path.startsWith('/')) return Platform.DOUYU;

  // Default to Douyu
  return Platform.DOUYU;
});

const platformItems = [
  { key: 'douyu', path: '/', logo: douyuLogo },
  { key: 'douyin', path: '/douyin', logo: douyinLogo },
  { key: 'huya', path: '/huya', logo: huyaLogo },
  { key: 'bilibili', path: '/bilibili', logo: bilibiliLogo },
];

const platformsRef = ref<HTMLElement | null>(null);
const platformItemRefs = new Map<string, HTMLElement>();

const highlight = reactive({ left: 0, width: 0, visible: false });

const highlightStyles = computed(() => ({
  transform: `translateX(${highlight.left}px)`,
  width: `${highlight.width}px`,
  opacity: highlight.visible ? 1 : 0,
}));

const setPlatformItemRef = (key: string, el: any) => {
  if (!el) {
    platformItemRefs.delete(key);
    return;
  }
  const element = el.$el || el;
  if (element instanceof HTMLElement) {
    platformItemRefs.set(key, element);
  }
};

const updateHighlightForKey = async (key: string | null) => {
  await nextTick();
  if (!key) {
    highlight.visible = false;
    return;
  }
  const el = platformItemRefs.get(key);
  const container = platformsRef.value;
  if (!el || !container) {
    highlight.visible = false;
    return;
  }
  const rect = el.getBoundingClientRect();
  const containerRect = container.getBoundingClientRect();
  highlight.left = rect.left - containerRect.left;
  highlight.width = rect.width;
  highlight.visible = true;
};

let hoverTimeout: number | null = null;
const handlePlatformHover = (key: string) => {
  if (hoverTimeout) {
    clearTimeout(hoverTimeout);
    hoverTimeout = null;
  }
  updateHighlightForKey(key);
};

const clearPlatformHover = () => {
  hoverTimeout = window.setTimeout(() => {
    highlight.visible = false;
    hoverTimeout = null;
  }, 200);
};

watch(() => route.path, () => {
  // set highlight to active platform on route change
  const match = platformItems.find(p => p.path === route.path);
  updateHighlightForKey(match ? match.key : null);
}, { immediate: true });

onMounted(() => {
  window.addEventListener('resize', () => updateHighlightForKey(null));
  document.addEventListener('mousedown', handleDocumentMouseDown);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', () => updateHighlightForKey(null));
  document.removeEventListener('mousedown', handleDocumentMouseDown);
});

const placeholderText = computed(() => {
  if (currentPlatform.value === Platform.DOUYU) return '搜索斗鱼主播';
  if (currentPlatform.value === Platform.HUYA) return '搜索虎牙主播';
  if (currentPlatform.value === Platform.DOUYIN) return '搜索抖音房间ID';
  if (currentPlatform.value === Platform.BILIBILI) return '搜索B站主播';
  return '搜索主播';
});

const toggleTheme = () => {
  const currentTheme = themeStore.getEffectiveTheme();
  if (currentTheme === 'light') {
    themeStore.setUserPreference('dark');
  } else {
    themeStore.setUserPreference('light');
  }
};

const openGithub = async () => {
  try {
    await openUrl('https://github.com/chen-zeong/DTV/releases');
  } catch (error) {
    if (typeof window !== 'undefined') {
      window.open('https://github.com/chen-zeong/DTV/releases', '_blank', 'noopener,noreferrer');
      return;
    }
    console.error('[Header] Failed to open GitHub', error);
  }
};

onBeforeUnmount(() => {});

let searchTimeout: number | null = null;

const isPureNumeric = (value: string): boolean => /^\d+$/.test(value);

const resetSearchState = () => {
  if (searchTimeout) {
    clearTimeout(searchTimeout);
    searchTimeout = null;
  }
  searchQuery.value = '';
  searchResults.value = [];
  searchError.value = null;
  showResults.value = false;
  isLoadingSearch.value = false;
};

const handleSearch = () => {
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  searchError.value = null;
  isLoadingSearch.value = true;
  
  searchTimeout = window.setTimeout(() => {
    performSearchBasedOnInput();
  }, 500);
};

const performSearchBasedOnInput = async () => {
  const query = trimmedQuery.value;
  if (!query) {
    searchResults.value = [];
    showResults.value = false;
    isLoadingSearch.value = false;
    return;
  }
  searchQuery.value = query;

  if (currentPlatform.value === Platform.DOUYIN) {
    await performDouyinIdSearch(query);
  } else if (currentPlatform.value === Platform.HUYA) {
    await performHuyaSearch(query);
  } else if (currentPlatform.value === Platform.BILIBILI) {
    await performBilibiliSearch(query);
  } else {
    await performDouyuSearch(query);
  }
  isLoadingSearch.value = false;
};

const performDouyinIdSearch = async (userInputRoomId: string) => {
  searchResults.value = [];
  searchError.value = null;
  isLoadingSearch.value = true;
  try {
    const payloadData = { args: { room_id_str: userInputRoomId } };
    const douyinInfo = await invoke<DouyinApiStreamInfo>('fetch_douyin_streamer_info', {
      payload: payloadData,
    });
    isLoadingSearch.value = false;
      if (douyinInfo) {
        if (douyinInfo.anchor_name) {
          const isLive = douyinInfo.status === 2;
          const webId = (douyinInfo as any).web_rid ?? userInputRoomId;
          searchResults.value = [{
            platform: Platform.DOUYIN,
            roomId: webId,
            webId,
            userName: douyinInfo.anchor_name || '未知抖音主播',
            roomTitle: douyinInfo.title || null,
            avatar: douyinInfo.avatar || null,
            liveStatus: isLive,
            rawStatus: douyinInfo.status,
        }];
        }
    } else {
      searchError.value = '搜索服务暂时不可用，请稍后再试。';
    }
  } catch (e: any) {
    isLoadingSearch.value = false;
    searchError.value = '搜索服务暂时不可用，请稍后再试。';
  }
  showResults.value = true;
};

const performHuyaSearch = async (keyword: string) => {
  searchResults.value = [];
  searchError.value = null;
  isLoadingSearch.value = true;
  try {
    const items = await invoke<HuyaAnchorItem[]>('search_huya_anchors', { keyword, page: 1 });
    // Ensure static proxy server is running for Huya avatars
    await ensureProxyStarted();
    isLoadingSearch.value = false;
    if (Array.isArray(items) && items.length > 0) {
      searchResults.value = items.map((item): SearchResultItem => ({
        platform: Platform.HUYA,
        roomId: item.room_id,
        userName: item.user_name || '虎牙主播',
        roomTitle: item.title || null,
        avatar: proxify(item.avatar || null),
        liveStatus: !!item.live_status,
      }));
      searchError.value = null;
    }
  } catch (e) {
    isLoadingSearch.value = false;
    searchError.value = '搜索服务暂时不可用，请稍后再试。';
  }
  showResults.value = true;
};

const performDouyuSearch = async (keyword: string) => {
  searchResults.value = [];
  searchError.value = null;
  isLoadingSearch.value = true;
  try {
    const response = await invoke<string>('search_anchor', { keyword });
    isLoadingSearch.value = false;
    const data = JSON.parse(response);
    if (data.error === 0 && data.data && data.data.relateUser) {
      searchResults.value = data.data.relateUser
        .filter((item: any) => item.type === 1)
        .map((item: any): SearchResultItem => {
          const anchorInfo = item.anchorInfo;
          const isReallyLive = anchorInfo.isLive === 1 && anchorInfo.videoLoop !== 1;
          return {
            platform: Platform.DOUYU,
            roomId: anchorInfo.rid.toString(),
            userName: anchorInfo.nickName,
            roomTitle: anchorInfo.roomName || anchorInfo.description || null,
            avatar: anchorInfo.avatar,
            liveStatus: isReallyLive,
            fansCount: anchorInfo.fansNumStr,
            category: anchorInfo.cateName,
          };
        });
      searchError.value = null;
    } else {
      searchError.value = '搜索服务暂时不可用，请稍后再试。';
    }
  } catch (e) {
    isLoadingSearch.value = false;
    searchError.value = '搜索服务暂时不可用，请稍后再试。';
  }
  showResults.value = true;
};

const doSearch = () => {
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  isLoadingSearch.value = true;
  performSearchBasedOnInput();
};

const handleBlur = () => {
  setTimeout(() => {
    if (!isLoadingSearch.value && !searchError.value) {
       showResults.value = false;
    }
  }, 300);
};

type BilibiliSearchItem = {
  room_id: string;
  title: string;
  cover: string;
  anchor: string;
  avatar: string;
  watching: string;
  area: string;
  is_live: boolean;
};

const performBilibiliSearch = async (keyword: string) => {
  searchResults.value = [];
  searchError.value = null;
  isLoadingSearch.value = true;
  try {
    const response = await invoke<BilibiliSearchItem[]>('search_bilibili_rooms', {
      keyword,
      page: 1,
    });
    await ensureProxyStarted();
    if (Array.isArray(response) && response.length > 0) {
      searchResults.value = response.map((item) => ({
        platform: Platform.BILIBILI,
        roomId: item.room_id,
        webId: item.room_id,
        userName: item.anchor || '未知B站主播',
        roomTitle: item.title || null,
        avatar: proxify(item.avatar),
        liveStatus: item.is_live,
        fansCount: item.watching,
        category: item.area,
      }));
    }
  } catch (e) {
    searchError.value = '搜索服务暂时不可用，请稍后再试。';
  } finally {
    isLoadingSearch.value = false;
    showResults.value = true;
  }
};

const selectAnchor = (anchor: SearchResultItem) => {
  emit('selectAnchor', {
    id: anchor.webId || anchor.roomId,
    platform: anchor.platform,
    nickname: anchor.userName,
    avatarUrl: anchor.avatar,
    currentRoomId: undefined,
  });
  resetSearchState();
};

const tryEnterRoom = (roomId: string) => {
  if (!roomId) return;
  emit('selectAnchor', {
    id: roomId,
    platform: currentPlatform.value,
    nickname: roomId,
    avatarUrl: null,
    currentRoomId: undefined,
  });
  resetSearchState();
};

</script>

<style scoped>
.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
  background: transparent;
  height: 80px;
  box-sizing: border-box;
  position: sticky;
  top: 0;
  z-index: 1000;
  transition: all 0.4s ease;
}

.search-container {
  width: 360px;
  max-width: 100%;
  position: relative;
}

.search-box {
  display: flex;
  align-items: center;
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-radius: var(--radius-lg);
  padding: 0 20px;
  border: 1px solid var(--glass-border);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  height: 44px;
  box-shadow: none;
}

.search-box:focus-within {
  border-color: var(--glass-border);
  box-shadow: none;
  transform: none;
}

.search-input {
  flex-grow: 1;
  padding: 10px 12px;
  border: none;
  outline: none;
  font-size: 15px;
  background-color: transparent;
  color: var(--primary-text);
  font-weight: 500;
}

.search-input::placeholder {
  color: var(--dim-text);
}

.search-button {
  background: transparent;
  border: none;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  cursor: pointer;
  color: var(--secondary-text);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.search-button:hover:not(:disabled) {
  color: var(--accent-color);
  background: var(--hover-bg);
}

.search-results-wrapper {
  position: absolute;
  top: calc(100% + 12px);
  left: 0;
  right: auto;
  width: min(360px, 90vw);
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-radius: var(--radius-md);
  box-shadow: var(--glass-shadow);
  max-height: 480px;
  overflow-y: auto;
  z-index: 1001;
  border: 1px solid var(--glass-border);
  padding: 8px;
  animation: slideDown 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes slideDown {
  from { opacity: 0; transform: translateY(-12px) scale(0.98); }
  to { opacity: 1; transform: translateY(0) scale(1); }
}

.search-result-item {
  display: flex;
  align-items: center;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
  gap: 10px;
}

.search-result-item:hover {
  background: var(--hover-bg);
  transform: translateX(4px);
}

.result-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  overflow: hidden;
  background: var(--tertiary-bg);
  border: 2px solid var(--border-color);
  flex-shrink: 0;
}

.avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.result-main-content {
  flex: 1;
  min-width: 0;
}

.result-line-1-main {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 1px;
}

.result-name {
  font-weight: 600;
  font-size: 13px;
  color: var(--primary-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.result-room-title {
  font-size: 12px;
  color: var(--secondary-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  position: static;
  right: auto;
  top: auto;
  transform: none;
}

.platforms {
  display: flex;
  gap: 10px;
  align-items: center;
}

.platform-pill {
  width: 46px;
  height: 46px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 18px;
  background: transparent;
  border: none;
  box-shadow: none;
  transition: transform 0.18s ease, box-shadow 0.18s ease;
}

.platform-pill img {
  width: 24px;
  height: 24px;
  object-fit: contain;
}

.platform-pill:hover {
  transform: translateY(-3px);
}

.platform-pill.is-active {
  transform: translateY(-2px) scale(1.03);
  box-shadow: 0 10px 28px rgba(0,0,0,0.12);
  background: linear-gradient(180deg, rgba(255,255,255,0.04), rgba(255,255,255,0.02));
  border: 1px solid rgba(255,255,255,0.06);
}

.platforms-wrapper {
  position: relative;
  display: flex;
  gap: 10px;
  align-items: center;
  padding: 6px;
  border-radius: 26px;
  background: var(--glass-bg);
  border: 1px solid var(--glass-border);
  box-shadow: none;
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
}

.platforms-highlight {
  position: absolute;
  top: 6px;
  bottom: 6px;
  left: 0;
  border-radius: 18px;
  background: linear-gradient(90deg, rgba(255,255,255,0.06), rgba(255,255,255,0.02));
  box-shadow: 0 8px 30px rgba(0,0,0,0.08);
  transition: transform 0.28s cubic-bezier(0.2, 0.9, 0.2, 1), width 0.28s cubic-bezier(0.2, 0.9, 0.2, 1), opacity 0.18s ease;
  pointer-events: none;
  z-index: 0;
}

.platform-pill {
  z-index: 1;
}

.spacer { flex: 1; }

.theme-btn {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  color: var(--secondary-text);
  border: 1px solid var(--glass-border);
  border-radius: 50%;
  width: 44px;
  height: 44px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: none;
}

.theme-btn:hover {
  background: var(--hover-bg);
  color: var(--accent-color);
  border-color: var(--accent-color);
  transform: rotate(15deg) scale(1.1);
}

.github-btn {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  color: var(--secondary-text);
  border: 1px solid var(--glass-border);
  border-radius: 50%;
  width: 44px;
  height: 44px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: none;
}

.github-btn:hover {
  background: var(--hover-bg);
  color: var(--accent-color);
  border-color: var(--accent-color);
  transform: translateY(-1px) scale(1.04);
}

.github-btn svg {
  width: 20px;
  height: 20px;
}


:root[data-theme="dark"] .platforms-wrapper,
:root[data-theme="dark"] .search-box,
:root[data-theme="dark"] .theme-btn,
:root[data-theme="dark"] .github-btn {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.16);
}

.mini-spinner {
  width: 18px;
  height: 18px;
  border: 2px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: mini-spin 0.8s linear infinite;
}

@keyframes mini-spin {
  to { transform: rotate(360deg); }
}

.styled-badge {
  font-size: 10px;
  padding: 2px 10px;
  border-radius: 100px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.live-status-badge.is-live {
  background: rgba(255, 62, 62, 0.15);
  color: #ff3e3e;
  border: 1px solid rgba(255, 62, 62, 0.2);
}

.platform-tag {
  background: var(--hover-bg);
  color: var(--secondary-text);
  border: 1px solid var(--glass-border);
}

.platform-tag.douyu { color: #ff7a1c; }
.platform-tag.douyin { color: #fe2c55; }
.platform-tag.huya { color: #f5a623; }
.platform-tag.bilibili { color: #fb7299; }
</style>
