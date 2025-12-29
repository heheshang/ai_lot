import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface HistoryItem {
  path: string;
  title: string;
  timestamp: number;
}

export const useNavigationStore = defineStore('navigation', () => {
  // 状态
  const history = ref<HistoryItem[]>([]);
  const maxHistorySize = 20;

  // 页面标题映射
  const pageTitles: Record<string, string> = {
    '/dashboard': '仪表盘',
    '/market': '行情',
    '/strategy': '策略列表',
    '/strategy/instances': '运行实例',
    '/strategy/editor': '策略编辑器',
    '/backtest': '回测',
    '/trade': '交易控制台',
    '/risk': '风险监控',
    '/risk/dashboard': '风险概览',
    '/risk/rules': '规则配置',
    '/risk/alerts': '告警历史',
    '/settings': '系统设置',
  };

  // 添加到历史记录
  function addToHistory(path: string, title?: string) {
    const pageTitle = title || pageTitles[path] || '未知页面';

    // 移除重复项
    const existingIndex = history.value.findIndex(item => item.path === path);
    if (existingIndex !== -1) {
      history.value.splice(existingIndex, 1);
    }

    // 添加到前面
    history.value.unshift({
      path,
      title: pageTitle,
      timestamp: Date.now(),
    });

    // 限制历史记录大小
    if (history.value.length > maxHistorySize) {
      history.value = history.value.slice(0, maxHistorySize);
    }
  }

  // 清除历史记录
  function clearHistory() {
    history.value = [];
  }

  // 移除特定历史项
  function removeFromHistory(path: string) {
    const index = history.value.findIndex(item => item.path === path);
    if (index !== -1) {
      history.value.splice(index, 1);
    }
  }

  // 获取最近访问的页面（排除当前页面）
  function getRecentPages(currentPath: string, limit = 5) {
    return history.value.filter(item => item.path !== currentPath).slice(0, limit);
  }

  return {
    history,
    addToHistory,
    clearHistory,
    removeFromHistory,
    getRecentPages,
  };
});
