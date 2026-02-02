import { ref } from "vue";

export function useHeaderScroll(threshold = 80) {
  const isHeaderCollapsed = ref(false);
  const isScrolling = ref(false);
  let scrollTimer: number | null = null;
  let rafId: number | null = null;

  const handleScroll = (e: Event) => {
    const target = e.target as HTMLElement;
    
    // Throttling updates using rAF
    if (!rafId) {
      rafId = window.requestAnimationFrame(() => {
        // Only update if value actually changes to avoid triggering watchers
        const newState = target.scrollTop > threshold;
        if (isHeaderCollapsed.value !== newState) {
          isHeaderCollapsed.value = newState;
        }
        rafId = null;
      });
    }

    if (!isScrolling.value) isScrolling.value = true;
    
    if (scrollTimer) clearTimeout(scrollTimer);
    scrollTimer = window.setTimeout(() => {
      isScrolling.value = false;
    }, 150);
  };

  return {
    isHeaderCollapsed,
    isScrolling,
    handleScroll,
  };
}
