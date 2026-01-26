import { defineStore } from "pinia";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow"; // Changed import to WebviewWindow

interface ThemeState {
  userPreference: "dark";
  primaryColor: string;
}

const COLOR_KEY = "primary_color";

export const useThemeStore = defineStore("theme", {
  state: (): ThemeState => ({
    userPreference: "dark", 
    primaryColor: "#be8e26", // Default Gold
  }),
  actions: {
    async initTheme() {
      let storedColor = localStorage.getItem(COLOR_KEY);

      // Migration: Clear the old default purple if it was stored
      if (storedColor === "#a855f7") {
        localStorage.removeItem(COLOR_KEY);
        storedColor = null;
      }

      if (storedColor) {
        this.primaryColor = storedColor;
        this._applyTheme();
      } else {
        // If no stored color, don't overwrite CSS variable.
        // Just ensure other theme-related attributes are set.
        document.documentElement.setAttribute("data-theme", "dark");
        try {
          const win = WebviewWindow.getCurrent(); 
          await win.setTheme("dark");
        } catch (error) {
          console.error("[ThemeStore] Error setting Tauri window theme:", error);
        }
      }
    },

    setPrimaryColor(color: string) {
      this.primaryColor = color;
      localStorage.setItem(COLOR_KEY, color);
      this._applyTheme();
    },

    // ... (rest of random color logic)

    async _applyTheme() {
      const themeToApply = "dark";
      document.documentElement.setAttribute("data-theme", themeToApply);
      document.documentElement.style.setProperty("--accent", this.primaryColor);

      try {
        const win = WebviewWindow.getCurrent(); 
        await win.setTheme(themeToApply);
      } catch (error) {
        console.error("[ThemeStore] Error setting Tauri window theme:", error);
      }
    },

    getEffectiveTheme(): "dark" {
      return "dark";
    },
  },
});
