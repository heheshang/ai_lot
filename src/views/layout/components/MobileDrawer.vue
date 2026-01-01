<!--
  MobileDrawer Component

  Mobile navigation menu drawer.

  @example
  ```vue
  <MobileDrawer v-model:open="isDrawerOpen" :size="280" />
  ```

  @features
  - Full-height navigation drawer
  - Data-driven menu from MENU_GROUPS
  - User profile section at bottom
  - Logout functionality
  - Smooth slide-in animation

  @props
  - open - Drawer open state (v-model)
  - size - Drawer width in pixels (default: 280)
  - direction - Slide direction: 'ltr' | 'rtl' (default: 'ltr')

  @events
  - update:open - Emitted when drawer state changes

  @author AI-LOT Team
-->
<template>
  <el-drawer
    v-model="isOpen"
    direction="ltr"
    :size="280"
    class="mobile-drawer"
    :with-header="false"
  >
    <div class="mobile-menu">
      <!-- Close Button -->
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
        <el-icon class="close-btn" @click="isOpen = false">
          <Close />
        </el-icon>
      </div>

      <!-- Menu List -->
      <div class="mobile-menu-list">
        <!-- Using data-driven menu from constants -->
        <template v-for="group in menuGroups" :key="group.id">
          <div class="menu-group-title">{{ group.label }}</div>
          <div
            v-for="item in group.items"
            :key="item.path"
            class="menu-item"
            :class="{ 'is-active': isActive(item.path) }"
            @click="navigateTo(item.path)"
          >
            <el-icon>
              <component :is="item.icon" />
            </el-icon>
            <span>{{ item.title }}</span>
          </div>
        </template>
      </div>

      <!-- User Info -->
      <div class="mobile-menu-footer">
        <div class="user-info" @click="goToProfile">
          <el-avatar :size="40" :src="userStore.user?.avatar" :icon="UserFilled" />
          <div class="user-details">
            <div class="user-name">{{ userStore.user?.displayName || userStore.username }}</div>
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
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { TrendCharts, Close, UserFilled, SwitchButton } from '@element-plus/icons-vue';
import { useUserStore } from '@/store';
import { useUserMenu } from '@/composables/useUserMenu';
import { MENU_GROUPS } from '../constants/menu';

const props = defineProps<{
  open: boolean;
}>();

const emit = defineEmits<{
  'update:open': [value: boolean];
}>();

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();
const { handleLogout } = useUserMenu();

const isOpen = computed({
  get: () => props.open,
  set: (value) => emit('update:open', value),
});

const menuGroups = MENU_GROUPS;

function isActive(path: string): boolean {
  return route.path === path || route.path.startsWith(path + '/');
}

function navigateTo(path: string) {
  router.push(path);
  isOpen.value = false;
}

function goToProfile() {
  router.push('/profile');
  isOpen.value = false;
}
</script>

<style scoped lang="scss">
:deep(.mobile-drawer) .el-drawer__body {
  padding: 0;
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

    &.is-active {
      background: rgba(102, 126, 234, 0.25);
      color: #a5b4fc;
      box-shadow: 0 0 0 1px rgba(165, 180, 252, 0.3);
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
    cursor: pointer;
    transition: all 0.2s ease;

    &:hover {
      background: rgba(255, 255, 255, 0.1);
    }

    &:active {
      transform: scale(0.98);
    }

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