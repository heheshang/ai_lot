pub mod market;
pub mod user;
pub mod strategy;
pub mod strategy_test;
pub mod strategy_engine;
pub mod strategy_instance;
pub mod trade;
pub mod risk;
pub mod emergency;
pub mod config;
pub mod backup;
pub mod exchange;

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
    strategy_engine_pause,
    strategy_engine_resume,
    strategy_engine_list,
    strategy_engine_get,
};
pub use strategy_instance::{
    instance_list,
    instance_get,
    instance_create,
    instance_update_status,
    instance_update_stats,
    instance_delete,
    instance_list_running,
    strategy_instance_list_all,
    strategy_instance_get,
};
pub use trade::{
    trade_place_order,
    trade_cancel_order,
    trade_get_order,
    trade_get_orders,
    trade_get_open_orders,
    trade_sync_order_status,
    trade_get_positions,
    trade_get_balance,
    trade_cancel_all_orders,
    trade_close_position,
};
pub use risk::{
    get_risk_overview,
    get_active_alerts,
    handle_alert,
    ignore_alert,
    get_risk_rules,
    update_risk_rule,
    get_alert_history,
    get_alert_detail,
    add_alert_note,
    delete_alert,
};
pub use emergency::{emergency_stop};
pub use config::{
    config_get,
    config_update,
    config_reset,
};
pub use backup::{
    backup_create,
    backup_restore,
    backup_list,
    backup_delete,
    backup_cleanup,
    backup_verify_integrity,
    backup_start_auto,
    backup_stop_auto,
};
pub use exchange::{
    exchange_add,
    exchange_update,
    exchange_list,
    exchange_get,
    exchange_delete,
    exchange_update_status,
};
