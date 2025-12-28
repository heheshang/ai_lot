/**
 * Performance Utilities (P6-05)
 *
 * Provides utility functions and composables for performance optimization:
 * - debounce: Delay function execution
 * - throttle: Limit function execution rate
 * - useVirtualList: Virtual scrolling for large lists
 * - useInfiniteScroll: Infinite scroll loading
 */

import type { Ref } from 'vue'
import { ref, computed } from 'vue'

/**
 * Debounce function execution
 * Delays execution until after wait milliseconds have elapsed
 * since the last time the debounced function was invoked.
 *
 * @param fn - Function to debounce
 * @param delay - Delay in milliseconds
 * @returns Debounced function
 */
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function (this: any, ...args: Parameters<T>) {
    if (timeoutId) {
      clearTimeout(timeoutId)
    }

    timeoutId = setTimeout(() => {
      fn.apply(this, args)
      timeoutId = null
    }, delay)
  }
}

/**
 * Throttle function execution
 * Limits execution to once every wait milliseconds.
 *
 * @param fn - Function to throttle
 * @param interval - Minimum interval between executions in milliseconds
 * @returns Throttled function
 */
export function throttle<T extends (...args: any[]) => any>(
  fn: T,
  interval: number
): (...args: Parameters<T>) => void {
  let lastTime = 0
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function (this: any, ...args: Parameters<T>) {
    const now = Date.now()
    const timeSinceLastCall = now - lastTime

    if (timeSinceLastCall >= interval) {
      lastTime = now
      fn.apply(this, args)
    } else if (!timeoutId) {
      const remainingTime = interval - timeSinceLastCall
      timeoutId = setTimeout(() => {
        lastTime = Date.now()
        fn.apply(this, args)
        timeoutId = null
      }, remainingTime)
    }
  }
}

/**
 * Virtual list composable for rendering large lists efficiently
 *
 * @param items - Array of items to render
 * @param itemHeight - Height of each item in pixels
 * @param containerHeight - Height of the visible container in pixels
 * @returns Virtual list state and methods
 */
export function useVirtualList<T>(
  items: Ref<T[]>,
  itemHeight: number,
  containerHeight: number
) {
  const scrollTop = ref(0)
  const containerRef = ref<HTMLElement | null>(null)

  // Calculate which items are visible
  const visibleData = computed(() => {
    const start = Math.floor(scrollTop.value / itemHeight)
    const visibleCount = Math.ceil(containerHeight / itemHeight)
    const end = start + visibleCount

    // Add buffer items for smooth scrolling
    const bufferSize = Math.max(5, Math.floor(visibleCount * 0.5))
    const bufferStart = Math.max(0, start - bufferSize)
    const bufferEnd = Math.min(items.value.length, end + bufferSize)

    return {
      items: items.value.slice(bufferStart, bufferEnd),
      offset: bufferStart * itemHeight,
      totalHeight: items.value.length * itemHeight,
      startIndex: bufferStart,
      endIndex: bufferEnd,
    }
  })

  // Handle scroll events
  const handleScroll = throttle((event: Event) => {
    const target = event.target as HTMLElement
    scrollTop.value = target.scrollTop
  }, 16) // ~60fps

  // Scroll to specific item
  const scrollToItem = (index: number) => {
    if (containerRef.value) {
      const targetScrollTop = index * itemHeight
      containerRef.value.scrollTop = targetScrollTop
      scrollTop.value = targetScrollTop
    }
  }

  // Get current scroll position
  const scrollPercentage = computed(() => {
    if (visibleData.value.totalHeight === 0) return 0
    return (scrollTop.value / (visibleData.value.totalHeight - containerHeight)) * 100
  })

  return {
    containerRef,
    visibleData,
    handleScroll,
    scrollToItem,
    scrollPercentage,
    scrollTop,
  }
}

/**
 * Infinite scroll composable
 *
 * @param callback - Async function to load more data
 * @param threshold - Distance from bottom (in pixels) to trigger load
 * @returns Infinite scroll state and methods
 */
export function useInfiniteScroll(
  callback: () => Promise<void>,
  threshold = 200
) {
  const isLoading = ref(false)
  const hasMore = ref(true)
  const error = ref<Error | null>(null)
  const targetRef = ref<HTMLElement | null>(null)

  // Load more data
  const loadMore = async () => {
    if (isLoading.value || !hasMore.value) return

    isLoading.value = true
    error.value = null

    try {
      await callback()
    } catch (e) {
      error.value = e as Error
    } finally {
      isLoading.value = false
    }
  }

  // Handle scroll events
  const handleScroll = throttle(() => {
    if (!targetRef.value || isLoading.value || !hasMore.value) return

    const { scrollTop, scrollHeight, clientHeight } = targetRef.value
    const distanceFromBottom = scrollHeight - scrollTop - clientHeight

    if (distanceFromBottom <= threshold) {
      loadMore()
    }
  }, 100)

  // Reset state (e.g., for search/filter changes)
  const reset = () => {
    hasMore.value = true
    error.value = null
    isLoading.value = false
  }

  // Set hasMore to false when all data is loaded
  const setEnd = () => {
    hasMore.value = false
  }

  return {
    targetRef,
    isLoading,
    hasMore,
    error,
    handleScroll,
    loadMore,
    reset,
    setEnd,
  }
}

/**
 * Image lazy loading composable
 *
 * @returns Lazy loading state and methods
 */
export function useLazyLoad() {
  const loadImage = (src: string): Promise<HTMLImageElement> => {
    return new Promise((resolve, reject) => {
      const img = new Image()
      img.onload = () => resolve(img)
      img.onerror = reject
      img.src = src
    })
  }

  const preloadImages = async (urls: string[]) => {
    const promises = urls.map((url) => loadImage(url).catch(() => null))
    return Promise.all(promises)
  }

  return {
    loadImage,
    preloadImages,
  }
}

/**
 * Request animation frame throttle
 * Throttles a function to run once per animation frame
 *
 * @param fn - Function to throttle
 * @returns Throttled function
 */
export function rafThrottle<T extends (...args: any[]) => any>(
  fn: T
): (...args: Parameters<T>) => void {
  let rafId: number | null = null

  return function (this: any, ...args: Parameters<T>) {
    if (rafId !== null) {
      cancelAnimationFrame(rafId)
    }

    rafId = requestAnimationFrame(() => {
      fn.apply(this, args)
      rafId = null
    })
  }
}

/**
 * Measure execution time of a function
 *
 * @param fn - Function to measure
 * @param label - Label for logging
 * @returns Result of the function
 */
export async function measureTime<T>(
  fn: () => Promise<T>,
  label: string
): Promise<T> {
  const start = performance.now()
  const result = await fn()
  const end = performance.now()

  console.log(`[Performance] ${label}: ${(end - start).toFixed(2)}ms`)

  return result
}

/**
 * Measure synchronous execution time
 *
 * @param fn - Function to measure
 * @param label - Label for logging
 * @returns Result of the function
 */
export function measureTimeSync<T>(fn: () => T, label: string): T {
  const start = performance.now()
  const result = fn()
  const end = performance.now()

  console.log(`[Performance] ${label}: ${(end - start).toFixed(2)}ms`)

  return result
}

/**
 * Performance observer for measuring metrics
 */
export class PerformanceMonitor {
  private metrics: Map<string, number[]> = new Map()

  record(name: string, duration: number) {
    if (!this.metrics.has(name)) {
      this.metrics.set(name, [])
    }
    this.metrics.get(name)!.push(duration)
  }

  measure<T>(name: string, fn: () => T): T {
    const start = performance.now()
    const result = fn()
    const duration = performance.now() - start
    this.record(name, duration)
    return result
  }

  async measureAsync<T>(name: string, fn: () => Promise<T>): Promise<T> {
    const start = performance.now()
    const result = await fn()
    const duration = performance.now() - start
    this.record(name, duration)
    return result
  }

  getStats(name: string) {
    const values = this.metrics.get(name)
    if (!values || values.length === 0) {
      return null
    }

    const sorted = [...values].sort((a, b) => a - b)
    const sum = values.reduce((a, b) => a + b, 0)

    return {
      count: values.length,
      min: sorted[0],
      max: sorted[sorted.length - 1],
      avg: sum / values.length,
      p50: sorted[Math.floor(sorted.length * 0.5)],
      p95: sorted[Math.floor(sorted.length * 0.95)],
      p99: sorted[Math.floor(sorted.length * 0.99)],
    }
  }

  getAllStats() {
    const stats: Record<string, ReturnType<typeof this.getStats>> = {}
    for (const name of this.metrics.keys()) {
      stats[name] = this.getStats(name)
    }
    return stats
  }

  clear(name?: string) {
    if (name) {
      this.metrics.delete(name)
    } else {
      this.metrics.clear()
    }
  }

  printReport() {
    console.table(this.getAllStats())
  }
}

/**
 * Create a performance monitor instance
 */
export function createPerformanceMonitor() {
  return new PerformanceMonitor()
}

/**
 * Memoize expensive function results
 *
 * @param fn - Function to memoize
 * @returns Memoized function
 */
export function memoize<T extends (...args: any[]) => any>(
  fn: T,
  keyGenerator?: (...args: Parameters<T>) => string
): T {
  const cache = new Map<string, ReturnType<T>>()

  return ((...args: Parameters<T>) => {
    const key = keyGenerator
      ? keyGenerator(...args)
      : JSON.stringify(args)

    if (cache.has(key)) {
      return cache.get(key)!
    }

    const result = fn(...args)
    cache.set(key, result)
    return result
  }) as T
}
