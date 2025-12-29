import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type {
  LoginRequest,
  LoginResponse,
  User,
  Kline,
  Strategy,
  StrategyConfig,
  InstanceInfo,
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
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (error) {
    // 更好的错误处理
    if (error instanceof Error) {
      throw error;
    }
    // 将错误转换为 Error 对象
    const errorMessage = typeof error === 'string'
      ? error
      : JSON.stringify(error);
    throw new Error(errorMessage || 'Unknown error');
  }
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
  save: (strategy: Strategy) => {
    // 转换参数格式：将 options 数组转换为符合后端的格式
    const parameters = strategy.parameters.map(param => ({
      name: param.name,
      type: param.type,
      default: param.default,
      min: param.min,
      max: param.max,
      step: param.step,
      options: param.options ? JSON.stringify(param.options) : undefined,
      description: param.description,
    }));

    // 后端期望参数包装在 request 对象中
    return invokeRaw<Strategy>('strategy_save', {
      request: {
        id: strategy.id || undefined,
        user_id: strategy.userId,
        name: strategy.name,
        description: strategy.description,
        code: strategy.code,
        language: strategy.language,
        parameters,
        parameter_values: strategy.parameterValues,
        category: strategy.category,
        tags: strategy.tags,
      },
    });
  },

  /**
   * 删除策略
   */
  delete: (id: string) =>
    invokeRaw('strategy_delete', { id }),
};

// ============== Strategy Engine API ==============
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

// ============== Strategy Instance API ==============
export const strategyInstanceApi = {
  /**
   * 列出策略实例
   */
  list: (userId?: string) =>
    invokeRaw<InstanceInfo[]>('instance_list', { user_id: userId }),

  /**
   * 获取策略实例
   */
  get: (instanceId: string) =>
    invokeRaw<InstanceInfo>('instance_get', { instance_id: instanceId }),

  /**
   * 创建策略实例
   */
  create: (config: StrategyConfig) =>
    invokeRaw<string>('instance_create', { config }),

  /**
   * 更新策略实例状态
   */
  updateStatus: (instanceId: string, status: 'running' | 'paused' | 'stopped' | 'error') =>
    invokeRaw<void>('instance_update_status', { instance_id: instanceId, status }),

  /**
   * 更新策略实例统计
   */
  updateStats: (instanceId: string, stats: any) =>
    invokeRaw<void>('instance_update_stats', { instance_id: instanceId, stats }),

  /**
   * 删除策略实例
   */
  delete: (instanceId: string) =>
    invokeRaw<void>('instance_delete', { instance_id: instanceId }),

  /**
   * 列出运行中的策略实例
   */
  listRunning: () =>
    invokeRaw<InstanceInfo[]>('instance_list_running'),

  /**
   * 列出所有策略实例
   */
  listAll: () =>
    invokeRaw<InstanceInfo[]>('strategy_instance_list_all'),

  /**
   * 获取策略实例详情
   */
  getInstance: (instanceId: string) =>
    invokeRaw<InstanceInfo>('strategy_instance_get', { instance_id: instanceId }),
};

// ============== Strategy Test API ==============
export const strategyTestApi = {
  /**
   * 执行策略测试
   */
  execute: (strategyId: string, testData?: any) =>
    invokeRaw<any>('strategy_test_execute', { strategy_id: strategyId, test_data: testData }),

  /**
   * 验证策略代码
   */
  validateCode: (code: string, language: string) =>
    invokeRaw<{ valid: boolean; errors?: string[] }>('strategy_validate_code', { code, language }),
};

// ============== Strategy Debug API ==============
export interface DebugLog {
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  timestamp: number;
  line?: number;
  function?: string;
  instance_id?: string;
}

export interface DebugVariable {
  name: string;
  value: any;
  var_type: string;
  timestamp: number;
}

export interface PerformanceMetrics {
  execution_times: Record<string, number[]>;
  call_counts: Record<string, number>;
  total_execution_time_ms: number;
  error_count: number;
  warning_count: number;
}

export const strategyDebugApi = {
  /**
   * 获取策略日志
   */
  getLogs: (instanceId: string, minLevel?: string, since?: number, limit?: number) =>
    invokeRaw<DebugLog[]>('get_strategy_logs', {
      instance_id: instanceId,
      min_level: minLevel,
      since,
      limit,
    }),

  /**
   * 获取性能指标
   */
  getMetrics: (instanceId: string) =>
    invokeRaw<PerformanceMetrics>('get_strategy_metrics', {
      instance_id: instanceId,
    }),

  /**
   * 获取监控变量
   */
  getVariables: (instanceId: string) =>
    invokeRaw<Record<string, DebugVariable>>('get_strategy_variables', {
      instance_id: instanceId,
    }),

  /**
   * 清除日志
   */
  clearLogs: (instanceId: string) =>
    invoke('clear_strategy_logs', { instance_id: instanceId }),

  /**
   * 设置日志级别
   */
  setLogLevel: (instanceId: string, level: string) =>
    invoke('set_strategy_log_level', {
      instance_id: instanceId,
      level,
    }),

  /**
   * 获取日志级别
   */
  getLogLevel: (instanceId: string) =>
    invokeRaw<string>('get_strategy_log_level', {
      instance_id: instanceId,
    }),

  /**
   * 生成测试日志
   */
  generateTestLogs: (instanceId: string) =>
    invoke('generate_test_logs', { instance_id: instanceId }),
};

// ============== Backtest API ==============
export interface BacktestJob {
  id: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  config: any;
  result?: any;
  created_at: number;
  updated_at: number;
}

export const backtestApi = {
  /**
   * 创建回测任务
   */
  createJob: (config: any) =>
    invokeRaw<string>('backtest_create_job', { config }),

  /**
   * 获取回测任务
   */
  getJob: (jobId: string) =>
    invokeRaw<BacktestJob>('backtest_get_job', { job_id: jobId }),

  /**
   * 列出回测任务
   */
  listJobs: () =>
    invokeRaw<BacktestJob[]>('backtest_list_jobs'),

  /**
   * 运行回测任务
   */
  runJob: (jobId: string) =>
    invokeRaw<any>('backtest_run_job', { job_id: jobId }),

  /**
   * 快速运行回测
   */
  run: (config: any) =>
    invokeRaw<any>('backtest_run', { config }),

  /**
   * 删除回测任务
   */
  deleteJob: (jobId: string) =>
    invokeRaw<void>('backtest_delete_job', { job_id: jobId }),

  /**
   * 获取回测结果
   */
  getResult: (jobId: string) =>
    invokeRaw<any>('backtest_get_result', { job_id: jobId }),
};

// ============== Trade API ==============
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
    invoke('trade_cancel_order', { order_id: orderId }),

  /**
   * 获取订单
   */
  getOrder: (orderId: string) =>
    invoke<Order>('trade_get_order', { order_id: orderId }),

  /**
   * 获取所有订单
   */
  getOrders: (symbol?: string, limit?: number) =>
    invoke<Order[]>('trade_get_orders', { symbol, limit }),

  /**
   * 获取未成交订单
   */
  getOpenOrders: (symbol?: string) =>
    invoke<Order[]>('trade_get_open_orders', { symbol }),

  /**
   * 同步订单状态
   */
  syncOrderStatus: (orderId?: string) =>
    invoke<void>('trade_sync_order_status', { order_id: orderId }),

  /**
   * 获取持仓
   */
  getPositions: (symbol?: string) =>
    invoke<Position[]>('trade_get_positions', { symbol }),

  /**
   * 获取余额
   */
  getBalance: () =>
    invoke<any>('trade_get_balance'),

  /**
   * 撤销所有订单
   */
  cancelAllOrders: (symbol?: string) =>
    invoke<number>('trade_cancel_all_orders', { symbol }),

  /**
   * 平仓
   */
  closePosition: (symbol: string, quantity?: number) =>
    invoke<any>('trade_close_position', { symbol, quantity }),
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

// ============== Config API ==============
export interface AppConfig {
  app: {
    language: string;
    theme: string;
    auto_save_interval: number;
  };
  database: {
    path: string;
    backup_interval_hours: number;
    backup_retention_days: number;
  };
  risk: {
    enabled: boolean;
    default_action: string;
  };
  notifications: {
    dingtalk_webhook?: string;
    smtp_server?: string;
    smtp_port?: number;
    smtp_username?: string;
    smtp_password?: string;
    notification_emails?: string;
  };
}

export const configApi = {
  /**
   * 获取系统配置
   */
  get: () =>
    invokeRaw<AppConfig>('config_get'),

  /**
   * 更新系统配置
   */
  update: (updater: Record<string, any>) =>
    invokeRaw<AppConfig>('config_update', { updater }),

  /**
   * 重置系统配置
   */
  reset: () =>
    invokeRaw<AppConfig>('config_reset'),
};

// ============== Exchange API ==============
export interface ExchangeConfig {
  id?: string;
  exchangeName: string;
  displayName: string;
  apiKey?: string;
  apiSecret?: string;
  passphrase?: string;
  isTestnet: boolean;
  status?: string;
  createdAt?: number;
  updatedAt?: number;
  apiKeyMasked?: string;
}

export interface ExchangeConfigDetail {
  id: string;
  exchangeName: string;
  displayName: string;
  apiKey: string;
  apiSecret: string;
  passphrase?: string;
  isTestnet: boolean;
  status: string;
  createdAt: number;
  updatedAt: number;
}

export const exchangeApi = {
  /**
   * 添加交易所配置
   */
  add: (userId: string, config: ExchangeConfig) =>
    invokeRaw<string>('exchange_add', {
      userId,
      request: {
        exchangeName: config.exchangeName,
        displayName: config.displayName,
        apiKey: config.apiKey,
        apiSecret: config.apiSecret,
        passphrase: config.passphrase,
        isTestnet: config.isTestnet,
      },
    }),

  /**
   * 更新交易所配置
   */
  update: (configId: string, config: ExchangeConfig) =>
    invokeRaw<void>('exchange_update', {
      configId,
      request: {
        exchangeName: config.exchangeName,
        displayName: config.displayName,
        apiKey: config.apiKey,
        apiSecret: config.apiSecret,
        passphrase: config.passphrase,
        isTestnet: config.isTestnet,
      },
    }),

  /**
   * 列出交易所配置
   */
  list: (userId: string) =>
    invokeRaw<ExchangeConfig[]>('exchange_list', { userId }),

  /**
   * 获取交易所配置
   */
  get: (configId: string) =>
    invokeRaw<ExchangeConfig>('exchange_get', { configId }),

  /**
   * 获取交易所配置详情（包含解密后的密钥，用于编辑）
   */
  getDetail: (configId: string) =>
    invokeRaw<ExchangeConfigDetail>('exchange_get_detail', { configId }),

  /**
   * 删除交易所配置
   */
  delete: (configId: string) =>
    invokeRaw<void>('exchange_delete', { configId }),

  /**
   * 更新交易所状态
   */
  updateStatus: (configId: string, status: string) =>
    invokeRaw<void>('exchange_update_status', {
      configId,
      status,
    }),
};

// ============== Backup API ==============
export interface BackupInfo {
  id: string;
  name: string;
  size: number;
  created_at: number;
  type: 'manual' | 'auto';
}

export const backupApi = {
  /**
   * 创建备份
   */
  create: (name?: string) =>
    invokeRaw<BackupInfo>('backup_create', { name }),

  /**
   * 恢复备份
   */
  restore: (backupId: string) =>
    invokeRaw<void>('backup_restore', { backup_id: backupId }),

  /**
   * 列出备份
   */
  list: () =>
    invokeRaw<BackupInfo[]>('backup_list'),

  /**
   * 删除备份
   */
  delete: (backupId: string) =>
    invokeRaw<void>('backup_delete', { backup_id: backupId }),

  /**
   * 清理旧备份
   */
  cleanup: (keepCount: number) =>
    invokeRaw<number>('backup_cleanup', { keep_count: keepCount }),

  /**
   * 验证备份完整性
   */
  verifyIntegrity: (backupId: string) =>
    invokeRaw<boolean>('backup_verify_integrity', { backup_id: backupId }),

  /**
   * 启动自动备份
   */
  startAuto: (intervalMinutes: number) =>
    invokeRaw<void>('backup_start_auto', { interval_minutes: intervalMinutes }),

  /**
   * 停止自动备份
   */
  stopAuto: () =>
    invokeRaw<void>('backup_stop_auto'),
};

// ============== Emergency API ==============
export const emergencyApi = {
  /**
   * 紧急停止 - 停止所有策略并平仓
   */
  stop: () =>
    invokeRaw<void>('emergency_stop'),
};
