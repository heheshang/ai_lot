/**
 * Route and navigation utilities
 */

import type { Component } from 'vue';
import {
  Odometer,
  TrendCharts,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Coin,
  Grid,
  VideoPlay,
  Plus,
  HomeFilled,
} from '@element-plus/icons-vue';

/**
 * Icon mapping for routes
 */
const ICON_MAP: Record<string, Component> = {
  '/dashboard': Odometer,
  '/market': TrendCharts,
  '/strategy': Grid,
  '/strategy/editor': Plus,
  '/strategy/instances': VideoPlay,
  '/backtest': DataAnalysis,
  '/trade': ShoppingCart,
  '/risk': Warning,
  '/settings': Setting,
  '/settings/exchange': Coin,
};

/**
 * Get icon component for a given path
 * Uses exact match first, then prefix match
 */
export function getPageIcon(path: string): Component {
  // Exact match
  if (ICON_MAP[path]) {
    return ICON_MAP[path];
  }

  // Prefix match (sorted by key length descending for specific match)
  const sortedKeys = Object.keys(ICON_MAP).sort((a, b) => b.length - a.length);
  for (const key of sortedKeys) {
    if (path.startsWith(key) && key !== '/') {
      return ICON_MAP[key];
    }
  }

  return HomeFilled; // Default icon
}
