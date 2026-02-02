<template>
  <div>
    <!-- Trigger Zone (Only in Mini State) -->
    <div
      v-if="isMiniState && !isExpanded"
      class="fixed top-0 left-0 right-0 h-8 z-40 flex justify-center pointer-events-auto group/trigger"
      @mouseenter="showMini"
      @mousemove="resetHideTimer"
    >
       <!-- Home Indicator Bar -->
       <div 
         class="w-24 h-1 bg-brand/20 hover:bg-brand/80 rounded-b-full shadow-[0_2px_8px_rgba(0,0,0,0.2)] transition-all duration-300 ease-out group-hover/trigger:w-28 group-hover/trigger:bg-brand"
         :class="isMiniVisible ? 'opacity-0 -translate-y-2' : 'opacity-100 translate-y-0'"
       ></div>
    </div>

    <!-- Main Island Container -->
    <div
      ref="islandRef"
      class="fixed left-1/2 z-50 -translate-x-1/2 will-change-transform flex items-center gap-2 pointer-events-auto transition-all duration-500 ease-[cubic-bezier(0.16,1,0.3,1)]"
      :class="[
        isMiniState && !isExpanded 
          ? (isMiniVisible ? 'top-4 opacity-100 scale-100' : '-top-12 opacity-0 scale-95 pointer-events-none') 
          : 'top-6 opacity-100 scale-100'
      ]"
      @mouseenter="handleIslandEnter"
      @mouseleave="handleIslandLeave"
    >
      <div
        class="pointer-events-auto relative flex items-center justify-center"
      >
        <div
          class="flex items-center rounded-full border border-border-strong bg-surface-low/80 shadow-2xl backdrop-blur-2xl transition-all duration-500 ease-[cubic-bezier(0.16,1,0.3,1)] will-change-[width,height,padding]"
          :class="[
            isMiniState && !isExpanded
              ? 'h-9 w-24 px-0 cursor-pointer border-transparent bg-black/40 text-white'
              : 'p-1.5 hover:scale-[1.02] hover:bg-surface-low'
          ]"
          @click="handleIslandClick"
        >
          <!-- Mini State Content -->
          <div v-if="isMiniState && !isExpanded" class="flex w-full items-center justify-center gap-1.5 px-3">
             <div 
               class="h-1.5 w-1.5 rounded-full transition-colors duration-300"
               :class="[miniStatus.color, { 'animate-pulse': miniStatus.animate }]"
             ></div>
             <span class="text-[10px] font-black tracking-widest opacity-80 whitespace-nowrap">{{ miniStatus.text }}</span>
          </div>
  
          <!-- Full State Content -->
          <div v-else class="flex items-center gap-2 overflow-hidden whitespace-nowrap animate-in fade-in slide-in-from-top-1 duration-500 fill-mode-both">
            
            <!-- Back Button (Only in Player Route) -->
            <button
              v-if="isPlayerRoute"
              @click.stop="router.back()"
              class="flex h-10 w-10 items-center justify-center rounded-full hover:bg-surface-high transition-colors"
              title="返回列表"
            >
              <ArrowLeft class="size-4" />
            </button>
  
            <div v-if="isPlayerRoute" class="h-4 w-px bg-border-main mx-1"></div>
  
            <!-- Platform Switcher -->
            <div class="relative flex items-center">
              <button
                @click.stop="showPlatformMenu = !showPlatformMenu"
                class="group flex h-10 items-center gap-2.5 rounded-full px-4 py-2 transition-all hover:bg-surface-high"
              >
                <img
                  :src="activePlatformIcon"
                  class="size-5 rounded-md object-contain transition-transform group-hover:rotate-12"
                />
                <span class="text-sm font-black tracking-tight">{{ activePlatformName }}</span>
                <ChevronDown class="size-3.5 text-text-muted transition-transform" :class="{ 'rotate-180': showPlatformMenu }" />
              </button>
  
              <!-- Platform Menu Popover -->
              <div v-if="showPlatformMenu" class="absolute top-full left-0 mt-3 w-40 overflow-hidden rounded-2xl border border-border-strong bg-surface-low p-1.5 shadow-2xl backdrop-blur-xl">
                <button
                  v-for="plt in PLATFORMS"
                  :key="plt.id"
                  @click.stop="handlePlatformChange(plt.id); showPlatformMenu = false"
                  class="flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-bold transition-all hover:bg-surface-high"
                  :class="{ 'bg-brand/10 text-brand': plt.id === activePlatform }"
                >
                  <img :src="plt.icon" class="size-5 rounded-md object-contain" />
                  {{ plt.name }}
                </button>
              </div>
            </div>
  
            <div class="h-6 w-px bg-border-main mx-1"></div>
  
            <!-- Category Selector Trigger -->
            <button
              @click.stop="showCategoryPopover = !showCategoryPopover"
              class="flex h-10 items-center gap-3 rounded-full px-5 transition-all hover:bg-surface-high"
            >
              <LayoutGrid class="size-4 text-brand" />
              <span class="max-w-[120px] truncate text-sm font-bold">{{ currentCategoryName || "全部分类" }}</span>
              <ChevronDown class="size-3.5 text-text-muted" :class="{ 'rotate-180': showCategoryPopover }" />
            </button>
  
            <!-- Search Button -->
            <button
              @click.stop="emit('openSearch')"
              class="flex h-10 w-10 items-center justify-center rounded-full text-text-muted transition-all hover:bg-surface-high hover:text-text-main"
            >
              <Search class="size-4" />
            </button>
  
            <!-- Active Streamer Capsules Chain (Merged) -->
            <template v-if="activeStreamers.length > 0 && (!isPlayerRoute || isExpanded)">
              <div class="h-6 w-px bg-border-main mx-1"></div>
              
              <transition-group
                enter-active-class="transition duration-500 ease-[cubic-bezier(0.23,1,0.32,1)]"
                enter-from-class="opacity-0 translate-x-[-10px] scale-90"
                enter-to-class="opacity-100 translate-x-0 scale-100"
                leave-active-class="transition duration-300 ease-in absolute"
                leave-from-class="opacity-100 scale-100"
                leave-to-class="opacity-0 scale-50"
                move-class="transition duration-500 ease-[cubic-bezier(0.23,1,0.32,1)]"
              >
                <IslandCapsule
                  v-for="streamer in activeStreamers"
                  :key="`${streamer.platform}:${streamer.roomId}`"
                  :streamer="streamer"
                  :is-active="isStreamerActive(streamer)"
                  :is-followed="followStore.isFollowed(streamer.platform, streamer.roomId)"
                  @click="activateStream(streamer)"
                  @close="closeStream(streamer)"
                  @mute="toggleMute(streamer)"
                  @fav="toggleFav(streamer)"
                />
              </transition-group>
  
              <!-- Clear All Button -->
              <button
                v-if="activeStreamers.length > 1"
                @click.stop="clearAllStreams"
                class="flex h-8 w-8 items-center justify-center rounded-full text-text-muted hover:bg-red-500/20 hover:text-red-500 transition-all ml-1"
                title="关闭全部"
              >
                <Trash2 class="size-3.5" />
              </button>
            </template>
          </div>
        </div>
      </div>
  
      <!-- Category Mega Popover (Teleported) -->
      <Teleport to="body">
        <!-- ... (Popover content remains unchanged) ... -->
        <div v-if="showCategoryPopover" class="pointer-events-auto fixed inset-0 z-[60] flex items-start justify-center pt-24 pb-12 px-12" @click.self="showCategoryPopover = false">
           <div class="absolute inset-0 bg-black/20 backdrop-blur-[2px]" @click="showCategoryPopover = false"></div>
           
           <div class="relative flex h-full max-h-[80vh] w-full max-w-5xl flex-col overflow-hidden rounded-3xl border border-border-strong bg-surface-low shadow-2xl animate-in zoom-in-95 duration-200">
            <div class="flex items-center justify-between border-b border-border-main px-8 py-6">
              <h3 class="text-2xl font-black tracking-tight">浏览分类</h3>
              <button @click="showCategoryPopover = false" class="rounded-full bg-surface-high p-2 hover:bg-surface-mid">
                <X class="size-5" />
              </button>
            </div>
  
            <div class="flex flex-1 min-h-0">
              <!-- Left: Cate1 -->
              <div class="w-64 overflow-y-auto border-r border-border-main bg-surface-mid/30 p-4">
                 <button
                    v-for="group in categoryGroups"
                    :key="group.id"
                    @mouseenter="activeCate1Id = group.id"
                    class="mb-1 flex w-full items-center justify-between rounded-xl px-4 py-3 text-left text-sm font-bold transition-all"
                    :class="activeCate1Id === group.id ? 'bg-brand text-white' : 'hover:bg-surface-high text-text-dim'"
                 >
                   {{ group.title }}
                   <ChevronRight v-if="activeCate1Id === group.id" class="size-4" />
                 </button>
              </div>
  
              <!-- Right: Cate2 -->
              <div class="flex-1 overflow-y-auto p-8 bg-surface-low">
                 <div v-if="favoriteCategoriesForActivePlatform.length > 0" class="mb-8">
                    <div class="flex items-center gap-2 mb-4 text-brand">
                       <Star class="size-4 fill-current" />
                       <span class="text-xs font-black tracking-widest uppercase">已收藏</span>
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-3">
                       <div
                         v-for="fav in favoriteCategoriesForActivePlatform"
                         :key="fav.id"
                         class="group relative flex cursor-pointer flex-col items-center justify-center rounded-2xl border border-border-main bg-surface-mid p-4 transition-all hover:border-brand hover:bg-surface-high"
                         @click="handleCategorySelect(fav); showCategoryPopover = false"
                       >
                         <span class="text-center text-xs font-bold">{{ fav.title }}</span>
                         <button
                           @click.stop="toggleFavoriteCategory(fav)"
                           class="absolute top-2 right-2 text-brand"
                         >
                           <Star fill="currentColor" class="size-3.5" />
                         </button>
                       </div>
                    </div>
                    <div class="my-6 h-px w-full bg-border-main/50"></div>
                 </div>
  
                 <p class="mb-4 text-xs font-black tracking-widest text-text-muted uppercase">{{ currentCate1Title }}</p>
                 <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-3">
                    <div
                      v-for="cate2 in currentCate2List"
                      :key="cate2.id"
                      class="group relative flex cursor-pointer flex-col items-center justify-center rounded-2xl border border-border-main bg-surface-mid p-4 transition-all hover:border-brand hover:bg-surface-high"
                      :class="{ 'border-brand ring-1 ring-brand/30': currentSelectedId === cate2.id }"
                      @click="handleCategorySelect(cate2); showCategoryPopover = false"
                    >
                      <span class="text-center text-xs font-bold">{{ cate2.title }}</span>
                      <button
                        @click.stop="toggleFavoriteCategory({ ...cate2, type: 'cate2' })"
                        class="absolute top-2 right-2 opacity-0 transition-opacity group-hover:opacity-100"
                        :class="isFavorite(cate2.id) ? 'text-brand opacity-100' : 'text-text-muted hover:text-brand'"
                      >
                        <Star :fill="isFavorite(cate2.id) ? 'currentColor' : 'none'" class="size-3.5" />
                      </button>
                    </div>
                 </div>
              </div>
            </div>
          </div>
        </div>
      </Teleport>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onBeforeUnmount, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { storeToRefs } from 'pinia';
import {
  ChevronDown,
  LayoutGrid,
  Search,
  ArrowLeft,
  X,
  ChevronRight,
  Star,
  Trash2
} from 'lucide-vue-next';
import { useCategoryStore } from '../../store/categoryStore';
import { usePlayerStore } from '../../store/playerStore';
import { useFollowStore } from '../../store/followStore';
import { PLATFORMS } from '../../config/platforms';
import type { UiPlatform } from '../../types/app/platform';
import IslandCapsule from './IslandCapsule.vue';

const emit = defineEmits<{ (e: 'openSearch'): void }>();

const router = useRouter();
const route = useRoute();
const categoryStore = useCategoryStore();
const playerStore = usePlayerStore();
const followStore = useFollowStore();

const {
  activePlatform,
  selectedCategoryInfo,
  selectedCategory,
  categoryGroups,
  favoriteCategories
} = storeToRefs(categoryStore);

// State
const showPlatformMenu = ref(false);
const showCategoryPopover = ref(false);
const activeCate1Id = ref<string | null>(null);

const isExpanded = ref(false); 
const isMiniVisible = ref(false);
const hideTimer = ref<number | null>(null);

const islandRef = ref<HTMLElement | null>(null);

const isPlayerRoute = computed(() => route.name === 'StreamRoom');
const isMiniState = computed(() => isPlayerRoute.value);

const miniStatus = computed(() => {
  const s = playerStore.currentStreamer;
  if (!s) return { text: "OFFLINE", color: "bg-gray-400", animate: false };
  
  if (s.liveStatus === "LIVE") return { text: "LIVE", color: "bg-red-500", animate: true };
  if (s.liveStatus === "REPLAY") return { text: "REPLAY", color: "bg-amber-500", animate: false };
  
  // Fallback
  if (s.isLive) return { text: "LIVE", color: "bg-red-500", animate: true };
  return { text: "OFFLINE", color: "bg-gray-400", animate: false };
});

const activePlatformIcon = computed(() => PLATFORMS.find(p => p.id === activePlatform.value)?.icon);
const activePlatformName = computed(() => PLATFORMS.find(p => p.id === activePlatform.value)?.name);

const activeStreamers = computed(() => playerStore.activeStreamers);

const isStreamerActive = (s: any) => {
  return s.roomId === playerStore.currentStreamer?.roomId && s.platform === playerStore.currentStreamer?.platform;
};

const currentCategoryName = computed(() => {
  if (categoryStore.isDouyu) return selectedCategoryInfo.value?.name;
  return selectedCategory.value?.cate2Name;
});

const favoriteCategoriesForActivePlatform = computed(() =>
  favoriteCategories.value.filter(f => f.platform === activePlatform.value)
);

const currentCate1Title = computed(() =>
  categoryGroups.value.find(g => g.id === activeCate1Id.value)?.title || "选择大类"
);

const currentCate2List = computed(() =>
  categoryGroups.value.find(g => g.id === activeCate1Id.value)?.items || []
);

const currentSelectedId = computed(() => {
  if (categoryStore.isDouyu) return selectedCategoryInfo.value?.id ?? null;
  return selectedCategory.value?.cate2Href ?? null;
});

// Logic
const handlePlatformChange = (platform: UiPlatform) => {
  if (platform === activePlatform.value) return;
  router.push({ name: 'ChannelList', params: { platform } });
};

const handleCategorySelect = (item: { id: string; title: string }) => {
  categoryStore.handleCategorySelect(item);
};

const toggleFavoriteCategory = (item: any) => {
  categoryStore.toggleFavoriteCategory(item);
};

const isFavorite = (id: string) => categoryStore.isFavorite(id);

const activateStream = (s: any) => {
  const p = String(s.platform || "").toLowerCase();
  const r = s.roomId;
  if (!p || !r) return;
  
  // Set as current in store (important if clicking inactive shortcut)
  playerStore.setStreamerInfo(s); 

  router.push({
    name: 'StreamRoom',
    params: { platform: p, roomId: r }
  });
};

const closeStream = (s: any) => {
  playerStore.removeStreamer(s.platform, s.roomId);
};

const toggleMute = (s: any) => {
  playerStore.toggleMute(s.platform, s.roomId);
};

const toggleFav = (s: any) => {
  if (followStore.isFollowed(s.platform, s.roomId)) {
    followStore.unfollowStreamer(s.platform, s.roomId);
  } else {
    followStore.followStreamer({
      id: s.roomId,
      platform: s.platform,
      nickname: s.anchorName,
      avatarUrl: s.avatar,
      roomTitle: s.title,
      liveStatus: "LIVE"
    });
  }
};

const clearAllStreams = () => {
  playerStore.clearAllStreamers();
};

watch(categoryGroups, (newGroups) => {
  if (newGroups.length > 0 && !activeCate1Id.value) {
    activeCate1Id.value = newGroups[0].id;
  }
}, { immediate: true });


// --- Interactions ---

const showMini = () => {
  if (isMiniState.value) {
    isMiniVisible.value = true;
    resetHideTimer();
  }
};

const resetHideTimer = () => {
  if (hideTimer.value) clearTimeout(hideTimer.value);
  hideTimer.value = window.setTimeout(() => {
    // Only hide if not expanded and not hovering island
    if (!isExpanded.value) {
      isMiniVisible.value = false;
    }
  }, 2000);
};

const handleIslandEnter = () => {
  if (isMiniState.value) {
    isMiniVisible.value = true; // Keep visible
    if (hideTimer.value) clearTimeout(hideTimer.value);
  }
};

const handleIslandLeave = () => {
  if (isMiniState.value) {
    resetHideTimer();
  }
};

const handleIslandClick = () => {
  if (isMiniState.value && !isExpanded.value) {
    isExpanded.value = true;
    isMiniVisible.value = true;
  }
};

// Global click to collapse
const handleGlobalClick = (event: MouseEvent) => {
  if (!islandRef.value) return;
  if (!isExpanded.value) return;
  if (showPlatformMenu.value || showCategoryPopover.value) return;

  const rect = islandRef.value.getBoundingClientRect();
  const inside =
    event.clientX >= rect.left &&
    event.clientX <= rect.right &&
    event.clientY >= rect.top &&
    event.clientY <= rect.bottom;

  if (!inside) {
    isExpanded.value = false;
    // Also hide mini after a delay if not hovering
    resetHideTimer();
  }
};

watch(isPlayerRoute, (val) => {
  if (val) {
    isExpanded.value = false;
    isMiniVisible.value = false;
  } else {
    isExpanded.value = true; // Reset to expanded for non-mini views? Actually just layout change.
  }
});

onMounted(() => {
  window.addEventListener("click", handleGlobalClick, { passive: true });
});

onBeforeUnmount(() => {
  if (hideTimer.value) clearTimeout(hideTimer.value);
  window.removeEventListener("click", handleGlobalClick);
});

</script>
