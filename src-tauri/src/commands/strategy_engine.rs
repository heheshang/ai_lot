use crate::core::response::{ApiResponse, ApiError};
use crate::core::strategy::StrategyConfig;
use crate::infrastructure::database::Database;
use crate::models::exchange::ExchangeConfig;
use crate::models::StrategyDto;
use crate::repository::ExchangeRepository;
use tauri::State;
use uuid::Uuid;

/// 启动策略实例
#[tauri::command]
pub async fn strategy_engine_start(
    db: State<'_, Database>,
    user_id: String,
    config: StrategyConfig,
) -> Result<ApiResponse<String>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Starting strategy engine instance: {} for user: {}", request_id, config.name, user_id);

    // 获取或创建 EventBus
    let _event_bus = db.get_event_bus();

    // 获取或创建 StrategyEngine
    let engine = db.get_strategy_engine();

    // 首先确保策略存在于 strategies 表中
    let strategy_id = if let Some(id) = &config.id {
        // 检查策略是否存在
        let strategy_repo = db.strategy_repo();
        let existing = match strategy_repo.find_by_id_dto(id).await {
            Ok(existing) => existing,
            Err(e) => {
                log::error!("[{}] Failed to check strategy existence: {}", request_id, e);
                return Ok(ApiResponse::error(ApiError::database_error(format!("查询策略失败: {}", e))).with_request_id(request_id));
            }
        };

        if existing.is_some() {
            id.clone()
        } else {
            // 策略不存在，需要先保存
            log::info!("[{}] Strategy {} not found in database, saving as draft", request_id, id);
            let dto = StrategyDto {
                id: id.clone(),
                user_id: user_id.clone(),
                name: config.name.clone(),
                description: Some("Auto-saved when starting instance".to_string()),
                code: config.code.clone(),
                language: "javascript".to_string(),
                parameters: vec![],
                category: None,
                tags: vec![],
                version: 1,
                parent_id: None,
                status: "draft".to_string(),
                created_at: 0,
                updated_at: chrono::Utc::now().timestamp(),
            };

            if let Err(e) = strategy_repo.save(&dto).await {
                log::error!("[{}] Failed to save strategy to database: {}", request_id, e);
                return Ok(ApiResponse::error(ApiError::database_error(format!("保存策略失败: {}", e))).with_request_id(request_id));
            }
            log::info!("[{}] Saved strategy {} to database", request_id, id);
            id.clone()
        }
    } else {
        // config.id 为空，生成新 ID 并保存策略
        let new_id = Uuid::new_v4().to_string();
        log::info!("[{}] No strategy ID provided, creating new strategy: {}", request_id, new_id);

        let dto = StrategyDto {
            id: new_id.clone(),
            user_id: user_id.clone(),
            name: config.name.clone(),
            description: Some("Auto-created when starting instance".to_string()),
            code: config.code.clone(),
            language: "javascript".to_string(),
            parameters: vec![],
            category: None,
            tags: vec![],
            version: 1,
            parent_id: None,
            status: "draft".to_string(),
            created_at: 0,
            updated_at: chrono::Utc::now().timestamp(),
        };

        let strategy_repo = db.strategy_repo();
        if let Err(e) = strategy_repo.save(&dto).await {
            log::error!("[{}] Failed to save new strategy to database: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("保存策略失败: {}", e))).with_request_id(request_id));
        }
        log::info!("[{}] Created and saved new strategy {} to database", request_id, new_id);
        new_id
    };

    // 更新 config 以确保包含有效的 strategy_id
    let updated_config = StrategyConfig {
        id: Some(strategy_id.clone()),
        ..config
    };

    // 获取或创建用户的默认交易所配置
    let exchange_repo = ExchangeRepository::new(db.pool.clone());
    let exchanges = match exchange_repo.find_by_user(&user_id).await {
        Ok(exchanges) => exchanges,
        Err(e) => {
            log::error!("[{}] Failed to query exchanges: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询交易所失败: {}", e))).with_request_id(request_id));
        }
    };

    let exchange_id = if exchanges.is_empty() {
        // 创建默认的模拟交易交易所
        let exchange_id = Uuid::new_v4().to_string();
        let default_exchange = match ExchangeConfig::create_encrypted(
            exchange_id.clone(),
            user_id.clone(),
            "binance".to_string(),
            "默认模拟交易".to_string(),
            "paper_trading",  // dummy API key for paper trading
            "paper_trading",  // dummy API secret for paper trading
            None,
            true,  // is_testnet
        ) {
            Ok(config) => config,
            Err(e) => {
                log::error!("[{}] Failed to create exchange config: {}", request_id, e);
                return Ok(ApiResponse::error(ApiError::operation_failed("创建交易所配置失败")).with_request_id(request_id));
            }
        };

        if let Err(e) = exchange_repo.create(&default_exchange).await {
            log::error!("[{}] Failed to create default exchange: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("创建交易所失败: {}", e))).with_request_id(request_id));
        }
        log::info!("[{}] Created default paper trading exchange: {} for user: {}", request_id, exchange_id, user_id);
        exchange_id
    } else {
        exchanges[0].id.clone()
    };

    // 启动策略实例
    match engine.start_instance(updated_config, user_id, Some(exchange_id)).await {
        Ok(instance_id) => {
            log::info!("[{}] Strategy engine started successfully: {}", request_id, instance_id);
            Ok(ApiResponse::success(instance_id).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to start strategy: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("启动策略失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// 停止策略实例
#[tauri::command]
pub async fn strategy_engine_stop(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Stopping strategy engine instance: {}", request_id, id);

    let engine = db.get_strategy_engine();
    match engine.stop_instance(&id).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to stop strategy: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("停止策略失败")).with_request_id(request_id))
        }
    }
}

/// 暂停策略实例
#[tauri::command]
pub async fn strategy_engine_pause(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Pausing strategy engine instance: {}", request_id, id);

    let engine = db.get_strategy_engine();
    match engine.pause_instance(&id).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to pause strategy: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("暂停策略失败")).with_request_id(request_id))
        }
    }
}

/// 恢复策略实例
#[tauri::command]
pub async fn strategy_engine_resume(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Resuming strategy engine instance: {}", request_id, id);

    let engine = db.get_strategy_engine();
    match engine.resume_instance(&id).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to resume strategy: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("恢复策略失败")).with_request_id(request_id))
        }
    }
}

/// 列出所有策略实例
#[tauri::command]
pub async fn strategy_engine_list(
    db: State<'_, Database>,
) -> Result<ApiResponse<Vec<crate::core::strategy::InstanceInfo>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] [strategy_engine_list] Listing all strategy instances", request_id);
    let engine = db.get_strategy_engine();
    let instances = engine.list_instances().await;
    log::info!("[{}] [strategy_engine_list] Found {} instances", request_id, instances.len());
    for (i, instance) in instances.iter().enumerate() {
        log::info!("[{}] [strategy_engine_list] Instance {}: id={}, name={}, status={:?}", request_id, i, instance.id, instance.name, instance.status);
    }
    Ok(ApiResponse::success(instances).with_request_id(request_id))
}

/// 获取单个策略实例信息
#[tauri::command]
pub async fn strategy_engine_get(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<Option<crate::core::strategy::InstanceInfo>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] strategy_engine_get called: id={}", request_id, id);

    let engine = db.get_strategy_engine();
    let instance = engine.get_instance(&id).await;
    Ok(ApiResponse::success(instance).with_request_id(request_id))
}
