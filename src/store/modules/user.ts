/**
 * 用户状态管理 Store
 *
 * 使用安全存储替代 localStorage
 * 支持 JWT 访问令牌和刷新令牌
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { User, LoginRequest, RefreshTokenRequest } from '@/types';
import * as api from '@/api/tauri';
import {
  saveAccessToken,
  getAccessToken,
  saveRefreshToken,
  getRefreshToken,
  saveUser,
  getUser,
  clearAuthData,
  initSecureStorage,
  migrateFromLocalStorage,
} from '@/utils/secureStorage';

/**
 * 用户认证状态存储
 */
export const useUserStore = defineStore('user', () => {
  // State
  const user = ref<User | null>(null);
  const accessToken = ref<string | null>(null);
  const refreshToken = ref<string | null>(null);
  const loading = ref(false);
  const isInitialized = ref(false);

  // Getters
  const isLoggedIn = computed(() => !!accessToken.value);

  const username = computed(() => user.value?.username ?? '');

  const roleName = computed(() => user.value?.roleName ?? '');

  /**
   * 检查用户是否有指定权限
   * @param permission - 权限标识符
   * @returns 是否有权限
   */
  const hasPermission = computed(() => {
    return (permission: string) => {
      if (!user.value) return false;

      // 管理员拥有全部权限
      if (user.value.roleName === '管理员') {
        return true;
      }

      // 根据角色检查权限
      // TODO: 实现完整的权限检查系统
      // 当前简化实现：非管理员用户需要明确的权限检查
      const rolePermissions: Record<string, string[]> = {
        '策略开发者': ['strategy:read', 'strategy:write', 'backtest:execute', 'market:read'],
        '交易员': ['trade:execute', 'market:read', 'position:read', 'order:read', 'order:write'],
        '审计员': ['audit:read', 'trade:execute', 'position:read'],
      };

      const permissions = rolePermissions[user.value.roleName] || [];
      return permissions.includes(permission);
    };
  });

  /**
   * 用户登录
   * @param request - 登录请求
   * @returns 登录是否成功
   */
  async function login(request: LoginRequest): Promise<boolean> {
    loading.value = true;
    try {
      console.log('[UserStore] Attempting login with username:', request.username);
      const response = await api.userApi.login(request);
      console.log('[UserStore] Login response received:', response);

      // 保存用户信息
      user.value = response.user;
      console.log('[UserStore] User set:', user.value);

      // 保存令牌
      accessToken.value = response.access_token;
      refreshToken.value = response.refresh_token;
      console.log('[UserStore] Tokens saved');

      // 使用安全存储
      await saveAccessToken(response.access_token);
      await saveRefreshToken(response.refresh_token);
      await saveUser(response.user);
      console.log('[UserStore] Data persisted to secure storage');

      console.log('[UserStore] Login successful');
      return true;
    } catch (error) {
      console.error('[UserStore] Login failed:', error);
      console.error('[UserStore] Error details:', {
        message: (error as Error).message,
        stack: (error as Error).stack,
        name: (error as Error).name
      });
      return false;
    } finally {
      loading.value = false;
    }
  }

  /**
   * 用户登出
   */
  async function logout() {
    // 先调用后端退出登录API（记录审计日志）
    const currentUser = user.value;
    if (currentUser) {
      try {
        await api.userApi.logout({
          user_id: currentUser.id,
          username: currentUser.username,
        });
        console.log('[UserStore] Backend logout successful');
      } catch (error) {
        console.warn('[UserStore] Backend logout failed, continuing with local cleanup:', error);
      }
    }

    // 无论后端调用成功与否，都清除本地数据
    user.value = null;
    accessToken.value = null;
    refreshToken.value = null;

    await clearAuthData();
    console.log('[UserStore] Logout successful');
  }

  /**
   * 刷新访问令牌
   * @returns 刷新是否成功
   */
  async function refreshAccessToken(): Promise<boolean> {
    if (!refreshToken.value) {
      console.warn('[UserStore] No refresh token available');
      return false;
    }

    try {
      const request: RefreshTokenRequest = {
        refresh_token: refreshToken.value,
      };

      const response = await api.userApi.refreshAccessToken(request);

      // 更新访问令牌
      accessToken.value = response.access_token;
      await saveAccessToken(response.access_token);

      console.log('[UserStore] Access token refreshed');
      return true;
    } catch (error) {
      console.error('[UserStore] Failed to refresh access token:', error);

      // 如果刷新失败，清除所有认证数据
      await logout();
      return false;
    }
  }

  /**
   * 获取当前用户信息
   */
  async function fetchCurrentUser() {
    if (!accessToken.value) {
      console.warn('[UserStore] No access token, skipping fetch');
      return;
    }

    try {
      const userId = user.value?.id;
      if (!userId) {
        console.warn('[UserStore] No user ID, skipping fetch');
        return;
      }

      const currentUser = await api.userApi.getCurrentUser(userId);
      user.value = currentUser;
      await saveUser(currentUser);

      console.log('[UserStore] Current user fetched');
    } catch (error) {
      console.error('[UserStore] Fetch user failed:', error);

      // 如果是 401 错误，尝试刷新令牌
      if ((error as any)?.code === 'UNAUTHORIZED' || (error as any)?.status === 401) {
        const refreshed = await refreshAccessToken();
        if (refreshed) {
          // 刷新成功后重试
          await fetchCurrentUser();
        }
      } else {
        await logout();
      }
    }
  }

  /**
   * 从存储初始化状态
   * 应用启动时调用
   */
  async function initFromStorage() {
    if (isInitialized.value) {
      return;
    }

    try {
      // 初始化安全存储
      await initSecureStorage();

      // 尝试从 localStorage 迁移数据
      await migrateFromLocalStorage();

      // 从安全存储加载令牌和用户信息
      const storedAccessToken = await getAccessToken();
      const storedRefreshToken = await getRefreshToken();
      const storedUser = await getUser();

      if (storedAccessToken) {
        accessToken.value = storedAccessToken;
      }

      if (storedRefreshToken) {
        refreshToken.value = storedRefreshToken;
      }

      if (storedUser) {
        user.value = storedUser;
      }

      isInitialized.value = true;
      console.log('[UserStore] Initialized from storage');
    } catch (error) {
      console.error('[UserStore] Failed to initialize from storage:', error);
      isInitialized.value = true;
    }
  }

  /**
   * 从令牌恢复用户会话
   * 路由守卫使用
   */
  async function restoreUser(): Promise<User | null> {
    await initFromStorage();

    if (!accessToken.value) {
      console.warn('[UserStore] No access token found');
      return null;
    }

    // 如果有缓存的用户信息，直接返回
    if (user.value) {
      console.log('[UserStore] Restored user from cache');
      return user.value;
    }

    // 如果没有缓存的用户信息，尝试从服务器获取
    // 从 JWT token 中提取用户 ID（简化版本）
    try {
      // JWT payload 是 base64 编码的 JSON
      const payload = accessToken.value.split('.')[1];
      const decoded = JSON.parse(atob(payload));

      if (decoded.sub) {
        const currentUser = await api.userApi.getCurrentUser(decoded.sub);
        user.value = currentUser;
        await saveUser(currentUser);

        console.log('[UserStore] Restored user from server');
        return currentUser;
      }
    } catch (error) {
      console.error('[UserStore] Failed to restore user:', error);
    }

    // 如果所有方法都失败，清除认证数据
    await logout();
    return null;
  }

  return {
    // State
    user,
    accessToken,
    refreshToken,
    loading,
    isInitialized,

    // Getters
    isLoggedIn,
    username,
    roleName,
    hasPermission,

    // Actions
    login,
    logout,
    refreshAccessToken,
    fetchCurrentUser,
    initFromStorage,
    restoreUser,
  };
});