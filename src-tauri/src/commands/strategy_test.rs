use crate::core::strategy::ScriptExecutor;
use crate::core::trade::types::Kline;
use serde_json::json;

/// 测试策略代码执行
#[tauri::command]
pub async fn strategy_test_execute(
    code: String,
    parameters: String,
) -> Result<String, String> {
    log::info!("Testing strategy code execution");

    // 解析参数
    let params: serde_json::Value = serde_json::from_str(&parameters)
        .map_err(|e| format!("Invalid parameters JSON: {}", e))?;

    // 创建执行器
    let executor = ScriptExecutor::new()
        .map_err(|e| format!("Failed to create executor: {}", e))?;

    // 执行 onInit
    executor.on_init(&code, &params)
        .map_err(|e| format!("onInit failed: {}", e))?;

    // 创建测试 K线
    let test_kline = Kline {
        symbol: "BTCUSDT".to_string(),
        timeframe: "1h".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        open: 50000.0,
        high: 51000.0,
        low: 49000.0,
        close: 50500.0,
        volume: 100.0,
        quote_volume: Some(5050000.0),
    };

    // 执行 onBar
    let signal = executor.on_bar(&code, &test_kline, &params, &[])
        .map_err(|e| format!("onBar failed: {}", e))?;

    // 执行 onStop
    executor.on_stop(&code)
        .map_err(|e| format!("onStop failed: {}", e))?;

    // 返回结果
    let result = if let Some(sig) = signal {
        json!({
            "success": true,
            "signal": {
                "symbol": sig.symbol,
                "action": sig.action,
                "quantity": sig.quantity,
                "price": sig.price
            },
            "message": "Strategy executed successfully"
        }).to_string()
    } else {
        json!({
            "success": true,
            "signal": null,
            "message": "Strategy executed but returned no signal"
        }).to_string()
    };

    Ok(result)
}

/// 验证策略代码语法
#[tauri::command]
pub async fn strategy_validate_code(code: String) -> Result<bool, String> {
    log::info!("Validating strategy code syntax");

    // 尝试创建执行器并验证代码
    let executor = ScriptExecutor::new()
        .map_err(|e| format!("Failed to create executor: {}", e))?;

    // 尝试执行代码（语法检查）
    let params = json!({});
    let _ = executor.on_init(&code, &params)
        .map_err(|e| format!("Syntax error: {}", e))?;

    Ok(true)
}
