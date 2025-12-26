pub mod market;
pub mod user;
pub mod strategy;
pub mod strategy_test;
pub mod strategy_engine;
pub mod trade;

// 只导出命令函数
pub use market::{
    market_subscribe_ticker,
    market_get_klines,
    market_get_symbols,
    market_get_status,
    market_unsubscribe_ticker,
};
pub use user::{login, get_current_user};
pub use strategy::{
    strategy_list,
    strategy_get,
    strategy_save,
    strategy_delete,
};
pub use strategy_test::{
    strategy_test_execute,
    strategy_validate_code,
};
pub use strategy_engine::{
    strategy_engine_start,
    strategy_engine_stop,
    strategy_engine_list,
    strategy_engine_get,
};
