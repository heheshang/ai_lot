//! Cache commands for P6-05 performance monitoring
//!
//! Provides Tauri commands for cache statistics and management.

use crate::infrastructure::cache::{get_all_cache_stats, clear_all_caches, CacheStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatsResponse {
    pub name: String,
    pub hit_count: u64,
    pub miss_count: u64,
    pub total_requests: u64,
    pub hit_rate: f64,
}

impl From<(String, CacheStats)> for CacheStatsResponse {
    fn from((name, stats): (String, CacheStats)) -> Self {
        Self {
            name,
            hit_count: stats.hit_count,
            miss_count: stats.miss_count,
            total_requests: stats.total_requests,
            hit_rate: stats.hit_rate,
        }
    }
}

/// Get cache statistics for all caches
#[tauri::command]
pub async fn get_cache_stats() -> Result<Vec<CacheStatsResponse>, String> {
    let stats = get_all_cache_stats().await;
    Ok(stats.into_iter().map(CacheStatsResponse::from).collect())
}

/// Clear all caches
#[tauri::command]
pub async fn clear_all_caches_cmd() -> Result<(), String> {
    clear_all_caches().await;
    Ok(())
}

/// Get performance metrics including cache stats
#[tauri::command]
pub async fn get_performance_metrics() -> Result<PerformanceMetrics, String> {
    let cache_stats = get_all_cache_stats().await;
    let cache_stats_map: HashMap<String, CacheStatsResponse> = cache_stats
        .into_iter()
        .map(|(name, stats)| (name.clone(), CacheStatsResponse::from((name, stats))))
        .collect();

    Ok(PerformanceMetrics {
        cache_stats: cache_stats_map,
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cache_stats: HashMap<String, CacheStatsResponse>,
    pub timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stats_response_conversion() {
        let stats = CacheStats::new(100, 25);
        let response = CacheStatsResponse::from(("test".to_string(), stats));

        assert_eq!(response.name, "test");
        assert_eq!(response.hit_count, 100);
        assert_eq!(response.miss_count, 25);
        assert_eq!(response.total_requests, 125);
        assert!((response.hit_rate - 80.0).abs() < 0.01);
    }
}
