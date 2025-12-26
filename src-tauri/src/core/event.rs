use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::core::trade::types::*;

/// 市场事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketEvent {
    Ticker(Ticker),
    Kline(Kline),
}

/// 交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeEvent {
    OrderPlaced(Order),
    OrderFilled(Order),
    OrderCanceled(Order),
    PositionUpdated(Position),
}

/// 策略事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyEvent {
    StrategyStarted(String),
    StrategyStopped(String),
    SignalGenerated(Signal),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub action: String,
    pub quantity: f64,
    pub price: Option<f64>,
}

/// 事件总线
#[derive(Clone)]
pub struct EventBus {
    market_tx: broadcast::Sender<MarketEvent>,
    trade_tx: broadcast::Sender<TradeEvent>,
    strategy_tx: broadcast::Sender<StrategyEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (market_tx, _) = broadcast::channel(1000);
        let (trade_tx, _) = broadcast::channel(1000);
        let (strategy_tx, _) = broadcast::channel(1000);

        Self {
            market_tx,
            trade_tx,
            strategy_tx,
        }
    }

    // ========== Market Events ==========

    /// Publish a market event
    pub fn publish_market(&self, event: MarketEvent) {
        let _ = self.market_tx.send(event);
    }

    /// Subscribe to market events
    pub fn subscribe_market(&self) -> broadcast::Receiver<MarketEvent> {
        self.market_tx.subscribe()
    }

    /// Publish ticker update
    pub fn publish_ticker(&self, ticker: Ticker) {
        self.publish_market(MarketEvent::Ticker(ticker));
    }

    /// Publish kline update
    pub fn publish_kline(&self, kline: Kline) {
        self.publish_market(MarketEvent::Kline(kline));
    }

    // ========== Trade Events ==========

    /// Publish a trade event
    pub fn publish_trade(&self, event: TradeEvent) {
        let _ = self.trade_tx.send(event);
    }

    /// Subscribe to trade events
    pub fn subscribe_trade(&self) -> broadcast::Receiver<TradeEvent> {
        self.trade_tx.subscribe()
    }

    /// Publish order placed event
    pub fn publish_order_placed(&self, order: Order) {
        self.publish_trade(TradeEvent::OrderPlaced(order));
    }

    /// Publish order filled event
    pub fn publish_order_filled(&self, order: Order) {
        self.publish_trade(TradeEvent::OrderFilled(order));
    }

    /// Publish order canceled event
    pub fn publish_order_canceled(&self, order: Order) {
        self.publish_trade(TradeEvent::OrderCanceled(order));
    }

    /// Publish position updated event
    pub fn publish_position_updated(&self, position: Position) {
        self.publish_trade(TradeEvent::PositionUpdated(position));
    }

    // ========== Strategy Events ==========

    /// Publish a strategy event
    pub fn publish_strategy(&self, event: StrategyEvent) {
        let _ = self.strategy_tx.send(event);
    }

    /// Subscribe to strategy events
    pub fn subscribe_strategy(&self) -> broadcast::Receiver<StrategyEvent> {
        self.strategy_tx.subscribe()
    }

    /// Publish strategy started event
    pub fn publish_strategy_started(&self, strategy_id: String) {
        self.publish_strategy(StrategyEvent::StrategyStarted(strategy_id));
    }

    /// Publish strategy stopped event
    pub fn publish_strategy_stopped(&self, strategy_id: String) {
        self.publish_strategy(StrategyEvent::StrategyStopped(strategy_id));
    }

    /// Publish signal generated event
    pub fn publish_signal(&self, signal: Signal) {
        self.publish_strategy(StrategyEvent::SignalGenerated(signal));
    }

    /// Publish strategy error event
    pub fn publish_strategy_error(&self, error: String) {
        self.publish_strategy(StrategyEvent::Error(error));
    }

    // ========== Utility Methods ==========

    /// Get the number of receivers for market events
    pub fn market_receiver_count(&self) -> usize {
        self.market_tx.receiver_count()
    }

    /// Get the number of receivers for trade events
    pub fn trade_receiver_count(&self) -> usize {
        self.trade_tx.receiver_count()
    }

    /// Get the number of receivers for strategy events
    pub fn strategy_receiver_count(&self) -> usize {
        self.strategy_tx.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trade::types::Interval;

    #[tokio::test]
    async fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.market_receiver_count(), 0);
        assert_eq!(bus.trade_receiver_count(), 0);
        assert_eq!(bus.strategy_receiver_count(), 0);
    }

    #[tokio::test]
    async fn test_market_event_publish() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_market();

        // Create a test ticker
        let ticker = Ticker {
            symbol: "BTCUSDT".to_string(),
            price: 50000.0,
            price_change: 1000.0,
            price_change_percent: 2.0,
            high_24h: 51000.0,
            low_24h: 49000.0,
            volume_24h: 1000.0,
            timestamp: 1234567890,
        };

        bus.publish_ticker(ticker.clone());

        let received = rx.recv().await.unwrap();
        match received {
            MarketEvent::Ticker(t) => {
                assert_eq!(t.symbol, "BTCUSDT");
                assert_eq!(t.price, 50000.0);
            }
            _ => panic!("Expected ticker event"),
        }
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let bus = EventBus::new();

        // Create multiple subscribers
        let mut rx1 = bus.subscribe_market();
        let mut rx2 = bus.subscribe_market();
        let mut rx3 = bus.subscribe_market();

        assert_eq!(bus.market_receiver_count(), 3);

        // Publish event
        let ticker = Ticker {
            symbol: "ETHUSDT".to_string(),
            price: 3000.0,
            price_change: 100.0,
            price_change_percent: 3.44,
            high_24h: 3100.0,
            low_24h: 2900.0,
            volume_24h: 5000.0,
            timestamp: 1234567890,
        };

        bus.publish_ticker(ticker);

        // All subscribers should receive
        let recv1 = rx1.recv().await.unwrap();
        let recv2 = rx2.recv().await.unwrap();
        let recv3 = rx3.recv().await.unwrap();

        match (&recv1, &recv2, &recv3) {
            (MarketEvent::Ticker(t1), MarketEvent::Ticker(t2), MarketEvent::Ticker(t3)) => {
                assert_eq!(t1.symbol, "ETHUSDT");
                assert_eq!(t2.symbol, "ETHUSDT");
                assert_eq!(t3.symbol, "ETHUSDT");
            }
            _ => panic!("Expected all ticker events"),
        }
    }

    #[tokio::test]
    async fn test_trade_events() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_trade();

        // Test order placed
        let order = Order {
            id: "12345".to_string(),
            exchange_order_id: Some("EXCH123".to_string()),
            client_order_id: Some("CLIENT123".to_string()),
            symbol: "BTCUSDT".to_string(),
            side: "buy".to_string(),
            order_type: "limit".to_string(),
            price: Some(50000.0),
            quantity: 0.5,
            filled_quantity: 0.0,
            avg_price: None,
            status: "open".to_string(),
            commission: 0.001,
            created_at: 1234567890,
            filled_at: None,
        };

        bus.publish_order_placed(order.clone());

        let received = rx.recv().await.unwrap();
        match received {
            TradeEvent::OrderPlaced(o) => {
                assert_eq!(o.id, "12345");
                assert_eq!(o.symbol, "BTCUSDT");
            }
            _ => panic!("Expected order placed event"),
        }
    }

    #[tokio::test]
    async fn test_strategy_events() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_strategy();

        // Test signal generated
        let signal = Signal {
            symbol: "BTCUSDT".to_string(),
            action: "buy".to_string(),
            quantity: 1.0,
            price: Some(50000.0),
        };

        bus.publish_signal(signal.clone());

        let received = rx.recv().await.unwrap();
        match received {
            StrategyEvent::SignalGenerated(s) => {
                assert_eq!(s.symbol, "BTCUSDT");
                assert_eq!(s.action, "buy");
                assert_eq!(s.quantity, 1.0);
            }
            _ => panic!("Expected signal generated event"),
        }
    }

    #[tokio::test]
    async fn test_kline_event() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_market();

        let kline = Kline {
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            timestamp: 1234567890,
            open: 50000.0,
            high: 51000.0,
            low: 49000.0,
            close: 50500.0,
            volume: 100.0,
            quote_volume: Some(5000000.0),
        };

        bus.publish_kline(kline.clone());

        let received = rx.recv().await.unwrap();
        match received {
            MarketEvent::Kline(k) => {
                assert_eq!(k.symbol, "BTCUSDT");
                assert_eq!(k.close, 50500.0);
            }
            _ => panic!("Expected kline event"),
        }
    }

    #[tokio::test]
    async fn test_strategy_lifecycle_events() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_strategy();

        // Test started
        bus.publish_strategy_started("strategy-1".to_string());
        let event1 = rx.recv().await.unwrap();
        match event1 {
            StrategyEvent::StrategyStarted(id) => assert_eq!(id, "strategy-1"),
            _ => panic!("Expected strategy started"),
        }

        // Test stopped
        bus.publish_strategy_stopped("strategy-1".to_string());
        let event2 = rx.recv().await.unwrap();
        match event2 {
            StrategyEvent::StrategyStopped(id) => assert_eq!(id, "strategy-1"),
            _ => panic!("Expected strategy stopped"),
        }

        // Test error
        bus.publish_strategy_error("Test error".to_string());
        let event3 = rx.recv().await.unwrap();
        match event3 {
            StrategyEvent::Error(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected strategy error"),
        }
    }

    #[tokio::test]
    async fn test_default_event_bus() {
        let bus: EventBus = Default::default();
        assert_eq!(bus.market_receiver_count(), 0);
        assert_eq!(bus.trade_receiver_count(), 0);
        assert_eq!(bus.strategy_receiver_count(), 0);
    }

    #[tokio::test]
    async fn test_event_bus_clone() {
        let bus1 = EventBus::new();
        let bus2 = bus1.clone();

        let ticker = Ticker {
            symbol: "BTCUSDT".to_string(),
            price: 50000.0,
            price_change: 1000.0,
            price_change_percent: 2.0,
            high_24h: 51000.0,
            low_24h: 49000.0,
            volume_24h: 1000.0,
            timestamp: 1234567890,
        };

        // Subscribe to original bus
        let mut rx = bus1.subscribe_market();

        // Publish from cloned bus
        bus2.publish_ticker(ticker);

        // Should receive event
        let received = rx.recv().await.unwrap();
        match received {
            MarketEvent::Ticker(t) => {
                assert_eq!(t.symbol, "BTCUSDT");
            }
            _ => panic!("Expected ticker event"),
        }
    }

    #[tokio::test]
    async fn test_position_updated_event() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_trade();

        let position = Position {
            id: "pos-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "long".to_string(),
            quantity: 1.0,
            entry_price: 50000.0,
            current_price: Some(51000.0),
            unrealized_pnl: 1000.0,
            realized_pnl: 0.0,
            opened_at: 1234567890,
        };

        bus.publish_position_updated(position.clone());

        let received = rx.recv().await.unwrap();
        match received {
            TradeEvent::PositionUpdated(p) => {
                assert_eq!(p.id, "pos-1");
                assert_eq!(p.symbol, "BTCUSDT");
                assert_eq!(p.unrealized_pnl, 1000.0);
            }
            _ => panic!("Expected position updated event"),
        }
    }

    #[tokio::test]
    async fn test_missed_events() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe_market();

        // Publish multiple events
        for i in 0..5 {
            let ticker = Ticker {
                symbol: "BTCUSDT".to_string(),
                price: i as f64,
                price_change: 0.0,
                price_change_percent: 0.0,
                high_24h: 0.0,
                low_24h: 0.0,
                volume_24h: 0.0,
                timestamp: 0,
            };
            bus.publish_ticker(ticker);
        }

        // Should only receive latest event (broadcast channel behavior)
        let result = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            rx.recv()
        ).await;

        // Broadcast channels only keep the latest message
        // So we receive one message
        assert!(result.is_ok());
    }
}
