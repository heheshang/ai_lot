pub mod r#trait;
pub mod binance;
pub mod okx;
pub mod bybit;
pub mod signature;
pub mod client;

use std::sync::Arc;

pub use r#trait::{Exchange, ExchangeName};
pub use binance::BinanceExchange;
pub use okx::OkxExchange;
pub use bybit::BybitExchange;

/// Factory for creating exchange instances
pub struct ExchangeFactory;

impl ExchangeFactory {
    /// Create exchange by name with credentials
    pub fn create(
        name: ExchangeName,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Arc<dyn Exchange> {
        match name {
            ExchangeName::Binance => Arc::new(BinanceExchange::new(api_key, api_secret)),
            ExchangeName::OKX => Arc::new(OkxExchange::new(api_key, api_secret, passphrase)),
            ExchangeName::Bybit => Arc::new(BybitExchange::new(api_key, api_secret, passphrase)),
        }
    }

    /// Create exchange from config string
    pub fn create_from_config(
        exchange_name: &str,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Result<Arc<dyn Exchange>, String> {
        let name = match exchange_name.to_lowercase().as_str() {
            "binance" => ExchangeName::Binance,
            "okx" => ExchangeName::OKX,
            "bybit" => ExchangeName::Bybit,
            _ => return Err(format!("Unsupported exchange: {}", exchange_name)),
        };

        Ok(Self::create(name, api_key, api_secret, passphrase))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_create_binance() {
        let exchange = ExchangeFactory::create(
            ExchangeName::Binance,
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            None,
        );
        assert_eq!(exchange.name(), ExchangeName::Binance);
    }

    #[test]
    fn test_factory_create_okx() {
        let exchange = ExchangeFactory::create(
            ExchangeName::OKX,
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            Some("test_passphrase".to_string()),
        );
        assert_eq!(exchange.name(), ExchangeName::OKX);
    }

    #[test]
    fn test_factory_create_bybit() {
        let exchange = ExchangeFactory::create(
            ExchangeName::Bybit,
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            None,
        );
        assert_eq!(exchange.name(), ExchangeName::Bybit);
    }

    #[test]
    fn test_factory_create_from_config() {
        let exchange = ExchangeFactory::create_from_config(
            "okx",
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            Some("test_passphrase".to_string()),
        );
        assert!(exchange.is_ok());
        assert_eq!(exchange.unwrap().name(), ExchangeName::OKX);
    }

    #[test]
    fn test_factory_unsupported_exchange() {
        let result = ExchangeFactory::create_from_config(
            "unsupported",
            Some("test_key".to_string()),
            Some("test_secret".to_string()),
            None,
        );
        assert!(result.is_err());
    }
}
