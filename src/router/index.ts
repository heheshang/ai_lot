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
const Exchange = () => import('@/views/settings/Exchange.vue');

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
        meta: { title: '仪表盘' },
      },
      {
        path: 'market',
        name: 'Market',
        component: MarketView,
        meta: { title: '行情' },
      },
      {
        path: 'strategy',
        name: 'StrategyList',
        component: StrategyList,
        meta: { title: '策略列表' },
      },
      {
        path: 'strategy/editor/:id?',
        name: 'StrategyEditor',
        component: StrategyEditor,
        meta: { title: '策略编辑器' },
      },
      {
        path: 'strategy/instances',
        name: 'StrategyInstances',
        component: StrategyInstances,
        meta: { title: '运行实例' },
      },
      {
        path: 'backtest',
        name: 'Backtest',
        component: BacktestView,
        meta: { title: '回测' },
      },
      {
        path: 'trade',
        name: 'Trade',
        component: TradeConsole,
        meta: { title: '交易控制台' },
      },
      {
        path: 'risk',
        name: 'Risk',
        component: RiskMonitor,
        meta: { title: '风险监控' },
      },
      {
        path: 'risk/dashboard',
        name: 'RiskDashboard',
        component: RiskDashboard,
        meta: { title: '风险概览' },
      },
      {
        path: 'risk/rules',
        name: 'RuleConfig',
        component: RuleConfig,
        meta: { title: '规则配置' },
      },
      {
        path: 'risk/alerts',
        name: 'AlertHistory',
        component: AlertHistory,
        meta: { title: '告警历史' },
      },
      {
        path: 'settings',
        name: 'Settings',
        component: Settings,
        meta: { title: '系统设置' },
      },
      {
        path: 'settings/exchange',
        name: 'Exchange',
        component: Exchange,
        meta: { title: '交易所设置' },
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

  // 有 token 就放行，不需要额外检查 user 状态
  // user 状态会在登录时设置，如果刷新页面会由 App.vue 初始化
  next();
});

export default router;
