<!--
  AppHeader Component

  Main header with breadcrumb, search, quick actions, and user menu.

  @example
  ```vue
  <AppHeader
    :notification-count="3"
    @open-mobile-menu="handleMobileMenu"
    @toggle-collapse="toggleSidebar"
    @open-search="openSearch"
  />
  ```

  @features
  - Responsive breadcrumb navigation
  - Search box with keyboard shortcut (Ctrl+K)
  - Quick action buttons (theme, fullscreen, notifications)
  - Mobile menu button
  - Slot-based dropdown menus

  @slots
  - recent-dropdown - Slot for recent pages dropdown component
  - user-dropdown - Slot for user dropdown component

  @events
  - open-mobile-menu - Emitted when mobile menu button is clicked
  - toggle-collapse - Emitted when collapse toggle button is clicked
  - open-search - Emitted when search box is clicked
  - show-notifications - Emitted when notifications button is clicked
  - toggle-theme - Emitted when theme toggle button is clicked

  @author AI-LOT Team
-->
<template>
  <el-header class="app-header">
    <div class="header-left">
      <!-- 移动端菜单按钮 -->
      <el-icon class="mobile-menu-btn" @click="$emit('openMobileMenu')">
        <Menu />
      </el-icon>

      <!-- 折叠按钮 -->
      <el-icon class="collapse-btn desktop-only" @click="$emit('toggleCollapse')">
        <Fold v-if="!isCollapse" />
        <Expand v-else />
      </el-icon>

      <!-- 面包屑 -->
      <el-breadcrumb separator="/" class="breadcrumb">
        <el-breadcrumb-item :to="{ path: '/dashboard' }">
          <el-icon><HomeFilled /></el-icon>
          首页
        </el-breadcrumb-item>
        <el-breadcrumb-item v-if="currentRoute.meta.title">
          {{ currentRoute.meta.title }}
        </el-breadcrumb-item>
      </el-breadcrumb>

      <!-- 最近访问按钮 -->
      <el-tooltip content="最近访问 (Ctrl+H)" placement="bottom">
        <div class="recent-btn" @click="$emit('openRecent')">
          <el-icon class="recent-icon"><Clock /></el-icon>
          <span class="recent-text">最近访问</span>
        </div>
      </el-tooltip>
    </div>

    <div class="header-right">
      <!-- 搜索框 -->
      <div class="search-box" @click="$emit('openSearch')">
        <el-input
          placeholder="搜索... (Ctrl+K)"
          :prefix-icon="Search"
          readonly
          class="search-input"
        />
        <div class="search-shortcut">
          <kbd>⌘</kbd><kbd>K</kbd>
        </div>
      </div>

      <!-- 快捷操作 -->
      <div class="quick-actions">
        <!-- 通知 -->
        <el-tooltip content="通知" placement="bottom">
          <el-badge :value="notificationCount" :hidden="notificationCount === 0" class="action-item">
            <el-icon class="action-icon" @click="$emit('showNotifications')">
              <Bell />
            </el-icon>
          </el-badge>
        </el-tooltip>

        <!-- 全屏 -->
        <el-tooltip :content="isFullscreen ? '退出全屏' : '全屏'" placement="bottom">
          <el-icon class="action-icon" @click="toggleFullscreen">
            <FullScreen />
          </el-icon>
        </el-tooltip>

        <!-- 主题切换 -->
        <el-tooltip :content="isDark ? '浅色模式' : '深色模式'" placement="bottom">
          <el-icon class="action-icon theme-toggle" @click="$emit('toggleTheme')">
            <Sunny v-if="isDark" />
            <Moon v-else />
          </el-icon>
        </el-tooltip>
      </div>

      <!-- 分隔线 -->
      <el-divider direction="vertical" class="header-divider" />

      <!-- 用户头像按钮 -->
      <div class="user-trigger-btn" @click="$emit('openUser')">
        <el-avatar :size="36" :src="userStore.user?.avatar">
          <el-icon><UserFilled /></el-icon>
        </el-avatar>
        <span class="user-status-dot"></span>
      </div>
    </div>
  </el-header>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import {
  Menu,
  Fold,
  Expand,
  HomeFilled,
  UserFilled,
  Clock,
  Search,
  Bell,
  FullScreen,
  Sunny,
  Moon,
} from '@element-plus/icons-vue';
import { useLayoutState } from '@/composables/useLayoutState';
import { useFullscreen } from '@/composables/useFullscreen';
import { useUserStore } from '@/store';

defineProps<{
  notificationCount?: number;
}>();

defineEmits<{
  openMobileMenu: [];
  toggleCollapse: [];
  openSearch: [];
  openRecent: [];
  openUser: [];
  showNotifications: [];
  toggleTheme: [];
}>();

const route = useRoute();
const userStore = useUserStore();
const { isCollapse, isDark } = useLayoutState();
const { isFullscreen, toggleFullscreen } = useFullscreen();

const currentRoute = computed(() => route);
</script>

<style scoped lang="scss">
// Keyframe animations
@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.1); opacity: 0.9; }
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 64px;
  padding: 0 28px;
  background: linear-gradient(180deg, #ffffff 0%, #fafbfc 100%);
  border-bottom: 1px solid #e8ecf0;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.04);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.collapse-btn {
  font-size: 19px;
  cursor: pointer;
  color: #606266;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: scale(1.05);
  }

  &:active {
    transform: scale(0.95);
  }
}

.mobile-menu-btn {
  display: none;
  font-size: 20px;
  cursor: pointer;
  color: #606266;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: scale(1.05);
  }
}

.breadcrumb {
  :deep(.el-breadcrumb__item) {
    .el-breadcrumb__inner {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 13px;
      color: #606266;
      font-weight: 500;
      padding: 6px 12px;
      border-radius: 8px;
      transition: all 0.2s ease;

      &:hover {
        color: #409eff;
        background: #f5f7fa;
      }

      .el-icon {
        font-size: 14px;
      }
    }

    &:last-child {
      .el-breadcrumb__inner {
        color: #303133;
        font-weight: 600;

        &:hover {
          background: transparent;
        }
      }
    }

    .el-breadcrumb__separator {
      color: #c0c4cc;
      font-weight: 500;
    }
  }
}

.recent-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
  border: 1px solid #e8ecf0;

  .recent-icon {
    font-size: 16px;
    color: #606266;
    transition: all 0.2s ease;
  }

  .recent-text {
    font-size: 13px;
    font-weight: 600;
    color: #606266;
  }

  &:hover {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    border-color: #b3d8ff;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);

    .recent-icon {
      color: #409eff;
    }

    .recent-text {
      color: #409eff;
    }
  }

  &:active {
    transform: translateY(0);
  }
}

.user-trigger-btn {
  position: relative;
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 6px;
  border-radius: 12px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  .el-avatar {
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    border: 2px solid transparent;
  }

  .user-status-dot {
    position: absolute;
    bottom: 2px;
    right: 2px;
    width: 12px;
    height: 12px;
    background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
    border: 2px solid #fff;
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    animation: pulse 2.5s ease-in-out infinite;
  }

  &:hover {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);

    .el-avatar {
      border-color: #b3d8ff;
    }
  }

  &:active {
    transform: translateY(0);
  }
}

.header-right {
  display: flex;
  align-items: center;
  gap: 18px;
}

.search-box {
  position: relative;
  width: 280px;
  cursor: pointer;

  .search-input {
    :deep(.el-input__wrapper) {
      border-radius: 12px;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.04);
      transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
      cursor: pointer;
      border: 1px solid #e8ecf0;

      &:hover {
        box-shadow: 0 3px 10px rgba(0, 0, 0, 0.08);
        border-color: #d9d9d9;
      }
    }

    :deep(.el-input__inner) {
      cursor: pointer;
      font-size: 13px;
    }
  }

  .search-shortcut {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    gap: 2px;
    pointer-events: none;

    kbd {
      padding: 3px 6px;
      font-size: 10px;
      background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
      border: 1px solid #e4e7ed;
      border-radius: 6px;
      font-family: inherit;
      color: #606266;
      font-weight: 500;
    }
  }
}

.quick-actions {
  display: flex;
  align-items: center;
  gap: 4px;

  .el-badge {
    display: flex;
    align-items: center;
  }
}

.action-icon {
  font-size: 18px;
  color: #606266;
  cursor: pointer;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.2);
  }

  &:active {
    transform: translateY(0);
  }

  &.theme-toggle:hover {
    background: linear-gradient(135deg, #fff7e6 0%, #ffe7ba 100%);
    color: #e6a23c;
    box-shadow: 0 4px 12px rgba(230, 162, 60, 0.2);
  }
}

.header-divider {
  height: 24px;
  margin: 0 8px;
  border-color: #e8ecf0;
}

.desktop-only {
  @media (max-width: 768px) {
    display: none;
  }
}

// ========== 响应式 ==========
@media (max-width: 768px) {
  .mobile-menu-btn {
    display: flex !important;
  }

  .recent-btn {
    display: none;
  }

  .search-box {
    display: none;
  }

  .quick-actions {
    padding: 0;
    gap: 2px;
  }

  .header-divider {
    display: none;
  }

  .breadcrumb {
    display: none;
  }

  .app-header {
    padding: 0 16px;
    height: 56px;
  }
}
</style>