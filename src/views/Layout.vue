<!--
  Optimized Layout Component
  Main application layout with extracted sub-components
-->
<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <AppSidebar />

    <!-- 主内容区 -->
    <el-container class="main-container">
      <!-- 顶栏 -->
      <AppHeader
        :notification-count="notificationCount"
        @open-mobile-menu="mobileMenuOpen = true"
        @toggle-collapse="toggleCollapse"
        @open-search="showCommandPalette = true"
        @open-recent="showRecent = true"
        @open-user="showUser = true"
        @show-notifications="showNotifications"
        @toggle-theme="toggleThemeWithMessage"
      />

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

    <!-- 最近访问面板 -->
    <RecentPagesDropdown v-model:visible="showRecent" />

    <!-- 用户资料面板 -->
    <UserDropdown v-model:visible="showUser" />

    <!-- 移动端导航抽屉 -->
    <MobileDrawer v-model:open="mobileMenuOpen" />
  </el-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useUserStore, useNavigationStore } from '@/store';
import { useLayoutState } from '@/composables/useLayoutState';
import { useUserMenu } from '@/composables/useUserMenu';
import AppSidebar from './layout/components/AppSidebar.vue';
import AppHeader from './layout/components/AppHeader.vue';
import RecentPagesDropdown from './layout/components/RecentPagesDropdown.vue';
import UserDropdown from './layout/components/UserDropdown.vue';
import MobileDrawer from './layout/components/MobileDrawer.vue';
import CommandPalette from '@/components/CommandPalette.vue';

const route = useRoute();
const userStore = useUserStore();
const navigationStore = useNavigationStore();
const { isCollapse, toggleCollapse, toggleTheme } = useLayoutState();
const { notificationCount, showNotifications: showNotificationsMsg } = useUserMenu();

// Local state
const showCommandPalette = ref(false);
const showRecent = ref(false);
const showUser = ref(false);
const mobileMenuOpen = ref(false);

// Track navigation history
watch(
  () => route.path,
  (newPath) => {
    navigationStore.addToHistory(newPath);
  },
  { immediate: true } // Record initial page load
);

// Methods
function showNotifications() {
  showNotificationsMsg();
}

function toggleThemeWithMessage() {
  toggleTheme();
  // ElMessage added in component
}
</script>

<style scoped lang="scss">
.app-layout {
  height: 100vh;
  display: flex;
  flex-direction: row;
  overflow: hidden;
}

.main-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  overflow: hidden;
}

.app-main {
  background: linear-gradient(180deg, #f5f7fa 0%, #f0f2f5 100%);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px;
  flex: 1;

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
  .app-main {
    padding: 16px;
  }
}
</style>

<!-- Global styles for Element Plus container components -->
<style lang="scss">
// 确保el-container基础样式正确
.app-layout.el-container {
  // 继承scoped样式中的display和flex-direction
}

.main-container.el-container {
  // 继承scoped样式中的display和flex-direction
}

// 确保el-aside的宽度正确
.app-aside {
  flex-shrink: 0 !important;
}

// 确保el-header不自动调整大小
.app-header {
  flex-shrink: 0 !important;
}

// 确保el-main占据剩余空间
.app-main {
  flex: 1 !important;
}
</style>