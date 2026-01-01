/**
 * User-related utilities
 */

import { UserStatus } from '@/types';

/**
 * Get display text for user status
 */
export function getStatusText(status?: UserStatus): string {
  if (status === UserStatus.ACTIVE) return '正常';
  if (status === UserStatus.DISABLED) return '已禁用';
  if (status === UserStatus.LOCKED) return '已锁定';
  return '未知';
}

/**
 * Get Element Plus tag type for role name
 */
export function getRoleTagType(roleName: string): 'success' | 'warning' | 'info' | 'danger' | '' {
  const roleMap: Record<string, 'success' | 'warning' | 'info' | 'danger' | ''> = {
    '管理员': 'danger',
    '开发者': 'warning',
    '交易员': 'success',
    '审计员': 'info',
  };
  return roleMap[roleName] || '';
}

/**
 * Get display name for user
 */
export function getUserDisplayName(displayName?: string, username?: string): string {
  return displayName || username || '未知用户';
}
