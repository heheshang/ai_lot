<template>
  <teleport to="body">
    <transition name="fade">
      <div v-if="visible" class="command-palette-overlay" @click="close">
        <div class="command-palette-container" @click.stop>
          <!-- 搜索框 -->
          <div class="search-section">
            <el-icon class="search-icon"><Search /></el-icon>
            <input
              ref="searchInput"
              v-model="searchQuery"
              type="text"
              class="search-input"
              placeholder="搜索页面或输入命令..."
              @keydown="handleKeydown"
            >
            <div class="keyboard-hint">
              <kbd>ESC</kbd> 关闭
            </div>
          </div>

          <!-- 结果列表 -->
          <div class="results-section">
            <div v-if="filteredItems.length === 0" class="no-results">
              <el-icon class="no-results-icon"><DocumentDelete /></el-icon>
              <p>未找到相关结果</p>
            </div>
            <div v-else class="results-list">
              <div
                v-for="(group, groupIndex) in groupedResults"
                :key="groupIndex"
                class="result-group"
              >
                <div class="group-title">{{ group.title }}</div>
                <div
                  v-for="(item, index) in group.items"
                  :key="item.path"
                  :class="[
                    'result-item',
                    { active: selectedIndex === getGlobalIndex(group, index) }
                  ]"
                  @click="selectItem(item)"
                  @mouseenter="selectedIndex = getGlobalIndex(group, index)"
                >
                  <div class="item-icon">
                    <component :is="item.icon" />
                  </div>
                  <div class="item-content">
                    <div class="item-title">{{ item.title }}</div>
                    <div v-if="item.description" class="item-description">
                      {{ item.description }}
                    </div>
                  </div>
                  <div v-if="item.shortcut" class="item-shortcut">
                    <kbd>{{ item.shortcut }}</kbd>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 底部提示 -->
          <div class="footer-hints">
            <div class="hint-item">
              <kbd>↑</kbd><kbd>↓</kbd> 导航
            </div>
            <div class="hint-item">
              <kbd>Enter</kbd> 选择
            </div>
          </div>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  Search,
  DocumentDelete,
  Odometer,
  TrendCharts,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Grid,
  VideoPlay,
  Plus,
} from '@element-plus/icons-vue';

interface CommandItem {
  title: string;
  description?: string;
  path: string;
  icon: any;
  shortcut?: string;
  category: 'navigation' | 'action' | 'settings';
}

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const router = useRouter();
const searchQuery = ref('');
const searchInput = ref<HTMLInputElement>();
const selectedIndex = ref(0);

// 导航项配置
const navigationItems: CommandItem[] = [
  {
    title: '仪表盘',
    description: '查看系统概览和统计数据',
    path: '/dashboard',
    icon: Odometer,
    category: 'navigation',
  },
  {
    title: '行情',
    description: '查看市场行情和K线图',
    path: '/market',
    icon: TrendCharts,
    category: 'navigation',
  },
  {
    title: '策略列表',
    description: '管理和查看所有策略',
    path: '/strategy',
    icon: Grid,
    category: 'navigation',
  },
  {
    title: '运行实例',
    description: '查看正在运行的策略实例',
    path: '/strategy/instances',
    icon: VideoPlay,
    category: 'navigation',
  },
  {
    title: '新建策略',
    description: '创建新的交易策略',
    path: '/strategy/editor',
    icon: Plus,
    category: 'navigation',
  },
  {
    title: '回测',
    description: '策略回测分析',
    path: '/backtest',
    icon: DataAnalysis,
    category: 'navigation',
  },
  {
    title: '交易控制台',
    description: '手动交易和订单管理',
    path: '/trade',
    icon: ShoppingCart,
    category: 'navigation',
  },
  {
    title: '风险监控',
    description: '风险指标和告警管理',
    path: '/risk',
    icon: Warning,
    category: 'navigation',
  },
  {
    title: '系统设置',
    description: '应用程序设置和配置',
    path: '/settings',
    icon: Setting,
    category: 'settings',
  },
];

// 过滤结果
const filteredItems = computed(() => {
  if (!searchQuery.value.trim()) {
    return navigationItems;
  }

  const query = searchQuery.value.toLowerCase();
  return navigationItems.filter(item => {
    return (
      item.title.toLowerCase().includes(query) ||
      (item.description && item.description.toLowerCase().includes(query))
    );
  });
});

// 分组结果
const groupedResults = computed(() => {
  const groups: { title: string; items: CommandItem[] }[] = [];
  const categories = [
    { key: 'navigation', title: '导航' },
    { key: 'action', title: '操作' },
    { key: 'settings', title: '设置' },
  ];

  categories.forEach(cat => {
    const items = filteredItems.value.filter(item => item.category === cat.key);
    if (items.length > 0) {
      groups.push({ title: cat.title, items });
    }
  });

  return groups;
});

// 获取全局索引
function getGlobalIndex(group: { title: string; items: CommandItem[] }, index: number) {
  let globalIndex = 0;
  for (const g of groupedResults.value) {
    if (g === group) {
      return globalIndex + index;
    }
    globalIndex += g.items.length;
  }
  return 0;
}

// 选择项
function selectItem(item: CommandItem) {
  router.push(item.path);
  close();
}

// 处理键盘事件
function handleKeydown(e: KeyboardEvent) {
  const totalItems = filteredItems.value.length;

  switch (e.key) {
    case 'ArrowDown':
      e.preventDefault();
      selectedIndex.value = (selectedIndex.value + 1) % totalItems;
      break;
    case 'ArrowUp':
      e.preventDefault();
      selectedIndex.value = selectedIndex.value === 0 ? totalItems - 1 : selectedIndex.value - 1;
      break;
    case 'Enter':
      e.preventDefault();
      if (filteredItems.value[selectedIndex.value]) {
        selectItem(filteredItems.value[selectedIndex.value]);
      }
      break;
    case 'Escape':
      close();
      break;
  }
}

// 关闭面板
function close() {
  emit('update:visible', false);
}

// 监听显示状态，自动聚焦搜索框
watch(() => props.visible, (visible) => {
  if (visible) {
    nextTick(() => {
      searchInput.value?.focus();
      selectedIndex.value = 0;
    });
  } else {
    searchQuery.value = '';
  }
});

// 全局键盘快捷键
function handleGlobalKeydown(e: KeyboardEvent) {
  // Cmd/Ctrl + K 打开快速跳转
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault();
    emit('update:visible', !props.visible);
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleGlobalKeydown);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleGlobalKeydown);
});
</script>

<style scoped lang="scss">
.command-palette-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(8px);
  z-index: 9999;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 12vh;
  animation: fadeIn 0.15s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.command-palette-container {
  width: 620px;
  max-width: 90vw;
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  animation: slideDown 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-30px) scale(0.96);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

// 搜索区域
.search-section {
  display: flex;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #e8ecf0;
  gap: 14px;
  background: linear-gradient(180deg, #fafbfc 0%, #fff 100%);
}

.search-icon {
  font-size: 22px;
  color: #909399;
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 16px;
  color: #303133;
  font-weight: 500;
  background: transparent;
  background: transparent;

  &::placeholder {
    color: #c0c4cc;
  }
}

.keyboard-hint {
  flex-shrink: 0;
  font-size: 12px;
  color: #909399;

  kbd {
    padding: 4px 8px;
    background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
    border: 1px solid #e4e7ed;
    border-radius: 6px;
    font-family: inherit;
    font-weight: 500;
  }
}

// 结果区域
.results-section {
  max-height: 420px;
  overflow-y: auto;
  padding: 12px 0;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: #e4e7ed;
    border-radius: 3px;

    &:hover {
      background: #d3d4d6;
    }
  }
}

.no-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60px 20px;
  color: #909399;
}

.no-results-icon {
  font-size: 52px;
  margin-bottom: 16px;
  opacity: 0.45;
}

.results-list {
  padding: 0 16px;
}

.result-group {
  margin-bottom: 12px;

  &:last-child {
    margin-bottom: 0;
  }
}

.group-title {
  padding: 10px 12px 8px;
  font-size: 11px;
  font-weight: 700;
  color: #909399;
  text-transform: uppercase;
  letter-spacing: 0.8px;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 12px 14px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  margin-bottom: 4px;

  &:hover {
    background: linear-gradient(135deg, #f5f7fa 0%, #ecf5ff 100%);
    transform: translateX(2px);
  }

  &.active {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    box-shadow: 0 2px 8px rgba(64, 158, 255, 0.15);

    .item-icon {
      color: #409eff;
    }

    .item-title {
      color: #409eff;
    }
  }
}

.item-icon {
  font-size: 22px;
  color: #606266;
  flex-shrink: 0;
  transition: all 0.2s ease;
}

.item-content {
  flex: 1;
  min-width: 0;
}

.item-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.item-description {
  font-size: 12px;
  color: #909399;
  margin-top: 3px;
}

.item-shortcut {
  flex-shrink: 0;

  kbd {
    padding: 3px 8px;
    font-size: 11px;
    background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
    border: 1px solid #e4e7ed;
    border-radius: 6px;
    font-family: inherit;
    color: #606266;
    font-weight: 500;
  }
}

// 底部提示
.footer-hints {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 28px;
  padding: 14px 16px;
  border-top: 1px solid #e8ecf0;
  background: #fafbfc;
}

.hint-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #909399;
  font-weight: 500;

  kbd {
    padding: 3px 6px;
    background: #fff;
    border: 1px solid #e4e7ed;
    border-radius: 5px;
    font-family: inherit;
    font-weight: 600;
  }
}

// 过渡动画
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
