<template>
  <div class="flex h-full flex-col bg-transparent">
    <div
      class="flex items-center gap-3 px-6 py-6"
      :class="collapsed ? 'justify-center px-2' : 'justify-between'"
    >
      <h3
        v-if="!collapsed"
        class="flex items-center gap-3 text-2xl font-black tracking-tighter text-text-main"
        aria-label="关注列表"
      >
        <UsersRound class="h-6 w-6 text-brand" aria-hidden="true" :stroke-width="2.5" />
        关注列表
      </h3>
      <div
        class="flex items-center gap-2"
        :class="{ 'flex-col gap-3': collapsed }"
      >
        <template v-if="!collapsed">
          <button
            v-if="!isRefreshing"
            @click="refreshList"
            class="group flex h-10 w-10 items-center justify-center rounded-full bg-surface-high transition-all hover:bg-brand hover:text-white hover:shadow-lg hover:shadow-brand/20"
            title="刷新列表"
          >
            <span class="flex items-center justify-center">
              <svg
                v-if="!showCheckIcon"
                xmlns="http://www.w3.org/2000/svg"
                width="18"
                height="18"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2.5"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="transition-transform group-hover:rotate-180 duration-500"
              >
                <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
                <path d="M21 3v5h-5" />
              </svg>
              <Check v-else class="h-5 w-5" />
            </span>
          </button>
          <span
            v-if="isRefreshing"
            class="inline-flex h-10 items-center gap-2 rounded-full bg-brand/10 px-4 text-xs font-black text-brand"
            aria-live="polite"
          >
            <span
              class="h-3 w-3 animate-spin rounded-full border-2 border-brand border-t-transparent"
              aria-hidden="true"
            ></span>
            {{ progressCurrent }}/{{ progressTotal }}
          </span>
        </template>
      </div>
    </div>

    <div
      class="relative flex-1 overflow-y-auto pb-6 transition-[padding] duration-300"
      :class="collapsed ? 'px-2' : 'px-4'"
      ref="listRef"
      @scroll.passive="handleListScroll"
      @mouseleave="handleListMouseLeave"
    >
      <div
        class="pointer-events-none absolute top-0 left-0 h-8 w-full rounded-[12px] opacity-0"
      ></div>
      <div
        v-if="listItems.length === 0"
        class="flex flex-col items-center justify-center gap-3 py-10"
      >
        <div class="text-[var(--secondary-text)]">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="64"
            height="64"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="feather feather-heart"
            :class="{ 'h-8 w-8': collapsed }"
          >
            <path
              d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
            ></path>
          </svg>
        </div>
        <h3 v-if="!collapsed" class="text-base font-semibold">暂无关注主播</h3>
        <p v-if="!collapsed" class="text-sm">关注主播后，他们会出现在这里</p>
      </div>

        <ul
          ref="listElRef"
          v-else
          class="flex flex-col gap-2"
          :class="{ 'items-center': collapsed }"
        >
        <FollowListItem
          v-for="(item, index) in listItems"
          :key="
            item.type === 'folder'
              ? `folder_${item.data.id}`
              : `${item.data.platform}:${item.data.id}`
          "
          :item="item"
          :is-dragging="isDragging && draggedIndex === index"
          :is-drag-over="false"
          :can-accept-drop="false"
          :global-dragging="isDragging"
          :followed-anchors="props.followedAnchors"
          :just-added-ids="justAddedIds"
          :proxy-base="proxyBase"
          :get-avatar-src="getAvatarSrcForStreamer"
          :handle-img-error="handleImgError"
          :get-live-indicator-class="getLiveIndicatorClass"
          :get-streamer-item-class="getStreamerItemClass"
          :collapsed="collapsed"
          @mousedown="handleMouseDown($event, index)"
          @mouseenter="handleItemMouseEnter($event, index)"
          @mouseleave="handleItemMouseLeave(index)"
          @select-anchor="(s) => emit('selectAnchor', s)"
          @remove="handleRemove"
          @click="handleClick"
        />
      </ul>
    </div>

    <FollowOverlay
      :show="showOverlay"
      :items="filteredStreamers"
      :getAvatarSrc="getAvatarSrcForStreamer"
      :handleImgError="handleImgError"
      :getLiveIndicatorClass="getLiveIndicatorClass"
      :proxyBase="proxyBase"
      :alignTop="overlayAlignTop"
      :alignLeft="overlayAlignLeft"
      :isRefreshing="isRefreshing"
      :is-delete-mode="overlayDeleteMode"
      @select="selectFromOverlay"
      @close="closeOverlay"
      @refresh="refreshList"
      @toggle-remove="toggleOverlayDeleteMode"
      @remove="handleOverlayRemove"
    >
      <template #filters>
        <FilterChips
          :visiblePlatforms="visiblePlatforms"
          :activeFilter="activeFilter"
          @update:activeFilter="setFilter"
        />
      </template>
    </FollowOverlay>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch, onUnmounted } from "vue";
import type { FollowedStreamer } from "../../../types/models/streamer";
import { Platform, type SupportedPlatform } from "../../../types/app/platform";

import { refreshDouyuFollowedStreamer } from "../../../services/platforms/douyu/followListHelper";
import { refreshDouyinFollowedStreamer } from "../../../services/platforms/douyin/followListHelper";
import { getLiveStreamV2 } from "../../../api/live";
import FollowOverlay from "./FollowOverlay.vue";
import FilterChips from "./FilterChips.vue";
import FollowListItem from "./FollowListItem.vue";
import { useImageProxy } from "../composables/useProxy";
import { useFollowStore } from "../../../store/followStore";
import { UsersRound, Check } from "lucide-vue-next";
import Sortable from "sortablejs";

const expandBtnRef = ref<HTMLButtonElement | null>(null);
const overlayAlignTop = ref<number>(64);
const overlayAlignLeft = ref<number>(240);

const followStore = useFollowStore();
const { proxyBase, ensureProxyStarted, getAvatarSrc, proxify } = useImageProxy();
const getAvatarSrcForStreamer = (s: FollowedStreamer) =>
  getAvatarSrc(s.platform, s.avatarUrl);

const props = defineProps<{
  followedAnchors: FollowedStreamer[];
  collapsed?: boolean;
}>();

const emit = defineEmits<{
  (e: "selectAnchor", streamer: FollowedStreamer): void;
  (e: "unfollow", payload: { platform: Platform; id: string }): void;
  (e: "reorderList", newList: FollowedStreamer[]): void;
  (e: "toggleCollapse"): void;
}>();

const isRefreshing = ref(false);
const showCheckIcon = ref(false);
watch(isRefreshing, (newVal, oldVal) => {
  if (oldVal && !newVal) {
    showCheckIcon.value = true;
    setTimeout(() => (showCheckIcon.value = false), 1000);
  }
});

const listRef = ref<HTMLElement | null>(null);
const listElRef = ref<HTMLUListElement | null>(null);
const justAddedIds = ref<string[]>([]);
const animationTimeout = ref<number | null>(null);
const listItems = computed(() => followStore.displayList);
const streamers = computed(() => props.followedAnchors);
const isDragging = ref(false);
const draggedIndex = ref(-1);
let sortable: Sortable | null = null;

const handleMouseDown = (_e: MouseEvent, index: number) => {
  draggedIndex.value = index;
};

const handleImgError = (ev: Event, _s: FollowedStreamer) => {
  const target = ev.target as HTMLImageElement | null;
  if (target) target.src = "";
};

const applyHoverHighlight = (target: HTMLElement | null) => {
  const listEl = listRef.value;
  if (!listEl) return;
  
  if (!target) {
    listEl.style.setProperty("--hover-opacity", "0");
    return;
  }

  const listRect = listEl.getBoundingClientRect();
  const itemRect = target.getBoundingClientRect();
  const y = itemRect.top - listRect.top + listEl.scrollTop;
  const h = itemRect.height;

  listEl.style.setProperty("--hover-opacity", "1");
  listEl.style.setProperty("--hover-y", `${y}px`);
  listEl.style.setProperty("--hover-h", `${h}px`);
};

const handleItemMouseEnter = (event: MouseEvent, index: number) => {
  // Simple check if it's a valid item
  const target = event.currentTarget as HTMLElement;
  applyHoverHighlight(target);
};

const handleItemMouseLeave = (_index: number) => {
  // Optional: delay hiding or let list mouseleave handle it
  // We can just do nothing here and let the next enter or list leave handle it
};

const handleListMouseLeave = () => {
  applyHoverHighlight(null);
};

const handleListScroll = () => {
  // On scroll, we should probably hide highlight or update it.
  // Updating it requires tracking current hovered element which is complex during scroll.
  // Hiding is smoother.
  applyHoverHighlight(null);
};

const showOverlay = ref(false);
const overlayDeleteMode = ref(false);
type FilterType = "ALL" | Platform;
const activeFilter = ref<FilterType>("ALL");
const openOverlay = () => {
  const headerRect = document
    .querySelector(".app-header")
    ?.getBoundingClientRect() as DOMRect | undefined;
  overlayAlignTop.value = headerRect ? Math.round(headerRect.bottom + 8) : 72;
  const rect = expandBtnRef.value?.getBoundingClientRect();
  overlayAlignLeft.value = rect ? Math.round(rect.right + 12) : 240;
  overlayDeleteMode.value = false;
  showOverlay.value = true;
};
const closeOverlay = () => {
  showOverlay.value = false;
  overlayDeleteMode.value = false;
};
const toggleOverlayDeleteMode = () =>
  (overlayDeleteMode.value = !overlayDeleteMode.value);
const setFilter = (f: FilterType) => (activeFilter.value = f);

const platformsOrder: Platform[] = [
  Platform.DOUYU,
  Platform.DOUYIN,
  Platform.HUYA,
  Platform.BILIBILI,
];
const visiblePlatforms = computed(() => {
  const present = new Set<Platform>();
  for (const s of streamers.value)
    if (s.platform !== undefined) present.add(s.platform);
  return platformsOrder.filter((p) => present.has(p));
});
const filteredStreamers = computed(() => {
  if (activeFilter.value === "ALL") return streamers.value;
  return streamers.value.filter((s) => s.platform === activeFilter.value);
});

const selectFromOverlay = (s: FollowedStreamer) => {
  if (overlayDeleteMode.value) return;
  emit("selectAnchor", s);
  closeOverlay();
};
const handleOverlayRemove = (s: FollowedStreamer) =>
  emit("unfollow", { platform: s.platform, id: s.id });

const getStreamerItemClass = (streamer: FollowedStreamer) => ({
  "status-live": streamer.liveStatus === "LIVE",
  "status-replay": streamer.liveStatus === "REPLAY",
  "status-offline":
    streamer.liveStatus === "OFFLINE" ||
    !streamer.liveStatus ||
    streamer.liveStatus === "UNKNOWN",
});

const getLiveIndicatorClass = (streamer: FollowedStreamer) => {
  switch (streamer.liveStatus) {
    case "LIVE":
      return "is-live";
    case "REPLAY":
      return "is-replay";
    default:
      return "is-offline";
  }
};

watch(
  () => props.followedAnchors,
  (newVal, oldVal) => {
    if (!oldVal || oldVal.length === 0) return;
    const oldIds = oldVal.map((s) => s.id);
    const newStreamers = newVal.filter((s) => !oldIds.includes(s.id));
    if (newStreamers.length > 0) {
      newStreamers.forEach((s) => {
        justAddedIds.value.push(s.id);
        setTimeout(
          () =>
            (justAddedIds.value = justAddedIds.value.filter(
              (id) => id !== s.id,
            )),
          3000,
        );
      });
    }
  },
  { deep: true },
);

const handleClick = (e: MouseEvent, streamer: FollowedStreamer) => {
  if (isDragging.value && draggedIndex.value !== -1) {
    e.preventDefault();
    return;
  }
  emit("selectAnchor", streamer);
};

const handleRemove = (streamer: FollowedStreamer) => {
  followStore.unfollowStreamer(streamer.platform, streamer.id);
};


const clearAnimationTimeout = () => {
  if (animationTimeout.value !== null) {
    clearTimeout(animationTimeout.value);
    animationTimeout.value = null;
  }
};
const progressCurrent = ref(0);
const progressTotal = ref(0);

async function runWithConcurrency<T>(
  items: T[],
  worker: (item: T, index: number) => Promise<void>,
  limit: number,
) {
  let cursor = 0;
  const runners: Promise<void>[] = [];
  const runner = async () => {
    while (cursor < items.length) {
      const i = cursor++;
      await worker(items[i], i);
      await new Promise((res) => setTimeout(res, 0));
    }
  };
  const n = Math.min(limit, items.length);
  for (let k = 0; k < n; k++) runners.push(runner());
  await Promise.all(runners);
}

const refreshList = async () => {
  if (isRefreshing.value) return;

  const startTime = Date.now();

  isRefreshing.value = true;

  progressCurrent.value = 0;

  // Smart Refresh: Calculate visible items to prioritize them

  if (listRef.value) {
    const listRect = listRef.value.getBoundingClientRect();

    const items = listRef.value.querySelectorAll(".list-item-wrapper");

    items.forEach((el) => {
      const rect = el.getBoundingClientRect();

      // Simple visibility check: overlaps with container's vertical range

      if (rect.bottom > listRect.top && rect.top < listRect.bottom) {
        // Try to find streamer ID from DOM attributes or context if possible
        // Since we don't have IDs on the wrapper easily accessible without adding them,
        // let's rely on the store's order vs index if possible, OR
        // Add data-id to FollowListItem wrapper in template.
      }
    });
  }

  // Note: Since we didn't add data-id to the wrapper yet, let's just pass empty for now

  // and I will add data-id in the next step to make this work.

  // Actually, I can update refreshAll to just take the list from store,

  // but here I want to use the store's action.

  // Wait, I replaced refreshAll in store, but the local refreshList function here

  // currently implements its own concurrency logic. I should delegate to the store

  // or update the local logic to support priority.

  // The local logic is quite complex with progress tracking.

  // Let's UPDATE the local logic to use the new priority sort but keep the progress tracking here

  // or move progress tracking to store?

  // Moving progress tracking to store is better but requires more changes.

  // Let's keep local logic but sort the items `props.followedAnchors` based on visibility.

  // 1. Get visible IDs

  const visibleKeys = new Set<string>();

  if (listRef.value) {
    const listRect = listRef.value.getBoundingClientRect();

    // We need to add data-key to FollowListItem to make this efficient

    const itemElements = listRef.value.querySelectorAll("[data-streamer-key]");

    itemElements.forEach((el) => {
      const rect = el.getBoundingClientRect();

      if (rect.bottom > listRect.top && rect.top < listRect.bottom) {
        const key = el.getAttribute("data-streamer-key");

        if (key) visibleKeys.add(key);
      }
    });
  }

  // 2. Sort anchors: visible first

  const sortedAnchors = [...props.followedAnchors].sort((a, b) => {
    const keyA = toStreamerKey(a);

    const keyB = toStreamerKey(b);

    const aVis = visibleKeys.has(keyA);

    const bVis = visibleKeys.has(keyB);

    if (aVis && !bVis) return -1;

    if (!aVis && bVis) return 1;

    return 0;
  });

  const totalFromStore = followStore.followedStreamers?.length ?? 0;

  progressTotal.value =
    totalFromStore > 0 ? totalFromStore : sortedAnchors.length;

  try {
    const hasBiliOrHuya = sortedAnchors.some(
      (s) => s.platform === Platform.BILIBILI || s.platform === Platform.HUYA,
    );

    if (hasBiliOrHuya) await ensureProxyStarted();

    const updates: { originalKey: string; updated: FollowedStreamer }[] = [];

    // Use sortedAnchors instead of props.followedAnchors

    await runWithConcurrency(
      sortedAnchors,
      async (streamer) => {
        let updatedData: Partial<FollowedStreamer> = {
          lastUpdateFailed: false,
          lastError: undefined,
        };

        try {
          if (streamer.platform === Platform.DOUYU)
            updatedData = {
              ...updatedData,
              ...(await refreshDouyuFollowedStreamer(streamer)),
            };
          else if (streamer.platform === Platform.DOUYIN)
            updatedData = {
              ...updatedData,
              ...(await refreshDouyinFollowedStreamer(streamer)),
            };
          else if (
            streamer.platform === Platform.HUYA ||
            streamer.platform === Platform.BILIBILI
          ) {
            const resp = await getLiveStreamV2({
              platform: streamer.platform.toLowerCase() as SupportedPlatform,
              room_id: streamer.id,
              mode: "meta",
            });

            const live = resp.status === "live";

            updatedData = {
              ...updatedData,

              liveStatus: live ? "LIVE" : "OFFLINE",

              isLive: live,

              nickname: resp.room?.anchor_name ?? streamer.nickname,

              roomTitle: resp.room?.title ?? streamer.roomTitle,

              avatarUrl: resp.room?.avatar ?? streamer.avatarUrl,
            };
          }

          updates.push({
            originalKey: `${streamer.platform}:${streamer.id}`,
            updated: { ...streamer, ...updatedData } as FollowedStreamer,
          });
        } catch (e: any) {
          updates.push({
            originalKey: `${streamer.platform}:${streamer.id}`,

            updated: {
              ...streamer,

              lastUpdateFailed: true,

              lastError: e.message || String(e),
            },
          });
        } finally {
          progressCurrent.value++;
        }
      },
      FOLLOW_REFRESH_CONCURRENCY,
    );

    const validUpdates = updates.filter(
      (entry): entry is RefreshUpdateEntry => !!entry && !!entry.updated,
    );

    // Batch update store to avoid reactivity storm

    // Note: buildPostRefreshOrdering logic might need to be adjusted if we want to preserve partial updates

    // But since we refreshed everyone (just sorted), we can update all.

    // However, we should also update the Store's map with new data.

    // The previous logic only updated listOrder. We need to update streamer details too.

    validUpdates.forEach((u) => {
      followStore.updateStreamerDetails({
        ...u.updated,
        platform: u.updated.platform,
        id: u.updated.id,
      });
    });

    const result = buildPostRefreshOrdering(validUpdates);

    if (result) {
      followStore.updateListOrder(result.nextListOrder);

      if (
        JSON.stringify(result.streamerSequence) !==
        JSON.stringify(props.followedAnchors)
      )
        emit("reorderList", result.streamerSequence);
    }
  } finally {
    const elapsed = Date.now() - startTime;

    const finish = () => {
      isRefreshing.value = false;
      showCheckIcon.value = true;
      setTimeout(() => (showCheckIcon.value = false), 1000);
    };

    if (elapsed < MIN_ANIMATION_DURATION) {
      clearAnimationTimeout();

      animationTimeout.value = window.setTimeout(
        finish,
        MIN_ANIMATION_DURATION - elapsed,
      );
    } else finish();
  }
};

onMounted(async () => {
  if (!followStore.listOrder.length && props.followedAnchors.length > 0)
    followStore.initializeListOrder();
  if (props.followedAnchors.some((s) => s.platform === Platform.BILIBILI))
    await ensureProxyStarted();
  requestIdle(refreshList);

  if (listElRef.value) {
    sortable = new Sortable(listElRef.value, {
      animation: 160,
      handle: ".drag-handle",
      filter: "button, input, a",
      preventOnFilter: true,
      onStart: () => {
        isDragging.value = true;
      },
      onEnd: () => {
        isDragging.value = false;
        draggedIndex.value = -1;
        if (!listElRef.value) return;
        const keys = Array.from(listElRef.value.children)
          .map((el) => (el as HTMLElement).dataset.streamerKey || "")
          .filter(Boolean);
        if (!keys.length) return;
        followStore.updateListOrder(
          keys.map((id) => ({ type: "streamer" as const, id })),
        );
      },
    });
  }
});

onUnmounted(() => {
  clearAnimationTimeout();
  if (sortable) {
    sortable.destroy();
    sortable = null;
  }
});
</script>

<style src="./index.css" scoped></style>
