/**
 * Unified Error Handler for AI-LOT Frontend
 *
 * This module provides consistent error handling across the frontend,
 * including error parsing, user-friendly messages, and UI notifications.
 */

import { ElMessage } from 'element-plus';

/**
 * Application error interface
 */
export interface AppError {
  /** Error code for programmatic handling */
  code: string;
  /** User-friendly error message */
  message: string;
  /** Additional error details for debugging */
  details?: string;
  /** Original error object */
  original?: any;
}

/**
 * Error codes mapping to user-friendly messages
 */
const ERROR_MESSAGES: Record<string, string> = {
  DATABASE_ERROR: '数据库操作失败，请稍后重试',
  EXCHANGE_ERROR: '交易所连接失败，请检查网络设置',
  STRATEGY_ERROR: '策略执行出错，请检查策略配置',
  AUTH_ERROR: '身份验证失败，请重新登录',
  PERMISSION_ERROR: '权限不足，无法执行此操作',
  VALIDATION_ERROR: '输入数据验证失败，请检查输入内容',
  IO_ERROR: '文件操作失败',
  SERIALIZATION_ERROR: '数据处理失败',
  NETWORK_ERROR: '网络连接失败，请检查网络',
  RISK_LIMIT_ERROR: '超出风险限制，操作被拒绝',
  GENERIC_ERROR: '操作失败，请稍后重试',
};

/**
 * Error severity levels
 */
export enum ErrorSeverity {
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
}

/**
 * Error severity mapping by error code
 */
const ERROR_SEVERITY: Record<string, ErrorSeverity> = {
  VALIDATION_ERROR: ErrorSeverity.WARNING,
  RISK_LIMIT_ERROR: ErrorSeverity.WARNING,
  AUTH_ERROR: ErrorSeverity.ERROR,
  PERMISSION_ERROR: ErrorSeverity.ERROR,
  DATABASE_ERROR: ErrorSeverity.ERROR,
  EXCHANGE_ERROR: ErrorSeverity.ERROR,
  STRATEGY_ERROR: ErrorSeverity.ERROR,
  NETWORK_ERROR: ErrorSeverity.ERROR,
  IO_ERROR: ErrorSeverity.ERROR,
  SERIALIZATION_ERROR: ErrorSeverity.ERROR,
  GENERIC_ERROR: ErrorSeverity.ERROR,
};

/**
 * ErrorHandler class for unified error handling
 */
export class ErrorHandler {
  /**
   * Parse different error types into AppError
   */
  static handle(error: any): AppError {
    // Handle string errors
    if (typeof error === 'string') {
      return this.parseStringError(error);
    }

    // Handle Error objects
    if (error instanceof Error) {
      return this.parseErrorObject(error);
    }

    // Handle objects with error structure
    if (typeof error === 'object' && error !== null) {
      return this.parseErrorObject(error);
    }

    // Fallback for unknown error types
    return {
      code: 'GENERIC_ERROR',
      message: ERROR_MESSAGES.GENERIC_ERROR,
      details: String(error),
      original: error,
    };
  }

  /**
   * Parse string errors in format [ERROR_CODE] message
   */
  private static parseStringError(errorStr: string): AppError {
    // Match pattern: [ERROR_CODE] message
    const match = errorStr.match(/^\[([A-Z_]+)\]\s*(.+)$/);

    if (match) {
      const code = match[1];
      const message = match[2].trim();
      return {
        code,
        message,
        details: errorStr,
      };
    }

    // Try to detect error type from message content
    for (const [code, msg] of Object.entries(ERROR_MESSAGES)) {
      if (errorStr.toLowerCase().includes(code.toLowerCase()) ||
          errorStr.toLowerCase().includes(msg.toLowerCase())) {
        return {
          code,
          message: errorStr,
          details: errorStr,
        };
      }
    }

    // Fallback
    return {
      code: 'GENERIC_ERROR',
      message: errorStr || ERROR_MESSAGES.GENERIC_ERROR,
      details: errorStr,
    };
  }

  /**
   * Parse Error objects or error-like objects
   */
  private static parseErrorObject(error: any): AppError {
    const message = error.message || String(error);
    const stringError = this.parseStringError(message);

    return {
      ...stringError,
      details: error.stack || error.details || message,
      original: error,
    };
  }

  /**
   * Display error with ElMessage notification
   */
  static show(error: AppError | any): void {
    const parsedError = error instanceof Object && 'code' in error
      ? error as AppError
      : this.handle(error);

    const severity = ERROR_SEVERITY[parsedError.code] || ErrorSeverity.ERROR;

    ElMessage({
      message: parsedError.message,
      type: severity,
      duration: severity === ErrorSeverity.ERROR ? 5000 : 3000,
      showClose: true,
    });

    // Log to console for debugging
    if (parsedError.details) {
      console.error('[Error]', parsedError.code, parsedError.message, parsedError.details);
    } else {
      console.error('[Error]', parsedError.code, parsedError.message);
    }
  }

  /**
   * Log error without showing notification
   */
  static log(error: AppError | any, context?: string): void {
    const parsedError = error instanceof Object && 'code' in error
      ? error as AppError
      : this.handle(error);

    const logMessage = context
      ? `[${context}] ${parsedError.code}: ${parsedError.message}`
      : `${parsedError.code}: ${parsedError.message}`;

    console.error(logMessage);

    if (parsedError.details) {
      console.error('Details:', parsedError.details);
    }

    if (parsedError.original) {
      console.error('Original error:', parsedError.original);
    }
  }

  /**
   * Get user-friendly message for error
   */
  static getMessage(error: any): string {
    const parsedError = this.handle(error);
    return parsedError.message;
  }

  /**
   * Get error code
   */
  static getCode(error: any): string {
    const parsedError = this.handle(error);
    return parsedError.code;
  }

  /**
   * Check if error matches specific code
   */
  static isErrorCode(error: any, code: string): boolean {
    return this.getCode(error) === code;
  }

  /**
   * Check if error is validation error
   */
  static isValidationError(error: any): boolean {
    return this.isErrorCode(error, 'VALIDATION_ERROR');
  }

  /**
   * Check if error is auth error
   */
  static isAuthError(error: any): boolean {
    return this.isErrorCode(error, 'AUTH_ERROR');
  }

  /**
   * Check if error is permission error
   */
  static isPermissionError(error: any): boolean {
    return this.isErrorCode(error, 'PERMISSION_ERROR');
  }

  /**
   * Check if error is network error
   */
  static isNetworkError(error: any): boolean {
    return this.isErrorCode(error, 'NETWORK_ERROR');
  }

  /**
   * Check if error is risk limit error
   */
  static isRiskLimitError(error: any): boolean {
    return this.isErrorCode(error, 'RISK_LIMIT_ERROR');
  }
}

/**
 * API call wrapper with automatic error handling
 *
 * @param fn - Async function to execute
 * @param options - Options for error handling
 * @returns Promise with result or throws error
 */
export async function callApi<T>(
  fn: () => Promise<T>,
  options?: {
    /** Show error notification on failure (default: true) */
    showError?: boolean;
    /** Custom error message context */
    context?: string;
    /** Custom error handler */
    onError?: (error: AppError) => void;
  }
): Promise<T> {
  const {
    showError = true,
    context,
    onError,
  } = options || {};

  try {
    return await fn();
  } catch (error) {
    const appError = ErrorHandler.handle(error);

    // Log error
    if (context) {
      ErrorHandler.log(appError, context);
    } else {
      ErrorHandler.log(appError);
    }

    // Show notification
    if (showError) {
      ErrorHandler.show(appError);
    }

    // Call custom error handler
    if (onError) {
      onError(appError);
    }

    throw appError;
  }
}

/**
 * Decorator for Vue methods to handle errors automatically
 *
 * @param target - Target object
 * @param propertyKey - Method name
 * @param descriptor - Property descriptor
 */
export function handleErrors(
  _target: any,
  _propertyKey: string,
  descriptor: PropertyDescriptor
) {
  const originalMethod = descriptor.value;

  descriptor.value = async function (...args: any[]) {
    try {
      return await originalMethod.apply(this, args);
    } catch (error) {
      ErrorHandler.show(error);
      throw error;
    }
  };

  return descriptor;
}

/**
 * Vue composable for error handling
 */
export function useErrorHandler() {
  return {
    handleError: (error: any) => ErrorHandler.show(error),
    logError: (error: any, context?: string) => ErrorHandler.log(error, context),
    callApi,
    isValidationError: ErrorHandler.isValidationError.bind(ErrorHandler),
    isAuthError: ErrorHandler.isAuthError.bind(ErrorHandler),
    isPermissionError: ErrorHandler.isPermissionError.bind(ErrorHandler),
    isNetworkError: ErrorHandler.isNetworkError.bind(ErrorHandler),
    isRiskLimitError: ErrorHandler.isRiskLimitError.bind(ErrorHandler),
  };
}

export default ErrorHandler;
