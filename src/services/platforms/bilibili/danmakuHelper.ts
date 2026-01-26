import { listen, type Event as TauriEvent } from "@tauri-apps/api/event";
import type { Ref } from "vue";
import type {
  DanmakuMessage,
  DanmuOverlayInstance,
  DanmuRenderOptions,
} from "../../../types/models/danmaku";
import { v4 as uuidv4 } from "uuid";
import { startBilibiliDanmaku, stopBilibiliDanmaku as stopBilibiliDanmakuApi } from "../../../api/danmaku";

// 直播流获取已迁移到后端统一接口 get_live_stream_v2

// 统一的 Rust 弹幕事件负载（与 Douyin/Douyu/Huya 保持一致）
interface UnifiedRustDanmakuPayload {
  room_id: string;
  user: string;
  content: string;
  user_level: number;
  fans_club_level: number;
}

export async function startBilibiliDanmakuListener(
  roomId: string,
  danmuOverlay: DanmuOverlayInstance | null,
  danmakuMessagesRef: Ref<DanmakuMessage[]>,
  cookie?: string,
  renderOptions?: DanmuRenderOptions,
): Promise<() => void> {
  // 启动后端 B 站弹幕监听（cookie 可选）；若未传，则从 localStorage 兜底读取
  const effectiveCookie =
    cookie ??
    (typeof localStorage !== "undefined"
      ? localStorage.getItem("bilibili_cookie") || undefined
      : undefined);
  await startBilibiliDanmaku(
    { args: { room_id_str: roomId } },
    effectiveCookie || null,
  );

  const eventName = "danmaku-message";
  const unlisten = await listen<UnifiedRustDanmakuPayload>(
    eventName,
    (event: TauriEvent<UnifiedRustDanmakuPayload>) => {
      if (!event.payload || event.payload.room_id !== roomId) return;

      const frontendDanmaku: DanmakuMessage = {
        id: uuidv4(),
        nickname: event.payload.user || "未知用户",
        content: event.payload.content,
        level: String(event.payload.user_level ?? 0),
        badgeLevel:
          event.payload.fans_club_level != null
            ? String(event.payload.fans_club_level)
            : undefined,
        room_id: roomId,
      };

        const options = renderOptions?.buildCommentOptions?.(frontendDanmaku) ?? {};
        const shouldDisplay = renderOptions?.shouldDisplay?.() ?? true;

        if (shouldDisplay && danmuOverlay?.emit) {
          const styleFromOptions = options.style || {};
          
          danmuOverlay.emit({
            text: frontendDanmaku.content,
            mode: (options.mode === 'scroll' ? 'rtl' : options.mode) as any || 'rtl',
            style: {
              ...styleFromOptions,
              color: (frontendDanmaku as any).color || styleFromOptions.color || "#FFFFFF",
            },
          });
        }

      danmakuMessagesRef.value.push(frontendDanmaku);
      if (danmakuMessagesRef.value.length > 200) {
        danmakuMessagesRef.value.splice(
          0,
          danmakuMessagesRef.value.length - 200,
        );
      }
    },
  );
  return unlisten;
}

export async function stopBilibiliDanmaku(
  currentUnlistenFn: (() => void) | null,
): Promise<void> {
  if (currentUnlistenFn) {
    try {
      currentUnlistenFn();
    } catch {}
  }
  try {
    await stopBilibiliDanmakuApi();
  } catch {}
}
