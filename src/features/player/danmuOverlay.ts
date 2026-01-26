import Artplayer from "artplayer";
import Danmaku from "danmaku/dist/esm/danmaku.canvas.js";
import type { DanmakuComment } from "danmaku";
import { sanitizeDanmuArea, sanitizeDanmuOpacity } from "./constants";
import type { DanmuOverlayInstance } from "../../types/models/danmaku";
import type { DanmuUserSettings } from "./constants";

class DanmakuWrapper implements DanmuOverlayInstance {
  private instance: Danmaku;
  private container: HTMLElement;
  private _font_size: number;
  
  constructor(instance: Danmaku, container: HTMLElement, initialSettings: DanmuUserSettings) {
    this.instance = instance;
    this.container = container;
    this._font_size = parseInt(initialSettings.fontSize, 10);
    // _duration, _area, _opacity are applied via other means or ignored for now
    sanitizeDanmuOpacity(initialSettings.opacity); // check but ignore return
  }

  emit(comment: DanmakuComment) {
    // Inject global preferences into each comment
    if (!comment.style) comment.style = {};
    const engine = (this.instance as any).engine;
    
    if (engine === 'dom') {
       // DOM engine might need explicit pixel sizes
       (comment.style as any).fontSize = `${this._font_size}px`;
       (comment.style as any).color = (comment.style as any).color || '#ffffff';
    } else {
       // Canvas engine uses font string "20px sans-serif"
       (comment.style as any).fontSize = `${this._font_size}px`;
       (comment.style as any).font = `900 ${this._font_size}px "OPPO Sans", sans-serif`;
       (comment.style as any).fillStyle = (comment.style as any).color || '#ffffff';
       (comment.style as any).strokeStyle = this.container.style.getPropertyValue("--danmu-stroke-color") || "#444444";
       (comment.style as any).lineWidth = 1; // Stroke width
    }
    
    this.instance.emit(comment);
  }

  play() { this.instance.play(); }
  pause() { this.instance.pause(); }
  clear() { this.instance.clear(); }
  resize() { this.instance.resize(); }
  show() { this.instance.show(); }
  hide() { this.instance.hide(); }

  _setOpacity(opacity: number) {
    this.container.style.opacity = String(opacity);
  }

  _setFontSize(size: number) {
    this._font_size = size;
  }
  
  _setArea(_area: number) {
    // Area not supported directly on instance update yet
  }

  _setDuration(_duration: number) {
    // Duration not supported directly on instance update yet
  }
}

export const ensureDanmuOverlayHost = (
  player: Artplayer,
): HTMLElement | null => {
  const root = player.template?.$player as HTMLElement | undefined;
  if (!root) return null;

  let host = root.querySelector(".player-danmu-overlay") as HTMLElement | null;
  if (!host) {
    host = document.createElement("div");
    host.className = "player-danmu-overlay will-change-transform pointer-events-none";
  }

  Object.assign(host.style, {
    position: "absolute",
    inset: "0",
    zIndex: "30",
    transition: "opacity 0.2s ease",
    opacity: "var(--danmu-opacity, 1)",
    transform: "translateZ(0)",
    backfaceVisibility: "hidden"
  });

  if (host.parentElement !== root) {
    const video = player.template.$video;
    if (video && video.nextSibling) {
      root.insertBefore(host, video.nextSibling);
    } else {
      root.appendChild(host);
    }
  }

  return host;
};

const ensureDanmuOverlayStyle = (host: HTMLElement) => {
  const existing = host.querySelector("style[data-danmu-overlay]");
  if (existing) return existing;

  const styleEl = document.createElement("style");
  styleEl.setAttribute("data-danmu-overlay", "true");
  
  styleEl.textContent = `
.player-danmu-overlay span,
.player-danmu-overlay div {
  font-weight: 900;
  will-change: transform;
  /* Stroke simulation for DOM mode if needed */
  -webkit-text-stroke: 1px var(--danmu-stroke-color, #444);
  text-shadow: none !important; 
  font-family: var(--danmu-font-family, "OPPO Sans", sans-serif);
  contain: strict;
}
canvas {
  display: block;
}
  `.trim();

  host.appendChild(styleEl);
  return styleEl;
};

export const applyDanmuOverlayPreferences = (
  overlay: DanmuOverlayInstance | null,
  danmuSettings: DanmuUserSettings,
  isDanmuEnabled: boolean,
  playerRoot?: HTMLElement | null,
) => {
  if (!overlay) return;

  const host = playerRoot?.querySelector(".player-danmu-overlay") as HTMLElement | null;
  if (!host) return;

  const normalizedOpacity = sanitizeDanmuOpacity(danmuSettings.opacity);
  const nextOpacity = isDanmuEnabled ? normalizedOpacity : 0;

  try {
    if (isDanmuEnabled) {
      overlay.play();
      overlay.show();
      
      const fontSizeValue = parseInt(danmuSettings.fontSize, 10);
      if (!Number.isNaN(fontSizeValue)) overlay._setFontSize?.(fontSizeValue);
      
      const areaValue = sanitizeDanmuArea(danmuSettings.area);
      overlay._setArea?.(areaValue);

      const duration = danmuSettings.duration;
      overlay._setDuration?.(duration);

    } else {
      overlay.pause();
    }
    
    overlay._setOpacity?.(nextOpacity);
  } catch (error) {
    console.warn("[Player] Failed to sync danmu state/prefs:", error);
  }

  requestAnimationFrame(() => {
    host.style.setProperty("--danmu-opacity", String(nextOpacity));
    host.style.setProperty("--danmu-stroke-color", danmuSettings.strokeColor);
  });
};

export const syncDanmuEnabledState = (
  overlay: DanmuOverlayInstance | null,
  danmuSettings: DanmuUserSettings,
  isDanmuEnabled: boolean,
  playerRoot?: HTMLElement | null,
) => {
    applyDanmuOverlayPreferences(overlay, danmuSettings, isDanmuEnabled, playerRoot);
};

export const createDanmuOverlay = async (
  player: Artplayer | null,
  danmuSettings: DanmuUserSettings,
  isDanmuEnabled: boolean,
): Promise<DanmuOverlayInstance | null> => {
  if (!player) return null;

  const overlayHost = ensureDanmuOverlayHost(player);
  if (!overlayHost) return null;

  ensureDanmuOverlayStyle(overlayHost);
  
  if (overlayHost.children.length > 1) {
    const style = overlayHost.querySelector("style[data-danmu-overlay]");
    if (style) {
        // Clear everything except our style
        Array.from(overlayHost.children).forEach(child => {
            if (child !== style) overlayHost.removeChild(child);
        });
    }
  }

  try {
    // Instantiate 'danmaku' library (Full version)
    const danmaku = new Danmaku({
      container: overlayHost,
      engine: 'canvas', // Default to canvas for performance
      speed: 144 // Default speed, can be overridden by comment duration simulation
    });

    const wrapper = new DanmakuWrapper(danmaku, overlayHost, danmuSettings);
    
    // Initial sync
    setTimeout(() => {
        applyDanmuOverlayPreferences(
            wrapper, 
            danmuSettings, 
            isDanmuEnabled, 
            player.template.$player as HTMLElement
        );
        wrapper.resize();
    }, 100);
    
    return wrapper;
  } catch (error) {
    console.error("[Player] Failed to initialize danmaku overlay:", error);
    return null;
  }
};
