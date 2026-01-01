pub mod auth;
pub mod error;
pub mod event;
pub mod trade;
pub mod strategy;
pub mod risk;
pub mod response;
pub mod validation;

pub use auth::AuthService;
pub use error::{AppError, AppResult};
pub use event::{EventBus, MarketEvent, TradeEvent, StrategyEvent, Signal};
pub use strategy::{ScriptExecutor, StrategyEngine, StrategyConfig, InstanceInfo, InstanceStatus};
pub use risk::{RiskRule, RiskContext, RiskRuleConfig, RiskAction, RiskMonitor};
pub use response::{ApiResponse, ApiError, ErrorDetail, ToApiResponse, IntoApiError};
pub use validation::{Validator, StringValidator, NumberValidator, validate_string, validate_number};
