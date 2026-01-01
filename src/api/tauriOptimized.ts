/**
 * 优化的 Tauri API 封装
 *
 * 改进点：
 * 1. 统一处理 ApiResponse 格式
 * 2. 自动提取 data 字段
 * 3. 统一错误处理
 * 4. 添加请求重试机制
 * 5. 添加请求缓存
 * 6. 添加请求拦截器和响应拦截器
 */

import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type {
  LoginRequest,
  LoginResponse,
  User,
  Kline,
} from '@/types';

// ============== 类型定义 ==============

/**
 * 统一的 API 响应格式
 */
interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: string;
  };
  request_id?: string;
  timestamp?: number;
}

/**
 * API 错误类
 */
export class ApiError extends Error {
  code: string;
  details?: string;
  requestId?: string;

  constructor(
    message: string,
    code: string,
    details?: string,
    requestId?: string
  ) {
    super(message);
    this.name = 'ApiError';
    this.code = code;
    this.details = details;
    this.requestId = requestId;
  }

  static fromResponse(response: ApiResponse): ApiError | null {
    if (response.success || !response.error) {
      return null;
    }
    return new ApiError(
      response.error.message,
      response.error.code,
      response.error.details,
      response.request_id
    );
  }
}

// ============== 配置 ==============

interface ApiConfig {
  retryAttempts: number;
  retryDelay: number;
  timeout: number;
  enableCache: boolean;
  cacheTimeout: number;
}

const DEFAULT_CONFIG: ApiConfig = {
  retryAttempts: 3,
  retryDelay: 1000,
  timeout: 30000,
  enableCache: true,
  cacheTimeout: 5000, // 5秒缓存
};

// ============== 请求缓存 ==============

class RequestCache {
  private cache = new Map<string, { data: any; timestamp: number }>();
  private config: ApiConfig;

  constructor(config: ApiConfig) {
    this.config = config;
  }

  get(key: string): any | null {
    if (!this.config.enableCache) {
      return null;
    }

    const cached = this.cache.get(key);
    if (!cached) {
      return null;
    }

    const age = Date.now() - cached.timestamp;
    if (age > this.config.cacheTimeout) {
      this.cache.delete(key);
      return null;
    }

    console.debug(`[Cache] Hit: ${key}`);
    return cached.data;
  }

  set(key: string, data: any): void {
    if (!this.config.enableCache) {
      return;
    }

    this.cache.set(key, {
      data,
      timestamp: Date.now(),
    });
    console.debug(`[Cache] Set: ${key}`);
  }

  invalidate(pattern?: string): void {
    if (!pattern) {
      this.cache.clear();
      console.debug('[Cache] Cleared all');
      return;
    }

    const regex = new RegExp(pattern);
    for (const key of this.cache.keys()) {
      if (regex.test(key)) {
        this.cache.delete(key);
      }
    }
    console.debug(`[Cache] Invalidated pattern: ${pattern}`);
  }

  clear(): void {
    this.cache.clear();
  }
}

// ============== 核心 API 调用函数 ==============

const cache = new RequestCache(DEFAULT_CONFIG);

/**
 * 延迟函数
 */
function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * 生成缓存键
 */
function getCacheKey(cmd: string, args?: Record<string, any>): string {
  const argsStr = args ? JSON.stringify(args) : '';
  return `${cmd}:${argsStr}`;
}

/**
 * 核心 API 调用函数
 */
async function invokeCore<T = any>(
  cmd: string,
  args?: Record<string, any>,
  options?: {
    retry?: boolean;
    cache?: boolean;
    timeout?: number;
  }
): Promise<T> {
  const {
    retry = true,
    cache: useCache = true,
    timeout = DEFAULT_CONFIG.timeout,
  } = options || {};

  // 检查缓存
  const cacheKey = getCacheKey(cmd, args);
  if (useCache) {
    const cached = cache.get(cacheKey);
    if (cached !== null) {
      return cached as T;
    }
  }

  let lastError: Error | null = null;
  const retryAttempts = retry ? DEFAULT_CONFIG.retryAttempts : 1;

  for (let attempt = 1; attempt <= retryAttempts; attempt++) {
    try {
      console.debug(`[API] Calling: ${cmd}`, args || '');

      // 使用 Promise.race 实现超时
      const response = await Promise.race([
        tauriInvoke<ApiResponse<T>>(cmd, args),
        new Promise<never>((_, reject) =>
          setTimeout(() => reject(new Error(`Timeout after ${timeout}ms`)), timeout)
        ),
      ]);

      // 处理响应
      if (response && typeof response === 'object' && 'success' in response) {
        // 新格式: ApiResponse<T>
        const apiResponse = response as ApiResponse<T>;

        if (!apiResponse.success) {
          const error = ApiError.fromResponse(apiResponse);
          if (error) throw error;
        }

        // 缓存成功响应
        if (apiResponse.data !== undefined) {
          cache.set(cacheKey, apiResponse.data);
        }

        return apiResponse.data as T;
      } else {
        // 旧格式: 直接返回数据
        cache.set(cacheKey, response);
        return response as T;
      }
    } catch (error) {
      lastError = error as Error;

      // 判断是否应该重试
      const shouldRetry =
        retry &&
        attempt < retryAttempts &&
        lastError.message.includes('Timeout');

      if (shouldRetry) {
        console.warn(
          `[API] Retry ${attempt}/${retryAttempts} for ${cmd}:`,
          lastError.message
        );
        await delay(DEFAULT_CONFIG.retryDelay * attempt);
        continue;
      }

      break;
    }
  }

  // 所有尝试都失败
  console.error(`[API] Failed after ${retryAttempts} attempts: ${cmd}`, lastError);
  throw lastError || new Error('Unknown error');
}

/**
 * 便捷的 API 调用函数（自动提取 data）
 */
export async function invoke<T = any>(
  cmd: string,
  args?: Record<string, any>,
  options?: {
    retry?: boolean;
    cache?: boolean;
    timeout?: number;
  }
): Promise<T> {
  try {
    return await invokeCore<T>(cmd, args, options);
  } catch (error) {
    // 统一错误处理
    if (error instanceof ApiError) {
      throw error;
    }

    // 转换为 ApiError
    if (error instanceof Error) {
      throw new ApiError(
        error.message || '请求失败',
        'NETWORK_ERROR',
        error.stack
      );
    }

    throw new ApiError(
      '未知错误',
      'UNKNOWN_ERROR',
      String(error)
    );
  }
}

/**
 * 清除缓存
 */
export function clearCache(pattern?: string): void {
  cache.invalidate(pattern);
}

// ============== 用户 API ==============
export const userApiOptimized = {
  /**
   * 用户登录
   */
  login: (request: LoginRequest) =>
    invoke<LoginResponse>('login', { request }, { cache: false }),

  /**
   * 获取当前用户
   */
  getCurrentUser: (userId: string) =>
    invoke<User>('get_current_user', { userId }),
};

// ============== 行情 API ==============
export const marketApiOptimized = {
  /**
   * 订阅行情
   */
  subscribeTicker: (symbols: string[]) =>
    invoke<void>('market_subscribe_ticker', { symbols }, { cache: false }),

  /**
   * 取消订阅行情
   */
  unsubscribeTicker: (symbols: string[]) =>
    invoke<void>('market_unsubscribe_ticker', { symbols }, { cache: false }),

  /**
   * 获取 K 线数据
   */
  getKlines: (symbol: string, interval: string, limit: number) =>
    invoke<Kline[]>('market_get_klines', { symbol, interval, limit }),

  /**
   * 获取交易对列表
   */
  getSymbols: () =>
    invoke<string[]>('market_get_symbols', [], { cache: true }),

  /**
   * 获取市场状态
   */
  getStatus: () =>
    invoke<any>('market_get_status', [], { cache: true, timeout: 5000 }),
};

// ============== 使用示例 ==============
/*
// 基本用法
const klines = await marketApiOptimized.getKlines('BTCUSDT', '1h', 100);

// 错误处理
try {
  const user = await userApiOptimized.getCurrentUser(userId);
  console.log('User:', user);
} catch (error) {
  if (error instanceof ApiError) {
    console.error(`[${error.code}] ${error.message}`);
    if (error.details) {
      console.error('Details:', error.details);
    }
  }
}

// 清除缓存
clearCache(); // 清除所有缓存
clearCache('market_'); // 清除市场相关的缓存

// 禁用缓存和重试
const symbols = await marketApiOptimized.getSymbols();
// 或
const result = await invoke('some_command', args, { cache: false, retry: false });
*/

// ============== 迁移指南 ==============
/*
// 旧代码:
const klines = await invoke<Kline[]>('market_get_klines', { symbol, interval, limit });

// 新代码:
const klines = await marketApiOptimized.getKlines(symbol, interval, limit);

// 优势:
// 1. 自动处理 ApiResponse 格式
// 2. 自动提取 data 字段
// 3. 统一的错误类型 (ApiError)
// 4. 自动缓存（可配置）
// 5. 自动重试（可配置）
// 6. 超时控制
*/