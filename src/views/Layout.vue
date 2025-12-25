<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapse ? '64px' : '200px'" class="app-aside">
      <div class="logo">
        <span v-if="!isCollapse">AI-LOT</span>
        <span v-else>AI</span>
      </div>

      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapse"
        router
        class="app-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>仪表盘</template>
        </el-menu-item>

        <el-menu-item index="/market">
          <el-icon><TrendCharts /></el-icon>
          <template #title>行情</template>
        </el-menu-item>

        <el-sub-menu index="strategy">
          <template #title>
            <el-icon><Document /></el-icon>
            <span>策略</span>
          </template>
          <el-menu-item index="/strategy">策略列表</el-menu-item>
          <el-menu-item index="/strategy/editor">新建策略</el-menu-item>
        </el-sub-menu>

        <el-menu-item index="/backtest">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>回测</template>
        </el-menu-item>

        <el-menu-item index="/trade">
          <el-icon><ShoppingCart /></el-icon>
          <template #title>交易</template>
        </el-menu-item>

        <el-menu-item index="/risk">
          <el-icon><Warning /></el-icon>
          <template #title>风控</template>
        </el-menu-item>

        <el-menu-item index="/settings">
          <el-icon><Setting /></el-icon>
          <template #title>设置</template>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <!-- 主内容区 -->
    <el-container>
      <!-- 顶栏 -->
      <el-header class="app-header">
        <div class="header-left">
          <el-icon
            class="collapse-btn"
            @click="isCollapse = !isCollapse"
          >
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>

          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">
              首页
            </el-breadcrumb-item>
            <el-breadcrumb-item v-if="currentRoute.meta.title">
              {{ currentRoute.meta.title }}
            </el-breadcrumb-item>
          </el-breadcrumb>
        </div>

        <div class="header-right">
          <!-- 用户信息 -->
          <el-dropdown @command="handleCommand">
            <span class="user-info">
              <el-avatar :size="32" :icon="UserFilled" />
              <span class="username">{{ userStore.username }}</span>
              <span class="role">({{ userStore.roleName }})</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="profile">
                  <el-icon><User /></el-icon>
                  个人资料
                </el-dropdown-item>
                <el-dropdown-item command="settings">
                  <el-icon><Setting /></el-icon>
                  设置
                </el-dropdown-item>
                <el-dropdown-item divided command="logout">
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
        <router-view />
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
} from '@element-plus/icons-vue';
import { useUserStore } from '@/store';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();

const isCollapse = ref(false);
const currentRoute = computed(() => route);

const activeMenu = computed(() => {
  const path = route.path;
  // 精确匹配策略编辑器路径
  if (path.startsWith('/strategy/editor')) {
    return '/strategy/editor';
  }
  return path;
});

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

<style scoped>
.app-layout {
  height: 100vh;
}

.app-aside {
  background-color: #304156;
  transition: width 0.3s;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
  color: #fff;
  background-color: #263445;
}

.app-menu {
  border-right: none;
  background-color: #304156;
}

:deep(.el-menu-item),
:deep(.el-sub-menu__title) {
  color: #bfcbd9;
}

:deep(.el-menu-item:hover),
:deep(.el-sub-menu__title:hover) {
  background-color: #263445 !important;
}

:deep(.el-menu-item.is-active) {
  color: #409eff !important;
  background-color: #263445 !important;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #e4e7ed;
  background-color: #fff;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.collapse-btn {
  font-size: 20px;
  cursor: pointer;
  color: #909399;
}

.collapse-btn:hover {
  color: #409eff;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.username {
  font-size: 14px;
  color: #303133;
}

.role {
  font-size: 12px;
  color: #909399;
}

.app-main {
  background-color: #f5f7fa;
  padding: 20px;
}
</style>
