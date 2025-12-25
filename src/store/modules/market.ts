import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Ticker, Kline } from '@/types';

export const useMarketStore = defineStore('market', () => {
  const currentSymbol = ref('BTCUSDT');
  const currentTimeframe = ref('1h');
  const tickers = ref<Map<string, Ticker>>(new Map());
  const klines = ref<Map<string, Kline[]>>(new Map());
  const wsConnected = ref(false);

  function setCurrentSymbol(symbol: string) {
    currentSymbol.value = symbol;
  }

  function setCurrentTimeframe(timeframe: string) {
    currentTimeframe.value = timeframe;
  }

  return {
    currentSymbol,
    currentTimeframe,
    tickers,
    klines,
    wsConnected,
    setCurrentSymbol,
    setCurrentTimeframe,
  };
});
