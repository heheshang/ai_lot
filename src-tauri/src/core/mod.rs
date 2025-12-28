pub mod auth;
pub mod error;
pub mod event;
pub mod trade;
pub mod strategy;
pub mod risk;

pub use auth::AuthService;
pub use error::{AppError, AppResult};
pub use event::{EventBus, MarketEvent, TradeEvent, StrategyEvent, Signal};
pub use strategy::{ScriptExecutor, StrategyEngine, StrategyConfig, InstanceInfo, InstanceStatus};
pub use risk::{RiskRule, RiskContext, RiskRuleConfig, RiskAction, RiskMonitor};
