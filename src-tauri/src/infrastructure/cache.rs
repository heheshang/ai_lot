//! Cache Manager Module (P6-05)
//!
//! Provides high-performance async caching using moka.
//! Thread-safe, with TTL support and automatic eviction.

use moka::future::Cache as MokaCache;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

/// Cache statistics for monitoring hit rates
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub total_requests: u64,
    pub hit_rate: f64,
}

impl CacheStats {
    pub fn new(hit_count: u64, miss_count: u64) -> Self {
        let total_requests = hit_count + miss_count;
        let hit_rate = if total_requests > 0 {
            (hit_count as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        Self {
            hit_count,
            miss_count,
            total_requests,
            hit_rate,
        }
    }
}

/// Generic cache manager with async operations
pub struct CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static + std::fmt::Debug,
    V: Clone + Send + Sync + 'static,
{
    cache: MokaCache<K, V>,
    name: String,
    hit_count: Arc<RwLock<u64>>,
    miss_count: Arc<RwLock<u64>>,
}

impl<K, V> CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static + std::fmt::Debug,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache manager
    ///
    /// # Arguments
    /// * `name` - Cache name for logging
    /// * `max_capacity` - Maximum number of entries
    /// * `ttl_seconds` - Time-to-live in seconds
    pub fn new(name: impl Into<String>, max_capacity: u64, ttl_seconds: u64) -> Self {
        let name = name.into();
        let cache = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        log::info!(
            "Cache '{}' created: capacity={}, TTL={}s",
            name,
            max_capacity,
            ttl_seconds
        );

        Self {
            cache,
            name,
            hit_count: Arc::new(RwLock::new(0)),
            miss_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Get a value from cache
    pub async fn get(&self, key: &K) -> Option<V> {
        match self.cache.get(key).await {
            Some(value) => {
                let mut hits = self.hit_count.write().await;
                *hits += 1;
                log::trace!("Cache '{}' HIT: key={:?}", self.name, key);
                Some(value)
            }
            None => {
                let mut misses = self.miss_count.write().await;
                *misses += 1;
                log::trace!("Cache '{}' MISS: key={:?}", self.name, key);
                None
            }
        }
    }

    /// Insert a value into cache
    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
        log::trace!("Cache '{}' INSERT", self.name);
    }

    /// Invalidate a specific entry
    pub async fn invalidate(&self, key: &K) {
        self.cache.invalidate(key).await;
        log::debug!("Cache '{}' INVALIDATE: key={:?}", self.name, key);
    }

    /// Clear all entries in cache
    pub async fn clear(&self) {
        self.cache.invalidate_all();
        log::info!("Cache '{}' CLEARED", self.name);
    }

    /// Get current cache size
    pub fn size(&self) -> u64 {
        self.cache.entry_count()
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let hits = *self.hit_count.read().await;
        let misses = *self.miss_count.read().await;
        CacheStats::new(hits, misses)
    }

    /// Reset statistics counters
    pub async fn reset_stats(&self) {
        let mut hits = self.hit_count.write().await;
        let mut misses = self.miss_count.write().await;
        *hits = 0;
        *misses = 0;
        log::debug!("Cache '{}' stats reset", self.name);
    }
}

/// Global cache instances using lazy_static
mod global_caches {
    use super::*;

    /// Ticker cache - 5 second TTL
    pub struct TickerCacheKey {
        pub exchange: String,
        pub symbol: String,
    }

    impl TickerCacheKey {
        pub fn new(exchange: impl Into<String>, symbol: impl Into<String>) -> Self {
            Self {
                exchange: exchange.into(),
                symbol: symbol.into(),
            }
        }
    }

    impl std::hash::Hash for TickerCacheKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.exchange.hash(state);
            self.symbol.hash(state);
        }
    }

    impl PartialEq for TickerCacheKey {
        fn eq(&self, other: &Self) -> bool {
            self.exchange == other.exchange && self.symbol == other.symbol
        }
    }

    impl Eq for TickerCacheKey {}

    impl std::fmt::Debug for TickerCacheKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("TickerCacheKey")
                .field(&self.exchange)
                .field(&self.symbol)
                .finish()
        }
    }

    pub type TickerValue = serde_json::Value;

    /// Kline cache - 30 second TTL
    pub struct KlineCacheKey {
        pub exchange: String,
        pub symbol: String,
        pub timeframe: String,
    }

    impl KlineCacheKey {
        pub fn new(exchange: impl Into<String>, symbol: impl Into<String>, timeframe: impl Into<String>) -> Self {
            Self {
                exchange: exchange.into(),
                symbol: symbol.into(),
                timeframe: timeframe.into(),
            }
        }
    }

    impl std::hash::Hash for KlineCacheKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.exchange.hash(state);
            self.symbol.hash(state);
            self.timeframe.hash(state);
        }
    }

    impl PartialEq for KlineCacheKey {
        fn eq(&self, other: &Self) -> bool {
            self.exchange == other.exchange && self.symbol == other.symbol && self.timeframe == other.timeframe
        }
    }

    impl Eq for KlineCacheKey {}

    impl std::fmt::Debug for KlineCacheKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("KlineCacheKey")
                .field(&self.exchange)
                .field(&self.symbol)
                .field(&self.timeframe)
                .finish()
        }
    }

    pub type KlineValue = Vec<crate::core::trade::types::Kline>;

    /// Account cache - 10 second TTL
    pub struct AccountCacheKey {
        pub user_id: String,
        pub exchange_id: String,
    }

    impl AccountCacheKey {
        pub fn new(user_id: impl Into<String>, exchange_id: impl Into<String>) -> Self {
            Self {
                user_id: user_id.into(),
                exchange_id: exchange_id.into(),
            }
        }
    }

    impl std::hash::Hash for AccountCacheKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.user_id.hash(state);
            self.exchange_id.hash(state);
        }
    }

    impl PartialEq for AccountCacheKey {
        fn eq(&self, other: &Self) -> bool {
            self.user_id == other.user_id && self.exchange_id == other.exchange_id
        }
    }

    impl Eq for AccountCacheKey {}

    impl std::fmt::Debug for AccountCacheKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("AccountCacheKey")
                .field(&self.user_id)
                .field(&self.exchange_id)
                .finish()
        }
    }

    pub type AccountValue = serde_json::Value;
}

use global_caches::*;

lazy_static::lazy_static! {
    /// Global ticker cache (5s TTL, 1000 entries)
    pub static ref TICKER_CACHE: Arc<CacheManager<TickerCacheKey, TickerValue>> =
        Arc::new(CacheManager::new("ticker", 1000, 5));

    /// Global kline cache (30s TTL, 500 entries)
    pub static ref KLINE_CACHE: Arc<CacheManager<KlineCacheKey, KlineValue>> =
        Arc::new(CacheManager::new("kline", 500, 30));

    /// Global account cache (10s TTL, 100 entries)
    pub static ref ACCOUNT_CACHE: Arc<CacheManager<AccountCacheKey, AccountValue>> =
        Arc::new(CacheManager::new("account", 100, 10));
}

/// Convenience functions for accessing global caches

pub use global_caches::{TickerCacheKey, KlineCacheKey, AccountCacheKey};

/// Get ticker from cache
pub async fn get_ticker(exchange: &str, symbol: &str) -> Option<TickerValue> {
    TICKER_CACHE.get(&TickerCacheKey::new(exchange, symbol)).await
}

/// Insert ticker into cache
pub async fn insert_ticker(exchange: &str, symbol: &str, value: TickerValue) {
    TICKER_CACHE.insert(TickerCacheKey::new(exchange, symbol), value).await
}

/// Invalidate ticker cache
pub async fn invalidate_ticker(exchange: &str, symbol: &str) {
    TICKER_CACHE.invalidate(&TickerCacheKey::new(exchange, symbol)).await
}

/// Get klines from cache
pub async fn get_klines(exchange: &str, symbol: &str, timeframe: &str) -> Option<KlineValue> {
    KLINE_CACHE.get(&KlineCacheKey::new(exchange, symbol, timeframe)).await
}

/// Insert klines into cache
pub async fn insert_klines(exchange: &str, symbol: &str, timeframe: &str, value: KlineValue) {
    KLINE_CACHE.insert(KlineCacheKey::new(exchange, symbol, timeframe), value).await
}

/// Invalidate kline cache
pub async fn invalidate_klines(exchange: &str, symbol: &str, timeframe: &str) {
    KLINE_CACHE.invalidate(&KlineCacheKey::new(exchange, symbol, timeframe)).await
}

/// Get account from cache
pub async fn get_account(user_id: &str, exchange_id: &str) -> Option<AccountValue> {
    ACCOUNT_CACHE.get(&AccountCacheKey::new(user_id, exchange_id)).await
}

/// Insert account into cache
pub async fn insert_account(user_id: &str, exchange_id: &str, value: AccountValue) {
    ACCOUNT_CACHE.insert(AccountCacheKey::new(user_id, exchange_id), value).await
}

/// Invalidate account cache
pub async fn invalidate_account(user_id: &str, exchange_id: &str) {
    ACCOUNT_CACHE.invalidate(&AccountCacheKey::new(user_id, exchange_id)).await
}

/// Get all cache statistics
pub async fn get_all_cache_stats() -> Vec<(String, CacheStats)> {
    let ticker_stats = TICKER_CACHE.stats().await;
    let kline_stats = KLINE_CACHE.stats().await;
    let account_stats = ACCOUNT_CACHE.stats().await;

    vec![
        ("ticker".to_string(), ticker_stats),
        ("kline".to_string(), kline_stats),
        ("account".to_string(), account_stats),
    ]
}

/// Clear all caches
pub async fn clear_all_caches() {
    TICKER_CACHE.clear().await;
    KLINE_CACHE.clear().await;
    ACCOUNT_CACHE.clear().await;
    log::info!("All caches cleared");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = CacheManager::new("test", 10, 60);

        // Insert and get
        cache.insert("key1".to_string(), "value1".to_string()).await;
        let result = cache.get(&"key1".to_string()).await;
        assert_eq!(result, Some("value1".to_string()));

        // Miss
        let result = cache.get(&"key2".to_string()).await;
        assert_eq!(result, None);

        // Invalidate
        cache.invalidate(&"key1".to_string()).await;
        let result = cache.get(&"key1".to_string()).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = CacheManager::new("test_stats", 10, 60);

        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.get(&"key1".to_string()).await; // hit
        cache.get(&"key2".to_string()).await; // miss

        let stats = cache.stats().await;
        assert_eq!(stats.hit_count, 1);
        assert_eq!(stats.miss_count, 1);
        assert_eq!(stats.total_requests, 2);
        assert!((stats.hit_rate - 50.0).abs() < 0.01);
    }
}
