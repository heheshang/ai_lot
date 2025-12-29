import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { User, LoginRequest } from '@/types';
import * as api from '@/api/tauri';

export const useUserStore = defineStore('user', () => {
  // State
  const user = ref<User | null>(null);
  const token = ref<string | null>(null);
  const loading = ref(false);

  // Getters
  const isLoggedIn = computed(() => !!token.value); // 只要有 token 就认为已登录
  const username = computed(() => user.value?.username ?? '');
  const roleName = computed(() => user.value?.roleName ?? '');
  const hasPermission = computed(() => {
    return (permission: string) => {
      if (!user.value) return false;
      // 简单实现，管理员拥有全部权限
      // TODO: 实现 permission 参数的完整权限检查
      void permission; // 暂时忽略参数，避免 unused 警告
      return user.value.roleName === '管理员';
    };
  });

  // Actions
  async function login(request: LoginRequest) {
    loading.value = true;
    try {
      const response = await api.userApi.login(request);
      user.value = response.user;
      token.value = response.token;
      localStorage.setItem('token', response.token);
      localStorage.setItem('user', JSON.stringify(response.user)); // 同时保存用户信息
      return true;
    } catch (error) {
      console.error('Login failed:', error);
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function logout() {
    user.value = null;
    token.value = null;
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  }

  async function fetchCurrentUser() {
    if (!token.value) return;

    try {
      const userId = user.value?.id;
      if (!userId) return;

      const currentUser = await api.userApi.getCurrentUser(userId);
      user.value = currentUser;
      localStorage.setItem('user', JSON.stringify(currentUser)); // 更新缓存
    } catch (error) {
      console.error('Fetch user failed:', error);
      logout();
    }
  }

  function initFromStorage() {
    const storedToken = localStorage.getItem('token');
    const storedUser = localStorage.getItem('user');

    if (storedToken) {
      token.value = storedToken;
    }

    if (storedUser) {
      try {
        user.value = JSON.parse(storedUser);
      } catch (error) {
        console.error('Failed to parse stored user:', error);
        localStorage.removeItem('user');
      }
    }
  }

  /**
   * Restore user session from token
   * Used by route guard to restore user state
   */
  async function restoreUser() {
    const storedToken = localStorage.getItem('token');
    if (!storedToken) {
      throw new Error('No token found');
    }

    token.value = storedToken;

    // 尝试从 localStorage 恢复用户信息
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      try {
        user.value = JSON.parse(storedUser);
        return user.value;
      } catch (error) {
        console.error('Failed to parse stored user:', error);
      }
    }

    // 如果没有缓存的用户信息，尝试从服务器获取
    const userId = storedToken.split(':')[0];
    if (userId) {
      try {
        const currentUser = await api.userApi.getCurrentUser(userId);
        user.value = currentUser;
        localStorage.setItem('user', JSON.stringify(currentUser));
        return currentUser;
      } catch (error) {
        throw new Error('Failed to restore user');
      }
    }

    throw new Error('Invalid token format');
  }

  return {
    user,
    token,
    loading,
    isLoggedIn,
    username,
    roleName,
    hasPermission,
    login,
    logout,
    fetchCurrentUser,
    initFromStorage,
    restoreUser,
  };
});
