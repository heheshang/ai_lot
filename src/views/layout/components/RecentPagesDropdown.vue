<!--
  RecentPagesDropdown Component

  Displays navigation history and quick access as overlay panel.
  Matched style with CommandPalette component.

  @example
  ```vue
  <RecentPagesDropdown v-model:visible="showRecent" />
  ```

  @features
  - Full-screen overlay panel (like CommandPalette)
  - Quick access shortcuts grid
  - Recent page history with timestamps
  - Clear history functionality
  - Keyboard navigation support

  @author AI-LOT Team
-->
<template>
  <teleport to="body">
    <transition name="fade">
      <div v-if="visible" class="recent-overlay" @click="close">
        <div class="recent-container" @click.stop>
          <!-- Header -->
          <div class="recent-header-section">
            <div class="header-left">
              <div class="header-icon-wrapper">
                <el-icon class="header-icon"><Clock /></el-icon>
              </div>
              <div class="header-text">
                <div class="header-title">最近访问</div>
                <div class="header-subtitle">{{ recentPagesCount }} 个页面</div>
              </div>
            </div>
            <div class="header-actions">
              <el-button text size="small" class="clear-btn" @click="navigationStore.clearHistory()">
                <el-icon><Delete /></el-icon>
                清空历史
              </el-button>
              <div class="keyboard-hint">
                <kbd>ESC</kbd> 关闭
              </div>
            </div>
          </div>

          <!-- Content Area -->
          <div class="recent-content">
            <!-- Quick Access -->
            <div class="quick-section">
              <div class="section-title">快速访问</div>
              <div class="quick-grid">
                <div
                  v-for="item in quickItems"
                  :key="item.path"
                  :class="['quick-item', { 'is-active': route.path === item.path || route.path.startsWith(item.path + '/') }]"
                  @click="navigateTo(item.path)"
                >
                  <div class="quick-item-icon" :style="{ background: item.gradient }">
                    <el-icon><component :is="item.icon" /></el-icon>
                  </div>
                  <span class="quick-item-text">{{ item.title }}</span>
                </div>
              </div>
            </div>

            <!-- Recent List -->
            <div v-if="recentPages.length > 0" class="history-section">
              <div class="section-title">浏览历史</div>
              <div class="history-list">
                <div
                  v-for="(item, index) in recentPages"
                  :key="item.path"
                  :class="['history-item', { 'is-current': item.path === route.path }]"
                  @click="navigateTo(item.path)"
                >
                  <div class="history-item-icon">
                    <el-icon><component :is="getPageIcon(item.path)" /></el-icon>
                  </div>
                  <div class="history-item-content">
                    <div class="history-item-main">
                      <span class="history-title">{{ item.title }}</span>
                      <div class="history-item-badges">
                        <el-tag v-if="item.path === route.path" size="small" type="success" effect="dark" round>当前</el-tag>
                        <el-tag v-else-if="index === 0" size="small" type="danger" effect="plain" round>最新</el-tag>
                      </div>
                    </div>
                    <div class="history-item-meta">
                      <span class="history-time">{{ formatTime(item.timestamp) }}</span>
                    </div>
                  </div>
                  <div class="history-item-action">
                    <el-icon class="action-icon"><ArrowRight /></el-icon>
                  </div>
                </div>
              </div>
            </div>

            <!-- Empty State -->
            <div v-else class="empty-state">
              <el-icon class="empty-icon"><Clock /></el-icon>
              <p class="empty-text">暂无浏览历史</p>
              <p class="empty-hint">访问页面后会自动记录在这里</p>
            </div>
          </div>

          <!-- Footer Stats -->
          <div class="recent-footer">
            <div class="footer-stats">
              <div class="stat-item">
                <div class="stat-value">{{ navigationStore.history.length }}</div>
                <div class="stat-label">总访问</div>
              </div>
              <el-divider direction="vertical" />
              <div class="stat-item">
                <div class="stat-value">{{ uniquePagesCount }}</div>
                <div class="stat-label">页面数</div>
              </div>
            </div>
            <el-button text class="footer-home-btn" @click="navigateTo('/dashboard')">
              <el-icon><HomeFilled /></el-icon>
              返回首页
            </el-button>
          </div>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import {
  Clock,
  Delete,
  HomeFilled,
  TrendCharts,
  Grid,
  DataAnalysis,
  ArrowRight,
} from '@element-plus/icons-vue';
import { useNavigationStore } from '@/store';
import { formatTime } from '@/utils/date';
import { getPageIcon } from '@/utils/route';

defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const route = useRoute();
const router = useRouter();
const navigationStore = useNavigationStore();

// Quick access items
const quickItems = [
  { path: '/dashboard', title: '仪表盘', icon: HomeFilled, gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' },
  { path: '/market', title: '行情', icon: TrendCharts, gradient: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)' },
  { path: '/strategy', title: '策略', icon: Grid, gradient: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)' },
  { path: '/backtest', title: '回测', icon: DataAnalysis, gradient: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)' },
];

const recentPages = computed(() => navigationStore.getRecentPages(route.path, 10));
const recentPagesCount = computed(() => recentPages.value.length);
const uniquePagesCount = computed(() => new Set(navigationStore.history.map(h => h.path)).size);

// Navigate to path
function navigateTo(path: string) {
  router.push(path);
  close();
}

// Close panel
function close() {
  emit('update:visible', false);
}
</script>

<style scoped lang="scss">
.recent-overlay {
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
  from { opacity: 0; }
  to { opacity: 1; }
}

.recent-container {
  width: 600px;
  max-width: 90vw;
  max-height: 80vh;
  background: #fff;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  display: flex;
  flex-direction: column;
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

// Header Section
.recent-header-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid #e8ecf0;
  background: linear-gradient(180deg, #fafbfc 0%, #fff 100%);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 14px;
}

.header-icon-wrapper {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  box-shadow: 0 4px 12px rgba(14, 165, 233, 0.3);
}

.header-icon {
  font-size: 22px;
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.header-title {
  font-size: 16px;
  font-weight: 700;
  color: #303133;
}

.header-subtitle {
  font-size: 12px;
  color: #909399;
  font-weight: 500;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.clear-btn {
  color: #606266;
  font-weight: 500;

  &:hover {
    color: #dc2626;
  }
}

.keyboard-hint {
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

// Content Area
.recent-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px 24px;

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

.section-title {
  font-size: 11px;
  font-weight: 700;
  color: #909399;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  margin-bottom: 12px;
  padding: 0 4px;
}

// Quick Access Grid
.quick-section {
  margin-bottom: 24px;
}

.quick-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.quick-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 18px 12px;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
  border: 1px solid #e8ecf0;

  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    background: #fff;
  }

  &.is-active {
    border-color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
  }
}

.quick-item-icon {
  width: 42px;
  height: 42px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 20px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.quick-item-text {
  font-size: 13px;
  font-weight: 600;
  color: #303133;
}

// History Section
.history-section {
  margin-bottom: 8px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.history-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
  border: 1px solid #e8ecf0;

  &:hover {
    background: linear-gradient(135deg, #f5f7fa 0%, #ecf5ff 100%);
    transform: translateX(3px);
    border-color: #d9ecff;
  }

  &.is-current {
    background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
    border-color: #93c5fd;
  }
}

.history-item-icon {
  width: 36px;
  height: 36px;
  border-radius: 10px;
  background: linear-gradient(135deg, #fff 0%, #f5f7fa 100%);
  border: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: #606266;
  flex-shrink: 0;
}

.history-item-content {
  flex: 1;
  min-width: 0;
}

.history-item-main {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.history-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.history-item-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}

.history-time {
  font-size: 12px;
  color: #909399;
}

.history-item-action {
  flex-shrink: 0;
  opacity: 0;
  transition: opacity 0.2s ease;

  .action-icon {
    font-size: 16px;
    color: #909399;
  }
}

.history-item:hover .history-item-action {
  opacity: 1;
}

// Empty State
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60px 20px;
  color: #909399;
}

.empty-icon {
  font-size: 52px;
  margin-bottom: 16px;
  opacity: 0.45;
}

.empty-text {
  font-size: 15px;
  font-weight: 600;
  color: #606266;
  margin: 0 0 8px 0;
}

.empty-hint {
  font-size: 13px;
  color: #909399;
  margin: 0;
}

// Footer
.recent-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  border-top: 1px solid #e8ecf0;
  background: #fafbfc;
}

.footer-stats {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.stat-value {
  font-size: 18px;
  font-weight: 700;
  color: #303133;
}

.stat-label {
  font-size: 11px;
  color: #909399;
  font-weight: 600;
  text-transform: uppercase;
}

.footer-home-btn {
  font-size: 13px;
  font-weight: 500;
  color: #606266;

  &:hover {
    color: #409eff;
  }
}

// Transitions
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

// Responsive
@media (max-width: 768px) {
  .recent-container {
    width: 95vw;
  }

  .quick-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .header-actions {
    gap: 8px;
  }

  .keyboard-hint {
    display: none;
  }
}
</style>
