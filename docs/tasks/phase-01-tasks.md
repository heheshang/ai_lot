# Phase 1: 基础框架 - 详细任务规范

## 目标

搭建项目基础架构，实现用户认证系统和基础 UI 框架。

---

## 任务状态跟踪

| 任务 ID | 任务名称 | 状态 | 完成日期 |
|---------|----------|------|----------|
| P1-01 | 更新 package.json 依赖 | ✅ 已完成 | 2025-12-25 |
| P1-02 | 创建项目目录结构 | ✅ 已完成 | 2025-12-25 |
| P1-03 | 配置 TypeScript 类型定义 | ✅ 已完成 | 2025-12-25 |
| P1-04 | 配置 ESLint + Prettier | ✅ 已完成 | 2025-12-25 |
| P1-05 | 配置 Tauri 基础设置 | ✅ 已完成 | 2025-12-25 |
| P1-06 | 创建数据库迁移文件 | ✅ 已完成 | 2025-12-25 |
| P1-07 | 实现 sqlx 数据库连接 | ✅ 已完成 | 2025-12-25 |
| P1-08 | 实现数据模型 | ✅ 已完成 | 2025-12-25 |
| P1-09 | 实现 Repository Trait | ✅ 已完成 | 2025-12-25 |
| P1-10 | 实现 UserRepository | ✅ 已完成 | 2025-12-25 |
| P1-11 | 实现密码哈希 | ✅ 已完成 | 2025-12-25 |
| P1-12 | 实现用户登录 Tauri 命令 | ✅ 已完成 | 2025-12-25 |
| P1-13 | 实现权限检查中间件 | ✅ 已完成 | 2025-12-25 |
| P1-14 | 实现操作审计日志 | ✅ 已完成 | 2025-12-25 |
| P1-15 | 配置 Vue Router | ✅ 已完成 | 2025-12-25 |
| P1-16 | 配置 Pinia Store | ✅ 已完成 | 2025-12-25 |
| P1-17 | 实现 Tauri API 封装 | ✅ 已完成 | 2025-12-25 |
| P1-18 | 实现 Login 页面 | ✅ 已完成 | 2025-12-25 |
| P1-19 | 实现 Dashboard 布局 | ✅ 已完成 | 2025-12-25 |
| P1-20 | 实现用户状态管理 | ✅ 已完成 | 2025-12-25 |

---

## 任务列表

### P1-01: 更新 package.json 依赖 ✅

**估时**: 0.5h | **优先级**: P0 | **依赖**: 无

#### 实施步骤

1. 更新 `package.json` 依赖：
```json
{
  "dependencies": {
    "vue": "^3.5.13",
    "vue-router": "^4.4.0",
    "pinia": "^2.2.4",
    "element-plus": "^2.8.4",
    "@element-plus/icons-vue": "^2.3.1",
    "echarts": "^5.5.0",
    "monaco-editor": "^0.50.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.2.1",
    "typescript": "~5.6.2",
    "vite": "^6.0.3",
    "vue-tsc": "^2.1.10",
    "@vue/tsconfig": "^0.5.1",
    "eslint": "^9.14.0",
    "prettier": "^3.3.3",
    "@types/node": "^22.10.1"
  }
}
```

2. 运行安装：
```bash
bun install
```

#### 验收标准
- [x] 依赖安装完成，无报错
- [x] `bun run dev` 可正常启动

#### 产物
- `package.json`
- `bun.lockb`

---

### P1-02: 创建项目目录结构 ✅

**估时**: 1h | **优先级**: P0 | **依赖**: P1-01

#### 实施步骤

1. 创建前端目录结构：
```bash
src/
├── api/                # API 调用层
│   └── tauri.ts
├── assets/             # 静态资源
├── components/         # 通用组件
│   ├── common/
│   └── charts/
├── router/             # 路由配置
│   ├── index.ts
│   └── routes.ts
├── store/              # 状态管理
│   ├── index.ts
│   └── modules/
│       ├── user.ts
│       ├── market.ts
│       ├── strategy.ts
│       └── trade.ts
├── styles/             # 样式文件
│   ├── index.scss
│   └── variables.scss
├── types/              # 类型定义
│   └── index.ts
├── utils/              # 工具函数
│   ├── format.ts
│   └── validation.ts
├── views/              # 页面组件
│   ├── auth/
│   │   └── Login.vue
│   ├── Dashboard.vue
│   └── settings/
├── App.vue
└── main.ts
```

2. 创建后端目录结构：
```bash
src-tauri/src/
├── commands/           # Tauri 命令
│   ├── mod.rs
│   ├── user.rs
│   ├── market.rs
│   ├── strategy.rs
│   └── trade.rs
├── core/               # 核心业务逻辑
│   ├── mod.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   └── password.rs
│   └── trade/
│       └── mod.rs
├── infrastructure/     # 基础设施
│   ├── mod.rs
│   ├── database.rs
│   ├── crypto.rs
│   └── audit.rs
├── models/             # 数据模型
│   ├── mod.rs
│   ├── user.rs
│   └── exchange.rs
├── repository/         # 数据访问层
│   ├── mod.rs
│   └── user_repo.rs
├── utils/              # 工具函数
│   └── mod.rs
├── lib.rs
└── main.rs
```

3. 创建数据库迁移目录：
```bash
src-tauri/migrations/
└── 001_initial_schema.sql
```

#### 验收标准
- [ ] 所有目录创建完成
- [ ] 所有 `mod.rs` 文件包含 `pub mod xxx;` 声明

#### 产物
- 完整的项目目录结构

---

### P1-03: 配置 TypeScript 类型定义 ✅

**估时**: 1h | **优先级**: P0 | **依赖**: P1-02

#### 实施步骤

1. 创建 `src/types/index.ts`：
```typescript
// ============== 通用类型 ==============
export type Nullable<T> = T | null;
export type Optional<T> = T | undefined;

// ============== 用户类型 ==============
export enum UserRole {
  ADMIN = 'admin',
  DEVELOPER = 'developer',
  TRADER = 'trader',
  AUDITOR = 'auditor',
}

export enum UserStatus {
  ACTIVE = 'active',
  DISABLED = 'disabled',
  LOCKED = 'locked',
}

export interface User {
  id: string;
  username: string;
  displayName?: string;
  roleId: string;
  roleName: string;
  status: UserStatus;
  createdAt: number;
  updatedAt: number;
}

// ============== 认证类型 ==============
export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  user: User;
  token: string;
}

// ============== 交易所类型 ==============
export enum ExchangeName {
  BINANCE = 'binance',
  OKX = 'okx',
  BYBIT = 'bybit',
}

export interface ExchangeConfig {
  id: string;
  exchangeName: ExchangeName;
  displayName: string;
  isTestnet: boolean;
  status: 'active' | 'inactive' | 'error';
  createdAt: number;
}

// ============== 行情类型 ==============
export interface Ticker {
  symbol: string;
  price: number;
  priceChange: number;
  priceChangePercent: number;
  high24h: number;
  low24h: number;
  volume24h: number;
  timestamp: number;
}

export interface Kline {
  symbol: string;
  timeframe: string;
  timestamp: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

// ============== 策略类型 ==============
export interface StrategyParameter {
  name: string;
  type: 'number' | 'string' | 'boolean' | 'select';
  default: any;
  min?: number;
  max?: number;
  step?: number;
  options?: { label: string; value: any }[];
  description?: string;
}

export interface Strategy {
  id: string;
  userId: string;
  name: string;
  description?: string;
  code: string;
  language: 'javascript' | 'typescript';
  parameters: StrategyParameter[];
  parameterValues: Record<string, any>;
  category?: string;
  tags: string[];
  version: number;
  status: 'draft' | 'testing' | 'active' | 'archived';
  createdAt: number;
  updatedAt: number;
}

export interface StrategyInstance {
  id: string;
  strategyId: string;
  strategyName: string;
  exchangeId: string;
  symbol: string;
  timeframe: string;
  mode: 'backtest' | 'paper_trading' | 'live';
  status: 'stopped' | 'running' | 'paused' | 'error';
  parameters: Record<string, any>;
  startTime?: number;
  stopTime?: number;
  totalTrades: number;
  totalPnl: number;
  maxDrawdown: number;
  createdAt: number;
}

// ============== 交易类型 ==============
export enum OrderSide {
  BUY = 'buy',
  SELL = 'sell',
}

export enum OrderType {
  MARKET = 'market',
  LIMIT = 'limit',
  STOP_LIMIT = 'stop_limit',
  STOP_MARKET = 'stop_market',
}

export enum OrderStatus {
  PENDING = 'pending',
  OPEN = 'open',
  PARTIALLY_FILLED = 'partially_filled',
  FILLED = 'filled',
  CANCELED = 'canceled',
  REJECTED = 'rejected',
}

export interface Order {
  id: string;
  exchangeOrderId?: string;
  clientOrderId?: string;
  symbol: string;
  side: OrderSide;
  type: OrderType;
  price?: number;
  quantity: number;
  filledQuantity: number;
  avgPrice?: number;
  status: OrderStatus;
  commission: number;
  createdAt: number;
  filledAt?: number;
}

export interface Position {
  id: string;
  symbol: string;
  side: 'long' | 'short';
  quantity: number;
  entryPrice: number;
  currentPrice?: number;
  unrealizedPnl: number;
  realizedPnl: number;
  openedAt: number;
}

// ============== 回测类型 ==============
export interface BacktestParams {
  strategyId: string;
  symbol: string;
  timeframe: string;
  startTime: number;
  endTime: number;
  initialBalance: number;
  commissionRate: number;
  slippage: number;
  strategyParams: Record<string, any>;
}

export interface BacktestReport {
  id: string;
  strategyId: string;
  params: BacktestParams;
  totalReturn: number;
  sharpeRatio: number;
  maxDrawdown: number;
  winRate: number;
  totalTrades: number;
  winningTrades: number;
  losingTrades: number;
  trades: any[];
  equityCurve: number[];
  status: 'pending' | 'running' | 'completed' | 'failed';
}

// ============== 风控类型 ==============
export enum RiskRuleType {
  POSITION_LIMIT = 'position_limit',
  DRAWDOWN_LIMIT = 'drawdown_limit',
  LOSS_LIMIT = 'loss_limit',
}

export enum RiskAction {
  ALERT = 'alert',
  CLOSE_POSITION = 'close_position',
  STOP_STRATEGY = 'stop_strategy',
}

export interface RiskRule {
  id: string;
  name: string;
  description?: string;
  ruleType: RiskRuleType;
  config: Record<string, any>;
  thresholdValue: number;
  action: RiskAction;
  isEnabled: boolean;
}

export interface RiskAlert {
  id: string;
  ruleId: string;
  severity: 'info' | 'warning' | 'critical';
  title: string;
  message: string;
  currentValue: number;
  thresholdValue: number;
  status: 'active' | 'acknowledged' | 'resolved';
  createdAt: number;
}

// ============== API 响应类型 ==============
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  code?: number;
}

// ============== Tauri 命令类型 ==============
export interface TauriCommand {
  cmd: string;
  payload?: any;
}
```

2. 更新 `tsconfig.json`：
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,

    /* Path mapping */
    "baseUrl": ".",
    "paths": {
      "@/*": ["src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

#### 验收标准
- [x] 类型定义完整，无编译错误
- [x] 路径别名 `@/` 可正常使用

#### 产物
- `src/types/index.ts`
- `tsconfig.json`

---

### P1-04: 配置 ESLint + Prettier ✅

**估时**: 0.5h | **优先级**: P1 | **依赖**: P1-02

#### 实施步骤

1. 创建 `.eslintrc.cjs`：
```javascript
module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:vue/vue3-recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    ecmaVersion: 'latest',
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
  },
  plugins: ['vue', '@typescript-eslint'],
  rules: {
    'vue/multi-word-component-names': 'off',
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
  },
};
```

2. 创建 `.prettierrc`：
```json
{
  "semi": false,
  "singleQuote": true,
  "printWidth": 100,
  "trailingComma": "es5",
  "arrowParens": "always"
}
```

3. 更新 `package.json` scripts：
```json
{
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "lint": "eslint src --ext .vue,.ts",
    "lint:fix": "eslint src --ext .vue,.ts --fix",
    "format": "prettier --write src/**/*.{vue,ts,scss}"
  }
}
```

#### 验收标准
- [x] `bun run lint` 无报错
- [x] `bun run format` 可格式化代码

#### 产物
- `.eslintrc.cjs`
- `.prettierrc`

---

### P1-05: 配置 Tauri 基础设置 ✅

**估时**: 0.5h | **优先级**: P0 | **依赖**: P1-02

#### 实施步骤

1. 更新 `src-tauri/tauri.conf.json`：
```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "AI-LOT",
  "version": "0.1.0",
  "identifier": "com.ai-lot.app",
  "build": {
    "beforeDevCommand": "bun run dev",
    "beforeBuildCommand": "bun run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "AI-LOT 量化交易",
        "width": 1400,
        "height": 900,
        "minWidth": 1200,
        "minHeight": 700,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png"]
  }
}
```

2. 更新 `src-tauri/Cargo.toml` 依赖：
```toml
[package]
name = "ai-lot"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
argon2 = "0.5"
aes-gcm = "0.10"
```

#### 验收标准
- [x] `bun run tauri dev` 可正常启动应用
- [x] 窗口标题、尺寸符合配置

#### 产物
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`

---

### P1-06: 创建数据库迁移文件 ✅

**估时**: 2h | **优先级**: P0 | **依赖**: P1-02

#### 实施步骤

1. 创建 `src-tauri/migrations/001_initial_schema.sql`：
```sql
-- ============== 用户与权限 ==============
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    role_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    salt TEXT NOT NULL,
    failed_attempts INTEGER DEFAULT 0,
    locked_until INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE roles (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    permissions TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_role_id ON users(role_id);

-- 预置角色
INSERT INTO roles (id, name, description, permissions, created_at) VALUES
('role_admin', '管理员', '全部权限', '["*"]', 0),
('role_developer', '策略开发者', '策略编写、回测', '["strategy:read","strategy:write","backtest:execute","market:read"]', 0),
('role_trader', '交易员', '执行实盘交易', '["trade:execute","market:read","position:read","order:read","order:write"]', 0),
('role_auditor', '审计员', '只读查看日志', '["audit:read","trade:read","position:read"]', 0);

-- 预置管理员账户 (密码: admin123)
INSERT INTO users (id, username, password_hash, display_name, role_id, status, salt, created_at, updated_at) VALUES
('u_admin', 'admin', '$argon2id$v=19$m=4096,t=3,p=1$xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx', '系统管理员', 'role_admin', 'active', 'xxxxxxxxxxxxxxxx', 0, 0);

-- ============== 交易所配置 ==============
CREATE TABLE exchanges (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,
    is_testnet INTEGER DEFAULT 0,
    status TEXT DEFAULT 'inactive',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_exchanges_user_id ON exchanges(user_id);

-- ============== 策略相关 ==============
CREATE TABLE strategies (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    code TEXT NOT NULL,
    language TEXT NOT NULL,
    parameters TEXT,
    category TEXT,
    tags TEXT,
    version INTEGER DEFAULT 1,
    parent_id TEXT,
    status TEXT DEFAULT 'draft',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES strategies(id)
);

CREATE TABLE strategy_instances (
    id TEXT PRIMARY KEY,
    strategy_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    parameters TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    mode TEXT NOT NULL,
    status TEXT DEFAULT 'stopped',
    error_message TEXT,
    start_time INTEGER,
    stop_time INTEGER,
    total_trades INTEGER DEFAULT 0,
    total_pnl REAL DEFAULT 0,
    max_drawdown REAL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id)
);

CREATE INDEX idx_strategies_user_id ON strategies(user_id);
CREATE INDEX idx_strategies_status ON strategies(status);
CREATE INDEX idx_strategy_instances_strategy_id ON strategy_instances(strategy_id);
CREATE INDEX idx_strategy_instances_user_id ON strategy_instances(user_id);
CREATE INDEX idx_strategy_instances_status ON strategy_instances(status);

-- ============== 交易相关 ==============
CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    strategy_instance_id TEXT,
    exchange_order_id TEXT UNIQUE,
    client_order_id TEXT UNIQUE,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    order_type TEXT NOT NULL,
    price REAL,
    quantity REAL NOT NULL,
    filled_quantity REAL DEFAULT 0,
    avg_price REAL,
    status TEXT NOT NULL,
    commission REAL DEFAULT 0,
    commission_asset TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    filled_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE TABLE positions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    strategy_instance_id TEXT,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    quantity REAL NOT NULL,
    entry_price REAL NOT NULL,
    current_price REAL,
    unrealized_pnl REAL DEFAULT 0,
    realized_pnl REAL DEFAULT 0,
    opened_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    closed_at INTEGER,
    status TEXT DEFAULT 'open',
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE TABLE trades (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    exchange_trade_id TEXT UNIQUE,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    price REAL NOT NULL,
    quantity REAL NOT NULL,
    commission REAL DEFAULT 0,
    commission_asset TEXT,
    pnl REAL,
    timestamp INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_exchange_id ON orders(exchange_id);
CREATE INDEX idx_orders_strategy_instance_id ON orders(strategy_instance_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX idx_positions_user_id ON positions(user_id);
CREATE INDEX idx_positions_exchange_id ON positions(exchange_id);
CREATE INDEX idx_positions_strategy_instance_id ON positions(strategy_instance_id);
CREATE INDEX idx_positions_status ON positions(status);
CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_exchange_id ON trades(exchange_id);
CREATE INDEX idx_trades_order_id ON trades(order_id);
CREATE INDEX idx_trades_timestamp ON trades(timestamp DESC);

-- ============== 行情数据 ==============
CREATE TABLE klines (
    exchange_name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume REAL NOT NULL,
    PRIMARY KEY (exchange_name, symbol, timeframe, timestamp)
);

CREATE INDEX idx_klines_symbol_timeframe ON klines(symbol, timeframe);
CREATE INDEX idx_klines_timestamp ON klines(timestamp DESC);

-- ============== 回测相关 ==============
CREATE TABLE backtests (
    id TEXT PRIMARY KEY,
    strategy_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    parameters TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    initial_balance REAL NOT NULL,
    commission_rate REAL DEFAULT 0.001,
    slippage REAL DEFAULT 0,
    total_return REAL,
    sharpe_ratio REAL,
    max_drawdown REAL,
    win_rate REAL,
    total_trades INTEGER,
    winning_trades INTEGER,
    losing_trades INTEGER,
    status TEXT DEFAULT 'pending',
    error_message TEXT,
    trades_data TEXT,
    equity_curve TEXT,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_backtests_strategy_id ON backtests(strategy_id);
CREATE INDEX idx_backtests_user_id ON backtests(user_id);
CREATE INDEX idx_backtests_created_at ON backtests(created_at DESC);

-- ============== 风控相关 ==============
CREATE TABLE risk_rules (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    rule_type TEXT NOT NULL,
    config TEXT NOT NULL,
    threshold_value REAL NOT NULL,
    action TEXT NOT NULL,
    apply_to TEXT,
    target_id TEXT,
    is_enabled INTEGER DEFAULT 1,
    trigger_count INTEGER DEFAULT 0,
    last_triggered_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE risk_alerts (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    strategy_instance_id TEXT,
    symbol TEXT,
    current_value REAL NOT NULL,
    threshold_value REAL NOT NULL,
    status TEXT DEFAULT 'active',
    handled_by TEXT,
    handled_at INTEGER,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (rule_id) REFERENCES risk_rules(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_risk_rules_user_id ON risk_rules(user_id);
CREATE INDEX idx_risk_rules_is_enabled ON risk_rules(is_enabled);
CREATE INDEX idx_risk_alerts_user_id ON risk_alerts(user_id);
CREATE INDEX idx_risk_alerts_status ON risk_alerts(status);

-- ============== 审计日志 ==============
CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    before_data TEXT,
    after_data TEXT,
    result TEXT NOT NULL,
    error_message TEXT,
    ip_address TEXT,
    user_agent TEXT,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
```

#### 验收标准
- [x] SQL 文件语法正确
- [x] 表结构和索引符合设计文档
- [x] 外键约束正确设置

#### 产物
- `src-tauri/migrations/001_initial_schema.sql`

---

### P1-07: 实现 sqlx 数据库连接 ✅

**估时**: 2h | **优先级**: P0 | **依赖**: P1-06

#### 实施步骤

1. 创建 `src-tauri/src/infrastructure/database.rs`：
```rust
use anyhow::Result;
use sqlx::{sqlite::SqlitePool, Sqlite, Pool};
use std::path::PathBuf;
use tauri::AppHandle;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    /// 创建数据库连接
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // 构建数据库 URL
        let db_url = format!("sqlite:{}", db_path.display());

        // 创建连接池
        let pool = SqlitePool::connect(&db_url).await?;

        // 启用外键约束
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    /// 运行数据库迁移
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    /// 从 AppHandle 获取数据库实例
    pub fn from_app(handle: &AppHandle) -> Result<&Self> {
        handle.state::<Self>()
            .try_get()
            .ok_or_else(|| anyhow::anyhow!("Database not found in app state"))
    }
}

// 全局类型别名
pub type DbPool = SqlitePool;
```

2. 创建 `src-tauri/src/infrastructure/mod.rs`：
```rust
pub mod database;
pub mod crypto;
pub mod audit;

pub use database::Database;
```

3. 更新 `src-tauri/src/main.rs`：
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ai_lot::infrastructure::Database;
use std::path::PathBuf;
use tauri::Manager;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            // 获取数据目录
            let data_dir = app.path().app_data_dir()
                .expect("Failed to get data dir");
            let db_path = data_dir.join("ai-lot.db");

            // 创建数据库连接
            tauri::async_runtime::block_on(async {
                let db = Database::new(db_path).await
                    .expect("Failed to create database");

                // 运行迁移
                db.run_migrations().await
                    .expect("Failed to run migrations");

                // 注册到 Tauri 状态
                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::user::login,
            commands::user::get_current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 验收标准
- [x] 数据库文件正确创建在数据目录
- [x] 迁移自动执行，所有表创建成功
- [x] 可通过 Database::from_app 获取数据库实例

#### 产物
- `src-tauri/src/infrastructure/database.rs`
- `src-tauri/src/infrastructure/mod.rs`
- `src-tauri/src/main.rs`

---

### P1-08: 实现数据模型 ✅

**估时**: 2h | **优先级**: P0 | **依赖**: P1-07

#### 实施步骤

1. 创建 `src-tauri/src/models/user.rs`：
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub role_id: String,
    pub status: String,
    pub salt: String,
    pub failed_attempts: i32,
    pub locked_until: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRole {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub role_id: String,
    pub role_name: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}
```

2. 创建 `src-tauri/src/models/exchange.rs`：
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExchangeName {
    Binance,
    OKX,
    Bybit,
}

impl ExchangeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Binance => "binance",
            Self::OKX => "okx",
            Self::Bybit => "bybit",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "binance" => Some(Self::Binance),
            "okx" => Some(Self::OKX),
            "bybit" => Some(Self::Bybit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExchangeConfig {
    pub id: String,
    pub user_id: String,
    pub exchange_name: String,
    pub display_name: String,
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}
```

3. 创建 `src-tauri/src/models/mod.rs`：
```rust
pub mod user;
pub mod exchange;

pub use user::{User, Role, UserWithRole};
pub use exchange::{ExchangeConfig, ExchangeName};
```

#### 验收标准
- [x] 所有模型定义与数据库表结构一致
- [x] 模型可正确序列化/反序列化 JSON

#### 产物
- `src-tauri/src/models/user.rs`
- `src-tauri/src/models/exchange.rs`
- `src-tauri/src/models/mod.rs`

---

### P1-09: 实现 Repository Trait ✅

**估时**: 1h | **优先级**: P0 | **依赖**: P1-08

#### 实施步骤

1. 创建 `src-tauri/src/repository/mod.rs`：
```rust
use async_trait::async_trait;
use anyhow::Result;

/// 通用 Repository Trait
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn insert(&self, entity: &T) -> Result<()>;
    async fn update(&self, entity: &T) -> Result<()>;
    async fn delete(&self, id: ID) -> Result<()>;
}

pub mod user_repo;
pub use user_repo::UserRepository;
```

2. 创建 `src-tauri/src/repository/user_repo.rs`：
```rust
use super::Repository;
use crate::models::{User, Role, UserWithRole};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 通过用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserWithRole>> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT u.id, u.username, u.display_name, u.role_id, r.name as role_name,
                   u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            WHERE u.username = ?
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    /// 通过 ID 查找用户
    pub async fn find_by_id_with_role(&self, id: &str) -> Result<Option<UserWithRole>> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT u.id, u.username, u.display_name, u.role_id, r.name as role_name,
                   u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            WHERE u.id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}

#[async_trait]
impl Repository<User, String> for UserRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    async fn insert(&self, entity: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, display_name, role_id, status,
                              salt, failed_attempts, locked_until, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entity.id)
        .bind(&entity.username)
        .bind(&entity.password_hash)
        .bind(&entity.display_name)
        .bind(&entity.role_id)
        .bind(&entity.status)
        .bind(&entity.salt)
        .bind(entity.failed_attempts)
        .bind(entity.locked_until)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &User) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET username = ?, display_name = ?, role_id = ?, status = ?,
                failed_attempts = ?, locked_until = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&entity.username)
        .bind(&entity.display_name)
        .bind(&entity.role_id)
        .bind(&entity.status)
        .bind(entity.failed_attempts)
        .bind(entity.locked_until)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
```

#### 验收标准
- [x] Repository Trait 定义完整
- [x] UserRepository 实现所有方法
- [x] 可通过用户名查询用户

#### 产物
- `src-tauri/src/repository/mod.rs`
- `src-tauri/src/repository/user_repo.rs`

---

### P1-10: 实现 UserRepository 集成

**估时**: 1h | **优先级**: P0 | **依赖**: P1-09

#### 实施步骤

1. 更新 `src-tauri/src/lib.rs`：
```rust
mod models;
mod repository;
mod infrastructure;

pub use infrastructure::Database;
```

2. 添加 Repository 工厂方法到 `Database`：
```rust
// 在 infrastructure/database.rs 中添加
impl Database {
    // ... 其他方法

    /// 获取 User Repository
    pub fn user_repo(&self) -> repository::UserRepository {
        repository::UserRepository::new(self.pool.clone())
    }
}
```

#### 验收标准
- [x] 可通过 Database 实例获取 UserRepository
- [x] 编译无错误

#### 产物
- 更新的 `src-tauri/src/infrastructure/database.rs`

---

### P1-11: 实现密码哈希

**估时**: 1h | **优先级**: P0 | **依赖**: P1-08

#### 实施步骤

1. 创建 `src-tauri/src/infrastructure/crypto.rs`：
```rust
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};

pub struct CryptoService;

impl CryptoService {
    /// 哈希密码
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    /// 验证密码
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    /// 生成随机盐
    pub fn generate_salt() -> String {
        SaltString::generate(&mut OsRng).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }
}
```

2. 更新 `src-tauri/src/infrastructure/mod.rs`：
```rust
pub mod database;
pub mod crypto;
pub mod audit;

pub use database::Database;
pub use crypto::CryptoService;
```

#### 验收标准
- [x] 密码哈希功能正常
- [x] 密码验证功能正常
- [x] 单元测试通过

#### 产物
- `src-tauri/src/infrastructure/crypto.rs`

---

### P1-12: 实现用户登录 Tauri 命令

**估时**: 1h | **优先级**: P0 | **依赖**: P1-11

#### 实施步骤

1. 创建 `src-tauri/src/commands/mod.rs`：
```rust
pub mod user;

// 使用 pub use 导出所有命令模块
pub use user::*;
```

2. 创建 `src-tauri/src/commands/user.rs`：
```rust
use crate::infrastructure::{CryptoService, Database};
use crate::models::UserWithRole;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserWithRole,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// 用户登录
#[tauri::command]
pub async fn login(
    db: State<'_, Database>,
    request: LoginRequest,
) -> Result<LoginResponse, String> {
    // 查找用户
    let user_repo = db.user_repo();
    let user = user_repo
        .find_by_username(&request.username)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "用户不存在".to_string())?;

    // 验证密码
    let is_valid = CryptoService::verify_password(&request.password, &user.password_hash)
        .map_err(|e| e.to_string())?;

    if !is_valid {
        return Err("密码错误".to_string());
    }

    // 检查用户状态
    if user.status != "active" {
        return Err(format!("用户状态异常: {}", user.status));
    }

    // 生成简单 token (生产环境应使用 JWT)
    let token = format!("{}:{}", user.id, chrono::Utc::now().timestamp());

    Ok(LoginResponse { user, token })
}

/// 获取当前登录用户
#[tauri::command]
pub async fn get_current_user(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Option<UserWithRole>, String> {
    let user_repo = db.user_repo();
    user_repo
        .find_by_id_with_role(&user_id)
        .await
        .map_err(|e| e.to_string())
}
```

3. 更新 `src-tauri/src/main.rs`：
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ai_lot::infrastructure::Database;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir()
                .expect("Failed to get data dir");
            let db_path = data_dir.join("ai-lot.db");

            tauri::async_runtime::block_on(async {
                let db = Database::new(db_path).await
                    .expect("Failed to create database");
                db.run_migrations().await
                    .expect("Failed to run migrations");
                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::get_current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 验收标准
- [x] 可通过正确的用户名密码登录
- [x] 错误的用户名密码返回错误
- [x] 返回的用户信息包含角色名称

#### 产物
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/user.rs`

---

### P1-13: 实现权限检查中间件

**估时**: 1h | **优先级**: P1 | **依赖**: P1-12

#### 实施步骤

1. 创建 `src-tauri/src/core/auth/mod.rs`：
```rust
use crate::models::UserWithRole;
use anyhow::Result;

/// 权限定义
pub const PERM_USER_READ: &str = "user:read";
pub const PERM_USER_WRITE: &str = "user:write";
pub const PERM_STRATEGY_READ: &str = "strategy:read";
pub const PERM_STRATEGY_WRITE: &str = "strategy:write";
pub const PERM_STRATEGY_DELETE: &str = "strategy:delete";
pub const PERM_BACKTEST_EXECUTE: &str = "backtest:execute";
pub const PERM_TRADE_EXECUTE: &str = "trade:execute";
pub const PERM_MARKET_READ: &str = "market:read";
pub const PERM_POSITION_READ: &str = "position:read";
pub const PERM_ORDER_READ: &str = "order:read";
pub const PERM_ORDER_WRITE: &str = "order:write";
pub const PERM_RISK_READ: &str = "risk:read";
pub const PERM_RISK_WRITE: &str = "risk:write";
pub const PERM_AUDIT_READ: &str = "audit:read";
pub const PERM_ALL: &str = "*";

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

    #[test]
    fn test_admin_has_all_permissions() {
        let user = UserWithRole {
            id: "1".to_string(),
            username: "admin".to_string(),
            display_name: None,
            role_id: "role_admin".to_string(),
            role_name: "管理员".to_string(),
            status: "active".to_string(),
            created_at: 0,
            updated_at: 0,
        };

        assert!(AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
    }

    #[test]
    fn test_developer_limited_permissions() {
        let user = UserWithRole {
            id: "1".to_string(),
            username: "dev".to_string(),
            display_name: None,
            role_id: "role_developer".to_string(),
            role_name: "策略开发者".to_string(),
            status: "active".to_string(),
            created_at: 0,
            updated_at: 0,
        };

        assert!(AuthService::has_permission(&user, PERM_STRATEGY_WRITE));
        assert!(!AuthService::has_permission(&user, PERM_TRADE_EXECUTE));
    }
}
```

#### 验收标准
- [x] 管理员拥有全部权限
- [x] 其他角色权限符合设计
- [x] 单元测试通过

#### 产物
- `src-tauri/src/core/auth/mod.rs`

---

### P1-14: 实现操作审计日志

**估时**: 1h | **优先级**: P1 | **依赖**: P1-12

#### 实施步骤

1. 创建 `src-tauri/src/infrastructure/audit.rs`：
```rust
use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub struct AuditLogger {
    pool: Pool<Sqlite>,
}

impl AuditLogger {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 记录操作日志
    pub async fn log(
        &self,
        entry: AuditLogEntry,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, user_id, username, operation_type, resource_type, resource_id,
                before_data, after_data, result, error_message, ip_address, user_agent, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entry.id)
        .bind(&entry.user_id)
        .bind(&entry.username)
        .bind(&entry.operation_type)
        .bind(&entry.resource_type)
        .bind(&entry.resource_id)
        .bind(&entry.before_data)
        .bind(&entry.after_data)
        .bind(&entry.result)
        .bind(&entry.error_message)
        .bind(&entry.ip_address)
        .bind(&entry.user_agent)
        .bind(entry.timestamp)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 记录成功操作
    pub async fn log_success(
        &self,
        user_id: &str,
        username: &str,
        operation_type: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        before_data: Option<serde_json::Value>,
        after_data: Option<serde_json::Value>,
    ) -> Result<()> {
        self.log(AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            operation_type: operation_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(|s| s.to_string()),
            before_data: before_data.map(|v| v.to_string()),
            after_data: after_data.map(|v| v.to_string()),
            result: "success".to_string(),
            error_message: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now().timestamp(),
        }).await
    }

    /// 记录失败操作
    pub async fn log_failure(
        &self,
        user_id: &str,
        username: &str,
        operation_type: &str,
        resource_type: &str,
        error_message: &str,
    ) -> Result<()> {
        self.log(AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            operation_type: operation_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: None,
            before_data: None,
            after_data: None,
            result: "failure".to_string(),
            error_message: Some(error_message.to_string()),
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now().timestamp(),
        }).await
    }
}

pub struct AuditLogEntry {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub operation_type: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub before_data: Option<String>,
    pub after_data: Option<String>,
    pub result: String,
    pub error_message: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: i64,
}
```

2. 添加到 `Database`：
```rust
// 在 infrastructure/database.rs 中添加
impl Database {
    // ... 其他方法

    /// 获取审计日志记录器
    pub fn audit_logger(&self) -> audit::AuditLogger {
        audit::AuditLogger::new(self.pool.clone())
    }
}
```

#### 验收标准
- [x] 可正确记录审计日志到数据库
- [x] 日志包含所有必需字段

#### 产物
- `src-tauri/src/infrastructure/audit.rs`

---

### P1-15: 配置 Vue Router

**估时**: 0.5h | **优先级**: P0 | **依赖**: P1-04

#### 实施步骤

1. 创建 `src/router/index.ts`：
```typescript
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
const BacktestView = () => import('@/views/Backtest/BacktestView.vue');
const TradeConsole = () => import('@/views/Trade/TradeConsole.vue');
const RiskMonitor = () => import('@/views/Risk/RiskMonitor.vue');
const Settings = () => import('@/views/Settings/Settings.vue');

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
```

#### 验收标准
- [x] 路由配置正确
- [x] 未登录自动跳转到登录页
- [x] 登录后不能访问登录页

#### 产物
- `src/router/index.ts`

---

### P1-16: 配置 Pinia Store

**估时**: 0.5h | **优先级**: P0 | **依赖**: P1-04

#### 实施步骤

1. 创建 `src/store/index.ts`：
```typescript
import { createPinia } from 'pinia';

const pinia = createPinia();

export default pinia;

// 导出所有 store
export * from './modules/user';
export * from './modules/market';
export * from './modules/strategy';
export * from './modules/trade';
```

2. 创建 `src/store/modules/user.ts`：
```typescript
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { User, LoginRequest, LoginResponse } from '@/types';
import * as api from '@/api/tauri';

export const useUserStore = defineStore('user', () => {
  // State
  const user = ref<User | null>(null);
  const token = ref<string | null>(null);
  const loading = ref(false);

  // Getters
  const isLoggedIn = computed(() => !!user.value && !!token.value);
  const username = computed(() => user.value?.username ?? '');
  const roleName = computed(() => user.value?.roleName ?? '');
  const hasPermission = computed(() => {
    return (permission: string) => {
      if (!user.value) return false;
      // 简单实现，实际应该在服务端验证
      return user.value.roleName === '管理员';
    };
  });

  // Actions
  async function login(request: LoginRequest) {
    loading.value = true;
    try {
      const response = await api.invoke<LoginResponse>('login', request);
      user.value = response.user;
      token.value = response.token;
      localStorage.setItem('token', response.token);
      return true;
    } catch (error) {
      console.error('Login failed:', error);
      return false;
    } finally {
      loading.value = false;
    }
  }

  async function logout() {
    user.value = null;
    token.value = null;
    localStorage.removeItem('token');
  }

  async function fetchCurrentUser() {
    if (!token.value) return;

    try {
      const userId = user.value?.id;
      if (!userId) return;

      const currentUser = await api.invoke<User>('get_current_user', { userId });
      user.value = currentUser;
    } catch (error) {
      console.error('Fetch user failed:', error);
      logout();
    }
  }

  function initFromStorage() {
    const storedToken = localStorage.getItem('token');
    if (storedToken) {
      token.value = storedToken;
      // 解析用户ID从token (简化实现)
      const userId = storedToken.split(':')[0];
      if (userId) {
        fetchCurrentUser();
      }
    }
  }

  return {
    user,
    token,
    loading,
    isLoggedIn,
    username,
    roleName,
    hasPermission,
    login,
    logout,
    fetchCurrentUser,
    initFromStorage,
  };
});
```

3. 创建 `src/store/modules/market.ts`（占位）：
```typescript
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Ticker, Kline } from '@/types';

export const useMarketStore = defineStore('market', () => {
  const currentSymbol = ref('BTCUSDT');
  const currentTimeframe = ref('1h');
  const tickers = ref<Map<string, Ticker>>(new Map());
  const klines = ref<Map<string, Kline[]>>(new Map());
  const wsConnected = ref(false);

  function setCurrentSymbol(symbol: string) {
    currentSymbol.value = symbol;
  }

  function setCurrentTimeframe(timeframe: string) {
    currentTimeframe.value = timeframe;
  }

  return {
    currentSymbol,
    currentTimeframe,
    tickers,
    klines,
    wsConnected,
    setCurrentSymbol,
    setCurrentTimeframe,
  };
});
```

4. 创建 `src/store/modules/strategy.ts`（占位）：
```typescript
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Strategy, StrategyInstance } from '@/types';

export const useStrategyStore = defineStore('strategy', () => {
  const strategies = ref<Strategy[]>([]);
  const currentStrategy = ref<Strategy | null>(null);
  const runningInstances = ref<StrategyInstance[]>([]);

  return {
    strategies,
    currentStrategy,
    runningInstances,
  };
});
```

5. 创建 `src/store/modules/trade.ts`（占位）：
```typescript
import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Order, Position } from '@/types';

export const useTradeStore = defineStore('trade', () => {
  const positions = ref<Position[]>([]);
  const activeOrders = ref<Order[]>([]);
  const orderHistory = ref<Order[]>([]);

  return {
    positions,
    activeOrders,
    orderHistory,
  };
});
```

#### 验收标准
- [x] Pinia store 配置完成
- [x] 用户登录状态可正常管理
- [x] 登录后刷新页面可恢复状态

#### 产物
- `src/store/index.ts`
- `src/store/modules/user.ts`
- `src/store/modules/market.ts`
- `src/store/modules/strategy.ts`
- `src/store/modules/trade.ts`

---

### P1-17: 实现 Tauri API 封装

**估时**: 1h | **优先级**: P0 | **依赖**: P1-03

#### 实施步骤

1. 创建 `src/api/tauri.ts`：
```typescript
import { invoke } from '@tauri-apps/api/core';
import type {
  LoginRequest,
  LoginResponse,
  User,
  ApiResponse,
  Ticker,
  Kline,
  Strategy,
  BacktestParams,
  BacktestReport,
  Order,
  Position,
} from '@/types';

/**
 * 调用 Tauri 命令的通用方法
 */
export async function invoke<T = any>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  try {
    const response = await invoke<any>('cmd_' + cmd, args ? { payload: args } : {});

    if (response.success === false) {
      throw new Error(response.error || 'Operation failed');
    }

    return response.data as T;
  } catch (error) {
    // 处理旧版本命令格式（直接返回数据）
    return invoke<any>(cmd, args) as Promise<T>;
  }
}

/**
 * 直接调用 Tauri 命令（不包装 ApiResponse）
 */
export async function invokeRaw<T = any>(
  cmd: string,
  args?: Record<string, any>
): Promise<T> {
  return invoke<T>(cmd, args);
}

// ============== 用户 API ==============
export const userApi = {
  /**
   * 用户登录
   */
  login: (request: LoginRequest) =>
    invokeRaw<LoginResponse>('login', request),

  /**
   * 获取当前用户
   */
  getCurrentUser: (userId: string) =>
    invokeRaw<User>('get_current_user', { userId }),

  /**
   * 用户登出
   */
  logout: () =>
    invoke('user_logout'),
};

// ============== 行情 API ==============
export const marketApi = {
  /**
   * 订阅行情
   */
  subscribeTicker: (symbols: string[]) =>
    invoke('market_subscribe_ticker', { symbols }),

  /**
   * 获取 K 线数据
   */
  getKlines: (symbol: string, interval: string, limit: number) =>
    invoke<Kline[]>('market_get_klines', { symbol, interval, limit }),

  /**
   * 获取交易对列表
   */
  getSymbols: () =>
    invoke<string[]>('market_get_symbols'),
};

// ============== 策略 API ==============
export const strategyApi = {
  /**
   * 获取策略列表
   */
  list: () =>
    invoke<Strategy[]>('strategy_list'),

  /**
   * 获取策略详情
   */
  get: (id: string) =>
    invoke<Strategy>('strategy_get', { id }),

  /**
   * 保存策略
   */
  save: (strategy: Strategy) =>
    invoke<string>('strategy_save', { strategy }),

  /**
   * 删除策略
   */
  delete: (id: string) =>
    invoke('strategy_delete', { id }),
};

// ============== 回测 API ==============
export const backtestApi = {
  /**
   * 运行回测
   */
  run: (params: BacktestParams) =>
    invoke<string>('backtest_run', { params }),

  /**
   * 获取回测报告
   */
  getReport: (id: string) =>
    invoke<BacktestReport>('backtest_get_report', { id }),
};

// ============== 交易 API ==============
export const tradeApi = {
  /**
   * 下单
   */
  placeOrder: (order: any) =>
    invoke<Order>('trade_place_order', { order }),

  /**
   * 撤单
   */
  cancelOrder: (orderId: string) =>
    invoke('trade_cancel_order', { orderId }),

  /**
   * 获取持仓
   */
  getPositions: () =>
    invoke<Position[]>('trade_get_positions'),

  /**
   * 获取订单
   */
  getOrders: () =>
    invoke<Order[]>('trade_get_orders'),
};
```

#### 验收标准
- [x] API 封装方法定义完整
- [x] TypeScript 类型正确

#### 产物
- `src/api/tauri.ts`

---

### P1-18: 实现 Login 页面

**估时**: 2h | **优先级**: P0 | **依赖**: P1-17

#### 实施步骤

1. 创建 `src/views/auth/Login.vue`：
```vue
<template>
  <div class="login-container">
    <el-card class="login-card">
      <template #header>
        <div class="login-header">
          <h2>AI-LOT 量化交易</h2>
          <p>登录到您的账户</p>
        </div>
      </template>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="0"
        size="large"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="username">
          <el-input
            v-model="form.username"
            placeholder="用户名"
            prefix-icon="User"
            clearable
          />
        </el-form-item>

        <el-form-item prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="密码"
            prefix-icon="Lock"
            show-password
            clearable
            @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item>
          <el-checkbox v-model="form.rememberMe">记住我</el-checkbox>
        </el-form-item>

        <el-form-item>
          <el-button
            type="primary"
            :loading="loading"
            style="width: 100%"
            @click="handleLogin"
          >
            登录
          </el-button>
        </el-form-item>
      </el-form>

      <div class="login-footer">
        <el-text type="info" size="small">
          默认账户: admin / admin123
        </el-text>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { ElMessage, type FormInstance, type FormRules } from 'element-plus';
import { useUserStore } from '@/store';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const formRef = ref<FormInstance>();
const loading = ref(false);

const form = reactive({
  username: '',
  password: '',
  rememberMe: false,
});

const rules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 50, message: '密码长度在 6 到 50 个字符', trigger: 'blur' },
  ],
};

async function handleLogin() {
  if (!formRef.value) return;

  const valid = await formRef.value.validate().catch(() => false);
  if (!valid) return;

  loading.value = true;

  try {
    const success = await userStore.login({
      username: form.username,
      password: form.password,
    });

    if (success) {
      ElMessage.success('登录成功');

      // 跳转到原来想访问的页面或首页
      const redirect = (route.query.redirect as string) || '/dashboard';
      router.push(redirect);
    } else {
      ElMessage.error('登录失败，请检查用户名和密码');
    }
  } catch (error) {
    ElMessage.error('登录失败：' + (error as Error).message);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.login-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.login-header {
  text-align: center;
}

.login-header h2 {
  margin: 0 0 8px 0;
  font-size: 24px;
  color: #303133;
}

.login-header p {
  margin: 0;
  font-size: 14px;
  color: #909399;
}

.login-footer {
  text-align: center;
  margin-top: 16px;
}
</style>
```

#### 验收标准
- [x] 登录表单正常显示
- [x] 表单验证正常工作
- [x] 登录成功后跳转到 dashboard
- [x] 登录失败显示错误提示

#### 产物
- `src/views/auth/Login.vue`

---

### P1-19: 实现 Dashboard 布局

**估时**: 2h | **优先级**: P0 | **依赖**: P1-18

#### 实施步骤

1. 创建 `src/views/Layout.vue`：
```vue
<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapse ? '64px' : '200px'" class="app-aside">
      <div class="logo">
        <span v-if="!isCollapse">AI-LOT</span>
        <span v-else>AI</span>
      </div>

      <el-menu
        :default-active="activeMenu"
        :collapse="isCollapse"
        router
        class="app-menu"
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>仪表盘</template>
        </el-menu-item>

        <el-menu-item index="/market">
          <el-icon><TrendCharts /></el-icon>
          <template #title>行情</template>
        </el-menu-item>

        <el-sub-menu index="strategy">
          <template #title>
            <el-icon><Document /></el-icon>
            <span>策略</span>
          </template>
          <el-menu-item index="/strategy">策略列表</el-menu-item>
          <el-menu-item index="/strategy/editor">新建策略</el-menu-item>
        </el-sub-menu>

        <el-menu-item index="/backtest">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>回测</template>
        </el-menu-item>

        <el-menu-item index="/trade">
          <el-icon><ShoppingCart /></el-icon>
          <template #title>交易</template>
        </el-menu-item>

        <el-menu-item index="/risk">
          <el-icon><Warning /></el-icon>
          <template #title>风控</template>
        </el-menu-item>

        <el-menu-item index="/settings">
          <el-icon><Setting /></el-icon>
          <template #title>设置</template>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <!-- 主内容区 -->
    <el-container>
      <!-- 顶栏 -->
      <el-header class="app-header">
        <div class="header-left">
          <el-icon
            class="collapse-btn"
            @click="isCollapse = !isCollapse"
          >
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>

          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">
              首页
            </el-breadcrumb-item>
            <el-breadcrumb-item v-if="currentRoute.meta.title">
              {{ currentRoute.meta.title }}
            </el-breadcrumb-item>
          </el-breadcrumb>
        </div>

        <div class="header-right">
          <!-- 用户信息 -->
          <el-dropdown @command="handleCommand">
            <span class="user-info">
              <el-avatar :size="32" :icon="UserFilled" />
              <span class="username">{{ userStore.username }}</span>
              <span class="role">({{ userStore.roleName }})</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="profile">
                  <el-icon><User /></el-icon>
                  个人资料
                </el-dropdown-item>
                <el-dropdown-item command="settings">
                  <el-icon><Setting /></el-icon>
                  设置
                </el-dropdown-item>
                <el-dropdown-item divided command="logout">
                  <el-icon><SwitchButton /></el-icon>
                  退出登录
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <!-- 主内容 -->
      <el-main class="app-main">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import {
  Odometer,
  TrendCharts,
  Document,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Fold,
  Expand,
  UserFilled,
  User,
  SwitchButton,
} from '@element-plus/icons-vue';
import { useUserStore } from '@/store';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();

const isCollapse = ref(false);
const currentRoute = computed(() => route);

const activeMenu = computed(() => {
  const path = route.path;
  // 精确匹配策略编辑器路径
  if (path.startsWith('/strategy/editor')) {
    return '/strategy/editor';
  }
  return path;
});

async function handleCommand(command: string) {
  switch (command) {
    case 'profile':
      ElMessage.info('个人资料功能待实现');
      break;
    case 'settings':
      router.push('/settings');
      break;
    case 'logout':
      try {
        await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning',
        });

        await userStore.logout();
        ElMessage.success('已退出登录');
        router.push('/login');
      } catch {
        // 取消退出
      }
      break;
  }
}
</script>

<style scoped>
.app-layout {
  height: 100vh;
}

.app-aside {
  background-color: #304156;
  transition: width 0.3s;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
  color: #fff;
  background-color: #263445;
}

.app-menu {
  border-right: none;
  background-color: #304156;
}

:deep(.el-menu-item),
:deep(.el-sub-menu__title) {
  color: #bfcbd9;
}

:deep(.el-menu-item:hover),
:deep(.el-sub-menu__title:hover) {
  background-color: #263445 !important;
}

:deep(.el-menu-item.is-active) {
  color: #409eff !important;
  background-color: #263445 !important;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #e4e7ed;
  background-color: #fff;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.collapse-btn {
  font-size: 20px;
  cursor: pointer;
  color: #909399;
}

.collapse-btn:hover {
  color: #409eff;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.username {
  font-size: 14px;
  color: #303133;
}

.role {
  font-size: 12px;
  color: #909399;
}

.app-main {
  background-color: #f5f7fa;
  padding: 20px;
}
</style>
```

2. 创建 `src/views/Dashboard.vue`：
```vue
<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <!-- 统计卡片 -->
      <el-col :span="6" v-for="stat in stats" :key="stat.title">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" :style="{ backgroundColor: stat.color }">
              <el-icon :size="32" :color="stat.iconColor">
                <component :is="stat.icon" />
              </el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stat.value }}</div>
              <div class="stat-title">{{ stat.title }}</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px;">
      <!-- 欢迎信息 -->
      <el-col :span="24">
        <el-card>
          <template #header>
            <span>欢迎使用 AI-LOT 量化交易系统</span>
          </template>
          <p>当前系统状态：<el-tag type="success">运行中</el-tag></p>
          <p>请从左侧菜单选择功能模块开始使用。</p>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  Wallet,
  TrendCharts,
  Document,
  DataLine,
} from '@element-plus/icons-vue';

const stats = ref([
  {
    title: '总资产',
    value: '--',
    icon: Wallet,
    color: '#ecf5ff',
    iconColor: '#409eff',
  },
  {
    title: '运行策略',
    value: '0',
    icon: TrendCharts,
    color: '#f0f9ff',
    iconColor: '#67c23a',
  },
  {
    title: '策略总数',
    value: '0',
    icon: Document,
    color: '#fef0f0',
    iconColor: '#f56c6c',
  },
  {
    title: '今日盈亏',
    value: '--',
    icon: DataLine,
    color: '#fdf6ec',
    iconColor: '#e6a23c',
  },
]);
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.stat-card {
  cursor: pointer;
  transition: all 0.3s;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #303133;
}

.stat-title {
  font-size: 14px;
  color: #909399;
  margin-top: 4px;
}
</style>
```

#### 验收标准
- [x] 布局正确显示
- [x] 侧边栏菜单可点击跳转
- [x] 折叠按钮正常工作
- [x] 用户下拉菜单可用
- [x] 退出登录功能正常

#### 产物
- `src/views/Layout.vue`
- `src/views/Dashboard.vue`

---

### P1-20: 实现用户状态管理

**估时**: 1h | **优先级**: P0 | **依赖**: P1-19

#### 实施步骤

1. 更新 `src/main.ts`：
```typescript
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';

import App from './App.vue';
import router from './router';
import pinia from './store';

const app = createApp(App);

// 注册 Element Plus 图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

app.use(pinia);
app.use(router);
app.use(ElementPlus);

// 初始化用户状态
import { useUserStore } from './store';
const userStore = useUserStore();
userStore.initFromStorage();

app.mount('#app');
```

2. 更新 `src/App.vue`：
```vue
<template>
  <router-view />
</template>

<script setup lang="ts">
// 主应用组件
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}
</style>
```

#### 验收标准
- [x] 应用启动后可正常显示登录页
- [x] 登录后进入 Dashboard
- [x] Element Plus 组件和图标正常显示

#### 产物
- 更新的 `src/main.ts`
- 更新的 `src/App.vue`

---

## Phase 1 验收标准

### 功能验收
- [ ] 用户可正常登录（admin/admin123）
- [ ] 登录后显示 Dashboard 布局
- [ ] 侧边栏菜单可正常导航
- [ ] 用户信息正确显示在顶栏
- [ ] 退出登录功能正常

### 技术验收
- [ ] 数据库文件正确创建
- [ ] 所有表结构正确
- [ ] TypeScript 编译无错误
- [ ] ESLint 检查通过

---

## 下一步

完成 Phase 1 后，可进入 **Phase 2: 行情数据模块** 开发。
