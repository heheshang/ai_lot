<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapse ? '64px' : '240px'" class="app-aside">
      <!-- Logo区域 -->
      <div class="logo-section">
        <div class="logo-wrapper">
          <div class="logo-icon">
            <el-icon :size="isCollapse ? 24 : 28"><TrendCharts /></el-icon>
          </div>
          <transition name="logo-text">
            <div v-if="!isCollapse" class="logo-content">
              <div class="logo-title">AI-LOT</div>
              <div class="logo-subtitle">量化交易终端</div>
            </div>
          </transition>
        </div>
      </div>

      <!-- 菜单区域 -->
      <div class="menu-section">
        <el-menu
          :default-active="activeMenu"
          :collapse="isCollapse"
          router
          class="app-menu"
        >
          <!-- 分组：概览 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>概览</span>
          </div>

          <el-menu-item index="/dashboard">
            <template #title>
              <el-icon class="menu-icon"><Odometer /></el-icon>
              <span class="menu-text">仪表盘</span>
            </template>
          </el-menu-item>

          <el-menu-item index="/market">
            <template #title>
              <el-icon class="menu-icon"><TrendCharts /></el-icon>
              <span class="menu-text">行情</span>
            </template>
          </el-menu-item>

          <!-- 分组：策略 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>策略</span>
          </div>

          <el-sub-menu index="strategy">
            <template #title>
              <el-icon class="menu-icon"><Document /></el-icon>
              <span class="menu-text">策略管理</span>
            </template>
            <el-menu-item index="/strategy">
              <el-icon><Grid /></el-icon>
              策略列表
            </el-menu-item>
            <el-menu-item index="/strategy/instances">
              <el-icon><VideoPlay /></el-icon>
              运行实例
            </el-menu-item>
            <el-menu-item index="/strategy/editor">
              <el-icon><Plus /></el-icon>
              新建策略
            </el-menu-item>
          </el-sub-menu>

          <!-- 分组：交易 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>交易</span>
          </div>

          <el-menu-item index="/backtest">
            <template #title>
              <el-icon class="menu-icon"><DataAnalysis /></el-icon>
              <span class="menu-text">回测</span>
            </template>
          </el-menu-item>

          <el-menu-item index="/trade">
            <template #title>
              <el-icon class="menu-icon"><ShoppingCart /></el-icon>
              <span class="menu-text">交易控制台</span>
            </template>
          </el-menu-item>

          <!-- 分组：系统 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>系统</span>
          </div>

          <el-menu-item index="/risk">
            <template #title>
              <el-icon class="menu-icon"><Warning /></el-icon>
              <span class="menu-text">风险监控</span>
            </template>
          </el-menu-item>

          <el-menu-item index="/settings">
            <template #title>
              <el-icon class="menu-icon"><Setting /></el-icon>
              <span class="menu-text">系统设置</span>
            </template>
          </el-menu-item>
        </el-menu>
      </div>

      <!-- 底部用户信息 -->
      <div class="user-section">
        <transition name="user-info">
          <div v-if="!isCollapse" class="user-card">
            <div class="user-avatar">
              <el-avatar :size="36" :icon="UserFilled" />
            </div>
            <div class="user-details">
              <div class="user-name">{{ userStore.username }}</div>
              <div class="user-role">{{ userStore.roleName }}</div>
            </div>
          </div>
          <div v-else class="user-mini">
            <el-avatar :size="32" :icon="UserFilled" />
          </div>
        </transition>
      </div>
    </el-aside>

    <!-- 主内容区 -->
    <el-container class="main-container">
      <!-- 顶栏 -->
      <el-header class="app-header">
        <div class="header-left">
          <!-- 折叠按钮 -->
          <el-icon class="collapse-btn" @click="toggleCollapse">
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
        </div>

        <div class="header-right">
          <!-- 搜索框 -->
          <div class="search-box">
            <el-input
              v-model="searchText"
              placeholder="搜索..."
              :prefix-icon="Search"
              clearable
              class="search-input"
              @keyup.enter="handleSearch"
            />
          </div>

          <!-- 快捷操作 -->
          <div class="quick-actions">
            <!-- 通知 -->
            <el-badge :value="notificationCount" :hidden="notificationCount === 0" class="action-item">
              <el-icon class="action-icon" @click="showNotifications">
                <Bell />
              </el-icon>
            </el-badge>

            <!-- 全屏 -->
            <el-icon class="action-icon" @click="toggleFullscreen">
              <FullScreen />
            </el-icon>

            <!-- 主题切换 -->
            <el-icon class="action-icon" @click="toggleTheme">
              <Sunny v-if="isDark" />
              <Moon v-else />
            </el-icon>
          </div>

          <!-- 用户菜单 -->
          <el-dropdown @command="handleCommand" class="user-dropdown">
            <div class="user-trigger">
              <el-avatar :size="36" :icon="UserFilled" />
              <span class="user-info-text">
                <span class="username">{{ userStore.username }}</span>
                <el-icon class="dropdown-icon"><ArrowDown /></el-icon>
              </span>
            </div>
            <template #dropdown>
              <el-dropdown-menu>
                <div class="user-profile-header">
                  <el-avatar :size="48" :icon="UserFilled" />
                  <div class="profile-info">
                    <div class="profile-name">{{ userStore.username }}</div>
                    <div class="profile-role">{{ userStore.roleName }}</div>
                  </div>
                </div>
                <el-dropdown-item divided command="profile">
                  <el-icon><User /></el-icon>
                  个人资料
                </el-dropdown-item>
                <el-dropdown-item command="settings">
                  <el-icon><Setting /></el-icon>
                  系统设置
                </el-dropdown-item>
                <el-dropdown-item divided command="logout" class="logout-item">
                  <el-icon><SwitchButton /></el-icon>
                  退出登录
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <!-- 主内容 -->
      <el-main class="app-main">
        <router-view v-slot="{ Component }">
          <transition name="page" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import {
  Odometer,
  TrendCharts,
  Document,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Fold,
  Expand,
  UserFilled,
  User,
  SwitchButton,
  Search,
  Bell,
  FullScreen,
  Sunny,
  Moon,
  ArrowDown,
  HomeFilled,
  Grid,
  VideoPlay,
  Plus,
} from '@element-plus/icons-vue';
import { useUserStore } from '@/store';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();

// 状态
const isCollapse = ref(false);
const searchText = ref('');
const notificationCount = ref(3);
const isDark = ref(false);

// 计算属性
const currentRoute = computed(() => route);

const activeMenu = computed(() => {
  const path = route.path;
  if (path.startsWith('/strategy/editor')) {
    return '/strategy/editor';
  }
  if (path.startsWith('/strategy/instances')) {
    return '/strategy/instances';
  }
  return path;
});

// 方法
function toggleCollapse() {
  isCollapse.value = !isCollapse.value;
}

function handleSearch() {
  if (searchText.value.trim()) {
    ElMessage.info(`搜索: ${searchText.value}`);
  }
}

function showNotifications() {
  ElMessage.info('通知功能开发中');
}

function toggleFullscreen() {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
}

function toggleTheme() {
  isDark.value = !isDark.value;
  ElMessage.success(isDark.value ? '已切换到深色模式' : '已切换到浅色模式');
}

async function handleCommand(command: string) {
  switch (command) {
    case 'profile':
      ElMessage.info('个人资料功能待实现');
      break;
    case 'settings':
      router.push('/settings');
      break;
    case 'logout':
      try {
        await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning',
        });

        await userStore.logout();
        ElMessage.success('已退出登录');
        router.push('/login');
      } catch {
        // 取消退出
      }
      break;
  }
}
</script>

<style scoped lang="scss">
.app-layout {
  height: 100vh;
  overflow: hidden;
}

// ========== 侧边栏 ==========
.app-aside {
  display: flex;
  flex-direction: column;
  background: linear-gradient(180deg, #1a1d2d 0%, #141722 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
}

// Logo区域
.logo-section {
  flex-shrink: 0;
  padding: 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.logo-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.logo-content {
  flex: 1;
  min-width: 0;
}

.logo-title {
  font-size: 18px;
  font-weight: 700;
  color: #fff;
  line-height: 1.2;
  background: linear-gradient(135deg, #fff 0%, #a5b4fc 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.logo-subtitle {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.5);
  margin-top: 2px;
}

// Logo文字过渡动画
.logo-text-enter-active,
.logo-text-leave-active {
  transition: all 0.3s ease;
}

.logo-text-enter-from,
.logo-text-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

// 菜单区域
.menu-section {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px 0;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;

    &:hover {
      background: rgba(255, 255, 255, 0.2);
    }
  }
}

.menu-group-title {
  padding: 16px 16px 8px 52px;
  font-size: 11px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.35);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.app-menu {
  border-right: none;
  background: transparent;

  :deep(.el-menu-item),
  :deep(.el-sub-menu__title) {
    color: rgba(255, 255, 255, 0.65);
    margin: 0 8px;
    border-radius: 8px;
    transition: all 0.2s ease;
    height: 44px;
    line-height: 44px;

    .menu-icon {
      font-size: 18px;
      margin-right: 8px;
    }

    .menu-text {
      font-size: 14px;
    }
  }

  :deep(.el-menu-item:hover),
  :deep(.el-sub-menu__title:hover) {
    background: rgba(255, 255, 255, 0.08) !important;
    color: #fff;
  }

  :deep(.el-menu-item.is-active) {
    background: linear-gradient(90deg, rgba(102, 126, 234, 0.2) 0%, rgba(102, 126, 234, 0.05) 100%) !important;
    color: #a5b4fc !important;
    position: relative;

    &::before {
      content: '';
      position: absolute;
      left: 0;
      top: 50%;
      transform: translateY(-50%);
      width: 3px;
      height: 20px;
      background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
      border-radius: 0 2px 2px 0;
    }
  }

  :deep(.el-sub-menu) {
    .el-sub-menu__title {
      .el-icon {
        font-size: 14px;
        margin-left: auto;
      }
    }

    .el-menu {
      background: rgba(0, 0, 0, 0.2);

      .el-menu-item {
        padding-left: 52px !important;
        height: 40px;
        line-height: 40px;

        &.is-active {
          background: rgba(102, 126, 234, 0.15) !important;
        }
      }
    }
  }
}

// 底部用户区域
.user-section {
  flex-shrink: 0;
  padding: 12px 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
  background: rgba(0, 0, 0, 0.2);
}

.user-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .user-avatar {
    flex-shrink: 0;
  }

  .user-details {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 13px;
    font-weight: 500;
    color: #fff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .user-role {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.5);
    margin-top: 2px;
  }
}

.user-mini {
  display: flex;
  justify-content: center;
  padding: 4px;
}

.user-info-enter-active,
.user-info-leave-active {
  transition: all 0.3s ease;
}

.user-info-enter-from,
.user-info-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

// ========== 主内容区 ==========
.main-container {
  flex: 1;
  overflow: hidden;
}

// ========== 顶部栏 ==========
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 60px;
  padding: 0 24px;
  background: #fff;
  border-bottom: 1px solid #ebeef5;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.collapse-btn {
  font-size: 18px;
  cursor: pointer;
  color: #606266;
  padding: 8px;
  border-radius: 8px;
  transition: all 0.2s ease;

  &:hover {
    color: #409eff;
    background: #ecf5ff;
  }
}

.breadcrumb {
  :deep(.el-breadcrumb__item) {
    .el-breadcrumb__inner {
      display: flex;
      align-items: center;
      gap: 4px;
      font-size: 13px;
      color: #606266;
      font-weight: 500;

      &:hover {
        color: #409eff;
      }

      .el-icon {
        font-size: 14px;
      }
    }

    &:last-child {
      .el-breadcrumb__inner {
        color: #303133;
      }
    }
  }
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

// 搜索框
.search-box {
  width: 240px;

  .search-input {
    :deep(.el-input__wrapper) {
      border-radius: 20px;
      box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
      transition: all 0.2s ease;

      &:hover {
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
      }

      &.is-focus {
        box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
      }
    }
  }
}

// 快捷操作
.quick-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 12px;
  border-left: 1px solid #ebeef5;
}

.action-icon {
  font-size: 18px;
  color: #606266;
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  transition: all 0.2s ease;

  &:hover {
    color: #409eff;
    background: #ecf5ff;
  }
}

// 用户下拉菜单
.user-dropdown {
  .user-trigger {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 12px;
    border-radius: 24px;
    cursor: pointer;
    transition: all 0.2s ease;

    &:hover {
      background: #f5f7fa;
    }

    .user-info-text {
      display: flex;
      align-items: center;
      gap: 4px;

      .username {
        font-size: 14px;
        font-weight: 500;
        color: #303133;
      }

      .dropdown-icon {
        font-size: 14px;
        color: #909399;
        transition: transform 0.2s ease;
      }
    }
  }

  &.is-active {
    .user-trigger {
      background: #ecf5ff;

      .dropdown-icon {
        transform: rotate(180deg);
      }
    }
  }
}

// 用户下拉菜单样式
:deep(.el-dropdown-menu) {
  padding: 0;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.12);
  border: none;
  overflow: hidden;

  .user-profile-header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 20px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

    .el-avatar {
      background: rgba(255, 255, 255, 0.2);
      color: #fff;
      border: 2px solid rgba(255, 255, 255, 0.3);
    }

    .profile-info {
      flex: 1;

      .profile-name {
        font-size: 15px;
        font-weight: 600;
        color: #fff;
        margin-bottom: 4px;
      }

      .profile-role {
        font-size: 12px;
        color: rgba(255, 255, 255, 0.8);
      }
    }
  }

  .el-dropdown-menu__item {
    padding: 12px 20px;
    font-size: 14px;
    color: #606266;
    transition: all 0.2s ease;

    .el-icon {
      margin-right: 8px;
      font-size: 16px;
    }

    &:hover {
      background: #f5f7fa;
      color: #409eff;
    }

    &.logout-item {
      color: #f56c6c;

      &:hover {
        background: #fef0f0;
        color: #f56c6c;
      }
    }
  }
}

// ========== 主内容区 ==========
.app-main {
  background: #f5f7fa;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 20px;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 6px;
    height: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 3px;

    &:hover {
      background: rgba(0, 0, 0, 0.2);
    }
  }
}

// ========== 页面过渡动画 ==========
.page-enter-active,
.page-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

// ========== 响应式 ==========
@media (max-width: 768px) {
  .search-box {
    display: none;
  }

  .quick-actions {
    padding: 0 8px;
  }

  .user-info-text {
    display: none;
  }
}
</style>
