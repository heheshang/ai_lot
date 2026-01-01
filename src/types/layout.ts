/**
 * Layout-related type definitions
 */

import type { Component } from 'vue';

// ========== Menu Types ==========

/**
 * Menu item interface
 */
export interface MenuItem {
  /** Route path */
  path: string;
  /** Display title */
  title: string;
  /** Icon component */
  icon: Component;
  /** Group label (for sectioning) */
  group?: string;
  /** Whether to show in collapsed sidebar */
  showWhenCollapsed?: boolean;
  /** Badge count */
  badge?: number;
  /** External link URL */
  external?: string;
  /** Menu item is disabled */
  disabled?: boolean;
}

/**
 * Menu group configuration
 */
export interface MenuGroup {
  /** Group identifier */
  id: string;
  /** Display label */
  label: string;
  /** Menu items in this group */
  items: MenuItem[];
  /** Group icon */
  icon?: Component;
}

/**
 * Menu state interface
 */
export interface MenuState {
  /** Currently active menu path */
  activeMenu: string;
  /** Default opened submenus */
  defaultOpeneds: string[];
  /** Menu groups */
  menuGroups: MenuGroup[];
}

// ========== Layout State Types ==========

/**
 * Layout state interface
 */
export interface LayoutState {
  /** Sidebar collapse state */
  isCollapse: boolean;
  /** Dark theme state */
  isDark: boolean;
  /** Mobile menu open state */
  isMobileMenuOpen: boolean;
}

/**
 * Layout config interface
 */
export interface LayoutConfig {
  /** Default sidebar collapsed state */
  defaultCollapsed?: boolean;
  /** Default dark theme state */
  defaultDark?: boolean;
  /** Sidebar width */
  sidebarWidth?: string | number;
  /** Header height */
  headerHeight?: string | number;
  /** Enable localStorage persistence */
  enablePersistence?: boolean;
}

// ========== Navigation Types ==========

/**
 * Navigation history item
 */
export interface NavigationHistoryItem {
  /** Route path */
  path: string;
  /** Page title */
  title: string;
  /** Visit timestamp */
  timestamp: number;
  /** Visit count */
  count?: number;
}

/**
 * Navigation state interface
 */
export interface NavigationState {
  /** Navigation history */
  history: NavigationHistoryItem[];
  /** Maximum history size */
  maxHistorySize: number;
}

// ========== User Dropdown Types ==========

/**
 * User menu command type
 */
export type UserMenuCommand = 'profile' | 'settings' | 'logout';

/**
 * User stats interface
 */
export interface UserStats {
  /** User ID display (last 6 chars) */
  idDisplay: string;
  /** Account creation date display */
  createdAtDisplay: string;
  /** User status display */
  statusDisplay: string;
}

// ========== Component Props Types ==========

/**
 * AppSidebar props
 */
export interface AppSidebarProps {
  /** Collapsed state */
  collapsed?: boolean;
  /** Menu groups */
  menuGroups?: MenuGroup[];
}

/**
 * AppHeader props
 */
export interface AppHeaderProps {
  /** Notification count */
  notificationCount?: number;
  /** Show breadcrumbs */
  showBreadcrumbs?: boolean;
  /** Show search */
  showSearch?: boolean;
}

/**
 * RecentPagesDropdown props
 */
export interface RecentPagesDropdownProps {
  /** Maximum items to display */
  maxItems?: number;
  /** Show quick access */
  showQuickAccess?: boolean;
  /** Show footer stats */
  showStats?: boolean;
}

/**
 * UserDropdown props
 */
export interface UserDropdownProps {
  /** User avatar size */
  avatarSize?: number;
  /** Show quick actions */
  showQuickActions?: boolean;
}

/**
 * MobileDrawer props
 */
export interface MobileDrawerProps {
  /** Drawer open state */
  open: boolean;
  /** Drawer size */
  size?: string | number;
  /** Direction */
  direction?: 'ltr' | 'rtl';
}

// ========== Animation Types ==========

/**
 * Transition name type
 */
export type TransitionName =
  | 'page'
  | 'fade'
  | 'slide-up'
  | 'slide-down'
  | 'slide-left'
  | 'slide-right'
  | 'scale'
  | 'bounce';

/**
 * Animation duration preset
 */
export type AnimationDuration = 'fast' | 'normal' | 'slow';

/**
 * Animation easing preset
 */
export type AnimationEasing = 'linear' | 'ease' | 'ease-in' | 'ease-out' | 'ease-in-out';

// ========== Breakpoint Types ==========

/**
 * Screen breakpoint
 */
export type Breakpoint = 'xs' | 'sm' | 'md' | 'lg' | 'xl' | 'xxl';

/**
 * Responsive config
 */
export interface ResponsiveConfig {
  /** Mobile breakpoint */
  mobile?: number;
  /** Tablet breakpoint */
  tablet?: number;
  /** Desktop breakpoint */
  desktop?: number;
}

// ========== Theme Types ==========

/**
 * Theme mode
 */
export type ThemeMode = 'light' | 'dark' | 'auto';

/**
 * Theme config
 */
export interface ThemeConfig {
  /** Current theme mode */
  mode?: ThemeMode;
  /** Primary color */
  primaryColor?: string;
  /** Border radius */
  borderRadius?: string | number;
}

// ========== Utility Types ==========

/**
 * Make specific properties optional
 */
export type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

/**
 * Make specific properties required
 */
export type RequiredBy<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

/**
 * Deep partial type
 */
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};
