import { createRouter, createWebHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';

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

// 路由守卫
router.beforeEach((to, _from, next) => {
  const token = localStorage.getItem('token');

  if (to.meta.requiresAuth !== false && !token) {
    next({ name: 'Login', query: { redirect: to.fullPath } });
  } else if (to.name === 'Login' && token) {
    next({ name: 'Dashboard' });
  } else {
    next();
  }
});

export default router;
