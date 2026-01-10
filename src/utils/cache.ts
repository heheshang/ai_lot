/**
 * Simple in-memory cache with TTL support
 * Provides request deduplication and response caching
 */

interface CacheEntry<T> {
  value: T
  expiresAt: number
}

interface PendingRequest<T> {
  promise: Promise<T>
  timestamp: number
}

class CacheStore {
  private store = new Map<string, CacheEntry<any>>()
  private pending = new Map<string, PendingRequest<any>>()
  private maxSize: number
  private defaultTTL: number

  constructor(options: { maxSize?: number; defaultTTL?: number } = {}) {
    this.maxSize = options.maxSize ?? 100
    this.defaultTTL = options.defaultTTL ?? 60000 // 1 minute
  }

  /**
   * Get a value from cache
   */
  get<T>(key: string): T | null {
    const entry = this.store.get(key)
    if (!entry) return null

    if (Date.now() > entry.expiresAt) {
      this.store.delete(key)
      return null
    }

    return entry.value as T
  }

  /**
   * Set a value in cache
   */
  set<T>(key: string, value: T, ttl?: number): void {
    // Evict oldest entries if at capacity
    if (this.store.size >= this.maxSize) {
      const oldestKey = this.store.keys().next().value
      if (oldestKey) {
        this.store.delete(oldestKey)
      }
    }

    this.store.set(key, {
      value,
      expiresAt: Date.now() + (ttl ?? this.defaultTTL),
    })
  }

  /**
   * Delete a value from cache
   */
  delete(key: string): boolean {
    return this.store.delete(key)
  }

  /**
   * Clear all entries
   */
  clear(): void {
    this.store.clear()
    this.pending.clear()
  }

  /**
   * Check if a key exists and is valid
   */
  has(key: string): boolean {
    const entry = this.store.get(key)
    if (!entry) return false
    if (Date.now() > entry.expiresAt) {
      this.store.delete(key)
      return false
    }
    return true
  }

  /**
   * Get or set with deduplication (prevents duplicate simultaneous requests)
   */
  async getOrSet<T>(key: string, fetchFn: () => Promise<T>, ttl?: number): Promise<T> {
    // Check cache first
    const cached = this.get<T>(key)
    if (cached !== null) {
      return cached
    }

    // Check for pending request
    const pending = this.pending.get(key)
    if (pending) {
      return pending.promise as Promise<T>
    }

    // Create new request
    const promise = fetchFn().then((value) => {
      this.pending.delete(key)
      this.set(key, value, ttl)
      return value
    })

    this.pending.set(key, { promise, timestamp: Date.now() })
    return promise
  }

  /**
   * Invalidate keys matching a pattern
   */
  invalidatePattern(pattern: RegExp): void {
    for (const key of this.store.keys()) {
      if (pattern.test(key)) {
        this.store.delete(key)
      }
    }
  }

  /**
   * Get cache statistics
   */
  getStats(): { size: number; pending: number } {
    return {
      size: this.store.size,
      pending: this.pending.size,
    }
  }
}

// Global cache instance
export const apiCache = new CacheStore({
  maxSize: 200,
  defaultTTL: 30000, // 30 seconds
})

/**
 * Cache decorator for functions
 */
export function cached<T extends (...args: any[]) => Promise<any>>(
  fn: T,
  options: {
    ttl?: number
    keyGenerator?: (...args: Parameters<T>) => string
  } = {}
): T {
  return ((...args: Parameters<T>) => {
    const key = options.keyGenerator
      ? options.keyGenerator(...args)
      : `${fn.name}(${JSON.stringify(args)})`
    return apiCache.getOrSet(key, () => fn(...args), options.ttl)
  }) as T
}

/**
 * Create a cached API call
 */
export async function cachedFetch<T>(
  key: string,
  fetchFn: () => Promise<T>,
  ttl?: number
): Promise<T> {
  return apiCache.getOrSet(key, fetchFn, ttl)
}

/**
 * Invalidate cache for a specific command
 */
export function invalidateCache(cmd: string): void {
  apiCache.invalidatePattern(new RegExp(`^${cmd}\\(`))
}

/**
 * Clear all cached data
 */
export function clearCache(): void {
  apiCache.clear()
}
