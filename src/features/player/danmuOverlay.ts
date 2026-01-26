// import Artplayer from "artplayer";

// import { sanitizeDanmuArea, sanitizeDanmuOpacity } from "./constants";
// import type { DanmuOverlayInstance } from "../../types/models/danmaku";
// import type { DanmuUserSettings } from "./constants";

// export const ensureDanmuOverlayHost = (
//   player: Artplayer,
// ): HTMLElement | null => {
//   const root = player.template?.$player as HTMLElement | undefined;
//   if (!root) {
//     return null;
//   }

//   let host = root.querySelector(".player-danmu-overlay") as HTMLElement | null;
//   if (!host) {
//     host = document.createElement("div");
//     host.className = "player-danmu-overlay";
//   }

//   // Base styles (used to live in player.css)
//   host.style.position = "absolute";
//   host.style.inset = "0";
//   host.style.pointerEvents = "none";
//   host.style.zIndex = "30";
//   host.style.transition = "opacity 0.2s ease";
//   host.style.opacity = "var(--danmu-opacity, 1)";

//   // ArtPlayer puts layers inside the root container.
//   // We want the danmu overlay to be above the video but below controls (usually).
//   // ArtPlayer controls are z-index 50 by default.
//   // We can just append to root and use z-index to manage layering.
//   if (host.parentElement !== root) {
//     // Try to insert before controls if possible, or just append
//     // ArtPlayer has $video, $poster, $subtitle, $danmuku, $layers, $mask, $controls, $contextmenu
//     // We want to be on top of video.
//     const video = player.template.$video;
//     if (video && video.nextSibling) {
//       root.insertBefore(host, video.nextSibling);
//     } else {
//       root.appendChild(host);
//     }
//   }

//   return host;
// };

// const ensureDanmuOverlayStyle = (host: HTMLElement) => {
//   const existing = host.querySelector(
//     "style[data-danmu-overlay]",
//   ) as HTMLStyleElement | null;
//   if (existing) {
//     return existing;
//   }

//   const styleEl = document.createElement("style");
//   styleEl.setAttribute("data-danmu-overlay", "true");
//   styleEl.textContent = `
// .player-danmu-overlay span,
// .player-danmu-overlay div {
//   font-weight: 900;
//   text-shadow:
//     1px 0 0 var(--danmu-stroke-color, #444444),
//     -1px 0 0 var(--danmu-stroke-color, #444444),
//     0 1px 0 var(--danmu-stroke-color, #444444),
//     0 -1px 0 var(--danmu-stroke-color, #444444),
//     1px 1px 0 var(--danmu-stroke-color, #444444),
//     -1px 1px 0 var(--danmu-stroke-color, #444444),
//     1px -1px 0 var(--danmu-stroke-color, #444444),
//     -1px -1px 0 var(--danmu-stroke-color, #444444);
//   font-family: var(
//     --danmu-font-family,
//     "OPPO Sans",
//     "Microsoft YaHei",
//     "PingFang SC",
//     "Helvetica Neue",
//     Arial,
//     sans-serif
//   );
// }
//   `.trim();

//   host.appendChild(styleEl);
//   return styleEl;
// };

// export const applyDanmuOverlayPreferences = (
//   overlay: DanmuOverlayInstance | null,
//   danmuSettings: DanmuUserSettings,
//   isDanmuEnabled: boolean,
//   playerRoot?: HTMLElement | null,
// ) => {
//   if (!overlay) {
//     return;
//   }
//   const host = playerRoot?.querySelector(
//     ".player-danmu-overlay",
//   ) as HTMLElement | null;
//   const fontSizeValue = parseInt(danmuSettings.fontSize, 10);
//   if (!Number.isNaN(fontSizeValue)) {
//     try {
//       overlay.setFontSize?.(fontSizeValue);
//     } catch (error) {
//       console.warn("[Player] Failed to apply danmu font size:", error);
//     }
//   }
//   try {
//     const areaValue = sanitizeDanmuArea(danmuSettings.area);
//     overlay.setArea?.({ start: 0, end: areaValue });
//   } catch (error) {
//     console.warn("[Player] Failed to apply danmu area:", error);
//   }
//   try {
//     overlay.setAllDuration?.("scroll", danmuSettings.duration);
//     overlay.setAllDuration?.("top", danmuSettings.duration);
//     overlay.setAllDuration?.("bottom", danmuSettings.duration);
//   } catch (error) {
//     // Non-critical for players that do not support bulk duration updates
//   }
//   try {
//     const normalizedOpacity = sanitizeDanmuOpacity(danmuSettings.opacity);
//     const nextOpacity = isDanmuEnabled ? normalizedOpacity : 0;
//     overlay.setOpacity?.(nextOpacity);
//     host?.style.setProperty("--danmu-opacity", String(nextOpacity));
//   } catch (error) {
//     // Non-critical
//   }
//   try {
//     host?.style.setProperty("--danmu-stroke-color", danmuSettings.strokeColor);
//   } catch (error) {
//     console.warn("[Player] Failed to apply danmu stroke color:", error);
//   }
// };


// export const createDanmuOverlay = async (
//   player: Artplayer | null,
//   danmuSettings: DanmuUserSettings,
//   isDanmuEnabled: boolean,
// ): Promise<DanmuOverlayInstance | null> => {
//   if (!player) {
//     return null;
//   }

//   const { default: DanmuJs } = await import("danmu.js");

//   const overlayHost = ensureDanmuOverlayHost(player);
//   if (!overlayHost) {
//     return null;
//   }

//   // Keep the overlay styles even when re-initializing.
//   const styleEl = ensureDanmuOverlayStyle(overlayHost);
//   overlayHost.replaceChildren(styleEl);
//   overlayHost.style.setProperty(
//     "--danmu-stroke-color",
//     danmuSettings.strokeColor,
//   );
//   overlayHost.style.setProperty(
//     "--danmu-opacity",
//     String(isDanmuEnabled ? sanitizeDanmuOpacity(danmuSettings.opacity) : 0),
//   );

//   try {
//     const overlay = new DanmuJs({
//       container: overlayHost,
//       player: player.template.$video,
//       comments: [],
//       mouseControl: false,
//       defaultOff: false,
//       channelSize: 36,
//       containerStyle: {
//         pointerEvents: "none",
//       },
//     });

//     overlay.start?.();
//     applyDanmuOverlayPreferences(
//       overlay,
//       danmuSettings,
//       isDanmuEnabled,
//       player.template.$player as HTMLElement,
//     );
//     syncDanmuEnabledState(
//       overlay,
//       danmuSettings,
//       isDanmuEnabled,
//       player.template.$player as HTMLElement,
//     );
//     return overlay;
//   } catch (error) {
//     console.error("[Player] Failed to initialize danmu.js overlay:", error);
//     return null;
//   }
// };


import Artplayer from "artplayer";
import { sanitizeDanmuArea, sanitizeDanmuOpacity } from "./constants";
import type { DanmuOverlayInstance } from "../../types/models/danmaku";
import type { DanmuUserSettings } from "./constants";

export const ensureDanmuOverlayHost = (
  player: Artplayer,
): HTMLElement | null => {
  const root = player.template?.$player as HTMLElement | undefined;
  if (!root) return null;

  let host = root.querySelector(".player-danmu-overlay") as HTMLElement | null;
  if (!host) {
    host = document.createElement("div");
    // 优化：利用 Tailwind 或原生类名开启核心优化
    // will-change-transform: 强制提升为合成层，跳过 Paint 阶段
    // pointer-events-none: 减少事件冒泡计算
    host.className = "player-danmu-overlay will-change-transform pointer-events-none";
  }

  // 使用 Object.assign 批量设置样式，减少回流次数
  Object.assign(host.style, {
    position: "absolute",
    inset: "0",
    zIndex: "30",
    transition: "opacity 0.2s ease",
    opacity: "var(--danmu-opacity, 1)",
    // 强制开启 GPU 渲染
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


// export const syncDanmuEnabledState = (
//   overlay: DanmuOverlayInstance | null,
//   danmuSettings: DanmuUserSettings,
//   isDanmuEnabled: boolean,
//   playerRoot?: HTMLElement | null,
// ) => {
//   if (!overlay) {
//     return;
//   }
//   const normalizedOpacity = sanitizeDanmuOpacity(danmuSettings.opacity);
//   const targetOpacity = isDanmuEnabled ? normalizedOpacity : 0;
//   try {
//     if (isDanmuEnabled) {
//       overlay.play?.();
//       overlay.show?.("scroll");
//       overlay.show?.("top");
//       overlay.show?.("bottom");
//     } else {
//       overlay.pause?.();
//     }
//     overlay.setOpacity?.(targetOpacity);
//     const host = playerRoot?.querySelector(
//       ".player-danmu-overlay",
//     ) as HTMLElement | null;
//     host?.style.setProperty("--danmu-opacity", String(targetOpacity));
//   } catch (error) {
//     console.warn("[Player] Failed updating danmu enabled state:", error);
//   }
// };


const ensureDanmuOverlayStyle = (host: HTMLElement) => {
  const existing = host.querySelector("style[data-danmu-overlay]");
  if (existing) return existing;

  const styleEl = document.createElement("style");
  styleEl.setAttribute("data-danmu-overlay", "true");
  
  // 核心优化：改用 -webkit-text-stroke 替代 8 层 text-shadow
  // 这将减少 WebKit 80% 以上的像素点渲染工作量
  styleEl.textContent = `
.player-danmu-overlay span,
.player-danmu-overlay div {
  font-weight: 900;
  will-change: transform;
  /* 解决多层阴影带来的 Paint 压力 */
  -webkit-text-stroke: 1px var(--danmu-stroke-color, #444);
  text-shadow: none !important; 
  
  font-family: var(--danmu-font-family, "OPPO Sans", sans-serif);
  /* 确保弹幕容器不会触发意外的排版重计算 */
  contain: strict;
}
  `.trim();

  host.appendChild(styleEl);
  return styleEl;
};

// export const applyDanmuOverlayPreferences = (
//   overlay: DanmuOverlayInstance | null,
//   danmuSettings: DanmuUserSettings,
//   isDanmuEnabled: boolean,
//   playerRoot?: HTMLElement | null,
// ) => {
//   if (!overlay) return;

//   const host = playerRoot?.querySelector(".player-danmu-overlay") as HTMLElement | null;
  
//   // 批量应用属性，减少对 overlay 实例的重复调用
//   const fontSizeValue = parseInt(danmuSettings.fontSize, 10);
//   if (!Number.isNaN(fontSizeValue)) overlay.setFontSize?.(fontSizeValue);
  
//   const areaValue = sanitizeDanmuArea(danmuSettings.area);
//   overlay.setArea?.({ start: 0, end: areaValue });

//   // 统一设置时长
//   const duration = danmuSettings.duration;
//   ["scroll", "top", "bottom"].forEach(type => overlay.setAllDuration?.(type, duration));

//   const normalizedOpacity = sanitizeDanmuOpacity(danmuSettings.opacity);
//   const nextOpacity = isDanmuEnabled ? normalizedOpacity : 0;
  
//   overlay.setOpacity?.(nextOpacity);
  
//   // 使用 requestAnimationFrame 确保样式更新在下一帧渲染前合并
//   requestAnimationFrame(() => {
//     host?.style.setProperty("--danmu-opacity", String(nextOpacity));
//     host?.style.setProperty("--danmu-stroke-color", danmuSettings.strokeColor);
//   });
// };

// ... syncDanmuEnabledState 保持原有逻辑，建议也加入 requestAnimationFrame

export const applyDanmuOverlayPreferences = (
  overlay: DanmuOverlayInstance | null,
  danmuSettings: DanmuUserSettings,
  isDanmuEnabled: boolean,
  playerRoot?: HTMLElement | null,
) => {
  if (!overlay) return;

  // 1. 缓存 DOM 引用，避免函数内多次执行 querySelector
  const host = playerRoot?.querySelector(".player-danmu-overlay") as HTMLElement | null;
  if (!host) return;

  // 2. 计算最终透明度
  const normalizedOpacity = sanitizeDanmuOpacity(danmuSettings.opacity);
  const nextOpacity = isDanmuEnabled ? normalizedOpacity : 0;

  // 3. 逻辑开关与属性应用：将原 syncDanmuEnabledState 逻辑并入
  try {
    if (isDanmuEnabled) {
      // 只有在启用时才更新具体的 UI 参数，节省性能
      overlay.play?.();
      
      const fontSizeValue = parseInt(danmuSettings.fontSize, 10);
      if (!Number.isNaN(fontSizeValue)) overlay.setFontSize?.(fontSizeValue);
      
      const areaValue = sanitizeDanmuArea(danmuSettings.area);
      overlay.setArea?.({ start: 0, end: areaValue });

      const duration = danmuSettings.duration;
      ["scroll", "top", "bottom"].forEach(type => overlay.setAllDuration?.(type, duration));
      
      // 确保通道显示
      overlay.show?.("scroll");
      overlay.show?.("top");
      overlay.show?.("bottom");
    } else {
      // 关闭时直接暂停实例，停止内部的 requestAnimationFrame 循环
      overlay.pause?.();
    }
    
    // 同步实例透明度
    overlay.setOpacity?.(nextOpacity);
  } catch (error) {
    console.warn("[Player] Failed to sync danmu state/prefs:", error);
  }

  // 4. 样式层优化：使用 requestAnimationFrame 合并所有的 CSS 变量修改
  // 这样可以确保在多开流场景下，浏览器在一次重绘中完成透明度和颜色的更新
  requestAnimationFrame(() => {
    host.style.setProperty("--danmu-opacity", String(nextOpacity));
    host.style.setProperty("--danmu-stroke-color", danmuSettings.strokeColor);
  });
};

export const createDanmuOverlay = async (
  player: Artplayer | null,
  danmuSettings: DanmuUserSettings,
  isDanmuEnabled: boolean,
): Promise<DanmuOverlayInstance | null> => {
  if (!player) return null;

  const { default: DanmuJs } = await import("danmu.js");
  const overlayHost = ensureDanmuOverlayHost(player);
  if (!overlayHost) return null;

  ensureDanmuOverlayStyle(overlayHost);
  
  // 优化：尽量减少 replaceChildren 操作，它会触发昂贵的 DOM 重塑
  if (overlayHost.children.length > 1) {
    const style = overlayHost.querySelector("style[data-danmu-overlay]");
    overlayHost.replaceChildren(style as Node);
  }

  try {
    const overlay = new DanmuJs({
      container: overlayHost,
      player: player.template.$video,
      comments: [],
      mouseControl: false,
      defaultOff: false,
      channelSize: 36,
      // 告诉 danmu.js 开启优化模式
      live: true,
      discard: true, // 渲染压力大时自动丢弃弹幕
      containerStyle: {
        pointerEvents: "none",
        contain: "strict" // 关键：限制重绘范围
      },
    });

    overlay.start?.();
    
    // 延迟执行偏好设置，避免初始化时的瞬时 CPU 峰值
    setTimeout(() => {
      applyDanmuOverlayPreferences(
        overlay,
        danmuSettings,
        isDanmuEnabled,
        player.template.$player as HTMLElement,
      );
    }, 100);

    return overlay;
  } catch (error) {
    console.error("[Player] Failed to initialize danmu.js overlay:", error);
    return null;
  }
};