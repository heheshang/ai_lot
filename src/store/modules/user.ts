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
  const isLoggedIn = computed(() => !!user.value && !!token.value);
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
  }

  async function fetchCurrentUser() {
    if (!token.value) return;

    try {
      const userId = user.value?.id;
      if (!userId) return;

      const currentUser = await api.userApi.getCurrentUser(userId);
      user.value = currentUser;
    } catch (error) {
      console.error('Fetch user failed:', error);
      logout();
    }
  }

  function initFromStorage() {
    const storedToken = localStorage.getItem('token');
    if (storedToken) {
      token.value = storedToken;
      // 解析用户ID从token (简化实现)
      const userId = storedToken.split(':')[0];
      if (userId) {
        fetchCurrentUser();
      }
    }
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
  };
});
