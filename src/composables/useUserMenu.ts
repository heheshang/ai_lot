/**
 * User Menu Composable
 * Manages user dropdown menu state and actions
 *
 * @example
 * ```ts
 * const {
 *   userIdDisplay,
 *   userCreatedAtDisplay,
 *   userStatusDisplay,
 *   handleCommand,
 *   handleLogout,
 *   goToProfile
 * } = useUserMenu();
 * ```
 *
 * @returns {UserMenuReturn} User menu state and action methods
 */

import { ref, computed, type ComputedRef } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import { useUserStore } from '@/store/modules/user';
import { formatDateShort } from '@/utils/date';
import { getStatusText, getRoleTagType } from '@/utils/user';
import { useFullscreen } from './useFullscreen';
import type { UserMenuCommand } from '@/types/layout';

/**
 * User menu return interface
 */
export interface UserMenuReturn {
  /** Fullscreen state */
  isFullscreen: ReturnType<typeof ref<boolean>>;
  /** Notification count */
  notificationCount: ReturnType<typeof ref<number>>;
  /** User ID display (last 6 characters) */
  userIdDisplay: ComputedRef<string>;
  /** User account creation date display */
  userCreatedAtDisplay: ComputedRef<string>;
  /** User status display */
  userStatusDisplay: ComputedRef<string>;
  /** Handle menu dropdown command */
  handleCommand: (command: UserMenuCommand) => Promise<void>;
  /** Handle user logout */
  handleLogout: () => Promise<void>;
  /** Navigate to user profile */
  goToProfile: () => void;
  /** Toggle fullscreen mode */
  toggleFullscreen: () => void;
  /** Toggle theme (placeholder) */
  toggleTheme: () => void;
  /** Show notifications (placeholder) */
  showNotifications: () => void;
  /** Get role tag type */
  getRoleTagType: typeof getRoleTagType;
  /** Format date to short format */
  formatDateShort: typeof formatDateShort;
  /** Get user status text */
  getStatusText: typeof getStatusText;
}

/**
 * User menu composable
 * Manages user dropdown menu state and actions
 *
 * Provides user profile navigation, logout functionality,
 * and various user-related computed properties.
 *
 * @returns User menu state and action methods
 */
export function useUserMenu(): UserMenuReturn {
  const router = useRouter();
  const userStore = useUserStore();
  const { isFullscreen, toggleFullscreen } = useFullscreen();

  const notificationCount = ref<number>(3);

  // Computed properties
  const userIdDisplay = computed(() =>
    userStore.user?.id?.slice(-6) || '------'
  );

  const userCreatedAtDisplay = computed(() =>
    formatDateShort(userStore.user?.createdAt)
  );

  const userStatusDisplay = computed(() =>
    getStatusText(userStore.user?.status)
  );

  // Handle menu command
  async function handleCommand(command: UserMenuCommand) {
    switch (command) {
      case 'profile':
        router.push('/profile');
        break;
      case 'settings':
        router.push('/settings');
        break;
      case 'logout':
        await handleLogout();
        break;
    }
  }

  // Handle logout
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
      // User cancelled
    }
  }

  // Navigate to profile
  function goToProfile() {
    router.push('/profile');
  }

  // Toggle theme (placeholder for future implementation)
  function toggleTheme() {
    ElMessage.info('主题切换功能开发中');
  }

  // Show notifications (placeholder)
  function showNotifications() {
    ElMessage.info('通知功能开发中');
  }

  return {
    // State
    isFullscreen,
    notificationCount,

    // Computed
    userIdDisplay,
    userCreatedAtDisplay,
    userStatusDisplay,

    // Methods
    handleCommand,
    handleLogout,
    goToProfile,
    toggleFullscreen,
    toggleTheme,
    showNotifications,
    getRoleTagType,
    formatDateShort,
    getStatusText,
  };
}

// Default export
export default useUserMenu;