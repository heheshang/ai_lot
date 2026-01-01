use crate::models::UserWithRole;

pub mod permission;
pub mod jwt;
pub use permission::*;
pub use jwt::{JwtManager, JwtConfig, Claims, generate_access_token, generate_refresh_token, verify_token, refresh_access_token};

/// 权限检查服务
pub struct AuthService;

impl AuthService {
    /// 检查用户是否有指定权限
    pub fn has_permission(user: &UserWithRole, permission: &str) -> bool {
        // 解析角色权限
        let permissions: Vec<String> = Self::parse_role_permissions(&user.role_name);

        // 检查是否有全部权限
        if permissions.contains(&PERM_ALL.to_string()) {
            return true;
        }

        // 检查是否有指定权限
        permissions.contains(&permission.to_string())
    }

    /// 检查用户是否有任一权限
    pub fn has_any_permission(user: &UserWithRole, permissions: &[&str]) -> bool {
        permissions.iter().any(|p| Self::has_permission(user, p))
    }

    /// 解析角色权限
    fn parse_role_permissions(role_name: &str) -> Vec<String> {
        match role_name {
            "管理员" => vec![PERM_ALL.to_string()],
            "策略开发者" => vec![
                PERM_STRATEGY_READ.to_string(),
                PERM_STRATEGY_WRITE.to_string(),
                PERM_BACKTEST_EXECUTE.to_string(),
                PERM_MARKET_READ.to_string(),
            ],
            "交易员" => vec![
                PERM_TRADE_EXECUTE.to_string(),
                PERM_MARKET_READ.to_string(),
                PERM_POSITION_READ.to_string(),
                PERM_ORDER_READ.to_string(),
                PERM_ORDER_WRITE.to_string(),
            ],
            "审计员" => vec![
                PERM_AUDIT_READ.to_string(),
                PERM_TRADE_EXECUTE.to_string(),
                PERM_POSITION_READ.to_string(),
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(role_name: &str) -> UserWithRole {
        UserWithRole {
            id: "1".to_string(),
            username: "test_user".to_string(),
            display_name: None,
            role_id: "role_test".to_string(),
            role_name: role_name.to_string(),
            status: "active".to_string(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_admin_has_all_permissions() {
        let user = create_test_user("管理员");

        assert!(AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
        assert!(AuthService::has_permission(&user, PERM_USER_WRITE));
        assert!(AuthService::has_permission(&user, PERM_AUDIT_READ));
    }

    #[test]
    fn test_developer_limited_permissions() {
        let user = create_test_user("策略开发者");

        assert!(AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(AuthService::has_permission(&user, PERM_STRATEGY_READ));
        assert!(AuthService::has_permission(&user, PERM_BACKTEST_EXECUTE));
        assert!(AuthService::has_permission(&user, PERM_MARKET_READ));
        assert!(!AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
        assert!(!AuthService::has_permission(&user, PERM_ORDER_WRITE));
    }

    #[test]
    fn test_trader_permissions() {
        let user = create_test_user("交易员");

        assert!(AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
        assert!(AuthService::has_permission(&user, PERM_MARKET_READ));
        assert!(AuthService::has_permission(&user, PERM_POSITION_READ));
        assert!(AuthService::has_permission(&user, PERM_ORDER_READ));
        assert!(AuthService::has_permission(&user, PERM_ORDER_WRITE));
        assert!(!AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(!AuthService::has_permission(&user, PERM_USER_WRITE));
    }

    #[test]
    fn test_auditor_permissions() {
        let user = create_test_user("审计员");

        assert!(AuthService::has_permission(&user, PERM_AUDIT_READ));
        assert!(AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
        assert!(AuthService::has_permission(&user, PERM_POSITION_READ));
        assert!(!AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(!AuthService::has_permission(&user, PERM_ORDER_WRITE));
    }

    #[test]
    fn test_unknown_role_no_permissions() {
        let user = create_test_user("未知角色");

        assert!(!AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(!AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
        assert!(!AuthService::has_permission(&user, PERM_AUDIT_READ));
    }

    #[test]
    fn test_has_any_permission() {
        let user = create_test_user("策略开发者");

        assert!(AuthService::has_any_permission(
            &user,
            &[PERM_STRATEGY_WRITE, PERM_TRADE_EXECUTE]
        ));

        assert!(!AuthService::has_any_permission(
            &user,
            &[PERM_TRADE_EXECUTE, PERM_ORDER_WRITE]
        ));
    }
}
