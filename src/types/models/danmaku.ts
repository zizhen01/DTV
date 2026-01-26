import type { SupportedPlatform, Platform } from "../app/platform";

export interface CommonDanmakuMessage {
  id?: string;
  platform: SupportedPlatform;
  type: "chat" | "gift" | "system" | "enter" | "other";
  sender: {
    uid?: string;
    nickname: string;
    level?: number | string;
    badgeName?: string;
    badgeLevel?: number;
  };
  content: string;
  timestamp?: number;
  color?: string;
  rawData?: any;
}

export interface DanmakuMessage {
  id: string;
  nickname: string;
  content: string;
  level?: string;
  badgeLevel?: string;
  room_id?: string;
  color?: string;
  isSystem?: boolean;
  type?: "error" | "success" | "info" | string;
}

export interface DanmakuComment {
  text: string;
  mode?: "ltr" | "rtl" | "top" | "bottom";
  time?: number;
  style?: Record<string, any>;
  render?: () => HTMLElement | HTMLCanvasElement;
}

export interface DanmuOverlayInstance {
  emit(comment: DanmakuComment): void;
  play(): void;
  pause(): void;
  clear(): void;
  resize(): void;
  show(): void;
  hide(): void;
  
  // Custom helpers we will implement in the wrapper or shim
  _setOpacity?(opacity: number): void;
  _setFontSize?(size: number): void;
  _setArea?(area: number): void; // 0-1
  _setDuration?(duration: number): void;
}

export interface DanmuRenderOptions {
  shouldDisplay?: () => boolean;
  buildCommentOptions?: (msg: DanmakuMessage) => {
    duration?: number;
    mode?: "scroll" | "top" | "bottom";
    style?: Record<string, any>;
  };
}

export interface RustGetStreamUrlPayload {
  args: {
    room_id_str: string;
  };
  platform: Platform;
}
