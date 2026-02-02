<template>
  <div class="relative h-full w-full overflow-hidden bg-app-bg text-text-main">
    <!-- Top Right: Follow List Trigger & Popover -->
    <div class="fixed top-6 right-6 z-50">
      <button
        @click="showFollowPopover = !showFollowPopover"
        ref="followTriggerRef"
        class="group relative flex h-12 w-12 items-center justify-center rounded-2xl border border-border-strong bg-surface-low/80 shadow-xl backdrop-blur-xl transition-all hover:scale-105 hover:bg-surface-low active:scale-95"
        :class="{ 'bg-surface-high border-brand/50 ring-2 ring-brand/20': showFollowPopover }"
      >
        <UsersRound class="size-5 text-text-main" />
        <div v-if="liveFollowersCount > 0" class="absolute -top-1.5 -right-1.5 flex h-5 min-w-[20px] items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-black text-white ring-4 ring-app-bg">
          {{ liveFollowersCount }}
        </div>
      </button>

      <!-- Follow Popover -->
      <transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="transform scale-95 opacity-0 -translate-y-2"
        enter-to-class="transform scale-100 opacity-100 translate-y-0"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="transform scale-100 opacity-100 translate-y-0"
        leave-to-class="transform scale-95 opacity-0 -translate-y-2"
      >
        <div
          v-if="showFollowPopover"
          ref="followPopoverRef"
          class="absolute top-full right-0 mt-3 w-80 overflow-hidden rounded-3xl border border-border-strong bg-surface-low/95 shadow-2xl backdrop-blur-xl ring-1 ring-white/10"
        >
          <div class="flex max-h-[60vh] flex-col">
            <div class="flex-1 overflow-hidden">
               <FollowingSidebar
                 :followed-anchors="followedStreamers"
                 @select-anchor="handleSelectHistoryFromDrawer"
                 @unfollow="handleUnfollow"
               />
            </div>
            
            <!-- Footer: Settings & Info -->
            <div class="border-t border-border-main p-3 bg-surface-mid/50 backdrop-blur-md">
               <div class="flex items-center justify-between">
                  <span class="text-[10px] font-bold text-text-muted px-2">v2.4.4</span>
                  <div class="flex gap-1">
                     <button
                       class="rounded-lg p-2 hover:bg-surface-high text-text-muted hover:text-text-main transition-colors"
                       :title="isDark ? '切换到日间模式' : '切换到夜间模式'"
                       @click="toggleTheme"
                     >
                        <Sun v-if="isDark" class="size-4" />
                        <Moon v-else class="size-4" />
                     </button>
                  </div>
               </div>
            </div>
          </div>
        </div>
      </transition>
    </div>

    <!-- Main Content -->
    <div
      ref="centerColumnRef"
      class="scrollbar-none h-full w-full overflow-y-auto"
      @scroll="handleScroll"
    >
      <!-- Large Hero Header (Spaced for Island) -->
      <div class="px-10 pt-32 pb-8">
          <h1 class="text-6xl font-black  text-brand pb-1 tracking">
            {{ currentCategoryName }}
          </h1>

        <!-- Sub Cate Navigation (Cate3) -->
        <div v-if="dynamicCate3List.length > 0" class="mt-8 flex flex-wrap gap-2">
           <button
            v-for="cate in dynamicCate3List"
            :key="cate.id"
            class="rounded-full border border-border-main px-5 py-2 text-sm font-bold whitespace-nowrap transition-all"
            :class="
              currentSelectedCate3Id === cate.id
                ? 'border-text-main bg-text-main text-app-bg'
                : 'bg-surface-high/50 text-text-dim hover:bg-surface-high hover:text-text-main'
            "
            @click="handleCate3Select(cate)"
          >
            {{ cate.name }}
          </button>
        </div>
      </div>

      <!-- Room Grid -->
      <div class="px-10 pb-20">
        <div v-if="isDouyu" class="min-h-0">
          <div v-if="selectedCategoryInfo" class="min-h-0">
            <RoomList
              :douyu-category="selectedCategoryInfo"
              platformName="douyu"
              playerRouteName="StreamRoom"
              :key="activePlatform + selectedCategoryInfo.id"
              :is-scrolling="isScrolling"
            />
          </div>
          <div v-else-if="isDouyuLoading" class="flex h-96 items-center justify-center">
            <LoadingDots />
          </div>
        </div>
        <div v-else class="min-h-0">
          <RoomList
            :selectedCategory="selectedCategory"
            :categoriesData="categoriesData"
            :default-page-size="PLATFORM_MAP[activePlatform].defaultPageSize"
            :platformName="activePlatform"
            :playerRouteName="PLATFORM_MAP[activePlatform].playerRouteName"
            :is-scrolling="isScrolling"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import { onClickOutside } from "@vueuse/core";
import { UsersRound, X, Moon, Sun } from "lucide-vue-next";
import { useFollowStore } from "../store/followStore";
import { useCategoryStore } from "../store/categoryStore";
import { useHeaderScroll } from "../hooks/useHeaderScroll";
import RoomList from "../features/rooms/components/RoomList.vue";
import LoadingDots from "../components/ui/LoadingDots.vue";
import FollowingSidebar from "../features/following/components/FollowingSidebar.vue";
import { useThemeStore } from "../store/theme";
import type { UiPlatform, Platform } from "../types/app/platform";
import type { FollowedStreamer } from "../types/models/streamer";
import { PLATFORMS, PLATFORM_MAP } from "../config/platforms";

const router = useRouter();
const route = useRoute();
const followStore = useFollowStore();
const categoryStore = useCategoryStore();
const themeStore = useThemeStore();
const { followedStreamers } = storeToRefs(followStore);
const {
  activePlatform,
  selectedCategory,
  selectedCategoryInfo,
  douyuSelectedC2,
  currentCate3List,
  isDouyuLoading,
  categoriesData,
  isDouyu,
  categoryGroups,
} = storeToRefs(categoryStore);

const { isScrolling, handleScroll } = useHeaderScroll();

// UI States
const showFollowPopover = ref(false);
const followPopoverRef = ref<HTMLElement | null>(null);
const followTriggerRef = ref<HTMLElement | null>(null);

const stopFollowPopoverOutside = ref<(() => void) | null>(null);

watch(showFollowPopover, (open) => {
  if (open) {
    stopFollowPopoverOutside.value = onClickOutside(
      followPopoverRef,
      () => (showFollowPopover.value = false),
      { ignore: [followTriggerRef] },
    );
  } else {
    stopFollowPopoverOutside.value?.();
    stopFollowPopoverOutside.value = null;
  }
});

const activePlatformName = computed(() => PLATFORMS.find(p => p.id === activePlatform.value)?.name);

const liveFollowersCount = computed(() =>
  followedStreamers.value.filter(s => s.liveStatus === 'LIVE' || s.isLive).length
);

const currentCategoryName = computed(() => {
  if (isDouyu.value) return selectedCategoryInfo.value?.name;
  return selectedCategory.value?.cate2Name;
});

const currentSelectedCate3Id = computed(() => {
  if (isDouyu.value && selectedCategoryInfo.value?.type === "cate3")
    return selectedCategoryInfo.value.id;
  if (isDouyu.value && selectedCategoryInfo.value?.type === "cate2" && currentCate3List.value.length > 0)
    return "all";
  return null;
});

const dynamicCate3List = computed(() => {
  if (isDouyu.value && currentCate3List.value.length > 0)
    return [{ id: "all", name: "全部" }, ...currentCate3List.value];
  return [];
});

const isDark = computed(() => themeStore.getEffectiveTheme() === "dark");
const toggleTheme = () => themeStore.toggleTheme();

const handleCate3Select = (cate: { id: string; name: string }) => {
  if (isDouyu.value) {
    if (cate.id === "all") {
       // Logic to reset to cate2
       const group = categoryGroups.value.find(g => g.items.some(i => i.id === selectedCategoryInfo.value?.id));
       if (group) {
          const item = group.items.find(i => i.id === selectedCategoryInfo.value?.id);
          if (item) categoryStore.handleCategorySelect(item);
       }
       return;
    }
    selectedCategoryInfo.value = { type: "cate3", id: cate.id, name: cate.name };
  }
};

const handleSelectHistoryFromDrawer = (streamer: FollowedStreamer) => {
  followStore.updateLastViewed(streamer.platform, streamer.id);
  router.push({
    name: "StreamRoom",
    params: { platform: streamer.platform.toLowerCase(), roomId: streamer.id },
  });
  showFollowPopover.value = false;
};

const handleUnfollow = (payload: { platform: Platform; id: string }) => {
  followStore.unfollowStreamer(payload.platform, payload.id);
};

watch(() => route.params.platform, (newPlatform) => {
  categoryStore.setPlatform((newPlatform as UiPlatform) || "douyu");
}, { immediate: true });

onMounted(() => {
  if (isDouyu.value) categoryStore.initDouyuData();
  else categoryStore.initCommonData();
});
</script>

<style scoped>
.bg-panel-gradient {
  background: radial-gradient(circle at 50% -20%, rgba(var(--brand-rgb), 0.1), transparent 80%);
}

.scrollbar-none::-webkit-scrollbar {
  display: none;
}
.scrollbar-none {
  scrollbar-width: none;
}
</style>
