declare module 'danmaku/dist/esm/danmaku.canvas.js' {
  import Danmaku from 'danmaku';
  export default Danmaku;
}

declare module 'danmaku' {
  export interface DanmakuOptions {
    container: HTMLElement;
    media?: HTMLMediaElement;
    comments?: any[];
    engine?: 'dom' | 'canvas';
    speed?: number; // pixels per second (v2)
    duration?: number; // duration for a comment to cross the screen (in some versions)
  }

  export interface DanmakuComment {
    text: string;
    mode?: 'ltr' | 'rtl' | 'top' | 'bottom';
    time?: number;
    style?: Partial<CSSStyleDeclaration> | CanvasRenderingContext2D;
    render?: () => HTMLElement | HTMLCanvasElement;
  }

  export default class Danmaku {
    constructor(options: DanmakuOptions);
    emit(comment: DanmakuComment): void;
    play(): void;
    pause(): void;
    resize(): void;
    clear(): void;
    show(): void;
    hide(): void;
    
    // Properties
    speed: number;
    // ... others
  }
}
