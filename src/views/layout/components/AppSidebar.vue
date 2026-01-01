<!--
  AppSidebar Component

  Main sidebar with logo, menu, and user info sections.

  @example
  ```vue
  <AppSidebar />
  ```

  @features
  - Collapsible sidebar with smooth transitions
  - Data-driven menu rendering from MENU_GROUPS
  - User info display at bottom
  - Active menu item highlighting
  - Responsive design for mobile/desktop

  @author AI-LOT Team
-->
<template>
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
        :default-active="activeMenuKey"
        :default-openeds="defaultOpeneds"
        :collapse="isCollapse"
        router
        class="app-menu"
      >
        <template v-for="group in menuGroups" :key="group.id">
          <!-- 分组标题 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>{{ group.label }}</span>
          </div>

          <!-- 菜单项 -->
          <template v-for="item in group.items" :key="item.path">
            <el-tooltip
              v-if="isCollapse && item.showWhenCollapsed !== false"
              :content="item.title"
              placement="right"
              :show-after="500"
            >
              <el-menu-item :index="item.path">
                <template #title>
                  <el-icon class="menu-icon">
                    <component :is="item.icon" />
                  </el-icon>
                  <span class="menu-text">{{ item.title }}</span>
                </template>
              </el-menu-item>
            </el-tooltip>
            <el-menu-item
              v-else-if="!isCollapse"
              :index="item.path"
            >
              <template #title>
                <el-icon class="menu-icon">
                  <component :is="item.icon" />
                </el-icon>
                <span class="menu-text">{{ item.title }}</span>
              </template>
            </el-menu-item>
          </template>
        </template>
      </el-menu>
    </div>

    <!-- 底部用户信息 -->
    <div class="user-section">
      <transition name="user-info">
        <div v-if="!isCollapse" class="user-card" @click="goToProfile">
          <div class="user-avatar">
            <el-avatar
              :size="36"
              :src="userStore.user?.avatar"
              :icon="UserFilled"
            />
          </div>
          <div class="user-details">
            <div class="user-name">{{ userStore.user?.displayName || userStore.username }}</div>
            <div class="user-role">{{ userStore.roleName }}</div>
          </div>
          <el-icon class="user-arrow"><ArrowRight /></el-icon>
        </div>
        <div v-else class="user-mini" @click="goToProfile">
          <el-avatar
            :size="32"
            :src="userStore.user?.avatar"
            :icon="UserFilled"
          />
        </div>
      </transition>
    </div>
  </el-aside>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { TrendCharts, UserFilled, ArrowRight } from '@element-plus/icons-vue';
import { useUserStore } from '@/store';
import { useLayoutState } from '@/composables/useLayoutState';
import { useMenuState } from '@/composables/useMenuState';
import { MENU_GROUPS } from '../constants/menu';

const router = useRouter();
const userStore = useUserStore();
const { isCollapse } = useLayoutState();
const { activeMenuKey, defaultOpeneds } = useMenuState();

const menuGroups = MENU_GROUPS;

function goToProfile() {
  router.push('/profile');
}
</script>

<style scoped lang="scss">
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

    .user-arrow {
      transform: translateX(4px);
      color: #fff;
    }
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

  .user-arrow {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.4);
    transition: all 0.25s ease;
    flex-shrink: 0;
  }
}

.user-mini {
  display: flex;
  justify-content: center;
  padding: 4px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    .el-avatar {
      transform: scale(1.1);
    }
  }

  &:active {
    .el-avatar {
      transform: scale(0.95);
    }
  }

  .el-avatar {
    transition: transform 0.2s ease;
  }
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
</style>