//! Risk commands for Tauri
//!
//! This module provides Tauri command handlers for risk management operations.

use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::Database;
use crate::repository::risk_alert_repo::RiskAlertRepository;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::collections::HashMap;
use tauri::State;

/// Risk overview data for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskOverview {
    pub balance: f64,
    pub today_pnl: f64,
    pub total_position_value: f64,
    pub current_drawdown_pct: f64,
    pub peak_equity: f64,
    pub active_alert_count: i64,
    pub rule_status: HashMap<String, String>,
}

/// Risk alert list item (for UI display)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RiskAlertListItem {
    pub id: String,
    pub rule_name: String,
    pub severity: String,
    pub status: String,
    pub message: String,
    pub created_at: i64,
}

/// Get risk overview for dashboard
#[tauri::command]
pub async fn get_risk_overview(
    db: State<'_, Database>,
    user_id: Option<String>,
) -> Result<ApiResponse<RiskOverview>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let pool = db.pool.clone();
    let uid = user_id.unwrap_or_else(|| "default_user".to_string());

    // ========== 1. 计算总持仓价值 ==========
    // 从 positions 表查询所有未平仓持仓
    let positions_query = r#"
        SELECT
            symbol,
            side,
            quantity,
            entry_price,
            unrealized_pnl
        FROM positions
        WHERE user_id = ? AND quantity > 0
    "#;

    let position_rows = match sqlx::query(positions_query)
        .bind(&uid)
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            log::error!("[{}] Failed to query positions: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询持仓失败: {}", e))).with_request_id(request_id));
        }
    };

    let total_position_value: f64 = position_rows
        .iter()
        .map(|row: &sqlx::sqlite::SqliteRow| {
            let quantity: f64 = row.get::<Option<f64>, _>("quantity").unwrap_or(0.0);
            let entry_price: f64 = row.get::<Option<f64>, _>("entry_price").unwrap_or(0.0);
            quantity * entry_price
        })
        .sum();

    // ========== 2. 计算今日 P&L ==========
    // 从已成交订单计算今日盈亏
    let today_start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let pnl_query = r#"
        SELECT
            SUM(
                CASE
                    WHEN side = 'BUY' THEN -1 * filled_quantity * avg_price
                    ELSE filled_quantity * avg_price
                END
            ) AS today_pnl
        FROM orders
        WHERE user_id = ?
            AND status IN ('filled', 'partially_filled')
            AND filled_at >= ?
    "#;

    let today_pnl: f64 = match sqlx::query(pnl_query)
        .bind(&uid)
        .bind(today_start)
        .fetch_one(&pool)
        .await
        {
            Ok(row) => row.get::<Option<f64>, _>("today_pnl").unwrap_or(0.0),
            Err(_) => 0.0,
        };

    // ========== 3. 获取账户余额 ==========
    // 从 positions 表的 realized_pnl 计算可用余额
    // 注意：实际余额应该从交易所获取，这里使用数据库的缓存值
    let balance_query = r#"
        SELECT
            COALESCE(SUM(realized_pnl), 0) AS realized_pnl
        FROM positions
        WHERE user_id = ?
    "#;

    let realized_pnl: f64 = match sqlx::query(balance_query)
        .bind(&uid)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => row.get::<Option<f64>, _>("realized_pnl").unwrap_or(0.0),
        Err(_) => 0.0,
    };

    // 基础余额（模拟 - 应该从配置或交易所获取）
    const BASE_BALANCE: f64 = 10000.0;
    let balance = BASE_BALANCE + realized_pnl;

    // ========== 4. 计算回撤 ==========
    // 获取峰值权益（从历史记录或使用当前值）
    let current_equity = balance + total_position_value;
    let peak_equity = current_equity.max(BASE_BALANCE); // 简化实现

    let current_drawdown_pct = if peak_equity > 0.0 {
        ((peak_equity - current_equity) / peak_equity * 100.0).max(0.0)
    } else {
        0.0
    };

    // ========== 5. 获取活跃告警数量 ==========
    let alert_repo = RiskAlertRepository::new(pool.clone());
    let active_alert_count = alert_repo
        .count_active_by_user(&uid)
        .await
        .unwrap_or(0) as i64;

    // ========== 6. 获取规则状态 ==========
    // 根据当前指标计算规则状态
    let mut rule_status = HashMap::new();

    // 持仓限制规则状态
    let position_ratio = if balance > 0.0 {
        (total_position_value / balance * 100.0).round()
    } else {
        0.0
    };
    rule_status.insert(
        "position_limit".to_string(),
        if position_ratio > 80.0 { "danger".to_string() }
        else if position_ratio > 60.0 { "warning".to_string() }
        else { "ok".to_string() },
    );

    // 回撤限制规则状态
    rule_status.insert(
        "drawdown_limit".to_string(),
        if current_drawdown_pct > 15.0 { "danger".to_string() }
        else if current_drawdown_pct > 10.0 { "warning".to_string() }
        else { "ok".to_string() },
    );

    // 损失限制规则状态（基于今日 P&L）
    rule_status.insert(
        "loss_limit".to_string(),
        if today_pnl < -500.0 { "danger".to_string() }
        else if today_pnl < -200.0 { "warning".to_string() }
        else { "ok".to_string() },
    );

    Ok(ApiResponse::success(RiskOverview {
        balance,
        today_pnl,
        total_position_value,
        current_drawdown_pct,
        peak_equity,
        active_alert_count,
        rule_status,
    }).with_request_id(request_id))
}

/// Get active alerts for dashboard
#[tauri::command]
pub async fn get_active_alerts(
    db: State<'_, Database>,
    user_id: Option<String>,
) -> Result<ApiResponse<Vec<RiskAlertListItem>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let pool = db.pool.clone();
    let alert_repo = RiskAlertRepository::new(pool);
    let uid = user_id.unwrap_or_else(|| "default_user".to_string());

    let alerts = match alert_repo.find_unresolved_by_user(&uid).await {
        Ok(alerts) => alerts,
        Err(e) => {
            log::error!("[{}] Failed to get alerts: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询告警失败")).with_request_id(request_id));
        }
    };

    let result: Vec<RiskAlertListItem> = alerts
        .into_iter()
        .map(|alert| RiskAlertListItem {
            id: alert.id,
            rule_name: alert.title,
            severity: alert.severity,
            status: alert.status,
            message: alert.message,
            created_at: alert.created_at,
        })
        .collect();

    Ok(ApiResponse::success(result).with_request_id(request_id))
}

/// Handle an alert (mark as handled)
#[tauri::command]
pub async fn handle_alert(
    alert_id: String,
    db: State<'_, Database>,
    user_id: Option<String>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let pool = db.pool.clone();
    let alert_repo = RiskAlertRepository::new(pool);
    let uid = user_id.unwrap_or_else(|| "default_user".to_string());

    match alert_repo.mark_handled(&alert_id, &uid).await {
        Ok(()) => {
            log::info!("[{}] Alert {} marked as handled by {}", request_id, alert_id, uid);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to handle alert: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("处理告警失败")).with_request_id(request_id))
        }
    }
}

/// Ignore an alert
#[tauri::command]
pub async fn ignore_alert(alert_id: String, db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let pool = db.pool.clone();
    let alert_repo = RiskAlertRepository::new(pool);

    match alert_repo.mark_ignored(&alert_id).await {
        Ok(()) => {
            log::info!("[{}] Alert {} marked as ignored", request_id, alert_id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to ignore alert: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("忽略告警失败")).with_request_id(request_id))
        }
    }
}

// ============== Risk Rule Configuration Commands ==============

/// Frontend risk rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendRiskRuleConfig {
    pub enabled: bool,
    pub action: String,
    pub notify_methods: Vec<String>,
    pub params: HashMap<String, f64>,
}

/// Frontend risk rule list item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRuleListItem {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub rule_type: String,
    pub config: FrontendRiskRuleConfig,
}

/// Get all risk rule configurations from database
///
/// Returns a list of all available risk rules with their current configurations.
#[tauri::command]
pub async fn get_risk_rules(
    db: State<'_, Database>,
) -> Result<ApiResponse<Vec<RiskRuleListItem>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let repo = db.risk_rule_repo();
    let rules = match repo.find_all().await {
        Ok(rules) => rules,
        Err(e) => {
            log::error!("[{}] Failed to fetch risk rules: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询风控规则失败")).with_request_id(request_id));
        }
    };

    let frontend_rules: Vec<RiskRuleListItem> = rules
        .into_iter()
        .map(|rule| {
            let params = rule.get_params()
                .map_err(|e| format!("Failed to parse params: {}", e))
                .unwrap_or_default();

            let notify_methods = rule.get_notify_methods()
                .map_err(|e| format!("Failed to parse notify methods: {}", e))
                .unwrap_or_default();

            RiskRuleListItem {
                name: rule.name,
                display_name: rule.display_name,
                description: rule.description,
                rule_type: rule.rule_type,
                config: FrontendRiskRuleConfig {
                    enabled: rule.enabled,
                    action: rule.action,
                    notify_methods,
                    params,
                },
            }
        })
        .collect();

    Ok(ApiResponse::success(frontend_rules).with_request_id(request_id))
}

/// Update a single risk rule configuration in database
///
/// # Arguments
/// * `rule_name` - The name of the rule to update (e.g., "position_limit", "drawdown_limit")
/// * `config` - The new configuration to apply
#[tauri::command]
pub async fn update_risk_rule(
    db: State<'_, Database>,
    rule_name: String,
    config: FrontendRiskRuleConfig,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Validate action
    if !["warning", "stop_strategy", "emergency_close"].contains(&config.action.as_str()) {
        return Ok(ApiResponse::error(ApiError::invalid_parameter("action")).with_request_id(request_id));
    }

    // Validate parameters based on rule type
    if let Err(e) = validate_rule_params(&rule_name, &config.params) {
        return Ok(ApiResponse::error(ApiError::validation_failed(&rule_name, e)).with_request_id(request_id));
    }

    let repo = db.risk_rule_repo();
    match repo.update(
        &rule_name,
        config.enabled,
        &config.action,
        &config.notify_methods,
        &config.params,
    ).await {
        Ok(()) => {
            log::info!(
                "[{}] Updated risk rule: {} (enabled: {}, action: {})",
                request_id, rule_name, config.enabled, config.action
            );
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to update risk rule: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("更新风控规则失败")).with_request_id(request_id))
        }
    }
}

/// Validate rule parameters based on rule type
fn validate_rule_params(rule_name: &str, params: &HashMap<String, f64>) -> Result<(), String> {
    match rule_name {
        "position_limit" => {
            let max_position = params
                .get("max_position_value")
                .ok_or("Missing max_position_value")?;
            let max_total = params.get("max_total_value").ok_or("Missing max_total_value")?;
            let max_ratio = params
                .get("max_direction_ratio")
                .ok_or("Missing max_direction_ratio")?;

            if *max_position <= 0.0 {
                return Err("max_position_value must be positive".to_string());
            }
            if *max_total <= 0.0 {
                return Err("max_total_value must be positive".to_string());
            }
            if *max_ratio <= 0.0 || *max_ratio > 1.0 {
                return Err("max_direction_ratio must be between 0 and 1".to_string());
            }
        }
        "drawdown_limit" => {
            let max_dd = params
                .get("max_drawdown_pct")
                .ok_or("Missing max_drawdown_pct")?;

            if *max_dd <= 0.0 || *max_dd > 100.0 {
                return Err("max_drawdown_pct must be between 0 and 100".to_string());
            }
        }
        _ => {
            return Err(format!("Unknown rule type: {}", rule_name));
        }
    }

    Ok(())
}

// ============== Alert History Commands ==============

/// Alert filter for querying alert history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertHistoryFilter {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub severity: Option<String>,
    pub status: Option<String>,
    pub rule_name: Option<String>,
    pub search_text: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// Paginated alert list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertListResponse {
    pub items: Vec<RiskAlertHistoryListItem>,
    pub total: i32,
}

/// Risk alert history list item (with additional fields)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RiskAlertHistoryListItem {
    pub id: String,
    pub rule_name: String,
    pub severity: String,
    pub status: String,
    pub message: String,
    pub instance_id: Option<String>,
    pub symbol: Option<String>,
    pub current_value: Option<f64>,
    pub threshold_value: Option<f64>,
    pub created_at: i64,
    pub handled_at: Option<i64>,
}

/// Get alert history with filtering and pagination
///
/// # Arguments
/// * `filter` - Alert filter criteria
///
/// # Returns
/// Paginated list of alert items
#[tauri::command]
pub async fn get_alert_history(
    filter: AlertHistoryFilter,
    db: State<'_, Database>,
) -> Result<ApiResponse<AlertListResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let page = filter.page.unwrap_or(1).max(1);
    let page_size = filter.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * page_size;

    // Build query conditions
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(start) = filter.start_date {
        conditions.push("created_at >= ?");
        params.push(start.to_string());
    }

    if let Some(end) = filter.end_date {
        conditions.push("created_at <= ?");
        params.push(end.to_string());
    }

    if let Some(ref severity) = filter.severity {
        if !severity.is_empty() {
            conditions.push("severity = ?");
            params.push(severity.clone());
        }
    }

    if let Some(ref status) = filter.status {
        if !status.is_empty() {
            conditions.push("status = ?");
            params.push(status.clone());
        }
    }

    if let Some(ref rule_name) = filter.rule_name {
        if !rule_name.is_empty() {
            // Match rule_id containing the rule name
            conditions.push("rule_id LIKE ?");
            params.push(format!("%{}%", rule_name));
        }
    }

    if let Some(ref search_text) = filter.search_text {
        if !search_text.is_empty() {
            conditions.push("(message LIKE ? OR title LIKE ?)");
            let pattern = format!("%{}%", search_text);
            params.push(pattern.clone());
            params.push(pattern);
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Get total count
    let count_query = format!(
        "SELECT COUNT(*) as count FROM risk_alerts {}",
        where_clause
    );

    let total: i64 = match sqlx::query_scalar(&count_query)
        .fetch_one(&db.pool)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            log::error!("[{}] Failed to count alerts: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("统计告警数量失败: {}", e))).with_request_id(request_id));
        }
    };

    // Get paginated alerts
    let query = format!(
        "SELECT
            id,
            rule_id as rule_name,
            severity,
            status,
            message,
            strategy_instance_id as instance_id,
            symbol,
            current_value,
            threshold_value,
            created_at,
            handled_at
        FROM risk_alerts
        {}
        ORDER BY created_at DESC
        LIMIT {} OFFSET {}",
        where_clause, page_size, offset
    );

    // Simplified - fetch without complex param binding for now
    let items: Vec<RiskAlertHistoryListItem> = match sqlx::query_as(&query)
        .fetch_all(&db.pool)
        .await
    {
        Ok(items) => items,
        Err(e) => {
            log::error!("[{}] Failed to fetch alerts: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询告警失败: {}", e))).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(AlertListResponse {
        items,
        total: total as i32,
    }).with_request_id(request_id))
}

/// Risk alert detail (full information)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RiskAlertDetail {
    pub id: String,
    pub rule_id: String,
    pub rule_name: String,
    pub severity: String,
    pub status: String,
    pub message: String,
    pub handling_note: Option<String>,
    pub instance_id: Option<String>,
    pub symbol: Option<String>,
    pub current_value: f64,
    pub threshold_value: f64,
    pub handled_by: Option<String>,
    pub created_at: i64,
    pub handled_at: Option<i64>,
}

/// Get full alert details by ID
///
/// # Arguments
/// * `id` - Alert ID
///
/// # Returns
/// Complete alert information
#[tauri::command]
pub async fn get_alert_detail(
    id: String,
    db: State<'_, Database>,
) -> Result<ApiResponse<RiskAlertDetail>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();

    let alert = match sqlx::query_as::<_, RiskAlertDetail>(
        "SELECT
            id,
            rule_id,
            rule_id as rule_name,
            severity,
            status,
            message,
            handled_by as handling_note,
            strategy_instance_id as instance_id,
            symbol,
            current_value,
            threshold_value,
            handled_by,
            created_at,
            handled_at
        FROM risk_alerts
        WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&db.pool)
    .await
    {
        Ok(Some(alert)) => alert,
        Ok(None) => {
            return Ok(ApiResponse::error(ApiError::not_found("告警")).with_request_id(request_id));
        }
        Err(e) => {
            log::error!("[{}] Failed to fetch alert detail: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询告警详情失败: {}", e))).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(alert).with_request_id(request_id))
}

/// Add handling note to an alert and mark it as handled
///
/// # Arguments
/// * `id` - Alert ID
/// * `note` - Handling note
///
/// # Returns
/// Success result
#[tauri::command]
pub async fn add_alert_note(
    id: String,
    note: String,
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();

    if note.trim().is_empty() {
        return Ok(ApiResponse::error(ApiError::validation_failed("note", "不能为空")).with_request_id(request_id));
    }

    // Update alert: mark as handled and store note
    let now = Utc::now().timestamp();

    match sqlx::query(
        "UPDATE risk_alerts
        SET status = 'handled',
            handled_by = ?,
            handled_at = ?
        WHERE id = ?"
    )
    .bind(&note)
    .bind(now)
    .bind(&id)
    .execute(&db.pool)
    .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("[{}] Alert {} marked as handled with note", request_id, id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Ok(_) => {
            Ok(ApiResponse::error(ApiError::not_found("告警")).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to add alert note: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("添加处理备注失败")).with_request_id(request_id))
        }
    }
}

/// Delete an alert
///
/// # Arguments
/// * `id` - Alert ID
///
/// # Returns
/// Success result
#[tauri::command]
pub async fn delete_alert(
    id: String,
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();

    match sqlx::query("DELETE FROM risk_alerts WHERE id = ?")
        .bind(&id)
        .execute(&db.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            log::info!("[{}] Alert {} deleted", request_id, id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Ok(_) => {
            Ok(ApiResponse::error(ApiError::not_found("告警")).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to delete alert: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("删除告警失败")).with_request_id(request_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_overview_serialization() {
        let overview = RiskOverview {
            balance: 10000.0,
            today_pnl: 250.0,
            total_position_value: 6500.0,
            current_drawdown_pct: 2.5,
            peak_equity: 10500.0,
            active_alert_count: 3,
            rule_status: HashMap::new(),
        };

        let json = serde_json::to_string(&overview);
        assert!(json.is_ok());
    }

    #[test]
    fn test_validate_position_limit_params() {
        let mut params = HashMap::new();
        params.insert("max_position_value".to_string(), 10000.0);
        params.insert("max_total_value".to_string(), 50000.0);
        params.insert("max_direction_ratio".to_string(), 0.7);

        assert!(validate_rule_params("position_limit", &params).is_ok());

        // Test invalid max_direction_ratio
        params.insert("max_direction_ratio".to_string(), 1.5);
        assert!(validate_rule_params("position_limit", &params).is_err());
    }

    #[test]
    fn test_validate_drawdown_limit_params() {
        let mut params = HashMap::new();
        params.insert("max_drawdown_pct".to_string(), 15.0);

        assert!(validate_rule_params("drawdown_limit", &params).is_ok());

        // Test invalid max_drawdown_pct
        params.insert("max_drawdown_pct".to_string(), 150.0);
        assert!(validate_rule_params("drawdown_limit", &params).is_err());
    }

    #[test]
    fn test_alert_history_filter() {
        let filter = AlertHistoryFilter {
            start_date: Some(1609459200),
            end_date: Some(1609545600),
            severity: Some("high".to_string()),
            status: Some("active".to_string()),
            rule_name: Some("PositionLimit".to_string()),
            search_text: Some("test".to_string()),
            page: Some(2),
            page_size: Some(50),
        };

        assert_eq!(filter.page, Some(2));
        assert_eq!(filter.page_size, Some(50));
        assert_eq!(filter.severity, Some("high".to_string()));
    }

    #[test]
    fn test_alert_list_response() {
        let response = AlertListResponse {
            items: vec![],
            total: 0,
        };

        assert_eq!(response.items.len(), 0);
        assert_eq!(response.total, 0);
    }
}
