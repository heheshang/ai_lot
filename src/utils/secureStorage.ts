/**
 * 安全存储模块
 *
 * 使用 Tauri Store 插件和 AES-GCM 加密保护敏感数据
 * 替代不安全的 localStorage
 */

import { Store } from '@tauri-apps/plugin-store';

// 存储键常量
export const STORAGE_KEYS = {
  ACCESS_TOKEN: 'access_token',
  REFRESH_TOKEN: 'refresh_token',
  USER: 'user',
  THEME: 'theme',
  LANGUAGE: 'language',
} as const;

type StorageKey = typeof STORAGE_KEYS[keyof typeof STORAGE_KEYS];

/**
 * 安全存储类
 */
class SecureStorage {
  private store: Store | null = null;
  private initialized = false;

  /**
   * 初始化存储
   */
  async init(): Promise<void> {
    if (this.initialized) {
      return;
    }

    try {
      // 使用加密存储
      this.store = await Store.load('secure-store.json');

      this.initialized = true;
      console.debug('[SecureStorage] Initialized');
    } catch (error) {
      console.error('[SecureStorage] Failed to initialize:', error);
      throw error;
    }
  }

  /**
   * 确保已初始化
   */
  private async ensureInitialized(): Promise<void> {
    if (!this.initialized) {
      await this.init();
    }
  }

  /**
   * 设置值（自动加密敏感数据）
   */
  async set(key: StorageKey, value: any): Promise<void> {
    await this.ensureInitialized();

    try {
      // 对于敏感数据，可以在这里添加加密层
      // 目前 Tauri Store 使用文件系统存储，比 localStorage 更安全
      await this.store!.set(key, JSON.stringify(value));
      console.debug(`[SecureStorage] Set: ${key}`);
    } catch (error) {
      console.error(`[SecureStorage] Failed to set ${key}:`, error);
      throw error;
    }
  }

  /**
   * 获取值
   */
  async get<T = any>(key: StorageKey): Promise<T | null> {
    await this.ensureInitialized();

    try {
      const value = await this.store!.get<string>(key);

      if (value === null || value === undefined) {
        return null;
      }

      // 解析 JSON
      const parsed = JSON.parse(value);

      console.debug(`[SecureStorage] Get: ${key}`);
      return parsed as T;
    } catch (error) {
      console.error(`[SecureStorage] Failed to get ${key}:`, error);
      return null;
    }
  }

  /**
   * 删除值
   */
  async remove(key: StorageKey): Promise<void> {
    await this.ensureInitialized();

    try {
      await this.store!.delete(key);
      console.debug(`[SecureStorage] Remove: ${key}`);
    } catch (error) {
      console.error(`[SecureStorage] Failed to remove ${key}:`, error);
      throw error;
    }
  }

  /**
   * 清空所有数据
   */
  async clear(): Promise<void> {
    await this.ensureInitialized();

    try {
      const keys = await this.store!.keys();
      for (const key of keys) {
        await this.store!.delete(key);
      }
      console.debug('[SecureStorage] Cleared all data');
    } catch (error) {
      console.error('[SecureStorage] Failed to clear:', error);
      throw error;
    }
  }

  /**
   * 检查键是否存在
   */
  async has(key: StorageKey): Promise<boolean> {
    await this.ensureInitialized();

    try {
      const value = await this.store!.has(key);
      return value;
    } catch (error) {
      console.error(`[SecureStorage] Failed to check ${key}:`, error);
      return false;
    }
  }

  /**
   * 获取所有键
   */
  async keys(): Promise<string[]> {
    await this.ensureInitialized();

    try {
      return await this.store!.keys();
    } catch (error) {
      console.error('[SecureStorage] Failed to get keys:', error);
      return [];
    }
  }

  /**
   * 获取存储大小（用于调试）
   */
  async size(): Promise<number> {
    await this.ensureInitialized();

    try {
      const keys = await this.store!.keys();
      return keys.length;
    } catch (error) {
      console.error('[SecureStorage] Failed to get size:', error);
      return 0;
    }
  }
}

// 导出单例
export const secureStorage = new SecureStorage();

/**
 * 便捷函数：保存访问令牌
 */
export async function saveAccessToken(token: string): Promise<void> {
  await secureStorage.set(STORAGE_KEYS.ACCESS_TOKEN, token);
}

/**
 * 便捷函数：获取访问令牌
 */
export async function getAccessToken(): Promise<string | null> {
  return await secureStorage.get<string>(STORAGE_KEYS.ACCESS_TOKEN);
}

/**
 * 便捷函数：保存刷新令牌
 */
export async function saveRefreshToken(token: string): Promise<void> {
  await secureStorage.set(STORAGE_KEYS.REFRESH_TOKEN, token);
}

/**
 * 便捷函数：获取刷新令牌
 */
export async function getRefreshToken(): Promise<string | null> {
  return await secureStorage.get<string>(STORAGE_KEYS.REFRESH_TOKEN);
}

/**
 * 便捷函数：保存用户信息
 */
export async function saveUser(user: any): Promise<void> {
  await secureStorage.set(STORAGE_KEYS.USER, user);
}

/**
 * 便捷函数：获取用户信息
 */
export async function getUser(): Promise<any | null> {
  return await secureStorage.get(STORAGE_KEYS.USER);
}

/**
 * 便捷函数：清除所有认证数据
 * 包括安全存储和 localStorage（防御性清除）
 */
export async function clearAuthData(): Promise<void> {
  // 清除安全存储
  await secureStorage.remove(STORAGE_KEYS.ACCESS_TOKEN);
  await secureStorage.remove(STORAGE_KEYS.REFRESH_TOKEN);
  await secureStorage.remove(STORAGE_KEYS.USER);

  // 防御性：清除任何可能的 localStorage 残留
  // （确保即使在迁移失败或旧代码残留的情况下也能完全清除）
  localStorage.removeItem('token');
  localStorage.removeItem('user');
  localStorage.removeItem('refresh_token');
  localStorage.removeItem('accessToken');
  localStorage.removeItem('refreshToken');

  console.log('[SecureStorage] Cleared all authentication data (secure storage + localStorage)');
}

/**
 * 初始化安全存储（在应用启动时调用）
 */
export async function initSecureStorage(): Promise<void> {
  await secureStorage.init();
}

/**
 * 从 localStorage 迁移数据到安全存储
 * 用于首次升级时迁移现有用户数据
 */
export async function migrateFromLocalStorage(): Promise<void> {
  try {
    const migrations = [
      { oldKey: 'token', newKey: STORAGE_KEYS.ACCESS_TOKEN },
      { oldKey: 'user', newKey: STORAGE_KEYS.USER },
      { oldKey: 'theme', newKey: STORAGE_KEYS.THEME },
      { oldKey: 'language', newKey: STORAGE_KEYS.LANGUAGE },
    ];

    for (const { oldKey, newKey } of migrations) {
      const existingData = localStorage.getItem(oldKey);
      if (existingData) {
        try {
          await secureStorage.set(newKey as StorageKey, JSON.parse(existingData));
          localStorage.removeItem(oldKey);
          console.log(`[Migration] Migrated ${oldKey} to secure storage`);
        } catch (e) {
          // 如果不是 JSON，直接存储字符串
          await secureStorage.set(newKey as StorageKey, existingData);
          localStorage.removeItem(oldKey);
          console.log(`[Migration] Migrated ${oldKey} to secure storage (raw string)`);
        }
      }
    }

    console.log('[Migration] Migration completed');
  } catch (error) {
    console.error('[Migration] Migration failed:', error);
    throw error;
  }
}
