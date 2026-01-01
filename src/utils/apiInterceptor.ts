/**
 * API 请求拦截器
 *
 * 功能：
 * - 自动注入认证令牌
 * - 处理令牌过期并自动刷新
 * - 统一错误处理
 * - 请求重试逻辑
 */

import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { getAccessToken } from './secureStorage';
import { useUserStore } from '@/store/modules/user';

/**
 * API 错误类型
 */
export class ApiInterceptorError extends Error {
  code: string;
  details?: string;
  isAuthError: boolean;

  constructor(message: string, code: string, details?: string) {
    super(message);
    this.name = 'ApiInterceptorError';
    this.code = code;
    this.details = details;
    this.isAuthError = code === 'UNAUTHORIZED' || code === 'TOKEN_EXPIRED';
  }
}

/**
 * 拦截器配置
 */
interface InterceptorConfig {
  retryAttempts: number;
  retryDelay: number;
  enableAuth: boolean;
  enableRetry: boolean;
}

const DEFAULT_CONFIG: InterceptorConfig = {
  retryAttempts: 3,
  retryDelay: 1000,
  enableAuth: true,
  enableRetry: true,
};

/**
 * API 请求拦截器
 */
export class ApiInterceptor {
  private config: InterceptorConfig;
  private isRefreshing = false;
  private refreshPromise: Promise<boolean> | null = null;

  constructor(config: Partial<InterceptorConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * 执行 API 请求
   */
  async request<T>(
    cmd: string,
    args?: Record<string, any>
  ): Promise<T> {
    let lastError: Error | null = null;
    const retryAttempts = this.config.enableRetry ? this.config.retryAttempts : 1;

    for (let attempt = 1; attempt <= retryAttempts; attempt++) {
      try {
        // 注入认证令牌
        const enhancedArgs = await this.injectAuth(args);

        console.debug(`[ApiInterceptor] ${cmd} (attempt ${attempt}/${retryAttempts})`);

        // 执行请求
        const response = await tauriInvoke<any>(cmd, enhancedArgs);

        // 处理新的 ApiResponse 格式
        if (response && typeof response === 'object' && 'success' in response) {
          if (!response.success) {
            const error = response.error;
            throw new ApiInterceptorError(
              error?.message || 'Operation failed',
              error?.code || 'UNKNOWN_ERROR',
              error?.details
            );
          }
          return response.data as T;
        }

        // 兼容旧格式（直接返回数据）
        return response as T;
      } catch (error) {
        lastError = error as Error;

        // 检查是否为认证错误
        if (error instanceof ApiInterceptorError && error.isAuthError) {
          console.warn('[ApiInterceptor] Auth error detected, attempting refresh');

          // 尝试刷新令牌
          const refreshed = await this.handleTokenRefresh();

          if (refreshed) {
            // 刷新成功，重试当前请求
            console.debug('[ApiInterceptor] Token refreshed, retrying request');
            continue;
          } else {
            // 刷新失败，跳转到登录页
            console.error('[ApiInterceptor] Token refresh failed, redirecting to login');
            this.handleAuthFailure();
            throw error;
          }
        }

        // 判断是否应该重试
        const shouldRetry =
          this.config.enableRetry &&
          attempt < retryAttempts &&
          this.isRetryableError(error);

        if (shouldRetry) {
          console.warn(
            `[ApiInterceptor] Retry ${attempt}/${retryAttempts} for ${cmd}:`,
            (error as Error).message
          );
          await this.delay(this.config.retryDelay * attempt);
          continue;
        }

        break;
      }
    }

    // 所有尝试都失败
    console.error(`[ApiInterceptor] Request failed after ${retryAttempts} attempts: ${cmd}`, lastError);

    // 统一错误转换
    if (lastError instanceof ApiInterceptorError) {
      throw lastError;
    }

    if (lastError instanceof Error) {
      throw new ApiInterceptorError(
        lastError.message || '请求失败',
        'NETWORK_ERROR',
        lastError.stack
      );
    }

    throw new ApiInterceptorError(
      '未知错误',
      'UNKNOWN_ERROR',
      String(lastError)
    );
  }

  /**
   * 注入认证令牌
   */
  private async injectAuth(args?: Record<string, any>): Promise<Record<string, any>> {
    if (!this.config.enableAuth) {
      return args || {};
    }

    try {
      const token = await getAccessToken();

      if (!token) {
        // 没有令牌，返回原始参数
        return args || {};
      }

      // 将令牌注入到参数中
      return {
        ...args,
        _auth: token, // 后端可以从参数中提取
      };
    } catch (error) {
      console.error('[ApiInterceptor] Failed to inject auth token:', error);
      return args || {};
    }
  }

  /**
   * 处理令牌刷新
   */
  private async handleTokenRefresh(): Promise<boolean> {
    // 如果已经在刷新中，等待刷新完成
    if (this.isRefreshing && this.refreshPromise) {
      return await this.refreshPromise;
    }

    // 开始刷新
    this.isRefreshing = true;
    this.refreshPromise = this.doTokenRefresh();

    try {
      const result = await this.refreshPromise;
      return result;
    } finally {
      this.isRefreshing = false;
      this.refreshPromise = null;
    }
  }

  /**
   * 执行令牌刷新
   */
  private async doTokenRefresh(): Promise<boolean> {
    try {
      const userStore = useUserStore();
      const success = await userStore.refreshAccessToken();

      if (success) {
        console.debug('[ApiInterceptor] Token refreshed successfully');
        return true;
      } else {
        console.warn('[ApiInterceptor] Token refresh failed');
        return false;
      }
    } catch (error) {
      console.error('[ApiInterceptor] Error during token refresh:', error);
      return false;
    }
  }

  /**
   * 处理认证失败
   */
  private handleAuthFailure(): void {
    // 清除认证数据
    const userStore = useUserStore();
    userStore.logout();

    // 触发登出事件（可以由路由守卫监听）
    window.dispatchEvent(new CustomEvent('auth:logout'));
  }

  /**
   * 判断错误是否可重试
   */
  private isRetryableError(error: unknown): boolean {
    if (error instanceof ApiInterceptorError) {
      // 认证错误由专门的逻辑处理，不在重试逻辑中
      if (error.isAuthError) {
        return false;
      }
      // 网络错误可以重试
      return error.code === 'NETWORK_ERROR' || error.code === 'TIMEOUT';
    }

    if (error instanceof Error) {
      const message = error.message.toLowerCase();
      return (
        message.includes('timeout') ||
        message.includes('network') ||
        message.includes('fetch')
      );
    }

    return false;
  }

  /**
   * 延迟函数
   */
  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  /**
   * 禁用认证（用于公开端点）
   */
  withoutAuth(): ApiInterceptor {
    return new ApiInterceptor({
      ...this.config,
      enableAuth: false,
    });
  }

  /**
   * 禁用重试（用于幂等性要求高的操作）
   */
  withoutRetry(): ApiInterceptor {
    return new ApiInterceptor({
      ...this.config,
      enableRetry: false,
    });
  }
}

// ============== 单例导出 ==============

/**
 * 默认拦截器实例（启用认证和重试）
 */
export const apiInterceptor = new ApiInterceptor();

/**
 * 公开 API 拦截器（不注入认证令牌）
 */
export const publicApiInterceptor = apiInterceptor.withoutAuth();

/**
 * 一次性 API 拦截器（不重试）
 */
export const oneShotApiInterceptor = apiInterceptor.withoutRetry();

// ============== 便捷函数 ==============

/**
 * 使用默认拦截器执行请求
 */
export async function apiRequest<T>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  return apiInterceptor.request<T>(cmd, args);
}

/**
 * 使用公开拦截器执行请求（无需认证）
 */
export async function publicApiRequest<T>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  return publicApiInterceptor.request<T>(cmd, args);
}

/**
 * 使用一次性拦截器执行请求（不重试）
 */
export async function oneShotApiRequest<T>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  return oneShotApiInterceptor.request<T>(cmd, args);
}

// ============== 使用示例 ==============

/*
// 基本用法（自动注入令牌、自动重试）
const user = await apiRequest<LoginResponse>('login', { request: loginData });

// 公开 API（无需认证）
const symbols = await publicApiRequest<string[]>('market_get_symbols');

// 一次性操作（不重试）
const result = await oneShotApiRequest<string>('strategy_delete', { id });

// 直接使用拦截器
const interceptor = new ApiInterceptor({ retryAttempts: 5 });
const data = await interceptor.request<Data>('some_command', args);

// 错误处理
try {
  const user = await apiRequest<User>('get_current_user', { userId });
} catch (error) {
  if (error instanceof ApiInterceptorError) {
    if (error.isAuthError) {
      // 处理认证错误
      console.error('认证失败:', error.message);
    } else {
      // 处理其他错误
      console.error(`[${error.code}] ${error.message}`);
    }
  }
}
*/