/**
 * Layout state management composable
 * Provides sidebar collapse state, theme state with localStorage persistence
 *
 * @example
 * ```ts
 * const { isCollapse, toggleCollapse, isDark, toggleTheme } = useLayoutState();
 * ```
 *
 * @returns {LayoutStateReturn} Layout state and actions
 */
import { ref, watch, onMounted } from 'vue';
import type { LayoutConfig } from '@/types/layout';

const STORAGE_KEYS = {
  SIDEBAR_COLLAPSE: 'layout:sidebar-collapse',
  THEME_DARK: 'layout:theme-dark',
} as const;

/**
 * Layout state return interface
 */
export interface LayoutStateReturn {
  /** Sidebar collapse state */
  isCollapse: ReturnType<typeof ref<boolean>>;
  /** Dark theme state */
  isDark: ReturnType<typeof ref<boolean>>;
  /** Toggle sidebar collapse */
  toggleCollapse: () => void;
  /** Toggle theme */
  toggleTheme: () => void;
  /** Set sidebar collapse state */
  setCollapse: (value: boolean) => void;
  /** Set theme state */
  setTheme: (value: boolean) => void;
}

/**
 * Default layout configuration
 */
const DEFAULT_CONFIG: Required<LayoutConfig> = {
  defaultCollapsed: false,
  defaultDark: false,
  sidebarWidth: '240px',
  headerHeight: '64px',
  enablePersistence: true,
};

/**
 * Layout state composable
 * Manages global layout state with persistence
 *
 * @param config - Layout configuration options
 * @returns Layout state and control methods
 */
export function useLayoutState(config?: LayoutConfig): LayoutStateReturn {
  // Merge with default config
  const finalConfig = { ...DEFAULT_CONFIG, ...config };

  // Sidebar collapse state
  const isCollapse = ref(finalConfig.defaultCollapsed);

  // Theme state
  const isDark = ref(finalConfig.defaultDark);

  // Load state from localStorage on mount (if persistence enabled)
  onMounted(() => {
    if (!finalConfig.enablePersistence) return;

    try {
      const savedCollapse = localStorage.getItem(STORAGE_KEYS.SIDEBAR_COLLAPSE);
      if (savedCollapse !== null) {
        isCollapse.value = JSON.parse(savedCollapse);
      }

      const savedTheme = localStorage.getItem(STORAGE_KEYS.THEME_DARK);
      if (savedTheme !== null) {
        isDark.value = JSON.parse(savedTheme);
      }
    } catch (error) {
      console.warn('Failed to load layout state from localStorage:', error);
    }
  });

  // Persist collapse state to localStorage
  watch(
    isCollapse,
    (value) => {
      if (!finalConfig.enablePersistence) return;
      try {
        localStorage.setItem(STORAGE_KEYS.SIDEBAR_COLLAPSE, JSON.stringify(value));
      } catch (error) {
        console.warn('Failed to persist collapse state:', error);
      }
    },
    { immediate: false }
  );

  // Persist theme state to localStorage
  watch(
    isDark,
    (value) => {
      if (!finalConfig.enablePersistence) return;
      try {
        localStorage.setItem(STORAGE_KEYS.THEME_DARK, JSON.stringify(value));
      } catch (error) {
        console.warn('Failed to persist theme state:', error);
      }
    },
    { immediate: false }
  );

  /**
   * Toggle sidebar collapse state
   */
  function toggleCollapse() {
    isCollapse.value = !isCollapse.value;
  }

  /**
   * Toggle theme state
   */
  function toggleTheme() {
    isDark.value = !isDark.value;
  }

  /**
   * Set sidebar collapse state
   */
  function setCollapse(value: boolean) {
    isCollapse.value = value;
  }

  /**
   * Set theme state
   */
  function setTheme(value: boolean) {
    isDark.value = value;
  }

  return {
    // State
    isCollapse,
    isDark,
    // Actions
    toggleCollapse,
    toggleTheme,
    setCollapse,
    setTheme,
  };
}

/**
 * Singleton instance for global state sharing
 */
let globalLayoutState: ReturnType<typeof useLayoutState> | null = null;

/**
 * Get global layout state instance
 * Use this to share layout state across multiple components
 */
export function useGlobalLayoutState() {
  if (!globalLayoutState) {
    globalLayoutState = useLayoutState();
  }
  return globalLayoutState;
}