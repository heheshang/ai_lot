import { onUnmounted } from 'vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { useMarketStore } from '@/store/modules/market';
import type { Ticker, Kline } from '@/types';

/**
 * Composable for handling real-time market events from WebSocket
 *
 * This composable sets up Tauri event listeners for:
 * - ticker_update: Real-time ticker price updates
 * - kline_update: Real-time kline/candlestick updates
 *
 * Events are automatically forwarded to the MarketStore.
 */
export function useMarketEvents() {
  const marketStore = useMarketStore();
  const unlisteners: UnlistenFn[] = [];

  /**
   * Initialize market event listeners
   */
  async function initEventListeners() {
    // Listen for ticker updates
    const unlistenTicker = await listen<Ticker>('ticker_update', (event) => {
      console.log('[MarketEvents] Ticker update:', event.payload);
      marketStore.updateTicker(event.payload);
    });
    unlisteners.push(unlistenTicker);

    // Listen for kline updates
    const unlistenKline = await listen<Kline>('kline_update', (event) => {
      console.log('[MarketEvents] Kline update:', event.payload);
      marketStore.updateKline(event.payload);
    });
    unlisteners.push(unlistenKline);

    // Listen for kline batch updates (multiple klines at once)
    const unlistenKlineBatch = await listen<Kline[]>('kline_batch_update', (event) => {
      console.log('[MarketEvents] Kline batch update:', event.payload.length, 'klines');
      marketStore.updateKlines_batch(event.payload);
    });
    unlisteners.push(unlistenKlineBatch);

    // Listen for connection status changes
    const unlistenConnection = await listen<{ connected: boolean }>('market_connection', (event) => {
      console.log('[MarketEvents] Connection status:', event.payload.connected);
      marketStore.wsConnected = event.payload.connected;
    });
    unlisteners.push(unlistenConnection);

    console.log('[MarketEvents] Initialized', unlisteners.length, 'event listeners');
  }

  /**
   * Cleanup event listeners
   */
  function cleanupEventListeners() {
    unlisteners.forEach((unlisten) => {
      unlisten();
    });
    unlisteners.length = 0;
    console.log('[MarketEvents] Cleaned up event listeners');
  }

  // Auto-cleanup on unmount
  onUnmounted(() => {
    cleanupEventListeners();
  });

  return {
    initEventListeners,
    cleanupEventListeners,
  };
}

/**
 * Composable for auto-subscribing to market data on mount
 *
 * @param symbols - Symbols to subscribe to (uses store symbols if not provided)
 * @param autoInit - Auto-initialize event listeners (default: true)
 */
export function useMarketSubscription(symbols?: string[], _autoInit = true) {
  const marketStore = useMarketStore();
  const { initEventListeners, cleanupEventListeners } = useMarketEvents();

  /**
   * Start market subscription
   */
  async function startSubscription() {
    // Initialize event listeners first
    await initEventListeners();

    // Subscribe to ticker updates
    const symbolsToSubscribe = symbols || marketStore.symbols.slice(0, 20);
    if (symbolsToSubscribe.length > 0) {
      await marketStore.subscribeTicker(symbolsToSubscribe);
    }
  }

  /**
   * Stop market subscription
   */
  async function stopSubscription() {
    cleanupEventListeners();

    const symbolsToUnsubscribe = symbols || Array.from(marketStore.tickers.keys());
    if (symbolsToUnsubscribe.length > 0) {
      await marketStore.unsubscribeTicker(symbolsToUnsubscribe);
    }
  }

  // Auto-start on mount
  onUnmounted(() => {
    // Cleanup happens in useMarketEvents
  });

  return {
    startSubscription,
    stopSubscription,
  };
}
