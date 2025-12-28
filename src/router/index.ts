import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';
import { useUserStore } from '@/store/modules/user';

// 布局组件
const Layout = () => import('@/views/Layout.vue');

// 页面组件
const Login = () => import('@/views/auth/Login.vue');
const Dashboard = () => import('@/views/Dashboard.vue');
const MarketView = () => import('@/views/Market/MarketView.vue');
const StrategyList = () => import('@/views/Strategy/StrategyList.vue');
const StrategyEditor = () => import('@/views/Strategy/StrategyEditor.vue');
const StrategyInstances = () => import('@/views/Strategy/StrategyInstances.vue');
const BacktestView = () => import('@/views/Backtest/BacktestView.vue');
const TradeConsole = () => import('@/views/Trade/TradeConsole.vue');
const RiskMonitor = () => import('@/views/Risk/RiskMonitor.vue');
const RiskDashboard = () => import('@/views/Risk/Dashboard.vue');
const RuleConfig = () => import('@/views/Risk/RuleConfig.vue');
const AlertHistory = () => import('@/views/Risk/AlertHistory.vue');
const Settings = () => import('@/views/settings/Settings.vue');

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: { requiresAuth: false },
  },
  {
    path: '/',
    component: Layout,
    redirect: '/dashboard',
    meta: { requiresAuth: true },
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: Dashboard,
      },
      {
        path: 'market',
        name: 'Market',
        component: MarketView,
      },
      {
        path: 'strategy',
        name: 'StrategyList',
        component: StrategyList,
      },
      {
        path: 'strategy/editor/:id?',
        name: 'StrategyEditor',
        component: StrategyEditor,
      },
      {
        path: 'strategy/instances',
        name: 'StrategyInstances',
        component: StrategyInstances,
      },
      {
        path: 'backtest',
        name: 'Backtest',
        component: BacktestView,
      },
      {
        path: 'trade',
        name: 'Trade',
        component: TradeConsole,
      },
      {
        path: 'risk',
        name: 'Risk',
        component: RiskMonitor,
      },
      {
        path: 'risk/dashboard',
        name: 'RiskDashboard',
        component: RiskDashboard,
      },
      {
        path: 'risk/rules',
        name: 'RuleConfig',
        component: RuleConfig,
      },
      {
        path: 'risk/alerts',
        name: 'AlertHistory',
        component: AlertHistory,
      },
      {
        path: 'settings',
        name: 'Settings',
        component: Settings,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// 路由守卫 - 全局登录控制
router.beforeEach(async (to, _from, next) => {
  const userStore = useUserStore();
  const token = localStorage.getItem('token');

  // 如果页面不需要认证（如登录页），直接放行
  if (to.meta.requiresAuth === false) {
    // 如果已登录用户访问登录页，重定向到首页
    if (to.name === 'Login' && userStore.user) {
      next({ name: 'Dashboard' });
    } else {
      next();
    }
    return;
  }

  // 需要认证的页面
  if (!token) {
    // 没有 token，跳转到登录页
    next({
      name: 'Login',
      query: { redirect: to.fullPath }
    });
    return;
  }

  // 有 token 但没有用户信息，尝试恢复用户状态
  if (!userStore.user) {
    try {
      await userStore.restoreUser();
      // 恢复成功，继续导航
      next();
    } catch (error) {
      // 恢复失败，清除无效 token 并跳转到登录页
      localStorage.removeItem('token');
      next({
        name: 'Login',
        query: { redirect: to.fullPath }
      });
    }
  } else {
    // 已登录用户，正常导航
    next();
  }
});

export default router;
