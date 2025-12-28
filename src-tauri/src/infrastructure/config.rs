use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AppConfig {
    pub app: AppSettings,
    pub database: DatabaseConfig,
    pub risk: RiskConfig,
    pub notifications: NotificationConfig,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 语言 (zh-CN, en-US)
    #[serde(default = "default_language")]
    pub language: String,

    /// 主题 (light, dark, auto)
    #[serde(default = "default_theme")]
    pub theme: String,

    /// 自动保存间隔 (秒)
    #[serde(default = "default_auto_save_interval")]
    pub auto_save_interval: u64,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库路径 (相对于数据目录)
    #[serde(default = "default_db_path")]
    pub path: String,

    /// 备份间隔 (小时)
    #[serde(default = "default_backup_interval")]
    pub backup_interval_hours: u32,

    /// 备份保留天数
    #[serde(default = "default_backup_retention")]
    pub backup_retention_days: u32,
}

/// 风控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    /// 是否启用风控
    #[serde(default = "default_risk_enabled")]
    pub enabled: bool,

    /// 默认风控动作
    #[serde(default = "default_risk_action")]
    pub default_action: String,
}

/// 通知配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct NotificationConfig {
    /// 钉钉 Webhook URL
    #[serde(default)]
    pub dingtalk_webhook: Option<String>,

    /// SMTP 服务器地址
    #[serde(default)]
    pub smtp_server: Option<String>,

    /// SMTP 端口
    #[serde(default)]
    pub smtp_port: Option<u16>,

    /// SMTP 用户名
    #[serde(default)]
    pub smtp_username: Option<String>,

    /// SMTP 密码
    #[serde(default)]
    pub smtp_password: Option<String>,

    /// 通知邮箱列表 (逗号分隔)
    #[serde(default)]
    pub notification_emails: Option<String>,
}

// 默认值函数
fn default_language() -> String {
    "zh-CN".to_string()
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_auto_save_interval() -> u64 {
    60 // 60秒
}

fn default_db_path() -> String {
    "ai-lot.db".to_string()
}

fn default_backup_interval() -> u32 {
    24 // 24小时
}

fn default_backup_retention() -> u32 {
    30 // 30天
}

fn default_risk_enabled() -> bool {
    true
}

fn default_risk_action() -> String {
    "alert".to_string()
}


impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: default_theme(),
            auto_save_interval: default_auto_save_interval(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: default_db_path(),
            backup_interval_hours: default_backup_interval(),
            backup_retention_days: default_backup_retention(),
        }
    }
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            enabled: default_risk_enabled(),
            default_action: default_risk_action(),
        }
    }
}


/// 配置管理器
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new(app_dir: PathBuf) -> Self {
        let config_path = app_dir.join("config.toml");
        Self { config_path }
    }

    /// 加载配置文件，如果不存在则创建默认配置
    pub fn load(&self) -> Result<AppConfig> {
        // 如果配置文件不存在，创建默认配置
        if !self.config_path.exists() {
            log::info!("Config file not found, creating default: {}", self.config_path.display());
            let default_config = AppConfig::default();
            self.save(&default_config)?;
            return Ok(default_config);
        }

        // 读取配置文件
        let content = fs::read_to_string(&self.config_path).map_err(|e| {
            anyhow::anyhow!("Failed to read config file: {}", e)
        })?;

        // 解析 TOML
        let config: AppConfig = toml::from_str(&content).map_err(|e| {
            anyhow::anyhow!("Failed to parse config TOML: {}", e)
        })?;

        log::info!("Config loaded successfully from: {}", self.config_path.display());
        Ok(config)
    }

    /// 保存配置到文件 (使用原子写入)
    pub fn save(&self, config: &AppConfig) -> Result<()> {
        // 序列化为 TOML
        let toml_string = toml::to_string_pretty(config).map_err(|e| {
            anyhow::anyhow!("Failed to serialize config: {}", e)
        })?;

        // 创建临时文件
        let temp_path = self.config_path.with_extension("tmp");

        // 写入临时文件
        fs::write(&temp_path, toml_string).map_err(|e| {
            anyhow::anyhow!("Failed to write temp config: {}", e)
        })?;

        // 原子性重命名
        fs::rename(&temp_path, &self.config_path).map_err(|e| {
            anyhow::anyhow!("Failed to rename config file: {}", e)
        })?;

        log::info!("Config saved successfully to: {}", self.config_path.display());
        Ok(())
    }

    /// 更新配置 (使用 updater 函数修改配置)
    pub fn update<F>(&self, updater: F) -> Result<AppConfig>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.load()?;
        updater(&mut config);
        self.save(&config)?;
        Ok(config)
    }

    /// 获取配置文件路径
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app.language, "zh-CN");
        assert_eq!(config.app.theme, "dark");
        assert_eq!(config.app.auto_save_interval, 60);
        assert_eq!(config.database.backup_interval_hours, 24);
        assert_eq!(config.risk.enabled, true);
        assert_eq!(config.risk.default_action, "alert");
    }

    #[test]
    fn test_config_save_and_load() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = ConfigManager::new(temp_dir.path().to_path_buf());

        // 创建配置
        let mut config = AppConfig::default();
        config.app.theme = "light".to_string();

        // 保存
        manager.save(&config)?;

        // 加载
        let loaded = manager.load()?;
        assert_eq!(loaded.app.theme, "light");

        Ok(())
    }

    #[test]
    fn test_config_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = ConfigManager::new(temp_dir.path().to_path_buf());

        // 更新配置
        let config = manager.update(|cfg| {
            cfg.app.language = "en-US".to_string();
            cfg.database.backup_interval_hours = 12;
        })?;

        assert_eq!(config.app.language, "en-US");
        assert_eq!(config.database.backup_interval_hours, 12);

        Ok(())
    }
}
