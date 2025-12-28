import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Ticker, Kline } from '@/types';
import * as api from '@/api/tauri';
import { listen } from '@tauri-apps/api/event';

// Tauri event names
const MARKET_TICKER_EVENT = 'market:ticker';
const MARKET_KLINE_EVENT = 'market:kline';

/**
 * Market Status from Tauri
 */
interface MarketStatus {
  connected: boolean;
  exchangesCount: number;
  subscriptionsCount: number;
  lastUpdate: number | null;
}

export const useMarketStore = defineStore('market', () => {
  // ========== State ==========
  const currentSymbol = ref('BTCUSDT');
  const currentTimeframe = ref('1h');
  const tickers = ref<Map<string, Ticker>>(new Map());
  const klines = ref<Map<string, Kline[]>>(new Map());
  const wsConnected = ref(false);
  const symbols = ref<string[]>([]);
  const marketStatus = ref<MarketStatus | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Event listeners cleanup functions
  let tickerUnlisten: (() => void) | null = null;
  let klineUnlisten: (() => void) | null = null;

  // ========== Getters (Computed) ==========
  const currentTicker = computed(() => tickers.value.get(currentSymbol.value));

  const currentKlines = computed(() => {
    const key = `${currentSymbol.value}_${currentTimeframe.value}`;
    return klines.value.get(key) || [];
  });

  const subscribedSymbols = computed(() => Array.from(tickers.value.keys()));

  const isConnected = computed(() => marketStatus.value?.connected ?? false);

  // ========== Actions ==========

  /**
   * 加载交易对列表
   */
  async function loadSymbols() {
    try {
      loading.value = true;
      error.value = null;
      const result = await api.marketApi.getSymbols();
      symbols.value = result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load symbols';
      console.error('Failed to load symbols:', err);
    } finally {
      loading.value = false;
    }
  }

  /**
   * 加载K线数据
   */
  async function loadKlines(symbol: string, interval: string, limit = 500) {
    try {
      loading.value = true;
      error.value = null;
      const data = await api.marketApi.getKlines(symbol, interval, limit);
      const key = `${symbol}_${interval}`;
      klines.value.set(key, data);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load klines';
      console.error('Failed to load klines:', err);
    } finally {
      loading.value = false;
    }
  }

  /**
   * 订阅行情更新
   */
  async function subscribeTicker(syms: string[]) {
    try {
      loading.value = true;
      error.value = null;
      await api.marketApi.subscribeTicker(syms);
      wsConnected.value = true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to subscribe ticker';
      wsConnected.value = false;
      console.error('Failed to subscribe ticker:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 取消订阅行情
   */
  async function unsubscribeTicker(syms: string[]) {
    try {
      await api.invokeRaw('market_unsubscribe_ticker', { symbols: syms });
      // Remove from local ticker map
      syms.forEach(sym => tickers.value.delete(sym));
    } catch (err) {
      console.error('Failed to unsubscribe ticker:', err);
      throw err;
    }
  }

  /**
   * 获取市场状态
   */
  async function getMarketStatus() {
    try {
      const status = await api.invokeRaw<MarketStatus>('market_get_status');
      marketStatus.value = status;
      return status;
    } catch (err) {
      console.error('Failed to get market status:', err);
      throw err;
    }
  }

  /**
   * 更新 Ticker 数据（从 WebSocket 事件）
   */
  function updateTicker(ticker: Ticker) {
    tickers.value.set(ticker.symbol, ticker);
  }

  /**
   * 更新 Kline 数据（从 WebSocket 事件）
   */
  function updateKline(kline: Kline) {
    const key = `${kline.symbol}_${kline.timeframe}`;
    const data = klines.value.get(key) || [];

    // Find if this kline already exists (by timestamp)
    const existingIndex = data.findIndex(k => k.timestamp === kline.timestamp);

    if (existingIndex >= 0) {
      // Update existing kline
      data[existingIndex] = kline;
    } else {
      // Add new kline
      data.push(kline);
      // Keep max 1000 klines in memory
      if (data.length > 1000) {
        data.shift();
      }
    }

    klines.value.set(key, data);
  }

  /**
   * 批量更新 Kline 数据
   */
  function updateKlines_batch(newKlines: Kline[]) {
    if (newKlines.length === 0) return;

    const firstKline = newKlines[0];
    const key = `${firstKline.symbol}_${firstKline.timeframe}`;
    const data = klines.value.get(key) || [];

    // Create a map of existing klines by timestamp
    const existingMap = new Map(data.map(k => [k.timestamp, k]));

    // Update or add new klines
    newKlines.forEach(kline => {
      existingMap.set(kline.timestamp, kline);
    });

    // Convert back to array and sort by timestamp
    const sorted = Array.from(existingMap.values()).sort((a, b) => a.timestamp - b.timestamp);

    // Keep max 1000 klines
    const trimmed = sorted.length > 1000 ? sorted.slice(-1000) : sorted;

    klines.value.set(key, trimmed);
  }

  /**
   * 设置当前交易对
   */
  function setCurrentSymbol(symbol: string) {
    currentSymbol.value = symbol;
    // Load klines for new symbol
    loadKlines(symbol, currentTimeframe.value);
  }

  /**
   * 设置当前时间周期
   */
  function setCurrentTimeframe(timeframe: string) {
    currentTimeframe.value = timeframe;
    // Reload klines with new timeframe
    loadKlines(currentSymbol.value, timeframe);
  }

  /**
   * 清除所有数据
   */
  function clear() {
    // Clean up event listeners
    if (tickerUnlisten) {
      tickerUnlisten();
      tickerUnlisten = null;
    }
    if (klineUnlisten) {
      klineUnlisten();
      klineUnlisten = null;
    }

    tickers.value.clear();
    klines.value.clear();
    symbols.value = [];
    wsConnected.value = false;
    marketStatus.value = null;
    error.value = null;
  }

  /**
   * 初始化市场数据和事件监听
   */
  async function initialize() {
    try {
      loading.value = true;

      // 设置 Tauri 事件监听器
      if (!tickerUnlisten) {
        tickerUnlisten = await listen<Ticker>(MARKET_TICKER_EVENT, (event) => {
          const ticker = event.payload;
          tickers.value.set(ticker.symbol, ticker);
          logDebug(`Received ticker update: ${ticker.symbol} @ ${ticker.price}`);
        });
      }

      if (!klineUnlisten) {
        klineUnlisten = await listen<Kline>(MARKET_KLINE_EVENT, (event) => {
          const kline = event.payload;
          const key = `${kline.symbol}_${kline.timeframe}`;
          const data = klines.value.get(key) || [];

          // Find if this kline already exists (by timestamp)
          const existingIndex = data.findIndex(k => k.timestamp === kline.timestamp);

          if (existingIndex >= 0) {
            // Update existing kline
            data[existingIndex] = kline;
          } else {
            // Add new kline
            data.push(kline);
            // Keep max 1000 klines in memory
            if (data.length > 1000) {
              data.shift();
            }
          }

          klines.value.set(key, data);
          logDebug(`Received kline update: ${kline.symbol} ${kline.timeframe}`);
        });
      }

      await Promise.all([
        loadSymbols(),
        getMarketStatus(),
      ]);
      // Load initial klines for default symbol
      await loadKlines(currentSymbol.value, currentTimeframe.value);
    } catch (err) {
      console.error('Failed to initialize market store:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Debug logging helper
   */
  function logDebug(message: string) {
    if (import.meta.env.DEV) {
      console.log(`[MarketStore] ${message}`);
    }
  }

  return {
    // State
    currentSymbol,
    currentTimeframe,
    tickers,
    klines,
    wsConnected,
    symbols,
    marketStatus,
    loading,
    error,

    // Getters
    currentTicker,
    currentKlines,
    subscribedSymbols,
    isConnected,

    // Actions
    loadSymbols,
    loadKlines,
    subscribeTicker,
    unsubscribeTicker,
    getMarketStatus,
    updateTicker,
    updateKline,
    updateKlines_batch,
    setCurrentSymbol,
    setCurrentTimeframe,
    clear,
    initialize,
  };
});
