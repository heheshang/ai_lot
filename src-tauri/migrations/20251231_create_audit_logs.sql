-- 重建审计日志表以支持新的审计事件系统
-- 首先删除旧的审计日志表（如果存在）
DROP TABLE IF EXISTS audit_logs;

-- 创建新的审计日志表
CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    event_data TEXT NOT NULL,
    user_id TEXT,
    timestamp INTEGER NOT NULL
);

-- 创建索引以提高查询性能
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
CREATE INDEX idx_audit_logs_user_timestamp ON audit_logs(user_id, timestamp DESC);
