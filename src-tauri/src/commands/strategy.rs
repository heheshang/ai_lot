use crate::infrastructure::Database;
use crate::models::{StrategyDto, SaveStrategyRequest};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// 获取策略列表
#[tauri::command]
pub async fn strategy_list(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<StrategyDto>, String> {
    log::info!("Get strategy list for user: {}", user_id);

    let strategy_repo = db.strategy_repo();
    let list_items = strategy_repo
        .find_by_user(&user_id)
        .await
        .map_err(|e| e.to_string())?;

    // 将列表项转换为完整的 DTO（包含空数组）
    let strategies: Vec<StrategyDto> = list_items
        .into_iter()
        .map(|item| {
            // 解析标签
            let tags: Vec<String> = item
                .tags
                .and_then(|t| serde_json::from_str(&t).ok())
                .unwrap_or_default();

            StrategyDto {
                id: item.id,
                user_id: user_id.clone(),
                name: item.name,
                description: None,
                code: String::new(),
                language: "javascript".to_string(),
                parameters: Vec::new(),
                category: item.category,
                tags,
                version: item.version,
                parent_id: None,
                status: item.status,
                created_at: 0,
                updated_at: item.updated_at,
            }
        })
        .collect();

    Ok(strategies)
}

/// 获取策略详情
#[tauri::command]
pub async fn strategy_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<StrategyDto>, String> {
    log::info!("Get strategy by id: {}", id);

    let strategy_repo = db.strategy_repo();
    strategy_repo
        .find_by_id_dto(&id)
        .await
        .map_err(|e| e.to_string())
}

/// 保存策略（新增或更新）
#[tauri::command]
pub async fn strategy_save(
    db: State<'_, Database>,
    request: SaveStrategyRequest,
) -> Result<StrategyDto, String> {
    log::info!(
        "Save strategy: {} (id: {:?})",
        request.name,
        request.id
    );

    let strategy_repo = db.strategy_repo();

    // 检查名称是否重复
    let exists = strategy_repo
        .name_exists(&request.user_id, &request.name, request.id.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    if exists {
        return Err("策略名称已存在".to_string());
    }

    // 转换为 DTO
    let mut dto: StrategyDto = request.into();
    let strategy_id = dto.id.clone();

    // 如果是更新，保留原有的版本号和时间戳
    if let Some(existing) = strategy_repo
        .find_by_id_dto(&strategy_id)
        .await
        .map_err(|e| e.to_string())?
    {
        dto.version = existing.version;
        dto.created_at = existing.created_at;
    }

    // 保存到数据库
    strategy_repo
        .save(&dto)
        .await
        .map_err(|e| e.to_string())?;

    log::info!("Strategy saved successfully: {}", strategy_id);

    Ok(dto)
}

/// 删除策略
#[tauri::command]
pub async fn strategy_delete(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("Delete strategy: {}", id);

    let strategy_repo = db.strategy_repo();
    strategy_repo
        .delete(&id)
        .await
        .map_err(|e| e.to_string())?;

    log::info!("Strategy deleted successfully: {}", id);

    Ok(())
}
