/**
 * 配置文件类型定义
 */

// ============== 应用设置 ==============
export interface AppSettings {
  /** 语言 (zh-CN, en-US) */
  language: string;
  /** 主题 (light, dark, auto) */
  theme: string;
  /** 自动保存间隔 (秒) */
  auto_save_interval: number;
}

// ============== 数据库配置 ==============
export interface DatabaseConfig {
  /** 数据库路径 (相对于数据目录) */
  path: string;
  /** 备份间隔 (小时) */
  backup_interval_hours: number;
  /** 备份保留天数 */
  backup_retention_days: number;
}

// ============== 风控配置 ==============
export interface RiskConfig {
  /** 是否启用风控 */
  enabled: boolean;
  /** 默认风控动作 */
  default_action: string;
}

// ============== 通知配置 ==============
export interface NotificationConfig {
  /** 钉钉 Webhook URL */
  dingtalk_webhook?: string;
  /** SMTP 服务器地址 */
  smtp_server?: string;
  /** SMTP 端口 */
  smtp_port?: number;
  /** SMTP 用户名 */
  smtp_username?: string;
  /** SMTP 密码 */
  smtp_password?: string;
  /** 通知邮箱列表 (逗号分隔) */
  notification_emails?: string;
}

// ============== 完整配置 ==============
export interface AppConfig {
  /** 应用设置 */
  app: AppSettings;
  /** 数据库配置 */
  database: DatabaseConfig;
  /** 风控配置 */
  risk: RiskConfig;
  /** 通知配置 */
  notifications: NotificationConfig;
}

// ============== 配置更新类型 (可选字段) ==============
export interface AppConfigUpdate {
  app?: Partial<AppSettings>;
  database?: Partial<DatabaseConfig>;
  risk?: Partial<RiskConfig>;
  notifications?: Partial<NotificationConfig>;
}

// ============== 配置验证规则 ==============
export interface ConfigValidationRule {
  field: string;
  required?: boolean;
  min?: number;
  max?: number;
  pattern?: RegExp;
  validator?: (value: any) => boolean | string;
  message?: string;
}

// ============== 配置验证规则定义 ==============
export const CONFIG_VALIDATION_RULES: Record<string, ConfigValidationRule[]> = {
  app: [
    {
      field: 'language',
      required: true,
      validator: (value: string) => ['zh-CN', 'en-US'].includes(value),
      message: '语言必须是 zh-CN 或 en-US',
    },
    {
      field: 'theme',
      required: true,
      validator: (value: string) => ['light', 'dark', 'auto'].includes(value),
      message: '主题必须是 light、dark 或 auto',
    },
    {
      field: 'auto_save_interval',
      required: true,
      min: 10,
      max: 3600,
      message: '自动保存间隔必须在 10-3600 秒之间',
    },
  ],
  database: [
    {
      field: 'path',
      required: true,
      validator: (value: string) => value.endsWith('.db'),
      message: '数据库文件必须以 .db 结尾',
    },
    {
      field: 'backup_interval_hours',
      required: true,
      min: 1,
      max: 168,
      message: '备份间隔必须在 1-168 小时之间',
    },
    {
      field: 'backup_retention_days',
      required: true,
      min: 1,
      max: 365,
      message: '备份保留天数必须在 1-365 天之间',
    },
  ],
  risk: [
    {
      field: 'default_action',
      required: true,
      validator: (value: string) => ['alert', 'close_position', 'stop_strategy'].includes(value),
      message: '风控动作必须是 alert、close_position 或 stop_strategy',
    },
  ],
  notifications: [
    {
      field: 'dingtalk_webhook',
      validator: (value: string) => !value || value.startsWith('https://'),
      message: '钉钉 Webhook 必须是 HTTPS URL',
    },
    {
      field: 'smtp_port',
      validator: (value: number) => !value || (value >= 1 && value <= 65535),
      message: 'SMTP 端口必须在 1-65535 之间',
    },
    {
      field: 'notification_emails',
      validator: (value: string) => {
        if (!value) return true;
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return value.split(',').every(email => emailRegex.test(email.trim()));
      },
      message: '邮箱格式不正确，多个邮箱请用逗号分隔',
    },
  ],
};

// ============== 配置默认值 ==============
export const DEFAULT_CONFIG: AppConfig = {
  app: {
    language: 'zh-CN',
    theme: 'dark',
    auto_save_interval: 60,
  },
  database: {
    path: 'ai-lot.db',
    backup_interval_hours: 24,
    backup_retention_days: 30,
  },
  risk: {
    enabled: true,
    default_action: 'alert',
  },
  notifications: {
    dingtalk_webhook: undefined,
    smtp_server: undefined,
    smtp_port: undefined,
    smtp_username: undefined,
    smtp_password: undefined,
    notification_emails: undefined,
  },
};

// ============== 配置验证函数 ==============
export function validateConfig(section: string, data: any): string[] {
  const errors: string[] = [];
  const rules = CONFIG_VALIDATION_RULES[section] || [];

  rules.forEach(rule => {
    const value = data[rule.field];

    // 检查必填字段
    if (rule.required && (value === undefined || value === null || value === '')) {
      errors.push(`${rule.field} 是必填项`);
      return;
    }

    // 如果值为空且非必填，跳过其他验证
    if (!rule.required && (value === undefined || value === null || value === '')) {
      return;
    }

    // 数值范围检查
    if (rule.min !== undefined && value < rule.min) {
      errors.push(rule.message || `${rule.field} 不能小于 ${rule.min}`);
    }
    if (rule.max !== undefined && value > rule.max) {
      errors.push(rule.message || `${rule.field} 不能大于 ${rule.max}`);
    }

    // 正则表达式检查
    if (rule.pattern && !rule.pattern.test(value)) {
      errors.push(rule.message || `${rule.field} 格式不正确`);
    }

    // 自定义验证器
    if (rule.validator) {
      const result = rule.validator(value);
      if (result !== true) {
        errors.push(typeof result === 'string' ? result : (rule.message || `${rule.field} 验证失败`));
      }
    }
  });

  return errors;
}
