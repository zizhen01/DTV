<template>
  <nav
    class="mica sticky top-0 z-50 flex h-[var(--navbar-height)] items-center gap-6 border-b border-[var(--border)] px-6 pl-4"
    data-tauri-drag-region
    :style="{
      paddingRight: shouldShowWindowsControls && !isMacPreview ? '160px' : '16px',
    }"
  >
    <div v-if="isMacPreview" class="inline-flex items-center gap-2 pl-1" aria-hidden="true">
      <span class="h-2.5 w-2.5 rounded-full bg-[#ff5f57] shadow-[inset_0_0_0_1px_rgba(0,0,0,0.12)]"></span>
      <span class="h-2.5 w-2.5 rounded-full bg-[#febc2e] shadow-[inset_0_0_0_1px_rgba(0,0,0,0.12)]"></span>
      <span class="h-2.5 w-2.5 rounded-full bg-[#28c840] shadow-[inset_0_0_0_1px_rgba(0,0,0,0.12)]"></span>
    </div>

    <div class="flex-none" data-tauri-drag-region>
      <div class="relative flex overflow-hidden rounded-[var(--radius-md)] bg-[var(--hover-bg)] p-1" ref="platformTabsRef" data-tauri-drag-region>
        <div
          class="absolute inset-y-1 left-0 z-[1] rounded-[var(--radius-sm)] bg-[var(--platform-active-bg)] shadow-[0_2px_8px_rgba(0,0,0,0.08)] transition-[transform,width,opacity] duration-200 ease-[cubic-bezier(0.16,1,0.3,1)]"
          :style="highlightStyles"
          data-tauri-drag-region
        />
      <button
        v-for="platform in platforms"
        :key="platform.id"
        type="button"
        class="relative z-[2] rounded-[var(--radius-sm)] px-4 py-1.5 text-sm font-medium text-[var(--text-secondary)] transition-[color,transform] duration-200 ease-[cubic-bezier(0.16,1,0.3,1)] hover:bg-[rgba(0,0,0,0.03)]"
        :class="{ 'font-bold text-[var(--text-primary)]': activePlatform === platform.id }"
        data-tauri-drag-region="false"
        :ref="(el) => setPlatformRef(platform.id, el)"
        @click="emit('platform-change', platform.id)"
      >
        {{ platform.name }}
      </button>
    </div>
    </div>

    <div class="relative flex flex-1 items-center justify-end gap-3" :class="{ 'static': shouldShowWindowsControls }" data-tauri-drag-region>
      <div class="relative ml-auto w-[min(360px,36vw)]" ref="searchContainerRef" data-tauri-drag-region="false">
          <div class="relative z-10 flex w-full max-w-[420px] items-center rounded-[var(--radius-lg)] bg-[var(--hover-bg)] transition-transform duration-200 ease-[cubic-bezier(0.16,1,0.3,1)]" :class="{ 'scale-[1.02] shadow-[var(--shadow-lg)]': isSearchFocused }">
            <input
              v-model="searchQuery"
              type="text"
              :placeholder="placeholderText"
              data-tauri-drag-region="false"
              class="w-full rounded-[var(--radius-lg)] border border-transparent bg-transparent px-4 py-2.5 text-sm text-[var(--text-primary)] outline-none"
              @focus="handleFocus"
              @blur="handleBlur"
              @input="handleSearch"
            />
            <button
              v-if="searchQuery"
              type="button"
              class="mr-1.5 flex h-8 w-8 items-center justify-center rounded-full text-[var(--text-secondary)] transition-all duration-200 hover:bg-[rgba(0,0,0,0.06)] hover:text-[var(--accent)]"
              data-tauri-drag-region="false"
              aria-label="清除搜索"
              @click="resetSearchState"
            >
              <X :size="14" />
            </button>
          </div>

          <div v-show="showResults" class="absolute left-0 right-0 top-[calc(100%+10px)] z-[1001] max-h-[480px] overflow-y-auto rounded-[var(--radius-md)] border border-[var(--glass-border)] bg-[var(--glass-bg)] p-2 shadow-[var(--glass-shadow)] [backdrop-filter:var(--glass-blur)] [-webkit-backdrop-filter:var(--glass-blur)]">
            <div v-if="isLoadingSearch" class="px-3 py-2 text-[13px] text-[var(--secondary-text)]">搜索中...</div>
            <div v-else-if="searchError" class="px-3 py-2 text-[13px] text-[var(--secondary-text)]">{{ searchError }}</div>
            <div v-else-if="searchResults.length > 0" class="space-y-1">
              <div
                v-for="anchor in searchResults"
                :key="anchor.platform + '-' + anchor.roomId"
                class="flex items-center gap-2.5 rounded-[var(--radius-md)] px-2.5 py-2 transition-all duration-200 hover:translate-x-1 hover:bg-[var(--hover-bg)]"
                @mousedown="selectAnchor(anchor)"
              >
                <div class="h-9 w-9 flex-shrink-0 overflow-hidden rounded-full border-2 border-[var(--border-color)] bg-[var(--tertiary-bg)]">
                  <img v-if="anchor.avatar" :src="anchor.avatar" :alt="anchor.userName" class="h-full w-full object-cover" />
                  <div v-else class="flex h-full w-full items-center justify-center bg-[var(--hover-bg)] text-[var(--text-primary)]">{{ anchor.userName[0] }}</div>
                </div>

                <div class="min-w-0 flex-1">
                  <div class="flex items-center gap-2">
                    <span class="truncate text-[13px] font-semibold text-[var(--primary-text)]" :title="anchor.userName">{{ anchor.userName }}</span>
                    <span
                      class="rounded-full border px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-[0.5px]"
                      :class="anchor.liveStatus ? 'border-[rgba(255,62,62,0.2)] bg-[rgba(255,62,62,0.15)] text-[#ff3e3e]' : 'border-[var(--glass-border)] bg-[var(--hover-bg)] text-[var(--secondary-text)]'"
                    >
                      {{ anchor.liveStatus ? '直播' : '未播' }}
                    </span>
                  </div>
                  <div class="flex items-center gap-2">
                    <span class="truncate text-xs text-[var(--secondary-text)]" :title="anchor.roomTitle || '暂无标题'">{{ anchor.roomTitle || '暂无标题' }}</span>
                    <span class="rounded-full border border-[var(--glass-border)] bg-[var(--hover-bg)] px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-[0.5px] text-[var(--secondary-text)]">{{ anchor.webId || anchor.roomId }}</span>
                  </div>
                </div>

                <div class="flex items-center">
                  <span
                    class="rounded-full border border-[var(--glass-border)] bg-[var(--hover-bg)] px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-[0.5px] text-[var(--secondary-text)]"
                    :class="[
                      anchor.platform.toLowerCase(),
                      { douyu: anchor.platform === Platform.DOUYU, douyin: anchor.platform === Platform.DOUYIN, huya: anchor.platform === Platform.HUYA }
                    ]"
                    :style="anchor.platform === Platform.DOUYU ? 'color: #ff7a1c' : anchor.platform === Platform.DOUYIN ? 'color: #fe2c55' : anchor.platform === Platform.HUYA ? 'color: #f5a623' : anchor.platform === Platform.BILIBILI ? 'color: #fb7299' : ''"
                  >
                    {{ anchor.platform === Platform.DOUYU ? '斗鱼' : (anchor.platform === Platform.DOUYIN ? '抖音' : (anchor.platform === Platform.HUYA ? '虎牙' : anchor.platform)) }}
                  </span>
                </div>
              </div>
            </div>

            <div v-else-if="trimmedQuery && !isLoadingSearch && !searchError" class="px-3 py-2 text-[13px] text-[var(--secondary-text)]">
              未找到结果
              <button
                v-if="isPureNumeric(trimmedQuery)"
                class="font-semibold text-[var(--accent)]"
                @mousedown.prevent="tryEnterRoom(trimmedQuery)"
                @click.prevent="tryEnterRoom(trimmedQuery)"
              >
                进入房间 {{ trimmedQuery }}
              </button>
            </div>
          </div>
      </div>

      <button type="button" class="relative flex items-center justify-center rounded-[var(--radius-md)] bg-[var(--hover-bg)] p-2.5 text-[var(--text-secondary)] transition-[background-color,transform] duration-200 hover:scale-[1.03] hover:bg-[rgba(0,0,0,0.05)] active:scale-95" data-tauri-drag-region="false" @click="openGithub">
        <Github :size="20" />
        <span class="absolute inset-x-0 bottom-0 rounded-b-[var(--radius-md)] bg-[rgba(17,24,39,0.08)] px-2 py-[2px] text-center text-[8px] font-bold uppercase leading-[1.1] tracking-[0.2px] text-[rgba(17,24,39,0.68)] opacity-95">{{ appVersion || '-' }}</span>
      </button>
      <button
        type="button"
        class="flex items-center justify-center rounded-[var(--radius-md)] bg-[var(--hover-bg)] p-2.5 text-[var(--text-secondary)] transition-[background-color,transform] duration-200 hover:scale-[1.03] hover:bg-[rgba(0,0,0,0.05)] active:scale-95"
        data-tauri-drag-region="false"
        @click="toggleTheme"
      >
        <Sun v-if="effectiveTheme === 'dark'" :size="20" />
        <Moon v-else :size="20" />
      </button>

      <div v-if="shouldShowWindowsControls && !isMacPreview" class="absolute right-[-1px] top-[-1px] flex flex-col items-end gap-1">
        <WindowsWindowControls />
      </div>
  </div>
  </nav>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import type { ComponentPublicInstance } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { platform as detectPlatform } from '@tauri-apps/plugin-os';
import { openUrl } from '@tauri-apps/plugin-opener';
import { getVersion } from '@tauri-apps/api/app';
import { useRoute } from 'vue-router';
import { Github, Moon, Sun, X } from 'lucide-vue-next';
import WindowsWindowControls from '../components/window-controls/WindowsWindowControls.vue';
import { useThemeStore } from '../stores/theme';
import { Platform } from '../platforms/common/types';
import type { Platform as UiPlatform } from './types';

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

const props = defineProps<{
  theme: 'light' | 'dark';
  searchQuery?: string;
  activePlatform: UiPlatform | 'all';
}>();

const emit = defineEmits<{
  (event: 'theme-toggle'): void;
  (event: 'search-change', value: string): void;
  (event: 'platform-change', value: UiPlatform | 'all'): void;
  (event: 'select-anchor', payload: { id: string; platform: Platform; nickname: string; avatarUrl: string | null; currentRoomId?: string }): void;
}>();

const platforms: { id: UiPlatform | 'all'; name: string }[] = [
  { id: 'douyu', name: '斗鱼' },
  { id: 'huya', name: '虎牙' },
  { id: 'douyin', name: '抖音' },
  { id: 'bilibili', name: 'Bilibili' },
];

const activePlatform = computed(() => props.activePlatform);
const searchQuery = ref(props.searchQuery ?? '');
const trimmedQuery = computed(() => searchQuery.value.trim());
const searchResults = ref<SearchResultItem[]>([]);
const showResults = ref(false);
const searchError = ref<string | null>(null);
const isLoadingSearch = ref(false);
const isSearchFocused = ref(false);
const searchContainerRef = ref<HTMLElement | null>(null);
const platformTabsRef = ref<HTMLElement | null>(null);
const platformItemRefs = new Map<UiPlatform | 'all', HTMLElement>();
const highlight = ref({ left: 0, width: 0, opacity: 0 });
const highlightStyles = computed(() => ({
  transform: `translateX(${highlight.value.left}px)`,
  width: `${highlight.value.width}px`,
  opacity: highlight.value.opacity,
}));

const themeStore = useThemeStore();
const effectiveTheme = computed(() => themeStore.getEffectiveTheme());
const route = useRoute();

const detectedPlatform = ref<string | null>(null);
const isMacPreview = false;
const appVersion = ref('');
const isWindowsPlatform = computed(() => {
  const name = detectedPlatform.value?.toLowerCase() ?? '';
  return name.startsWith('win');
});
const shouldShowWindowsControls = computed(() => isWindowsPlatform.value);

const proxyBase = ref<string | null>(null);
const ensureProxyStarted = async () => {
  if (!proxyBase.value) {
    try {
      const base = await invoke<string>('start_static_proxy_server');
      proxyBase.value = base;
    } catch (e) {
      console.error('[Navbar] Failed to start static proxy server', e);
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

const currentPlatform = computed<Platform>(() => {
  const name = route.name as string | undefined;
  const path = route.path;

  if (name) {
    if (name === 'douyinPlayer' || name === 'DouyinHome') return Platform.DOUYIN;
    if (name === 'huyaPlayer' || name === 'HuyaHome') return Platform.HUYA;
    if (name === 'bilibiliPlayer' || name === 'BilibiliHome') return Platform.BILIBILI;
    if (name === 'douyuPlayer' || name === 'DouyuHome') return Platform.DOUYU;
  }

  if (path.startsWith('/player/douyin') || path.startsWith('/douyin')) return Platform.DOUYIN;
  if (path.startsWith('/player/huya') || path.startsWith('/huya')) return Platform.HUYA;
  if (path.startsWith('/player/bilibili') || path.startsWith('/bilibili')) return Platform.BILIBILI;
  if (path.startsWith('/player/douyu') || path.startsWith('/')) return Platform.DOUYU;

  return Platform.DOUYU;
});

const placeholderText = computed(() => {
  if (currentPlatform.value === Platform.DOUYU) return '搜索斗鱼主播名称/房间号';
  if (currentPlatform.value === Platform.HUYA) return '搜索虎牙主播名称/房间号';
  if (currentPlatform.value === Platform.DOUYIN) return '搜索抖音房间号';
  if (currentPlatform.value === Platform.BILIBILI) return '搜索B站主播名称/房间号';
  return '搜索主播/房间';
});

onMounted(async () => {
  try {
    detectedPlatform.value = await detectPlatform();
  } catch (error) {
    console.error('[Navbar] Failed to detect platform', error);
    if (typeof navigator !== 'undefined') {
      const ua = navigator.userAgent.toLowerCase();
      if (ua.includes('windows')) {
        detectedPlatform.value = 'windows';
      }
    }
  }
});

onMounted(() => {
  if (isMacPreview && typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-platform', 'darwin');
  } else if (typeof document !== 'undefined') {
    document.documentElement.removeAttribute('data-platform');
  }
});

onMounted(async () => {
  try {
    appVersion.value = await getVersion();
  } catch (error) {
    console.error('[Navbar] Failed to read app version', error);
  }
});

const setPlatformRef = (key: UiPlatform | 'all', el: Element | ComponentPublicInstance | null) => {
  if (!el) {
    platformItemRefs.delete(key);
    return;
  }
  const element = (el as ComponentPublicInstance).$el ?? el;
  if (element instanceof HTMLElement) {
    platformItemRefs.set(key, element);
  }
};

const updateHighlight = async () => {
  await nextTick();
  const container = platformTabsRef.value;
  const active = platformItemRefs.get(props.activePlatform);
  if (!container || !active) {
    highlight.value.opacity = 0;
    return;
  }
  const rect = active.getBoundingClientRect();
  const containerRect = container.getBoundingClientRect();
  highlight.value = {
    left: rect.left - containerRect.left,
    width: rect.width,
    opacity: 1,
  };
};

watch(() => props.activePlatform, () => {
  updateHighlight();
}, { immediate: true });

onMounted(() => {
  window.addEventListener('resize', updateHighlight);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', updateHighlight);
});

const handleDocumentMouseDown = (event: MouseEvent) => {
  const target = event.target;
  if (!(target instanceof Node)) return;
  if (searchContainerRef.value && !searchContainerRef.value.contains(target)) {
    showResults.value = false;
    isSearchFocused.value = false;
  }
};

onMounted(() => {
  document.addEventListener('mousedown', handleDocumentMouseDown);
});

onBeforeUnmount(() => {
  document.removeEventListener('mousedown', handleDocumentMouseDown);
});

const toggleTheme = () => {
  emit('theme-toggle');
};

const openGithub = async () => {
  try {
    await openUrl('https://github.com/chen-zeong/DTV/releases');
  } catch (error) {
    if (typeof window !== 'undefined') {
      window.open('https://github.com/chen-zeong/DTV/releases', '_blank', 'noopener,noreferrer');
      return;
    }
    console.error('[Navbar] Failed to open GitHub', error);
  }
};

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
  emit('search-change', searchQuery.value);

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
    if (douyinInfo?.anchor_name) {
      const isLive = douyinInfo.status === 2;
      const webId = (douyinInfo as any).web_rid ?? userInputRoomId;
      searchResults.value = [{
        platform: Platform.DOUYIN,
        roomId: webId,
        webId,
        userName: douyinInfo.anchor_name || '抖音主播',
        roomTitle: douyinInfo.title || null,
        avatar: douyinInfo.avatar || null,
        liveStatus: isLive,
        rawStatus: douyinInfo.status,
      }];
    } else {
      searchError.value = '搜索失败，请重试。';
    }
  } catch (e) {
    isLoadingSearch.value = false;
    searchError.value = '搜索失败，请重试。';
  }
  showResults.value = true;
};

const performHuyaSearch = async (keyword: string) => {
  searchResults.value = [];
  searchError.value = null;
  isLoadingSearch.value = true;
  try {
    const items = await invoke<HuyaAnchorItem[]>('search_huya_anchors', { keyword, page: 1 });
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
    searchError.value = '搜索失败，请重试。';
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
      searchError.value = '搜索失败，请重试。';
    }
  } catch (e) {
    isLoadingSearch.value = false;
    searchError.value = '搜索失败，请重试。';
  }
  showResults.value = true;
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
        userName: item.anchor || 'B站主播',
        roomTitle: item.title || null,
        avatar: proxify(item.avatar),
        liveStatus: item.is_live,
        fansCount: item.watching,
        category: item.area,
      }));
    }
  } catch (e) {
    searchError.value = '搜索失败，请重试。';
  } finally {
    isLoadingSearch.value = false;
    showResults.value = true;
  }
};

const handleFocus = () => {
  isSearchFocused.value = true;
  showResults.value = true;
};

const handleBlur = () => {
  isSearchFocused.value = false;
  setTimeout(() => {
    if (!isLoadingSearch.value && !searchError.value) {
      showResults.value = false;
    }
  }, 300);
};

const selectAnchor = (anchor: SearchResultItem) => {
  emit('select-anchor', {
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
  emit('select-anchor', {
    id: roomId,
    platform: currentPlatform.value,
    nickname: roomId,
    avatarUrl: null,
    currentRoomId: undefined,
  });
  resetSearchState();
};
</script>
