/**
 * Menu configuration constants
 */

import type { Component } from 'vue';
import {
  Odometer,
  TrendCharts,
  Grid,
  VideoPlay,
  Plus,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Coin,
  Document,
} from '@element-plus/icons-vue';

/**
 * Menu item interface
 */
export interface MenuItem {
  /** Route path */
  path: string;
  /** Display title */
  title: string;
  /** Icon component */
  icon: Component;
  /** Group label (for sectioning) */
  group?: string;
  /** Whether to show in collapsed sidebar */
  showWhenCollapsed?: boolean;
}

/**
 * Menu groups configuration
 */
export interface MenuGroup {
  /** Group identifier */
  id: string;
  /** Display label */
  label: string;
  /** Menu items in this group */
  items: MenuItem[];
}

/**
 * Main menu configuration
 */
export const MENU_GROUPS: MenuGroup[] = [
  {
    id: 'overview',
    label: '概览',
    items: [
      { path: '/dashboard', title: '仪表盘', icon: Odometer, showWhenCollapsed: true },
      { path: '/market', title: '行情', icon: TrendCharts, showWhenCollapsed: true },
    ],
  },
  {
    id: 'strategy',
    label: '策略',
    items: [
      { path: '/strategy', title: '策略列表', icon: Grid, showWhenCollapsed: true },
      { path: '/strategy/instances', title: '运行实例', icon: VideoPlay, showWhenCollapsed: true },
      { path: '/strategy/editor', title: '新建策略', icon: Plus, showWhenCollapsed: true },
    ],
  },
  {
    id: 'trading',
    label: '交易',
    items: [
      { path: '/backtest', title: '回测', icon: DataAnalysis, showWhenCollapsed: true },
      { path: '/trade', title: '交易控制台', icon: ShoppingCart, showWhenCollapsed: true },
    ],
  },
  {
    id: 'system',
    label: '系统',
    items: [
      { path: '/risk', title: '风险监控', icon: Warning, showWhenCollapsed: true },
      { path: '/admin/audit-logs', title: '审计日志', icon: Document, showWhenCollapsed: true },
      { path: '/settings', title: '系统设置', icon: Setting, showWhenCollapsed: true },
      { path: '/settings/exchange', title: '交易所设置', icon: Coin, showWhenCollapsed: true },
    ],
  },
];

/**
 * Flattened menu items array for easy iteration
 */
export const MENU_ITEMS = MENU_GROUPS.flatMap(group => group.items);

/**
 * Get menu item by path
 */
export function getMenuItemByPath(path: string): MenuItem | undefined {
  return MENU_ITEMS.find(item => item.path === path || path.startsWith(item.path + '/'));
}

/**
 * Get menu group by path
 */
export function getMenuGroupByPath(path: string): MenuGroup | undefined {
  return MENU_GROUPS.find(group =>
    group.items.some(item => item.path === path || path.startsWith(item.path + '/'))
  );
}

/**
 * Quick access items for recent pages dropdown
 */
export const quickAccessItems = [
  { key: 'dashboard', path: '/dashboard', title: '仪表盘', icon: Odometer, gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' },
  { key: 'market', path: '/market', title: '行情', icon: TrendCharts, gradient: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)' },
  { key: 'strategy', path: '/strategy', title: '策略', icon: Grid, gradient: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)' },
  { key: 'trade', path: '/trade', title: '交易', icon: ShoppingCart, gradient: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)' },
];
