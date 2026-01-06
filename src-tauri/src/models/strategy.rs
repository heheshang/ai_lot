use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use anyhow::Result;

/// 策略参数定义
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StrategyParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub default: Option<serde_json::Value>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub options: Option<String>, // JSON string of options array
    pub description: Option<String>,
}

/// 策略实体（对应数据库 strategies 表）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Strategy {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub parameters: Option<String>, // JSON string of StrategyParameter array
    pub category: Option<String>,
    pub tags: Option<String>, // JSON string of tags array
    pub version: i32,
    pub parent_id: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 策略请求/响应 DTO（包含解析后的参数和标签）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub parameters: Vec<StrategyParameter>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub version: i32,
    pub parent_id: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 保存策略请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveStrategyRequest {
    pub id: Option<String>,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub parameters: Vec<StrategyParameter>,
    pub parameter_values: Option<serde_json::Value>, // 实际参数值（用于预览）
    pub category: Option<String>,
    pub tags: Vec<String>,
}

/// 策略列表项（用于列表显示）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StrategyListItem {
    pub id: String,
    pub name: String,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub version: i32,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Strategy {
    /// 创建新策略
    pub fn new(
        id: String,
        user_id: String,
        name: String,
        code: String,
        language: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            user_id,
            name,
            description: None,
            code,
            language,
            parameters: None,
            category: None,
            tags: None,
            version: 1,
            parent_id: None,
            status: "draft".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 转换为 DTO（解析 JSON 字段）
    pub fn to_dto(&self) -> Result<StrategyDto> {
        let parameters = self.parse_parameters()?;
        let tags = self.parse_tags()?;
        Ok(StrategyDto {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            code: self.code.clone(),
            language: self.language.clone(),
            parameters,
            category: self.category.clone(),
            tags,
            version: self.version,
            parent_id: self.parent_id.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }

    /// 解析参数 JSON
    fn parse_parameters(&self) -> Result<Vec<StrategyParameter>> {
        if let Some(params_json) = &self.parameters {
            serde_json::from_str(params_json)
                .map_err(|e| anyhow::anyhow!("Failed to parse parameters: {}", e))
        } else {
            Ok(Vec::new())
        }
    }

    /// 解析标签 JSON
    fn parse_tags(&self) -> Result<Vec<String>> {
        if let Some(tags_json) = &self.tags {
            serde_json::from_str(tags_json)
                .map_err(|e| anyhow::anyhow!("Failed to parse tags: {}", e))
        } else {
            Ok(Vec::new())
        }
    }
}

impl StrategyDto {
    /// 转换为数据库实体（序列化 JSON 字段）
    pub fn to_entity(&self) -> Result<Strategy> {
        let parameters_json = serde_json::to_string(&self.parameters)
            .map_err(|e| anyhow::anyhow!("Failed to serialize parameters: {}", e))?;
        let tags_json = serde_json::to_string(&self.tags)
            .map_err(|e| anyhow::anyhow!("Failed to serialize tags: {}", e))?;

        Ok(Strategy {
            id: self.id.clone(),
            user_id: self.user_id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            code: self.code.clone(),
            language: self.language.clone(),
            parameters: Some(parameters_json),
            category: self.category.clone(),
            tags: Some(tags_json),
            version: self.version,
            parent_id: self.parent_id.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

impl From<SaveStrategyRequest> for StrategyDto {
    fn from(req: SaveStrategyRequest) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: req.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            user_id: req.user_id,
            name: req.name,
            description: req.description,
            code: req.code,
            language: req.language,
            parameters: req.parameters,
            category: req.category,
            tags: req.tags,
            version: 1,
            parent_id: None,
            status: "draft".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}
