import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type {
  LoginRequest,
  LoginResponse,
  User,
  Kline,
  Strategy,
  BacktestParams,
  BacktestReport,
  Order,
  Position,
} from '@/types';

/**
 * 调用 Tauri 命令的通用方法
 */
export async function invoke<T = any>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  try {
    const response = await tauriInvoke<any>('cmd_' + cmd, args ? { payload: args } : {});

    if (response.success === false) {
      throw new Error(response.error || 'Operation failed');
    }

    return response.data as T;
  } catch (error) {
    // 处理旧版本命令格式（直接返回数据）
    return tauriInvoke<any>(cmd, args) as Promise<T>;
  }
}

/**
 * 直接调用 Tauri 命令（不包装 ApiResponse）
 */
export async function invokeRaw<T = any>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  return tauriInvoke<T>(cmd, args);
}

// ============== 用户 API ==============
export const userApi = {
  /**
   * 用户登录
   */
  login: (request: LoginRequest) =>
    invokeRaw<LoginResponse>('login', { request }),

  /**
   * 获取当前用户
   */
  getCurrentUser: (userId: string) =>
    invokeRaw<User>('get_current_user', { user_id: userId }),

  /**
   * 用户登出
   */
  logout: () =>
    invoke('user_logout'),
};

// ============== 行情 API ==============
export const marketApi = {
  /**
   * 订阅行情
   */
  subscribeTicker: (symbols: string[]) =>
    invoke('market_subscribe_ticker', { symbols }),

  /**
   * 获取 K 线数据
   */
  getKlines: (symbol: string, interval: string, limit: number) =>
    invoke<Kline[]>('market_get_klines', { symbol, interval, limit }),

  /**
   * 获取交易对列表
   */
  getSymbols: () =>
    invoke<string[]>('market_get_symbols'),
};

// ============== 策略 API ==============
export const strategyApi = {
  /**
   * 获取策略列表
   */
  list: () =>
    invoke<Strategy[]>('strategy_list'),

  /**
   * 获取策略详情
   */
  get: (id: string) =>
    invoke<Strategy>('strategy_get', { id }),

  /**
   * 保存策略
   */
  save: (strategy: Strategy) =>
    invoke<string>('strategy_save', { strategy }),

  /**
   * 删除策略
   */
  delete: (id: string) =>
    invoke('strategy_delete', { id }),
};

// ============== 回测 API ==============
export const backtestApi = {
  /**
   * 运行回测
   */
  run: (params: BacktestParams) =>
    invoke<string>('backtest_run', { params }),

  /**
   * 获取回测报告
   */
  getReport: (id: string) =>
    invoke<BacktestReport>('backtest_get_report', { id }),
};

// ============== 交易 API ==============
export const tradeApi = {
  /**
   * 下单
   */
  placeOrder: (order: any) =>
    invoke<Order>('trade_place_order', { order }),

  /**
   * 撤单
   */
  cancelOrder: (orderId: string) =>
    invoke('trade_cancel_order', { orderId }),

  /**
   * 获取持仓
   */
  getPositions: () =>
    invoke<Position[]>('trade_get_positions'),

  /**
   * 获取订单
   */
  getOrders: () =>
    invoke<Order[]>('trade_get_orders'),
};
