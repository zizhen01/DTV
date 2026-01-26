import { Platform as StreamingPlatform } from "../../types/app/platform";

export interface PlayerProps {
  roomId: string | null;
  platform: StreamingPlatform;
  isFollowed?: boolean;
  streamUrl?: string | null;
  title?: string | null;
  anchorName?: string | null;
  avatar?: string | null;
  isLive?: boolean | null;
  initialError?: string | null;
  cookie?: string | null;
}

export interface LineOption {
  key: string;
  label: string;
}
