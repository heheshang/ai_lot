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
  StrategyConfig,
  InstanceInfo,
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
   * 取消订阅行情
   */
  unsubscribeTicker: (symbols: string[]) =>
    invokeRaw('market_unsubscribe_ticker', { symbols }),

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

  /**
   * 获取市场状态
   */
  getStatus: () =>
    invokeRaw<any>('market_get_status'),
};

// ============== 策略 API ==============
export const strategyApi = {
  /**
   * 获取策略列表
   */
  list: (userId: string) =>
    invoke<Strategy[]>('strategy_list', { user_id: userId }),

  /**
   * 获取策略详情
   */
  get: (id: string) =>
    invokeRaw<Strategy | null>('strategy_get', { id }),

  /**
   * 保存策略
   */
  save: (strategy: Strategy) =>
    invokeRaw<Strategy>('strategy_save', {
      request: {
        id: strategy.id,
        user_id: strategy.userId,
        name: strategy.name,
        description: strategy.description,
        code: strategy.code,
        language: strategy.language,
        parameters: strategy.parameters,
        parameter_values: strategy.parameterValues,
        category: strategy.category,
        tags: strategy.tags,
      },
    }),

  /**
   * 删除策略
   */
  delete: (id: string) =>
    invokeRaw('strategy_delete', { id }),
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

// ============== StrategyEngine API ==============
export const strategyEngineApi = {
  /**
   * 启动策略实例
   */
  start: (config: StrategyConfig) =>
    invokeRaw<string>('strategy_engine_start', { config }),

  /**
   * 停止策略实例
   */
  stop: (id: string) =>
    invokeRaw<void>('strategy_engine_stop', { id }),

  /**
   * 暂停策略实例
   */
  pause: (id: string) =>
    invokeRaw<void>('strategy_engine_pause', { id }),

  /**
   * 恢复策略实例
   */
  resume: (id: string) =>
    invokeRaw<void>('strategy_engine_resume', { id }),

  /**
   * 列出所有策略实例
   */
  list: () =>
    invokeRaw<InstanceInfo[]>('strategy_engine_list'),

  /**
   * 获取单个策略实例信息
   */
  get: (id: string) =>
    invokeRaw<InstanceInfo | null>('strategy_engine_get', { id }),
};

// ============== Risk API ==============
import type { AlertFilter, RiskAlertHistory } from '@/types';

export const riskApi = {
  /**
   * 获取风险概览
   */
  getOverview: () =>
    invoke<any>('get_risk_overview'),

  /**
   * 获取活跃预警
   */
  getActiveAlerts: () =>
    invoke<any[]>('get_active_alerts'),

  /**
   * 处理预警
   */
  handleAlert: (alertId: string) =>
    invoke('handle_alert', { alert_id: alertId }),

  /**
   * 忽略预警
   */
  ignoreAlert: (alertId: string) =>
    invoke('ignore_alert', { alert_id: alertId }),

  /**
   * 获取风险规则配置
   */
  getRiskRules: () =>
    invoke<any[]>('get_risk_rules'),

  /**
   * 更新风险规则
   */
  updateRiskRule: (ruleName: string, config: any) =>
    invoke('update_risk_rule', { rule_name: ruleName, config }),

  /**
   * 获取预警历史
   */
  getAlertHistory: (filter: AlertFilter) =>
    invoke<{ items: any[]; total: number }>('get_alert_history', { filter }),

  /**
   * 获取预警详情
   */
  getAlertDetail: (id: string) =>
    invoke<RiskAlertHistory>('get_alert_detail', { id }),

  /**
   * 添加预警备注
   */
  addAlertNote: (id: string, note: string) =>
    invoke('add_alert_note', { id, note }),

  /**
   * 删除预警
   */
  deleteAlert: (id: string) =>
    invoke('delete_alert', { id }),
};
