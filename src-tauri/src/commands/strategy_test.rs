use crate::core::response::{ApiResponse, ApiError};
use crate::core::strategy::ScriptExecutor;
use crate::core::trade::types::Kline;
use serde_json::json;

/// 测试策略代码执行
#[tauri::command]
pub async fn strategy_test_execute(
    code: String,
    parameters: String,
) -> Result<ApiResponse<String>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Testing strategy code execution", request_id);

    // 解析参数
    let params: serde_json::Value = match serde_json::from_str(&parameters) {
        Ok(params) => params,
        Err(e) => {
            return Ok(ApiResponse::error(ApiError::validation_failed("parameters", format!("Invalid JSON: {}", e))).with_request_id(request_id));
        }
    };

    // 创建执行器
    let executor = match ScriptExecutor::new() {
        Ok(executor) => executor,
        Err(e) => {
            log::error!("[{}] Failed to create executor: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("创建执行器失败")).with_request_id(request_id));
        }
    };

    // 执行 onInit
    if let Err(e) = executor.on_init(&code, &params) {
        log::error!("[{}] onInit failed: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::operation_failed(format!("onInit失败: {}", e))).with_request_id(request_id));
    }

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
    let signal = match executor.on_bar(&code, &test_kline, &params, &[]) {
        Ok(signal) => signal,
        Err(e) => {
            log::error!("[{}] onBar failed: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed(format!("onBar失败: {}", e))).with_request_id(request_id));
        }
    };

    // 执行 onStop
    if let Err(e) = executor.on_stop(&code) {
        log::error!("[{}] onStop failed: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::operation_failed(format!("onStop失败: {}", e))).with_request_id(request_id));
    }

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

    Ok(ApiResponse::success(result).with_request_id(request_id))
}

/// 验证策略代码语法
#[tauri::command]
pub async fn strategy_validate_code(code: String) -> Result<ApiResponse<bool>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Validating strategy code syntax", request_id);

    // 尝试创建执行器并验证代码
    let executor = match ScriptExecutor::new() {
        Ok(executor) => executor,
        Err(e) => {
            log::error!("[{}] Failed to create executor: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("创建执行器失败")).with_request_id(request_id));
        }
    };

    // 尝试执行代码（语法检查）
    let params = json!({});
    if let Err(e) = executor.on_init(&code, &params) {
        log::error!("[{}] Syntax error: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::operation_failed(format!("语法错误: {}", e))).with_request_id(request_id));
    }

    Ok(ApiResponse::success(true).with_request_id(request_id))
}
