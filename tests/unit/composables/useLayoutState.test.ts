/**
 * Unit tests for useLayoutState composable
 *
 * @vitest-environment jsdom
 */

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { ref } from 'vue';
import { useLayoutState } from '@/composables/useLayoutState';

describe('useLayoutState', () => {
  let localStorageMock: Record<string, string>;

  beforeEach(() => {
    // Mock localStorage
    localStorageMock = {};
    vi.stubGlobal('localStorage', {
      getItem: (key: string) => localStorageMock[key] ?? null,
      setItem: (key: string, value: string) => {
        localStorageMock[key] = value;
      },
      removeItem: (key: string) => {
        delete localStorageMock[key];
      },
      clear: () => {
        localStorageMock = {};
      },
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    localStorage.clear();
  });

  describe('initial state', () => {
    it('should have default collapse state as false', () => {
      const { isCollapse } = useLayoutState();
      expect(isCollapse.value).toBe(false);
    });

    it('should have default dark theme state as false', () => {
      const { isDark } = useLayoutState();
      expect(isDark.value).toBe(false);
    });
  });

  describe('toggleCollapse', () => {
    it('should toggle collapse state', () => {
      const { isCollapse, toggleCollapse } = useLayoutState();

      expect(isCollapse.value).toBe(false);

      toggleCollapse();
      expect(isCollapse.value).toBe(true);

      toggleCollapse();
      expect(isCollapse.value).toBe(false);
    });

    it('should persist collapse state to localStorage', async () => {
      const { isCollapse, toggleCollapse } = useLayoutState();

      toggleCollapse();
      await new Promise(resolve => setTimeout(resolve, 0)); // Wait for watcher

      expect(localStorageMock['layout:sidebar-collapse']).toBe('true');
    });
  });

  describe('setCollapse', () => {
    it('should set collapse state to specific value', () => {
      const { isCollapse, setCollapse } = useLayoutState();

      setCollapse(true);
      expect(isCollapse.value).toBe(true);

      setCollapse(false);
      expect(isCollapse.value).toBe(false);
    });
  });

  describe('toggleTheme', () => {
    it('should toggle dark theme state', () => {
      const { isDark, toggleTheme } = useLayoutState();

      expect(isDark.value).toBe(false);

      toggleTheme();
      expect(isDark.value).toBe(true);

      toggleTheme();
      expect(isDark.value).toBe(false);
    });

    it('should persist theme state to localStorage', async () => {
      const { isDark, toggleTheme } = useLayoutState();

      toggleTheme();
      await new Promise(resolve => setTimeout(resolve, 0)); // Wait for watcher

      expect(localStorageMock['layout:theme-dark']).toBe('true');
    });
  });

  describe('localStorage persistence', () => {
    it('should load saved collapse state from localStorage', () => {
      localStorageMock['layout:sidebar-collapse'] = 'true';
      const { isCollapse } = useLayoutState();
      
      // Need to wait for onMounted
      return new Promise(resolve => {
        setTimeout(() => {
          expect(isCollapse.value).toBe(true);
          resolve(null);
        }, 0);
      });
    });

    it('should load saved theme state from localStorage', () => {
      localStorageMock['layout:theme-dark'] = 'true';
      const { isDark } = useLayoutState();
      
      return new Promise(resolve => {
        setTimeout(() => {
          expect(isDark.value).toBe(true);
          resolve(null);
        }, 0);
      });
    });

    it('should handle invalid localStorage data gracefully', () => {
      localStorageMock['layout:sidebar-collapse'] = 'invalid-json';
      const { isCollapse } = useLayoutState();
      
      // Should still work, just with default value
      expect(isCollapse.value).toBe(false);
    });
  });
});
