<template>
  <div v-if="isOpen" class="fixed inset-0 z-[100] flex items-start justify-center pt-[16vh] px-4">
    <div class="absolute inset-0 bg-black/60 backdrop-blur-sm transition-opacity" @click="close"></div>

    <div class="relative w-full max-w-3xl overflow-hidden rounded-2xl border border-border-strong bg-surface-low shadow-2xl ring-1 ring-white/10 transition-all">
      <div class="relative flex items-center border-b border-border-main px-4 py-4">
        <Search class="mr-3 size-6 text-text-muted" />
        <input
          ref="inputRef"
          v-model="searchQuery"
          type="text"
          class="h-full w-full bg-transparent text-xl font-medium text-text-main placeholder-text-muted outline-none"
          placeholder="搜索主播、房间号或分类..."
          @keydown.enter.prevent="handleEnter"
          @keydown.esc="close"
          @keydown.down.prevent="moveActive(1)"
          @keydown.up.prevent="moveActive(-1)"
        />
        <div class="ml-auto flex items-center gap-2 text-xs text-text-muted">
          <span class="rounded bg-surface-high px-1.5 py-0.5">ESC</span> 关闭
          <span class="rounded bg-surface-high px-1.5 py-0.5">↵</span> 打开
        </div>
      </div>

      <div class="max-h-[60vh] overflow-y-auto p-3">
        <div v-if="isLoading" class="flex items-center gap-2 px-3 py-2 text-xs text-text-muted">
          <span class="h-3 w-3 animate-spin rounded-full border-2 border-brand border-t-transparent"></span>
          正在搜索全平台...
        </div>

        <div v-if="searchQuery && groups.length === 0" class="p-6 text-center text-text-muted">
          没有匹配结果
        </div>

        <template v-for="group in groups" :key="group.id">
          <div class="px-2 pt-2 pb-1 text-[11px] font-black tracking-widest text-text-muted">
            {{ group.title }}
          </div>
          <ul class="flex flex-col gap-1">
            <li
              v-for="item in group.items"
              :key="item.key"
              class="group flex items-center gap-3 rounded-[12px] border border-border-main/60 bg-surface-low/50 px-3 py-2 transition-all hover:border-border-strong hover:bg-surface-high/70"
              :class="{
                'ring-1 ring-brand/40 bg-surface-high/80': item.key === activeKey,
                'border-brand/40 bg-brand/5': isFavorite(item) || isFollowed(item),
              }"
              @mouseenter="activeKey = item.key"
              @click="selectItem(item)"
            >
              <div class="relative h-9 w-9 shrink-0 overflow-hidden rounded-xl border border-white/10 bg-surface-high/60">
                <img
                  v-if="item.avatar"
                  :src="item.avatar"
                  class="h-full w-full object-cover"
                  :alt="item.title"
                />
                <div v-else class="flex h-full w-full items-center justify-center text-xs font-black">
                  {{ item.title[0] || "?" }}
                </div>
              </div>
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2">
                  <span
                    v-if="item.type !== 'category'"
                    class="h-1.5 w-1.5 rounded-full"
                    :class="item.isLive ? 'bg-emerald-400' : 'bg-amber-400'"
                  ></span>
                  <span class="truncate text-sm font-black text-text-main">{{ item.title }}</span>
                  <span
                    v-if="isFollowed(item)"
                    class="rounded-full border border-brand/30 bg-brand/10 px-2 py-0.5 text-[10px] font-black text-brand"
                    >已关注</span
                  >
                  <span
                    v-if="isFavorite(item)"
                    class="rounded-full border border-amber-500/40 bg-amber-500/10 px-2 py-0.5 text-[10px] font-black text-amber-400"
                    >已收藏</span
                  >
                </div>
                <div class="truncate text-[11px] font-bold text-text-muted">
                  {{ item.subtitle || platformLabel(item.platform) }}
                </div>
              </div>
              <div class="ml-auto flex items-center gap-2">
                <button
                  class="flex h-8 w-8 items-center justify-center rounded-full border border-border-main/70 bg-surface-high/50 text-text-muted transition-all hover:border-brand/40 hover:text-brand"
                  @click.stop="toggleFavorite(item)"
                  :title="favoriteLabel(item)"
                >
                  <template v-if="item.type === 'category'">
                    <Star :size="14" :fill="isFavorite(item) ? 'currentColor' : 'none'" />
                  </template>
                  <template v-else>
                    <UserMinus v-if="isFollowed(item)" :size="14" />
                    <UserPlus v-else :size="14" />
                  </template>
                </button>
                <span
                  v-if="item.isLive"
                  class="rounded-full border border-emerald-500/40 bg-emerald-500/10 px-2 py-0.5 text-[10px] font-black text-emerald-400"
                  >LIVE</span
                >
                <span
                  v-else-if="item.type === 'category'"
                  class="rounded-full border border-border-main/70 bg-surface-high/50 px-2 py-0.5 text-[10px] font-black text-text-muted"
                  >分类</span
                >
                <span
                  v-else
                  class="rounded-full border border-border-main/70 bg-surface-high/50 px-2 py-0.5 text-[10px] font-black text-text-muted"
                  >房间</span
                >
              </div>
            </li>
          </ul>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { Search, Star, UserMinus, UserPlus } from "lucide-vue-next";
import { useMagicKeys, whenever } from "@vueuse/core";
import { useGlobalSearch } from "../../features/search/useGlobalSearch";
import type { UiPlatform } from "../../types/app/platform";
import { Platform } from "../../types/app/platform";
import { useFollowStore } from "../../store/followStore";
import { useCategoryStore } from "../../store/categoryStore";

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "search", query: string): void;
}>();

const inputRef = ref<HTMLInputElement | null>(null);
const isOpen = ref(props.modelValue);
const activeKey = ref<string | null>(null);
const followStore = useFollowStore();
const categoryStore = useCategoryStore();

const {
  query,
  groupedResults,
  combinedResults,
  isLoadingRemote,
  selectResult,
  ensureDouyuCategories,
} = useGlobalSearch();

const searchQuery = computed({
  get: () => query.value,
  set: (val) => {
    query.value = val;
  },
});

const isLoading = computed(() => isLoadingRemote.value);
const groups = computed(() => groupedResults.value);

watch(
  () => props.modelValue,
  (val) => {
    isOpen.value = val;
    if (val) {
      ensureDouyuCategories();
      nextTick(() => inputRef.value?.focus());
    }
  },
);

watch(
  combinedResults,
  (items) => {
    if (!items.length) {
      activeKey.value = null;
      return;
    }
    if (!activeKey.value || !items.find((i) => i.key === activeKey.value)) {
      activeKey.value = items[0].key;
    }
  },
  { immediate: true },
);

const close = () => {
  emit("update:modelValue", false);
};

const handleEnter = () => {
  const current = combinedResults.value.find((i) => i.key === activeKey.value);
  if (current) {
    selectItem(current);
  }
};

const selectItem = (item: any) => {
  selectResult(item);
  emit("search", query.value);
  close();
};

const moveActive = (delta: number) => {
  const items = combinedResults.value;
  if (!items.length) return;
  const index = items.findIndex((i) => i.key === activeKey.value);
  const nextIndex = index === -1 ? 0 : (index + delta + items.length) % items.length;
  activeKey.value = items[nextIndex].key;
};

const platformLabel = (p?: UiPlatform) => {
  switch ((p || "").toLowerCase()) {
    case "douyu":
      return "斗鱼";
    case "douyin":
      return "抖音";
    case "huya":
      return "虎牙";
    case "bilibili":
      return "B站";
    default:
      return "";
  }
};

const toPlatformEnum = (p?: UiPlatform): Platform => {
  switch ((p || "").toLowerCase()) {
    case "douyu":
      return Platform.DOUYU;
    case "douyin":
      return Platform.DOUYIN;
    case "huya":
      return Platform.HUYA;
    case "bilibili":
      return Platform.BILIBILI;
    default:
      return Platform.DOUYU;
  }
};

const isFollowed = (item: any) => {
  if (!item?.platform || !item?.roomId) return false;
  return followStore.isFollowed(toPlatformEnum(item.platform), item.roomId);
};

const isFavorite = (item: any) => {
  if (!item?.platform || item.type !== "category" || !item.categoryId)
    return false;
  return categoryStore.favoriteCategories.some(
    (f) =>
      f.platform === item.platform &&
      f.id === item.categoryId &&
      f.type === (item.categoryType || "cate2"),
  );
};

const favoriteLabel = (item: any) => {
  if (item.type === "category") {
    return isFavorite(item) ? "取消收藏" : "收藏分类";
  }
  return isFollowed(item) ? "取消关注" : "关注主播";
};

const toggleFavorite = (item: any) => {
  if (!item) return;
  if (item.type === "category") {
    if (!item.platform || !item.categoryId) return;
    categoryStore.toggleFavoriteCategoryForPlatform(item.platform, {
      id: item.categoryId,
      title: item.title,
      type: item.categoryType || "cate2",
    });
    return;
  }
  if (!item.platform || !item.roomId) return;
  const platformEnum = toPlatformEnum(item.platform);
  if (isFollowed(item)) {
    followStore.unfollowStreamer(platformEnum, item.roomId);
  } else {
    followStore.followStreamer({
      platform: platformEnum,
      id: item.roomId,
      nickname: item.title,
      avatarUrl: item.avatar || null,
      roomTitle: item.subtitle || "",
      isLive: item.isLive ?? false,
      liveStatus: item.isLive ? "LIVE" : "OFFLINE",
    });
  }
};

const { Meta_K, Ctrl_K } = useMagicKeys();
whenever(() => Meta_K.value || Ctrl_K.value, () => {
  emit("update:modelValue", true);
});
</script>
