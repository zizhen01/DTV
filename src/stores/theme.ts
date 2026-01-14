import { defineStore } from "pinia";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow"; // Changed import to WebviewWindow

interface ThemeState {
  userPreference: "light" | "dark" | "system";
  currentSystemTheme: "light" | "dark";
  primaryColor: string;
}

const STORE_KEY = "theme_preference";
const COLOR_KEY = "primary_color";

export const useThemeStore = defineStore("theme", {
  state: (): ThemeState => ({
    userPreference: "system", // Default to 'system'
    currentSystemTheme: "light", // Will be updated
    primaryColor: "#a855f7", // Default purple
  }),
  actions: {
    async initTheme() {
      // Load user preference from localStorage
      const storedPreference = localStorage.getItem(STORE_KEY);
      if (
        storedPreference &&
        ["light", "dark", "system"].includes(storedPreference)
      ) {
        this.userPreference = storedPreference as ThemeState["userPreference"];
      } else {
        this.userPreference = "system"; // Default if nothing stored or invalid
      }

      const storedColor = localStorage.getItem(COLOR_KEY);
      if (storedColor) {
        this.primaryColor = storedColor;
      }

      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      this.currentSystemTheme = mediaQuery.matches ? "dark" : "light";

      mediaQuery.addEventListener("change", (e) => {
        this.currentSystemTheme = e.matches ? "dark" : "light";
        this._applyTheme();
      });

      this._applyTheme();
    },

    setUserPreference(preference: "light" | "dark" | "system") {
      if (this.userPreference === preference) return;
      this.userPreference = preference;
      localStorage.setItem(STORE_KEY, preference);
      this._applyTheme();
    },

    setPrimaryColor(color: string) {
      this.primaryColor = color;
      localStorage.setItem(COLOR_KEY, color);
      this._applyTheme();
    },

    setRandomPrimaryColor() {
      const colors = [
        "#ef4444", // red
        "#f97316", // orange
        "#f59e0b", // amber
        "#10b981", // emerald
        "#06b6d4", // cyan
        "#3b82f6", // blue
        "#6366f1", // indigo
        "#8b5cf6", // violet
        "#a855f7", // purple
        "#d946ef", // fuchsia
        "#ec4899", // pink
      ];
      const randomColor = colors[Math.floor(Math.random() * colors.length)];
      this.setPrimaryColor(randomColor);
    },

    async _applyTheme() {
      let themeToApply: "light" | "dark";
      if (this.userPreference === "system") {
        themeToApply = this.currentSystemTheme;
      } else {
        themeToApply = this.userPreference;
      }
      document.documentElement.setAttribute("data-theme", themeToApply);
      document.documentElement.style.setProperty("--accent", this.primaryColor);

      try {
        const win = WebviewWindow.getCurrent(); // Using WebviewWindow.getCurrent()
        await win.setTheme(themeToApply);
      } catch (error) {
        console.error("[ThemeStore] Error setting Tauri window theme:", error);
      }
    },

    // Getter to easily access the currently active theme in components
    getEffectiveTheme(): "light" | "dark" {
      if (this.userPreference === "system") {
        return this.currentSystemTheme;
      }
      return this.userPreference;
    },
  },
});
