/**
 * Menu State Composable
 * Manages menu state including active menu, open menus, and menu interactions
 */

import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { MENU_GROUPS, getMenuItemByPath } from '@/views/layout/constants/menu';

export function useMenuState() {
  const route = useRoute();

  // Active menu key (current route)
  const activeMenuKey = ref<string>('');

  // Default opened submenus (for future use with nested menus)
  const defaultOpeneds = ref<string[]>([]);

  // Computed: Active menu full path
  const activeMenuPath = computed(() => {
    const path = route.path;
    // Strategy related pages use parent strategy menu
    if (path.startsWith('/strategy')) {
      return path;
    }
    return path;
  });

  // Update active menu when route changes
  watch(
    () => route.path,
    (newPath) => {
      const menuItem = getMenuItemByPath(newPath);
      if (menuItem) {
        activeMenuKey.value = menuItem.path;
      }
    },
    { immediate: true }
  );

  // Get menu groups
  const getMenuGroups = () => MENU_GROUPS;

  // Check if a menu item is active
  const isMenuItemActive = (path: string): boolean => {
    const currentPath = route.path;
    if (path === currentPath) return true;
    // Check for prefix match (e.g., /strategy matches /strategy/editor)
    if (currentPath.startsWith(path) && path !== '/') {
      return true;
    }
    return false;
  };

  return {
    // State
    activeMenuKey,
    defaultOpeneds,

    // Computed
    activeMenuPath,

    // Methods
    getMenuGroups,
    isMenuItemActive,
  };
}

// Default export for convenience
export default useMenuState;