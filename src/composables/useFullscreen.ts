/**
 * Fullscreen API composable with proper cleanup
 *
 * @example
 * ```ts
 * const { isFullscreen, toggleFullscreen, enterFullscreen, exitFullscreen } = useFullscreen();
 *
 * // Toggle fullscreen
 * toggleFullscreen();
 * ```
 *
 * @returns {FullscreenReturn} Fullscreen state and control methods
 */

import { ref, onUnmounted } from 'vue';

/**
 * Fullscreen return interface
 */
export interface FullscreenReturn {
  /** Whether currently in fullscreen mode */
  isFullscreen: ReturnType<typeof ref<boolean>>;
  /** Toggle fullscreen mode */
  toggleFullscreen: () => void;
  /** Enter fullscreen mode */
  enterFullscreen: () => Promise<void>;
  /** Exit fullscreen mode */
  exitFullscreen: () => Promise<void>;
}

/**
 * Fullscreen state composable
 * Manages fullscreen state with proper event listeners and cleanup
 *
 * Handles the Fullscreen API with automatic event listener cleanup
 * to prevent memory leaks. Safe to use in SSR environments.
 *
 * @returns Fullscreen state and control methods
 */
export function useFullscreen(): FullscreenReturn {
  const isFullscreen = ref(false);

  /**
   * Handle fullscreen change event
   */
  const handleFullscreenChange = () => {
    isFullscreen.value = !!document.fullscreenElement;
  };

  /**
   * Enter fullscreen mode
   */
  async function enterFullscreen() {
    try {
      await document.documentElement.requestFullscreen();
    } catch (error) {
      console.warn('Failed to enter fullscreen:', error);
    }
  }

  /**
   * Exit fullscreen mode
   */
  async function exitFullscreen() {
    try {
      await document.exitFullscreen();
    } catch (error) {
      console.warn('Failed to exit fullscreen:', error);
    }
  }

  /**
   * Toggle fullscreen mode
   */
  function toggleFullscreen() {
    if (!document.fullscreenElement) {
      enterFullscreen();
    } else {
      exitFullscreen();
    }
  }

  // Set up event listeners
  if (typeof document !== 'undefined') {
    document.addEventListener('fullscreenchange', handleFullscreenChange);
  }

  // Cleanup event listeners on unmount
  onUnmounted(() => {
    if (typeof document !== 'undefined') {
      document.removeEventListener('fullscreenchange', handleFullscreenChange);
    }
  });

  return {
    isFullscreen,
    toggleFullscreen,
    enterFullscreen,
    exitFullscreen,
  };
}