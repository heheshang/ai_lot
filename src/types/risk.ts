// ============== 风控规则配置类型 ==============

/**
 * 风控规则配置
 */
export interface RiskRuleConfig {
  /** 规则是否启用 */
  enabled: boolean;
  /** 触发后的动作 */
  action: RiskActionType;
  /** 通知方式 */
  notify_methods: string[];
  /** 规则参数 */
  params: Record<string, number>;
}

/**
 * 风控动作类型
 */
export type RiskActionType = 'warning' | 'stop_strategy' | 'emergency_close';

/**
 * 风控规则列表项
 */
export interface RiskRuleListItem {
  /** 规则名称 (唯一标识) */
  name: string;
  /** 规则显示名称 */
  display_name: string;
  /** 规则描述 */
  description: string;
  /** 规则类型 */
  rule_type: 'position_limit' | 'drawdown_limit';
  /** 规则配置 */
  config: RiskRuleConfig;
}

/**
 * 仓位限制规则参数
 */
export interface PositionLimitParams {
  /** 单个仓位最大价值 */
  max_position_value: number;
  /** 总仓位最大价值 */
  max_total_value: number;
  /** 单方向最大比例 (0-1) */
  max_direction_ratio: number;
}

/**
 * 回撤限制规则参数
 */
export interface DrawdownLimitParams {
  /** 最大回撤百分比 (0-100) */
  max_drawdown_pct: number;
}

/**
 * 更新风控规则请求
 */
export interface UpdateRiskRuleRequest {
  /** 规则名称 */
  rule_name: string;
  /** 新的配置 */
  config: RiskRuleConfig;
}

/**
 * 风控规则定义 (用于 UI 渲染)
 */
export interface RiskRuleDefinition {
  name: string;
  display_name: string;
  description: string;
  rule_type: 'position_limit' | 'drawdown_limit';
  default_params: Record<string, number>;
  param_validations: Record<string, ParamValidation>;
}

/**
 * 参数验证规则
 */
export interface ParamValidation {
  min: number;
  max: number;
  step: number;
  unit: string;
  label: string;
}

// ============== 风险预警历史类型 ==============

/**
 * 预警历史筛选条件
 */
export interface AlertFilter {
  /** 开始日期 (Unix 时间戳) */
  start_date?: number;
  /** 结束日期 (Unix 时间戳) */
  end_date?: number;
  /** 严重程度过滤 */
  severity?: string;
  /** 状态过滤 */
  status?: string;
  /** 规则名称过滤 */
  rule_name?: string;
  /** 搜索文本 */
  search_text?: string;
  /** 页码 */
  page?: number;
  /** 每页数量 */
  page_size?: number;
}

/**
 * 风险预警历史详情
 */
export interface RiskAlertHistory {
  /** 预警 ID */
  id: string;
  /** 规则 ID */
  rule_id: string;
  /** 规则名称 */
  rule_name: string;
  /** 严重程度 */
  severity: 'low' | 'medium' | 'high' | 'critical';
  /** 状态 */
  status: 'active' | 'handled' | 'ignored';
  /** 预警信息 */
  message: string;
  /** 处理备注 */
  handling_note?: string;
  /** 策略实例 ID */
  instance_id?: string;
  /** 交易对 */
  symbol?: string;
  /** 当前值 */
  current_value: number;
  /** 阈值 */
  threshold_value: number;
  /** 处理人 */
  handled_by?: string;
  /** 创建时间 */
  created_at: number;
  /** 处理时间 */
  handled_at?: number;
}

/**
 * 风险预警列表项
 */
export interface RiskAlertListItem {
  /** 预警 ID */
  id: string;
  /** 规则名称 */
  rule_name: string;
  /** 严重程度 */
  severity: 'low' | 'medium' | 'high' | 'critical';
  /** 状态 */
  status: 'active' | 'handled' | 'ignored';
  /** 预警信息 */
  message: string;
  /** 策略实例 ID */
  instance_id?: string;
  /** 交易对 */
  symbol?: string;
  /** 当前值 */
  current_value?: number;
  /** 阈值 */
  threshold_value?: number;
  /** 创建时间 */
  created_at: number;
  /** 处理时间 */
  handled_at?: number;
}
