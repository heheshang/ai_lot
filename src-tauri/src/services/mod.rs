pub mod market_service;
pub mod trade_service;
pub mod emergency_service;
pub mod backup_service;

pub use market_service::MarketService;
pub use trade_service::TradeService;
pub use emergency_service::{EmergencyService, EmergencyReport};
pub use backup_service::{BackupService, BackupInfo};

// Re-export cache functions for convenience
pub use crate::infrastructure::cache::{
    get_all_cache_stats, clear_all_caches, CacheStats,
};
