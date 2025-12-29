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
          :default-openeds="defaultOpeneds"
          :collapse="isCollapse"
          router
          class="app-menu"
        >
          <!-- 分组：概览 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>概览</span>
          </div>

          <el-tooltip v-if="isCollapse" content="仪表盘" placement="right" :show-after="500">
            <el-menu-item index="/dashboard">
              <template #title>
                <el-icon class="menu-icon"><Odometer /></el-icon>
                <span class="menu-text">仪表盘</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/dashboard">
            <template #title>
              <el-icon class="menu-icon"><Odometer /></el-icon>
              <span class="menu-text">仪表盘</span>
            </template>
          </el-menu-item>

          <!-- 分组：策略 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>策略</span>
          </div>

          <el-tooltip v-if="isCollapse" content="策略列表" placement="right" :show-after="500">
            <el-menu-item index="/strategy">
              <template #title>
                <el-icon class="menu-icon"><Grid /></el-icon>
                <span class="menu-text">策略列表</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy">
            <template #title>
              <el-icon class="menu-icon"><Grid /></el-icon>
              <span class="menu-text">策略列表</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="运行实例" placement="right" :show-after="500">
            <el-menu-item index="/strategy/instances">
              <template #title>
                <el-icon class="menu-icon"><VideoPlay /></el-icon>
                <span class="menu-text">运行实例</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy/instances">
            <template #title>
              <el-icon class="menu-icon"><VideoPlay /></el-icon>
              <span class="menu-text">运行实例</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="新建策略" placement="right" :show-after="500">
            <el-menu-item index="/strategy/editor">
              <template #title>
                <el-icon class="menu-icon"><Plus /></el-icon>
                <span class="menu-text">新建策略</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy/editor">
            <template #title>
              <el-icon class="menu-icon"><Plus /></el-icon>
              <span class="menu-text">新建策略</span>
            </template>
          </el-menu-item>

          <!-- 分组：市场 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>市场</span>
          </div>

          <el-tooltip v-if="isCollapse" content="行情" placement="right" :show-after="500">
            <el-menu-item index="/market">
              <template #title>
                <el-icon class="menu-icon"><TrendCharts /></el-icon>
                <span class="menu-text">行情</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/market">
            <template #title>
              <el-icon class="menu-icon"><TrendCharts /></el-icon>
              <span class="menu-text">行情</span>
            </template>
          </el-menu-item>

          <!-- 分组：交易 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>交易</span>
          </div>

          <el-tooltip v-if="isCollapse" content="回测" placement="right" :show-after="500">
            <el-menu-item index="/backtest">
              <template #title>
                <el-icon class="menu-icon"><DataAnalysis /></el-icon>
                <span class="menu-text">回测</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/backtest">
            <template #title>
              <el-icon class="menu-icon"><DataAnalysis /></el-icon>
              <span class="menu-text">回测</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="交易控制台" placement="right" :show-after="500">
            <el-menu-item index="/trade">
              <template #title>
                <el-icon class="menu-icon"><ShoppingCart /></el-icon>
                <span class="menu-text">交易控制台</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/trade">
            <template #title>
              <el-icon class="menu-icon"><ShoppingCart /></el-icon>
              <span class="menu-text">交易控制台</span>
            </template>
          </el-menu-item>

          <!-- 分组：系统 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>系统</span>
          </div>

          <el-tooltip v-if="isCollapse" content="风险监控" placement="right" :show-after="500">
            <el-menu-item index="/risk">
              <template #title>
                <el-icon class="menu-icon"><Warning /></el-icon>
                <span class="menu-text">风险监控</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/risk">
            <template #title>
              <el-icon class="menu-icon"><Warning /></el-icon>
              <span class="menu-text">风险监控</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="系统设置" placement="right" :show-after="500">
            <el-menu-item index="/settings">
              <template #title>
                <el-icon class="menu-icon"><Setting /></el-icon>
                <span class="menu-text">系统设置</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/settings">
            <template #title>
              <el-icon class="menu-icon"><Setting /></el-icon>
              <span class="menu-text">系统设置</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="交易所设置" placement="right" :show-after="500">
            <el-menu-item index="/settings/exchange">
              <template #title>
                <el-icon class="menu-icon"><Coin /></el-icon>
                <span class="menu-text">交易所设置</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/settings/exchange">
            <template #title>
              <el-icon class="menu-icon"><Coin /></el-icon>
              <span class="menu-text">交易所设置</span>
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
          <!-- 移动端菜单按钮 -->
          <el-icon class="mobile-menu-btn" @click="mobileMenuOpen = true">
            <Menu />
          </el-icon>

          <!-- 折叠按钮 -->
          <el-icon class="collapse-btn desktop-only" @click="toggleCollapse">
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

          <!-- 最近访问下拉 -->
          <el-dropdown v-if="navigationStore.history.length > 1" class="recent-dropdown">
            <div class="recent-trigger">
              <el-icon><Clock /></el-icon>
              <span>最近访问</span>
              <el-icon class="dropdown-icon"><ArrowDown /></el-icon>
            </div>
            <template #dropdown>
              <el-dropdown-menu>
                <div class="recent-header">
                  <span>最近访问</span>
                  <el-button
                    text
                    size="small"
                    @click="navigationStore.clearHistory()"
                  >
                    清空
                  </el-button>
                </div>
                <el-dropdown-item
                  v-for="item in navigationStore.getRecentPages(route.path, 8)"
                  :key="item.path"
                  @click="router.push(item.path)"
                >
                  <div class="recent-item">
                    <div class="recent-item-content">
                      <div class="recent-title">{{ item.title }}</div>
                      <div class="recent-time">{{ formatTime(item.timestamp) }}</div>
                    </div>
                  </div>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>

        <div class="header-right">
          <!-- 搜索框 -->
          <div class="search-box" @click="showCommandPalette = true">
            <el-input
              v-model="searchText"
              placeholder="搜索... (Ctrl+K)"
              :prefix-icon="Search"
              clearable
              class="search-input"
              readonly
            />
            <div class="search-shortcut">
              <kbd>⌘</kbd><kbd>K</kbd>
            </div>
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

    <!-- 快速跳转面板 -->
    <CommandPalette v-model:visible="showCommandPalette" />

    <!-- 移动端导航抽屉 -->
    <el-drawer
      v-model="mobileMenuOpen"
      direction="ltr"
      :size="280"
      class="mobile-drawer"
      :with-header="false"
    >
      <div class="mobile-menu">
        <!-- 关闭按钮 -->
        <div class="mobile-menu-header">
          <div class="logo-section">
            <div class="logo-wrapper">
              <div class="logo-icon">
                <el-icon :size="24"><TrendCharts /></el-icon>
              </div>
              <div class="logo-content">
                <div class="logo-title">AI-LOT</div>
                <div class="logo-subtitle">量化交易终端</div>
              </div>
            </div>
          </div>
          <el-icon class="close-btn" @click="mobileMenuOpen = false">
            <Close />
          </el-icon>
        </div>

        <!-- 菜单列表 -->
        <div class="mobile-menu-list">
          <div class="menu-group-title">概览</div>
          <div class="menu-item" @click="navigateTo('/dashboard')">
            <el-icon><Odometer /></el-icon>
            <span>仪表盘</span>
          </div>
          <div class="menu-item" @click="navigateTo('/market')">
            <el-icon><TrendCharts /></el-icon>
            <span>行情</span>
          </div>

          <div class="menu-group-title">策略</div>
          <div class="menu-item" @click="navigateTo('/strategy')">
            <el-icon><Grid /></el-icon>
            <span>策略列表</span>
          </div>
          <div class="menu-item" @click="navigateTo('/strategy/instances')">
            <el-icon><VideoPlay /></el-icon>
            <span>运行实例</span>
          </div>
          <div class="menu-item" @click="navigateTo('/strategy/editor')">
            <el-icon><Plus /></el-icon>
            <span>新建策略</span>
          </div>

          <div class="menu-group-title">交易</div>
          <div class="menu-item" @click="navigateTo('/backtest')">
            <el-icon><DataAnalysis /></el-icon>
            <span>回测</span>
          </div>
          <div class="menu-item" @click="navigateTo('/trade')">
            <el-icon><ShoppingCart /></el-icon>
            <span>交易控制台</span>
          </div>

          <div class="menu-group-title">系统</div>
          <div class="menu-item" @click="navigateTo('/risk')">
            <el-icon><Warning /></el-icon>
            <span>风险监控</span>
          </div>
          <div class="menu-item" @click="navigateTo('/settings')">
            <el-icon><Setting /></el-icon>
            <span>系统设置</span>
          </div>
          <div class="menu-item" @click="navigateTo('/settings/exchange')">
            <el-icon><Coin /></el-icon>
            <span>交易所设置</span>
          </div>
        </div>

        <!-- 用户信息 -->
        <div class="mobile-menu-footer">
          <div class="user-info">
            <el-avatar :size="40" :icon="UserFilled" />
            <div class="user-details">
              <div class="user-name">{{ userStore.username }}</div>
              <div class="user-role">{{ userStore.roleName }}</div>
            </div>
          </div>
          <el-button text @click="handleLogout">
            <el-icon><SwitchButton /></el-icon>
            退出登录
          </el-button>
        </div>
      </div>
    </el-drawer>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import {
  Odometer,
  TrendCharts,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Coin,
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
  Clock,
  Menu,
  Close,
} from '@element-plus/icons-vue';
import { useUserStore, useNavigationStore } from '@/store';
import CommandPalette from '@/components/CommandPalette.vue';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();
const navigationStore = useNavigationStore();

// 状态
const isCollapse = ref(false);
const searchText = ref('');
const notificationCount = ref(3);
const isDark = ref(false);
const showCommandPalette = ref(false);
const mobileMenuOpen = ref(false);

// 计算属性
const currentRoute = computed(() => route);

const activeMenu = computed(() => {
  const path = route.path;
  // 策略相关页面统一使用 strategy 作为父级菜单
  if (path.startsWith('/strategy')) {
    return path;
  }
  return path;
});

// 控制子菜单展开状态（目前没有子菜单，保留备用）
const defaultOpeneds = computed(() => {
  return [];
});

// 方法
function toggleCollapse() {
  isCollapse.value = !isCollapse.value;
}

// 移动端导航
function navigateTo(path: string) {
  router.push(path);
  mobileMenuOpen.value = false;
}

async function handleLogout() {
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
}

// 格式化时间戳
function formatTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp;

  const minute = 60 * 1000;
  const hour = 60 * minute;
  const day = 24 * hour;

  if (diff < minute) {
    return '刚刚';
  } else if (diff < hour) {
    return `${Math.floor(diff / minute)}分钟前`;
  } else if (diff < day) {
    return `${Math.floor(diff / hour)}小时前`;
  } else if (diff < 7 * day) {
    return `${Math.floor(diff / day)}天前`;
  } else {
    const date = new Date(timestamp);
    return `${date.getMonth() + 1}/${date.getDate()}`;
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

// 监听路由变化，添加到历史记录
watch(
  () => route.path,
  (newPath, oldPath) => {
    // 只在路径变化且不是登录页时记录
    if (newPath !== oldPath && newPath !== '/login' && oldPath !== '/login') {
      navigationStore.addToHistory(newPath, route.meta.title as string);
    }
  },
  { immediate: true }
);
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
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.1);
}

// Logo区域
.logo-section {
  flex-shrink: 0;
  padding: 20px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);
}

.logo-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  transition: all 0.3s ease;

  &:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: translateX(4px);
  }
}

.logo-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.5);
  transition: all 0.3s ease;

  .logo-wrapper:hover & {
    transform: rotate(5deg) scale(1.05);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
  }
}

.logo-content {
  flex: 1;
  min-width: 0;
}

.logo-title {
  font-size: 19px;
  font-weight: 700;
  color: #fff;
  line-height: 1.2;
  background: linear-gradient(135deg, #fff 0%, #e0e7ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: 0.5px;
}

.logo-subtitle {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.55);
  margin-top: 3px;
  font-weight: 500;
  letter-spacing: 0.3px;
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
  padding: 16px 12px;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;

    &:hover {
      background: rgba(255, 255, 255, 0.25);
    }
  }
}

.menu-group-title {
  padding: 12px 12px 8px 12px;
  font-size: 11px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.4);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 4px;
}

.app-menu {
  border-right: none;
  background: transparent;

  // 工具提示包裹的菜单项需要正确显示
  :deep(.el-tooltip__popper) {
    z-index: 10000;
  }

  :deep(.el-menu-item),
  :deep(.el-sub-menu__title) {
    color: rgba(255, 255, 255, 0.7);
    margin: 0 0 6px 0;
    border-radius: 12px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    height: 48px;
    line-height: 48px;
    position: relative;
    overflow: hidden;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
      opacity: 0;
      transition: opacity 0.25s ease;
      border-radius: 12px;
    }

    .menu-icon {
      font-size: 19px;
      margin-right: 10px;
      position: relative;
      z-index: 1;
      transition: transform 0.25s ease;
    }

    .menu-text {
      font-size: 14px;
      font-weight: 500;
      position: relative;
      z-index: 1;
    }
  }

  :deep(.el-menu-item:hover),
  :deep(.el-sub-menu__title:hover) {
    background: rgba(255, 255, 255, 0.06) !important;
    color: #fff;
    transform: translateX(4px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);

    &::before {
      opacity: 1;
    }

    .menu-icon {
      transform: scale(1.1);
    }
  }

  :deep(.el-menu-item.is-active) {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.25) 0%, rgba(118, 75, 162, 0.25) 100%) !important;
    color: #fff !important;
    box-shadow: 0 4px 16px rgba(102, 126, 234, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(102, 126, 234, 0.3);

    &::before {
      opacity: 1;
    }

    .menu-icon {
      color: #a5b4fc;
      filter: drop-shadow(0 0 8px rgba(165, 180, 252, 0.5));
    }
  }

  :deep(.el-sub-menu) {
    .el-sub-menu__title {
      .el-icon {
        font-size: 14px;
        margin-left: auto;
        transition: transform 0.25s ease;
      }
    }

    .el-menu {
      background: rgba(0, 0, 0, 0.15);
      margin: 4px 0 8px 0;
      border-radius: 8px;
      padding: 4px 0;

      .el-menu-item {
        padding-left: 52px !important;
        height: 42px;
        line-height: 42px;
        font-size: 13px;
        margin: 2px 8px;
        border-radius: 8px;

        &.is-active {
          background: rgba(102, 126, 234, 0.2) !important;
        }
      }
    }
  }
}

// 底部用户区域
.user-section {
  flex-shrink: 0;
  padding: 12px 12px 16px 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);
}

.user-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.25s ease;
  border: 1px solid rgba(255, 255, 255, 0.05);

  &:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
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
    font-weight: 600;
    color: #fff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .user-role {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.55);
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

// 最近访问下拉
.recent-dropdown {
  margin-left: 8px;

  .recent-trigger {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    border-radius: 10px;
    font-size: 13px;
    color: #606266;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    border: 1px solid transparent;
    font-weight: 500;

    &:hover {
      background: linear-gradient(135deg, #f5f7fa 0%, #ecf5ff 100%);
      color: #409eff;
      border-color: rgba(64, 158, 255, 0.15);
      transform: translateY(-1px);
      box-shadow: 0 2px 8px rgba(64, 158, 255, 0.15);
    }

    .el-icon {
      font-size: 15px;
    }

    .dropdown-icon {
      font-size: 12px;
      transition: transform 0.25s ease;
    }
  }

  &.is-active {
    .recent-trigger {
      background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
      color: #409eff;
      border-color: rgba(64, 158, 255, 0.2);

      .dropdown-icon {
        transform: rotate(180deg);
      }
    }
  }
}

:deep(.el-dropdown-menu) {
  .recent-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    font-size: 12px;
    font-weight: 600;
    color: #909399;
    border-bottom: 1px solid #ebeef5;
    background: #fafbfc;
  }

  .recent-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0;
    transition: background 0.2s ease;

    &:hover {
      background: #f5f7fa;
    }
  }

  .recent-item-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex: 1;
    min-width: 0;
  }

  .recent-title {
    font-size: 14px;
    color: #303133;
    font-weight: 500;
  }

  .recent-time {
    font-size: 11px;
    color: #909399;
    margin-left: 12px;
    flex-shrink: 0;
  }
}

.header-right {
  display: flex;
  align-items: center;
  gap: 18px;
}

// 搜索框
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

// 快捷操作
.quick-actions {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 12px;
  border-left: 1px solid #e8ecf0;
}

.action-icon {
  font-size: 18px;
  color: #606266;
  cursor: pointer;
  padding: 9px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: translateY(-1px);
  }

  &:active {
    transform: translateY(0);
  }
}

// 用户下拉菜单
.user-dropdown {
  .user-trigger {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 14px;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    border: 1px solid transparent;

    &:hover {
      background: linear-gradient(135deg, #f5f7fa 0%, #ecf5ff 100%);
      border-color: rgba(64, 158, 255, 0.15);
      transform: translateY(-1px);
      box-shadow: 0 2px 8px rgba(64, 158, 255, 0.1);
    }

    .user-info-text {
      display: flex;
      align-items: center;
      gap: 5px;

      .username {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
      }

      .dropdown-icon {
        font-size: 13px;
        color: #909399;
        transition: transform 0.25s ease;
      }
    }
  }

  &.is-active {
    .user-trigger {
      background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
      border-color: rgba(64, 158, 255, 0.25);

      .dropdown-icon {
        transform: rotate(180deg);
      }
    }
  }
}

// 用户下拉菜单样式
:deep(.el-dropdown-menu) {
  padding: 0;
  border-radius: 14px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
  border: 1px solid #ebeef5;
  overflow: hidden;

  .user-profile-header {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 24px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

    .el-avatar {
      background: rgba(255, 255, 255, 0.25);
      color: #fff;
      border: 3px solid rgba(255, 255, 255, 0.35);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .profile-info {
      flex: 1;

      .profile-name {
        font-size: 16px;
        font-weight: 700;
        color: #fff;
        margin-bottom: 4px;
        letter-spacing: 0.3px;
      }

      .profile-role {
        font-size: 12px;
        color: rgba(255, 255, 255, 0.85);
        font-weight: 500;
      }
    }
  }

  .el-dropdown-menu__item {
    padding: 12px 20px;
    font-size: 14px;
    color: #606266;
    transition: all 0.2s ease;
    font-weight: 500;

    .el-icon {
      margin-right: 10px;
      font-size: 16px;
      color: #909399;
      transition: all 0.2s ease;
    }

    &:hover {
      background: linear-gradient(90deg, #f5f7fa 0%, #ecf5ff 100%);
      color: #409eff;

      .el-icon {
        color: #409eff;
      }
    }

    &.logout-item {
      color: #f56c6c;
      border-top: 1px solid #ebeef5;
      margin-top: 4px;

      .el-icon {
        color: #f56c6c;
      }

      &:hover {
        background: linear-gradient(90deg, #fef0f0 0%, #fde2e2 100%);
        color: #f56c6c;

        .el-icon {
          color: #f56c6c;
        }
      }
    }
  }
}

// ========== 主内容区 ==========
.app-main {
  background: linear-gradient(180deg, #f5f7fa 0%, #f0f2f5 100%);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px;

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
  .app-aside {
    display: none;
  }

  .collapse-btn {
    display: none;
  }

  .mobile-menu-btn {
    display: flex !important;
  }

  .search-box {
    display: none;
  }

  .quick-actions {
    padding: 0 8px;
  }

  .user-info-text {
    display: none;
  }

  .breadcrumb {
    display: none;
  }

  .recent-dropdown {
    display: none;
  }

  .app-header {
    padding: 0 16px;
    height: 56px;
  }

  .app-main {
    padding: 16px;
  }
}

// ========== 移动端菜单 ==========
.mobile-menu-btn {
  display: none;
}

.desktop-only {
  @media (max-width: 768px) {
    display: none;
  }
}

:deep(.mobile-drawer) {
  .el-drawer__body {
    padding: 0;
  }
}

.mobile-menu {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: linear-gradient(180deg, #1a1d2d 0%, #141722 100%);
}

.mobile-menu-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);

  .logo-section {
    .logo-wrapper {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 6px;
      border-radius: 10px;
      background: rgba(255, 255, 255, 0.05);
    }

    .logo-icon {
      width: 42px;
      height: 42px;
      border-radius: 12px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      box-shadow: 0 4px 16px rgba(102, 126, 234, 0.5);
    }

    .logo-title {
      font-size: 18px;
      font-weight: 700;
      color: #fff;
      line-height: 1.2;
    }

    .logo-subtitle {
      font-size: 11px;
      color: rgba(255, 255, 255, 0.55);
      margin-top: 2px;
      font-weight: 500;
    }
  }

  .close-btn {
    font-size: 20px;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 10px;
    border-radius: 10px;
    transition: all 0.25s ease;

    &:hover {
      color: #fff;
      background: rgba(255, 255, 255, 0.1);
    }

    &:active {
      transform: scale(0.95);
    }
  }
}

.mobile-menu-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 14px;

  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;
  }

  .menu-group-title {
    padding: 14px 14px 10px;
    font-size: 11px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px 16px;
    border-radius: 12px;
    color: rgba(255, 255, 255, 0.75);
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    margin-bottom: 6px;
    font-weight: 500;

    .el-icon {
      font-size: 22px;
      transition: transform 0.25s ease;
    }

    span {
      font-size: 15px;
      font-weight: 500;
    }

    &:hover {
      background: rgba(255, 255, 255, 0.08);
      color: #fff;
      transform: translateX(4px);

      .el-icon {
        transform: scale(1.1);
      }
    }

    &:active {
      background: rgba(102, 126, 234, 0.25);
      color: #a5b4fc;
      transform: scale(0.98);
    }
  }
}

.mobile-menu-footer {
  padding: 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(0, 0, 0, 0.2);

  .user-info {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
    margin-bottom: 12px;

    .user-details {
      flex: 1;

      .user-name {
        font-size: 14px;
        font-weight: 500;
        color: #fff;
      }

      .user-role {
        font-size: 11px;
        color: rgba(255, 255, 255, 0.5);
        margin-top: 2px;
      }
    }
  }

  .el-button {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: rgba(255, 255, 255, 0.6);
    padding: 12px;
    border-radius: 10px;
    transition: all 0.2s ease;

    &:hover {
      background: rgba(255, 255, 255, 0.05);
      color: rgba(255, 255, 255, 0.8);
    }

    .el-icon {
      font-size: 16px;
    }
  }
}
</style>
