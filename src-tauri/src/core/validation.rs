//! 输入验证模块
//!
//! 提供统一的参数验证功能

use crate::core::response::ApiError;
use regex::Regex;

/// 验证器trait
pub trait Validator {
    /// 验证输入，返回错误信息（如果有）
    fn validate(&self) -> Result<(), ApiError>;
}

/// 字符串验证器
pub struct StringValidator {
    value: String,
    field_name: String,
}

impl StringValidator {
    pub fn new(value: String, field_name: impl Into<String>) -> Self {
        Self {
            value,
            field_name: field_name.into(),
        }
    }

    /// 检查非空
    pub fn required(self) -> Self {
        self
    }

    /// 检查最小长度
    pub fn min_length(self, min: usize) -> Self {
        if self.value.len() < min {
            log::warn!("Validation failed: {} length < {}", self.field_name, min);
            panic!("Validation failed: {} length < {}", self.field_name, min);
        }
        self
    }

    /// 检查最大长度
    pub fn max_length(self, max: usize) -> Self {
        if self.value.len() > max {
            log::warn!("Validation failed: {} length > {}", self.field_name, max);
            panic!("Validation failed: {} length > {}", self.field_name, max);
        }
        self
    }

    /// 检查长度范围
    pub fn length(self, min: usize, max: usize) -> Self {
        self.min_length(min).max_length(max)
    }

    /// 检查匹配正则表达式
    pub fn matches(self, pattern: &str) -> Self {
        let re = Regex::new(pattern).unwrap_or_else(|_| {
            panic!("Invalid regex pattern: {}", pattern)
        });
        if !re.is_match(&self.value) {
            log::warn!("Validation failed: {} does not match pattern {}", self.field_name, pattern);
            panic!("Validation failed: {} does not match pattern {}", self.field_name, pattern);
        }
        self
    }

    /// 检查是否为合法的邮箱
    pub fn email(self) -> Self {
        self.matches(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
    }

    /// 检查是否为合法的用户名（字母数字下划线）
    pub fn username(self) -> Self {
        self.matches(r"^[a-zA-Z0-9_]{3,20}$")
    }

    /// 检查是否为合法的交易对符号
    pub fn symbol(self) -> Self {
        self.matches(r"^[A-Z]+[A-Z0-9_]+$")
    }

    /// 检查是否在可选列表中
    pub fn one_of(self, options: &[&str]) -> Self {
        if !options.iter().any(|opt| *opt == self.value) {
            log::warn!("Validation failed: {} not in options", self.field_name);
            panic!("Validation failed: {} not in options", self.field_name);
        }
        self
    }

    /// 执行验证（如果存在）
    pub fn validate(&self) -> Result<(), ApiError> {
        if !self.value.is_empty() {
            Ok(())
        } else {
            Err(ApiError::missing_parameter(&self.field_name))
        }
    }
}

/// 数值验证器
pub struct NumberValidator<T> {
    value: T,
    field_name: String,
}

impl<T> NumberValidator<T>
where
    T: PartialOrd + Copy + std::fmt::Display,
{
    pub fn new(value: T, field_name: impl Into<String>) -> Self {
        Self {
            value,
            field_name: field_name.into(),
        }
    }

    /// 检查最小值
    pub fn min(self, min: T) -> Self {
        if self.value < min {
            log::warn!("Validation failed: {} < {}", self.field_name, min);
            panic!("Validation failed: {} < {}", self.field_name, min);
        }
        self
    }

    /// 检查最大值
    pub fn max(self, max: T) -> Self {
        if self.value > max {
            log::warn!("Validation failed: {} > {}", self.field_name, max);
            panic!("Validation failed: {} > {}", self.field_name, max);
        }
        self
    }

    /// 检查范围
    pub fn range(self, min: T, max: T) -> Self {
        self.min(min).max(max)
    }

    /// 检查是否为正数
    pub fn positive(self) -> Self
    where
        T: std::ops::Neg<Output = T> + Default,
    {
        if self.value <= T::default() {
            log::warn!("Validation failed: {} is not positive", self.field_name);
            panic!("Validation failed: {} is not positive", self.field_name);
        }
        self
    }

    /// 执行验证
    pub fn validate(&self) -> Result<(), ApiError> {
        Ok(())
    }
}

/// 验证辅助函数
pub fn validate_string(value: String, field_name: impl Into<String>) -> StringValidator {
    StringValidator::new(value, field_name)
}

pub fn validate_number<T>(value: T, field_name: impl Into<String>) -> NumberValidator<T>
where
    T: PartialOrd + Copy + std::fmt::Display,
{
    NumberValidator::new(value, field_name)
}

/// 验证交易对符号
pub fn validate_symbol(symbol: &str) -> Result<(), ApiError> {
    if symbol.len() < 3 {
        return Err(ApiError::validation_failed("symbol", "长度不能小于3"));
    }
    if symbol.len() > 20 {
        return Err(ApiError::validation_failed("symbol", "长度不能大于20"));
    }

    let re = Regex::new(r"^[A-Z]+[A-Z0-9_]+$").map_err(|_| ApiError::internal_error("Invalid regex"))?;
    if !re.is_match(symbol) {
        return Err(ApiError::validation_failed("symbol", "格式不正确，必须为大写字母、数字或下划线"));
    }

    Ok(())
}

/// 验证K线间隔
pub fn validate_interval(interval: &str) -> Result<(), ApiError> {
    const VALID_INTERVALS: &[&str] = &[
        "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M",
    ];

    if !VALID_INTERVALS.contains(&interval) {
        return Err(ApiError::validation_failed(
            "interval",
            format!("必须是以下之一: {}", VALID_INTERVALS.join(", "))
        ));
    }

    Ok(())
}

/// 验证数量限制
pub fn validate_limit(limit: usize) -> Result<usize, ApiError> {
    if limit < 1 {
        return Err(ApiError::validation_failed("limit", "不能小于1"));
    }
    if limit > 1000 {
        return Err(ApiError::validation_failed("limit", "不能大于1000"));
    }

    Ok(limit)
}

/// 验证ID格式（UUID或特定格式）
pub fn validate_id(id: &str, field_name: impl Into<String>) -> Result<(), ApiError> {
    let field_name = field_name.into();
    if id.is_empty() {
        return Err(ApiError::validation_failed(&field_name, "不能为空"));
    }
    if id.len() > 100 {
        return Err(ApiError::validation_failed(&field_name, "长度不能大于100"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_validation() {
        assert!(validate_symbol("BTCUSDT").is_ok());
        assert!(validate_symbol("ETHUSDT").is_ok());
        assert!(validate_symbol("ETH-USDT").is_err()); // 无效字符
        assert!(validate_symbol("BTC").is_err()); // 太短
    }

    #[test]
    fn test_interval_validation() {
        assert!(validate_interval("1h").is_ok());
        assert!(validate_interval("5m").is_ok());
        assert!(validate_interval("2h").is_err()); // 无效间隔
    }

    #[test]
    fn test_limit_validation() {
        assert!(validate_limit(100).is_ok());
        assert!(validate_limit(0).is_err()); // 太小
        assert!(validate_limit(2000).is_err()); // 太大
    }

    #[test]
    fn test_id_validation() {
        assert!(validate_id("abc123", "test_id").is_ok());
        assert!(validate_id("", "test_id").is_err()); // 空
    }
}
