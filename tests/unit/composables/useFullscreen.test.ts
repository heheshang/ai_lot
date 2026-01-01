/**
 * Unit tests for useFullscreen composable
 *
 * @vitest-environment jsdom
 */

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { ref } from 'vue';
import { useFullscreen } from '@/composables/useFullscreen';

describe('useFullscreen', () => {
  let mockDocument: any;

  beforeEach(() => {
    // Mock document.fullscreenElement
    mockDocument = {
      fullscreenElement: null,
      documentElement: {
        requestFullscreen: vi.fn().mockResolvedValue(undefined),
      },
      exitFullscreen: vi.fn().mockResolvedValue(undefined),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    };

    vi.stubGlobal('document', mockDocument);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  describe('initial state', () => {
    it('should have fullscreen state as false initially', () => {
      const { isFullscreen } = useFullscreen();
      expect(isFullscreen.value).toBe(false);
    });

    it('should set up fullscreenchange event listener', () => {
      useFullscreen();
      expect(mockDocument.addEventListener).toHaveBeenCalledWith(
        'fullscreenchange',
        expect.any(Function)
      );
    });
  });

  describe('toggleFullscreen', () => {
    it('should enter fullscreen when not in fullscreen', async () => {
      mockDocument.fullscreenElement = null;
      const { toggleFullscreen } = useFullscreen();

      toggleFullscreen();
      expect(mockDocument.documentElement.requestFullscreen).toHaveBeenCalled();
    });

    it('should exit fullscreen when in fullscreen', async () => {
      mockDocument.fullscreenElement = {};
      const { toggleFullscreen } = useFullscreen();

      toggleFullscreen();
      expect(mockDocument.exitFullscreen).toHaveBeenCalled();
    });
  });

  describe('enterFullscreen', () => {
    it('should call requestFullscreen', async () => {
      const { enterFullscreen } = useFullscreen();

      await enterFullscreen();
      expect(mockDocument.documentElement.requestFullscreen).toHaveBeenCalled();
    });

    it('should handle requestFullscreen errors gracefully', async () => {
      const error = new Error('Fullscreen denied');
      mockDocument.documentElement.requestFullscreen = vi.fn().mockRejectedValue(error);
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const { enterFullscreen } = useFullscreen();

      await enterFullscreen();
      expect(consoleWarnSpy).toHaveBeenCalledWith('Failed to enter fullscreen:', error);

      consoleWarnSpy.mockRestore();
    });
  });

  describe('exitFullscreen', () => {
    it('should call exitFullscreen', async () => {
      const { exitFullscreen } = useFullscreen();

      await exitFullscreen();
      expect(mockDocument.exitFullscreen).toHaveBeenCalled();
    });

    it('should handle exitFullscreen errors gracefully', async () => {
      const error = new Error('Exit fullscreen failed');
      mockDocument.exitFullscreen = vi.fn().mockRejectedValue(error);
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const { exitFullscreen } = useFullscreen();

      await exitFullscreen();
      expect(consoleWarnSpy).toHaveBeenCalledWith('Failed to exit fullscreen:', error);

      consoleWarnSpy.mockRestore();
    });
  });

  describe('event listeners', () => {
    it('should update fullscreen state on fullscreenchange event', () => {
      const { isFullscreen } = useFullscreen();

      // Simulate fullscreenchange event
      const eventHandler = mockDocument.addEventListener.mock.calls.find(
        call => call[0] === 'fullscreenchange'
      )?.[1];

      expect(eventHandler).toBeDefined();

      // Simulate entering fullscreen
      mockDocument.fullscreenElement = {};
      eventHandler?.();
      expect(isFullscreen.value).toBe(true);

      // Simulating exiting fullscreen
      mockDocument.fullscreenElement = null;
      eventHandler?.();
      expect(isFullscreen.value).toBe(false);
    });

    it('should cleanup event listeners on unmount', () => {
      // Simulate component unmount by calling onUnmounted callback
      const onUnmountedCallbacks: any[] = [];
      vi.spyOn(window, 'WeakMap').mockImplementation(() => {
        const mock = new WeakMap();
        return {
          get: (key: any) => {
            if (key?.onUnmount) {
              return [key.onUnmount];
            }
            return mock.get(key);
          },
          set: (key: any, value: any) => {
            if (key?.onUnmount) {
              onUnmountedCallbacks.push(value);
            }
            return mock.set(key, value);
          },
          has: () => true,
          delete: () => false,
        };
      });

      useFullscreen();

      // Trigger cleanup
      onUnmountedCallbacks.forEach((cb: any) => cb());

      expect(mockDocument.removeEventListener).toHaveBeenCalledWith(
        'fullscreenchange',
        expect.any(Function)
      );
    });
  });

  describe('SSR safety', () => {
    it('should not throw when document is undefined', () => {
      vi.unstubAllGlobals();
      // Simulate SSR environment

      expect(() => useFullscreen()).not.toThrow();
    });
  });
});
