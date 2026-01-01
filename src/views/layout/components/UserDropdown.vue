<!--
  UserDropdown Component

  Displays user profile, settings, and logout options as overlay panel.
  Matched style with CommandPalette component.

  @example
  ```vue
  <UserDropdown v-model:visible="showUser" />
  ```

  @features
  - Full-screen overlay panel (like CommandPalette)
  - User profile header with avatar and stats
  - Account management menu items
  - Quick action buttons
  - Logout with confirmation

  @author AI-LOT Team
-->
<template>
  <teleport to="body">
    <transition name="fade">
      <div v-if="visible" class="user-overlay" @click="close">
        <div class="user-container" @click.stop>
          <!-- Header with User Profile -->
          <div class="user-header-section">
            <div class="profile-background">
              <div class="background-pattern"></div>
            </div>
            <div class="profile-content">
              <div class="profile-avatar-wrapper">
                <div class="avatar-ring"></div>
                <el-avatar :size="72" :src="userStore.user?.avatar" :icon="UserFilled"></el-avatar>
                <span class="avatar-online-badge"></span>
              </div>
              <div class="profile-info">
                <div class="profile-name">{{ userStore.user?.displayName || userStore.username }}</div>
                <div class="profile-username">@{{ userStore.username }}</div>
                <div class="profile-meta">
                  <el-tag :type="getRoleTagType(userStore.roleName)" size="small" effect="dark">
                    {{ userStore.roleName }}
                  </el-tag>
                  <span class="profile-status">
                    <el-icon class="status-dot"><SuccessFilled /></el-icon>
                    <span class="status-text">在线</span>
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- Content Area -->
          <div class="user-content">
            <!-- User Stats -->
            <div class="user-stats">
              <div class="stat-item">
                <div class="stat-value">{{ userIdDisplay }}</div>
                <div class="stat-label">用户ID</div>
              </div>
              <el-divider direction="vertical"></el-divider>
              <div class="stat-item">
                <div class="stat-value">{{ userCreatedAtDisplay }}</div>
                <div class="stat-label">加入时间</div>
              </div>
              <el-divider direction="vertical"></el-divider>
              <div class="stat-item">
                <div class="stat-value">{{ userStatusDisplay }}</div>
                <div class="stat-label">状态</div>
              </div>
            </div>

            <!-- Menu Items -->
            <div class="menu-section">
              <div class="section-title">账户管理</div>
              <div class="menu-item" @click="handleMenuClick('profile')">
                <div class="menu-item-icon">
                  <el-icon><User /></el-icon>
                </div>
                <div class="menu-item-content">
                  <div class="menu-item-title">个人资料</div>
                  <div class="menu-item-desc">编辑头像、昵称等信息</div>
                </div>
                <el-icon class="menu-item-arrow"><ArrowRight /></el-icon>
              </div>
              <div class="menu-item" @click="handleMenuClick('settings')">
                <div class="menu-item-icon">
                  <el-icon><Setting /></el-icon>
                </div>
                <div class="menu-item-content">
                  <div class="menu-item-title">偏好设置</div>
                  <div class="menu-item-desc">主题、语言、通知等</div>
                </div>
                <el-icon class="menu-item-arrow"><ArrowRight /></el-icon>
              </div>
            </div>

            <!-- Quick Actions -->
            <div class="quick-actions-section">
              <div class="section-title">快速操作</div>
              <div class="quick-actions-grid">
                <div class="quick-action-item">
                  <el-icon><Message /></el-icon>
                  <span>私信</span>
                  <el-badge :value="notificationCount" :hidden="notificationCount === 0"></el-badge>
                </div>
                <div class="quick-action-item">
                  <el-icon><Bell /></el-icon>
                  <span>提醒</span>
                  <el-badge :value="notificationCount" :hidden="notificationCount === 0"></el-badge>
                </div>
                <div class="quick-action-item">
                  <el-icon><Star /></el-icon>
                  <span>收藏</span>
                </div>
                <div class="quick-action-item">
                  <el-icon><Clock /></el-icon>
                  <span>动态</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Footer Logout -->
          <div class="user-footer">
            <div class="logout-btn" @click="handleMenuClick('logout')">
              <el-icon><SwitchButton /></el-icon>
              <span>退出登录</span>
              <span class="logout-shortcut">⌘Q</span>
            </div>
            <div class="keyboard-hint">
              <kbd>ESC</kbd> 关闭
            </div>
          </div>
        </div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { UserFilled, SuccessFilled, User, Setting, Message, Bell, Star, Clock, SwitchButton, ArrowRight } from '@element-plus/icons-vue';
import { useUserStore } from '@/store';
import { useUserMenu } from '@/composables/useUserMenu';
import { formatDateShort } from '@/utils/date';
import { getRoleTagType, getStatusText } from '@/utils/user';
import type { UserMenuCommand } from '@/types/layout';

defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
}>();

const userStore = useUserStore();
const { handleCommand, notificationCount } = useUserMenu();

// Computed properties
const userIdDisplay = computed(() => userStore.user?.id?.slice(-6) || '------');
const userCreatedAtDisplay = computed(() => formatDateShort(userStore.user?.createdAt));
const userStatusDisplay = computed(() => getStatusText(userStore.user?.status));

// Handle menu click
function handleMenuClick(command: UserMenuCommand) {
  close();
  // Use nextTick to allow panel to close before navigation
  setTimeout(() => {
    handleCommand(command);
  }, 100);
}

// Close panel
function close() {
  emit('update:visible', false);
}
</script>

<style scoped lang="scss">
// Keyframe animations
@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.1); opacity: 0.9; }
}

@keyframes backgroundShift {
  0% { opacity: 1; }
  100% { opacity: 0.7; }
}

@keyframes ringRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.user-overlay {
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

.user-container {
  width: 560px;
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
.user-header-section {
  position: relative;
  overflow: hidden;
}

.profile-background {
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
  z-index: 0;
}

.background-pattern {
  position: absolute;
  inset: 0;
  background-image:
    radial-gradient(circle at 20% 50%, rgba(255, 255, 255, 0.12) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(255, 255, 255, 0.1) 0%, transparent 40%);
  animation: backgroundShift 12s ease-in-out infinite alternate;
}

.profile-content {
  position: relative;
  z-index: 1;
  padding: 32px 28px 28px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.profile-avatar-wrapper {
  position: relative;
}

.avatar-ring {
  position: absolute;
  inset: -8px;
  border-radius: 50%;
  padding: 4px;
  background: conic-gradient(from 0deg, rgba(255, 255, 255, 0.6), rgba(255, 255, 255, 0.2), rgba(255, 255, 255, 0.6));
  animation: ringRotate 10s linear infinite;
}

.profile-avatar-wrapper .el-avatar {
  background: rgba(255, 255, 255, 0.15);
  color: #fff;
  border: 4px solid rgba(255, 255, 255, 0.4);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
}

.avatar-online-badge {
  position: absolute;
  bottom: 4px;
  right: 4px;
  width: 20px;
  height: 20px;
  background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
  border: 3px solid #fff;
  border-radius: 50%;
  animation: pulse 2.5s ease-in-out infinite;
}

.profile-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.profile-name {
  font-size: 20px;
  font-weight: 800;
  color: #fff;
  text-shadow: 0 3px 8px rgba(0, 0, 0, 0.2);
}

.profile-username {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.85);
  font-weight: 600;
}

.profile-meta {
  display: flex;
  align-items: center;
  gap: 8px;
}

.profile-status {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: rgba(255, 255, 255, 0.95);
  background: rgba(255, 255, 255, 0.2);
  padding: 4px 12px;
  border-radius: 14px;
}

.status-dot {
  font-size: 10px;
}

// Content Area
.user-content {
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

// User Stats
.user-stats {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px;
  margin-bottom: 20px;
  background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
  border-radius: 12px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-value {
  font-size: 16px;
  font-weight: 800;
  background: linear-gradient(135deg, #0f172a 0%, #475569 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.stat-label {
  font-size: 11px;
  color: #64748b;
  font-weight: 700;
  text-transform: uppercase;
}

// Menu Section
.menu-section {
  margin-bottom: 20px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
  border: 1px solid #e8ecf0;
  margin-bottom: 8px;

  &:hover {
    background: linear-gradient(90deg, #f5f7fa 0%, #ecf5ff 100%);
    transform: translateX(3px);
    border-color: #d9ecff;
  }

  &:last-child {
    margin-bottom: 0;
  }
}

.menu-item-icon {
  width: 38px;
  height: 38px;
  border-radius: 10px;
  background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
  border: 1px solid #b3d8ff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: #409eff;
  flex-shrink: 0;
}

.menu-item-content {
  flex: 1;
  min-width: 0;
}

.menu-item-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.menu-item-desc {
  font-size: 12px;
  color: #909399;
  margin-top: 2px;
}

.menu-item-arrow {
  flex-shrink: 0;
  font-size: 14px;
  color: #c0c4cc;
}

// Quick Actions
.quick-actions-section {
  margin-bottom: 8px;
}

.quick-actions-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}

.quick-action-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px 12px;
  border-radius: 12px;
  background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
  border: 1px solid #e8ecf0;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);

  &:hover {
    background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
    transform: translateY(-2px);
    border-color: #b3d8ff;
  }

  .el-icon {
    font-size: 20px;
    color: #606266;
  }

  span {
    font-size: 12px;
    font-weight: 600;
    color: #606266;
  }
}

// Footer
.user-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 24px;
  border-top: 1px solid #e8ecf0;
  background: #fafbfc;
}

.logout-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 20px;
  border-radius: 10px;
  background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
  color: #dc2626;
  font-size: 14px;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
    transform: translateY(-1px);
  }

  .el-icon {
    font-size: 16px;
  }
}

.logout-shortcut {
  font-size: 11px;
  background: rgba(239, 68, 68, 0.1);
  padding: 3px 8px;
  border-radius: 6px;
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
  .user-container {
    width: 95vw;
  }

  .quick-actions-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .keyboard-hint {
    display: none;
  }
}
</style>
