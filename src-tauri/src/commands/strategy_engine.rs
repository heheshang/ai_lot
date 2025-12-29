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
) -> Result<String, String> {
    log::info!("Starting strategy engine instance: {} for user: {}", config.name, user_id);

    // 获取或创建 EventBus
    let _event_bus = db.get_event_bus();

    // 获取或创建 StrategyEngine
    let engine = db.get_strategy_engine();

    // 首先确保策略存在于 strategies 表中
    let strategy_id = if let Some(id) = &config.id {
        // 检查策略是否存在
        let strategy_repo = db.strategy_repo();
        let existing = strategy_repo
            .find_by_id_dto(id)
            .await
            .map_err(|e| format!("Failed to check strategy existence: {}", e))?;

        if existing.is_some() {
            id.clone()
        } else {
            // 策略不存在，需要先保存
            log::info!("Strategy {} not found in database, saving as draft", id);
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

            strategy_repo
                .save(&dto)
                .await
                .map_err(|e| format!("Failed to save strategy to database: {}", e))?;
            log::info!("Saved strategy {} to database", id);
            id.clone()
        }
    } else {
        // config.id 为空，生成新 ID 并保存策略
        let new_id = Uuid::new_v4().to_string();
        log::info!("No strategy ID provided, creating new strategy: {}", new_id);

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
        strategy_repo
            .save(&dto)
            .await
            .map_err(|e| format!("Failed to save new strategy to database: {}", e))?;
        log::info!("Created and saved new strategy {} to database", new_id);
        new_id
    };

    // 更新 config 以确保包含有效的 strategy_id
    let updated_config = StrategyConfig {
        id: Some(strategy_id.clone()),
        ..config
    };

    // 获取或创建用户的默认交易所配置
    let exchange_repo = ExchangeRepository::new(db.pool.clone());
    let exchanges = exchange_repo
        .find_by_user(&user_id)
        .await
        .map_err(|e| format!("Failed to query exchanges: {}", e))?;

    let exchange_id = if exchanges.is_empty() {
        // 创建默认的模拟交易交易所
        let exchange_id = Uuid::new_v4().to_string();
        let default_exchange = ExchangeConfig::create_encrypted(
            exchange_id.clone(),
            user_id.clone(),
            "binance".to_string(),
            "默认模拟交易".to_string(),
            "paper_trading",  // dummy API key for paper trading
            "paper_trading",  // dummy API secret for paper trading
            None,
            true,  // is_testnet
        ).map_err(|e| format!("Failed to create exchange config: {}", e))?;

        exchange_repo
            .create(&default_exchange)
            .await
            .map_err(|e| format!("Failed to create default exchange: {}", e))?;
        log::info!("Created default paper trading exchange: {} for user: {}", exchange_id, user_id);
        exchange_id
    } else {
        exchanges[0].id.clone()
    };

    // 启动策略实例
    engine
        .start_instance(updated_config, user_id, Some(exchange_id))
        .await
        .map_err(|e| format!("Failed to start strategy: {}", e))
}

/// 停止策略实例
#[tauri::command]
pub async fn strategy_engine_stop(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("Stopping strategy engine instance: {}", id);

    let engine = db.get_strategy_engine();
    engine
        .stop_instance(&id)
        .await
        .map_err(|e| format!("Failed to stop strategy: {}", e))
}

/// 暂停策略实例
#[tauri::command]
pub async fn strategy_engine_pause(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("Pausing strategy engine instance: {}", id);

    let engine = db.get_strategy_engine();
    engine
        .pause_instance(&id)
        .await
        .map_err(|e| format!("Failed to pause strategy: {}", e))
}

/// 恢复策略实例
#[tauri::command]
pub async fn strategy_engine_resume(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("Resuming strategy engine instance: {}", id);

    let engine = db.get_strategy_engine();
    engine
        .resume_instance(&id)
        .await
        .map_err(|e| format!("Failed to resume strategy: {}", e))
}

/// 列出所有策略实例
#[tauri::command]
pub async fn strategy_engine_list(
    db: State<'_, Database>,
) -> Result<Vec<crate::core::strategy::InstanceInfo>, String> {
    let engine = db.get_strategy_engine();
    Ok(engine.list_instances().await)
}

/// 获取单个策略实例信息
#[tauri::command]
pub async fn strategy_engine_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<crate::core::strategy::InstanceInfo>, String> {
    let engine = db.get_strategy_engine();
    Ok(engine.get_instance(&id).await)
}
