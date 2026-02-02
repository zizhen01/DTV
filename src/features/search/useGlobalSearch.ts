import { computed, ref, shallowRef, watch } from "vue";
import Fuse from "fuse.js";
import { useRouter } from "vue-router";
import { useCategoryStore } from "../../store/categoryStore";
import { useFollowStore } from "../../store/followStore";
import { usePlayerStore } from "../../store/playerStore";
import type { UiPlatform } from "../../types/app/platform";
import {
  searchDouyuAnchor,
  searchHuyaAnchors,
  searchBilibiliRooms,
} from "../../api/search";
import { getLiveStreamV2 } from "../../api/live";
import { useImageProxy } from "../following/composables/useProxy";
import { douyinCategoriesData } from "../../services/platforms/douyin/douyinCategoriesData";
import { huyaCategoriesData } from "../../services/platforms/huya/huyaCategoriesData";
import { biliCategoriesData } from "../../services/platforms/bilibili/biliCategoriesData";

export type SearchItemType = "streamer" | "room" | "category";

export interface SearchItem {
  key: string;
  type: SearchItemType;
  platform?: UiPlatform;
  title: string;
  subtitle?: string;
  roomId?: string;
  categoryId?: string;
  categoryType?: "cate2" | "cate3";
  tags?: string[];
  avatar?: string | null;
  isLive?: boolean;
  isFollowed?: boolean;
  isFavorite?: boolean;
  score?: number;
  payload?: any;
}

const FUSE_OPTIONS: Fuse.IFuseOptions<SearchItem> = {
  keys: [
    { name: "title", weight: 0.6 },
    { name: "subtitle", weight: 0.25 },
    { name: "tags", weight: 0.15 },
  ],
  threshold: 0.35,
  includeScore: true,
};

const isNumeric = (value: string) => /^\d+$/.test(value);

const normalizePlatform = (p?: UiPlatform) =>
  (p || "").toLowerCase() as UiPlatform;

export function useGlobalSearch() {
  const router = useRouter();
  const categoryStore = useCategoryStore();
  const followStore = useFollowStore();
  const playerStore = usePlayerStore();
  const { proxify, ensureProxyStarted } = useImageProxy();

  const query = ref("");
  const isLoadingRemote = ref(false);
  const remoteItems = ref<SearchItem[]>([]);
  const fuseRef = shallowRef<Fuse<SearchItem> | null>(null);

  const favoriteCategoryKeys = computed(() =>
    new Set(
      categoryStore.favoriteCategories.map(
        (f) => `${f.platform}:${f.type}:${f.id}`,
      ),
    ),
  );

  const followedKeys = computed(() =>
    new Set(
      followStore.followedStreamers.map(
        (s) => `${s.platform}:${s.id}`,
      ),
    ),
  );

  const localItems = computed<SearchItem[]>(() => {
    const items: SearchItem[] = [];

    followStore.followedStreamers.forEach((s) => {
      items.push({
        key: `follow:${s.platform}:${s.id}`,
        type: "streamer",
        platform: normalizePlatform(s.platform),
        title: s.nickname || s.roomTitle || s.id,
        subtitle: s.roomTitle || "已关注主播",
        roomId: s.id,
        avatar: s.avatarUrl || null,
        isLive: s.liveStatus === "LIVE" || s.isLive,
        isFollowed: true,
        tags: [s.platform, "followed"],
        payload: s,
      });
    });

    playerStore.activeStreamers.forEach((s) => {
      const key = `active:${s.platform}:${s.roomId}`;
      if (items.some((item) => item.key === key)) return;
      items.push({
        key,
        type: "room",
        platform: normalizePlatform(s.platform),
        title: s.title || s.anchorName || s.roomId,
        subtitle: s.anchorName || "正在播放",
        roomId: s.roomId,
        avatar: s.avatar || null,
        isLive: s.isLive,
        isFollowed: followStore.isFollowed(s.platform, s.roomId),
        tags: [s.platform, "active"],
        payload: s,
      });
    });

    categoryStore.favoriteCategories.forEach((fav) => {
      items.push({
        key: `favcat:${fav.platform}:${fav.type}:${fav.id}`,
        type: "category",
        platform: fav.platform,
        title: fav.title,
        subtitle: "已收藏分类",
        categoryId: fav.id,
        categoryType: fav.type === "cate3" ? "cate3" : "cate2",
        isFavorite: true,
        tags: [fav.platform, "favorite"],
        payload: fav,
      });
    });

    const pushCategory = (
      platform: UiPlatform,
      id: string,
      title: string,
      type: "cate2" | "cate3" = "cate2",
      parent?: string,
    ) => {
      if (!id || !title) return;
      const key = `cat:${platform}:${type}:${id}`;
      items.push({
        key,
        type: "category",
        platform,
        title,
        subtitle: parent ? parent : `${platform} 分类`,
        categoryId: id,
        categoryType: type,
        isFavorite: favoriteCategoryKeys.value.has(
          `${platform}:${type}:${id}`,
        ),
        tags: [platform, parent || "category"],
      });
    };

    if (categoryStore.douyuCate2List.length > 0) {
      categoryStore.categoryGroups.forEach((group) => {
        group.items.forEach((c) => {
          pushCategory("douyu", c.id, c.title, "cate2", group.title);
        });
      });
    }

    douyinCategoriesData.forEach((c1) => {
      (c1.subcategories || []).forEach((c2) => {
        pushCategory(
          "douyin",
          c2.href || "",
          c2.title || "",
          "cate2",
          c1.title || "",
        );
      });
    });

    huyaCategoriesData.forEach((c1) => {
      (c1.subcategories || []).forEach((c2) => {
        pushCategory(
          "huya",
          c2.href || "",
          c2.title || "",
          "cate2",
          c1.title || "",
        );
      });
    });

    biliCategoriesData.forEach((c1) => {
      (c1.subcategories || []).forEach((c2) => {
        pushCategory(
          "bilibili",
          c2.href || "",
          c2.title || "",
          "cate2",
          c1.title || "",
        );
      });
    });

    return items;
  });

  watch(
    localItems,
    (items) => {
      fuseRef.value = new Fuse(items, FUSE_OPTIONS);
    },
    { immediate: true },
  );

  const localResults = computed<SearchItem[]>(() => {
    const q = query.value.trim();
    if (!q || !fuseRef.value) return [];
    return fuseRef.value.search(q).map((result) => ({
      ...result.item,
      score: result.score ?? 0.5,
    }));
  });

  const combinedResults = computed<SearchItem[]>(() => {
    const map = new Map<string, SearchItem>();
    localResults.value.forEach((item) => map.set(item.key, item));
    remoteItems.value.forEach((item) => map.set(item.key, item));

    const list = Array.from(map.values());
    return list.sort((a, b) => {
      const aFav = a.isFavorite || a.isFollowed ? -0.2 : 0;
      const bFav = b.isFavorite || b.isFollowed ? -0.2 : 0;
      const aLive = a.isLive ? -0.05 : 0;
      const bLive = b.isLive ? -0.05 : 0;
      const aScore = (a.score ?? 0.45) + aFav + aLive;
      const bScore = (b.score ?? 0.45) + bFav + bLive;
      return aScore - bScore;
    });
  });

  const groupedResults = computed(() => {
    const favorites: SearchItem[] = [];
    const rooms: SearchItem[] = [];
    const categories: SearchItem[] = [];

    combinedResults.value.forEach((item) => {
      if (item.isFavorite || item.isFollowed) favorites.push(item);
      else if (item.type === "category") categories.push(item);
      else rooms.push(item);
    });

    return [
      { id: "favorites", title: "已关注 / 已收藏", items: favorites.slice(0, 12) },
      { id: "rooms", title: "主播 / 房间", items: rooms.slice(0, 20) },
      { id: "categories", title: "分类", items: categories.slice(0, 20) },
    ].filter((group) => group.items.length > 0);
  });

  let requestToken = 0;

  const fetchRemote = async (q: string) => {
    const token = ++requestToken;
    const keyword = q.trim();
    if (!keyword || keyword.length < 2) {
      remoteItems.value = [];
      return;
    }
    isLoadingRemote.value = true;

    try {
      const tasks: Promise<SearchItem[]>[] = [];

      tasks.push(
        (async () => {
          try {
            const resp = await searchDouyuAnchor(keyword);
            const data = JSON.parse(resp);
            if (data.error !== 0 || !data.data?.relateUser) return [];
            return data.data.relateUser
              .filter((item: any) => item.type === 1)
              .map((item: any) => {
                const anchorInfo = item.anchorInfo;
                const isReallyLive =
                  anchorInfo.isLive === 1 && anchorInfo.videoLoop !== 1;
                const roomId = anchorInfo.rid?.toString() || "";
                return {
                  key: `douyu:${roomId}`,
                  type: "streamer" as const,
                  platform: "douyu" as UiPlatform,
                  title: anchorInfo.nickName,
                  subtitle: anchorInfo.roomName || anchorInfo.description || "",
                  roomId,
                  avatar: anchorInfo.avatar || null,
                  isLive: isReallyLive,
                  isFollowed: followedKeys.value.has(`douyu:${roomId}`),
                  tags: ["douyu"],
                } as SearchItem;
              });
          } catch (e) {
            console.warn("[Search] Douyu search failed", e);
            return [];
          }
        })(),
      );

      tasks.push(
        (async () => {
          try {
            const items = await searchHuyaAnchors(keyword);
            if (!Array.isArray(items)) return [];
            await ensureProxyStarted();
            return items.map((item) => {
              const roomId = item.room_id?.toString() || "";
              return {
                key: `huya:${roomId}`,
                type: "streamer" as const,
                platform: "huya" as UiPlatform,
                title: item.user_name || "虎牙主播",
                subtitle: item.title || "",
                roomId,
                avatar: proxify(item.avatar || null),
                isLive: !!item.live_status,
                isFollowed: followedKeys.value.has(`huya:${roomId}`),
                tags: ["huya"],
              } as SearchItem;
            });
          } catch (e) {
            console.warn("[Search] Huya search failed", e);
            return [];
          }
        })(),
      );

      tasks.push(
        (async () => {
          try {
            const items = await searchBilibiliRooms(keyword);
            if (!Array.isArray(items)) return [];
            await ensureProxyStarted();
            return items.map((item) => {
              const roomId = item.room_id?.toString() || "";
              return {
                key: `bilibili:${roomId}`,
                type: "room" as const,
                platform: "bilibili" as UiPlatform,
                title: item.anchor || item.title || "B站主播",
                subtitle: item.title || item.area || "",
                roomId,
                avatar: proxify(item.avatar || null),
                isLive: !!item.is_live,
                isFollowed: followedKeys.value.has(`bilibili:${roomId}`),
                tags: ["bilibili"],
              } as SearchItem;
            });
          } catch (e) {
            console.warn("[Search] Bilibili search failed", e);
            return [];
          }
        })(),
      );

      if (isNumeric(keyword)) {
        tasks.push(
          (async () => {
            try {
              const resp = await getLiveStreamV2({
                platform: "douyin",
                room_id: keyword,
                debug: false,
                mode: "meta",
              });
              if (resp.status === "error" || !resp.room?.anchor_name) return [];
              const webId = resp.room.web_rid ?? keyword;
              return [
                {
                  key: `douyin:${webId}`,
                  type: "room" as const,
                  platform: "douyin" as UiPlatform,
                  title: resp.room.anchor_name || "抖音主播",
                  subtitle: resp.room.title || "",
                  roomId: webId,
                  avatar: resp.room.avatar || null,
                  isLive: resp.status === "live",
                  isFollowed: followedKeys.value.has(`douyin:${webId}`),
                  tags: ["douyin"],
                },
              ] as SearchItem[];
            } catch (e) {
              console.warn("[Search] Douyin search failed", e);
              return [];
            }
          })(),
        );
      }

      const results = (await Promise.all(tasks)).flat();
      if (token !== requestToken) return;
      remoteItems.value = results.map((item) => ({
        ...item,
        isFavorite:
          item.isFavorite ||
          (item.categoryId
            ? favoriteCategoryKeys.value.has(
                `${item.platform}:${item.categoryType || "cate2"}:${item.categoryId}`,
              )
            : false),
      }));
    } finally {
      if (token === requestToken) isLoadingRemote.value = false;
    }
  };

  let debounceTimer: number | null = null;
  watch(query, (val) => {
    const v = val.trim();
    if (!v) {
      remoteItems.value = [];
      isLoadingRemote.value = false;
      return;
    }
    if (debounceTimer) window.clearTimeout(debounceTimer);
    debounceTimer = window.setTimeout(() => fetchRemote(v), 200);
  });

  const selectResult = async (item: SearchItem) => {
    if (!item) return;
    if (item.type === "category") {
      const platform = item.platform || "douyu";
      if (platform === "douyu" && categoryStore.douyuCate2List.length === 0) {
        await categoryStore.initDouyuData();
      }
      categoryStore.setPlatform(platform);
      if (item.categoryId) {
        categoryStore.handleCategorySelect({
          id: item.categoryId,
          title: item.title,
        });
      }
      await router.push({
        name: "ChannelList",
        params: { platform },
      });
      return;
    }

    if (item.roomId && item.platform) {
      await router.push({
        name: "StreamRoom",
        params: { platform: item.platform, roomId: item.roomId },
      });
    }
  };

  const ensureDouyuCategories = async () => {
    if (categoryStore.douyuCate2List.length === 0) {
      await categoryStore.initDouyuData();
    }
  };

  return {
    query,
    groupedResults,
    combinedResults,
    isLoadingRemote,
    fetchRemote,
    selectResult,
    ensureDouyuCategories,
  };
}
