use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::{Database, AuditEvent};
use crate::models::{StrategyDto, SaveStrategyRequest};
use tauri::State;
use uuid::Uuid;

/// 获取策略列表
#[tauri::command]
pub async fn strategy_list(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<StrategyDto>>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] Get strategy list for user: {}", request_id, user_id);

    // 验证 user_id
    if user_id.is_empty() {
        return Ok(ApiResponse::error(ApiError::validation_failed("user_id", "不能为空")));
    }

    let strategy_repo = db.strategy_repo();
    let list_items = match strategy_repo.find_by_user(&user_id).await {
        Ok(items) => items,
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("获取策略列表失败: {}", e))));
        }
    };

    // 将列表项转换为完整的 DTO
    let strategies: Vec<StrategyDto> = list_items
        .into_iter()
        .map(|item| {
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

    Ok(ApiResponse::success(strategies).with_request_id(request_id))
}

/// 获取策略详情
#[tauri::command]
pub async fn strategy_get(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<Option<StrategyDto>>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] Get strategy by id: {}", request_id, id);

    if id.is_empty() {
        return Ok(ApiResponse::error(ApiError::validation_failed("id", "不能为空")));
    }

    let strategy_repo = db.strategy_repo();
    let result = match strategy_repo.find_by_id_dto(&id).await {
        Ok(r) => r,
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("获取策略失败: {}", e))));
        }
    };

    Ok(ApiResponse::success(result).with_request_id(request_id))
}

/// 保存策略（新增或更新）
#[tauri::command]
pub async fn strategy_save(
    db: State<'_, Database>,
    request: SaveStrategyRequest,
) -> Result<ApiResponse<StrategyDto>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!(
        "[{}] Save strategy: {} (id: {:?})",
        request_id,
        request.name,
        request.id
    );

    let strategy_repo = db.strategy_repo();

    // 检查名称是否重复
    let exists = match strategy_repo
        .name_exists(&request.user_id, &request.name, request.id.as_deref())
        .await
    {
        Ok(e) => e,
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("检查策略名称失败: {}", e))));
        }
    };

    if exists {
        log::warn!("[{}] Strategy name already exists: {}", request_id, request.name);
        return Ok(ApiResponse::error(ApiError::already_exists("策略名称")));
    }

    // 转换为 DTO
    let mut dto: StrategyDto = request.into();
    let strategy_id = dto.id.clone();
    let is_update = !strategy_id.is_empty();

    // 如果是更新，保留原有的版本号和时间戳
    if is_update {
        if let Ok(Some(existing)) = strategy_repo.find_by_id_dto(&strategy_id).await {
            dto.version = existing.version;
            dto.created_at = existing.created_at;
        }
    }

    // 保存到数据库
    match strategy_repo.save(&dto).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("保存策略失败: {}", e))));
        }
    }

    // 记录审计日志
    let audit_service = db.audit_logger();
    let log_id = if is_update { &strategy_id } else { &dto.id };
    let event = if is_update {
        AuditEvent::StrategyUpdated {
            user_id: dto.user_id.clone(),
            strategy_id: log_id.clone(),
            strategy_name: dto.name.clone(),
        }
    } else {
        AuditEvent::StrategyCreated {
            user_id: dto.user_id.clone(),
            strategy_id: log_id.clone(),
            strategy_name: dto.name.clone(),
        }
    };

    if let Err(e) = audit_service.log(event).await {
        log::warn!("[{}] Failed to log audit event: {}", request_id, e);
    }

    log::info!("[{}] Strategy saved successfully: {}", request_id, log_id);

    Ok(ApiResponse::success(dto).with_request_id(request_id))
}

/// 删除策略
#[tauri::command]
pub async fn strategy_delete(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] Delete strategy: {}", request_id, id);

    if id.is_empty() {
        return Ok(ApiResponse::error(ApiError::validation_failed("id", "不能为空")));
    }

    let strategy_repo = db.strategy_repo();

    // 先获取策略信息用于审计日志
    let strategy = match strategy_repo.find_by_id_dto(&id).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("获取策略失败: {}", e))));
        }
    };

    if strategy.is_none() {
        return Ok(ApiResponse::error(ApiError::not_found("策略")));
    }

    match strategy_repo.delete(&id).await {
        Ok(_) => {},
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("删除策略失败: {}", e))));
        }
    }

    // 记录审计日志
    let audit_service = db.audit_logger();
    if let Some(s) = strategy {
        if let Err(e) = audit_service
            .log(AuditEvent::StrategyDeleted {
                user_id: s.user_id.clone(),
                strategy_id: id.clone(),
                strategy_name: s.name.clone(),
            })
            .await
        {
            log::warn!("[{}] Failed to log audit event: {}", request_id, e);
        }
    }

    log::info!("[{}] Strategy deleted successfully: {}", request_id, id);

    Ok(ApiResponse::success(()).with_request_id(request_id))
}
