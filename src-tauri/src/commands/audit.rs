//! 审计日志命令
//!
//! 提供审计日志查询接口

use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::{AuditFilter, AuditLog, Database};
use tauri::State;
use uuid::Uuid;

/// 获取审计日志列表
///
/// # 参数
/// - `db`: 数据库状态
/// - `filter`: 可选的审计过滤器
///
/// # 返回
/// 返回审计日志列表
#[tauri::command]
pub async fn get_audit_logs(
    db: State<'_, Database>,
    filter: Option<AuditFilter>,
) -> Result<ApiResponse<Vec<AuditLog>>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] get_audit_logs called with filter: {:?}", request_id, filter);

    let audit_service = db.audit_logger();
    let filter = filter.unwrap_or_default();

    match audit_service.get_logs(&filter).await {
        Ok(logs) => {
            log::info!("[{}] Returning {} audit logs", request_id, logs.len());
            Ok(ApiResponse::success(logs).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to get audit logs: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::database_error(format!("查询审计日志失败: {}", e))))
        }
    }
}

/// 导出审计日志为 CSV
///
/// # 参数
/// - `db`: 数据库状态
/// - `filter`: 可选的审计过滤器
///
/// # 返回
/// 返回 CSV 格式的审计日志
#[tauri::command]
pub async fn audit_export_csv(
    db: State<'_, Database>,
    filter: Option<AuditFilter>,
) -> Result<ApiResponse<String>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] audit_export_csv called", request_id);

    let audit_service = db.audit_logger();
    let filter = filter.unwrap_or_default();

    match audit_service.get_logs(&filter).await {
        Ok(logs) => {
            // 构建 CSV 内容
            let mut csv = String::from("ID,Event Type,User ID,Timestamp,Event Data\n");
            for log in &logs {
                csv.push_str(&format!("{},{},{},{},{}\n",
                    log.id,
                    log.event_type,
                    log.user_id.as_deref().unwrap_or(""),
                    log.timestamp,
                    log.event_data.replace(',', ";").replace('\n', " ")
                ));
            }

            log::info!("[{}] Exported {} audit logs as CSV", request_id, logs.len());
            Ok(ApiResponse::success(csv).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to export audit logs: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::database_error(format!("导出审计日志失败: {}", e))))
        }
    }
}
