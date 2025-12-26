# Phase 3: 策略开发模块 - 详细任务规范

## 目标

实现策略编辑器和回测引擎。

---

## 任务概览

| ID | 任务 | 估时 |
|----|------|------|
| P3-01 | 集成 Monaco Editor | 2h |
| P3-02 | 实现策略参数编辑表单 | 2h |
| P3-03 | 实现 StrategyEditor 页面 | 2h |
| P3-04 | 实现策略保存/加载 | 1h |
| P3-05 | 实现策略脚本执行 | 3h |
| P3-06 | 实现策略上下文 API | 2h |
| P3-07 | 实现 StrategyEngine | 3h |
| P3-08 | 实现策略实例管理 | 1h |
| P3-09 | 实现历史数据加载 | 1h |
| P3-10 | 实现模拟交易所 | 2h |
| P3-11 | 实现回测引擎 | 3h |
| P3-12 | 实现性能指标计算 | 2h |
| P3-13 | 实现回测报告生成 | 2h |
| P3-14 | 实现回测配置表单 | 1h |
| P3-15 | 实现回测进度显示 | 1h |
| P3-16 | 实现回测报告页面 | 2h |
| P3-17 | 实现策略列表页面 | 1h |

---

## 详细任务规范

### P3-01: 集成 Monaco Editor

**依赖**: 无

#### 实施步骤

1. 安装依赖：
```bash
bun add monaco-editor @monaco-editor/loader
```

2. 创建 `src/components/MonacoEditor.vue`：
```vue
<template>
  <div class="monaco-editor-container">
    <div ref="editorRef" class="editor" style="height: 100%"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import * as monaco from 'monaco-editor';
import loader from '@monaco-editor/loader';

const props = defineProps<{
  modelValue: string;
  language?: string;
  readOnly?: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const editorRef = ref<HTMLElement>();
let editor: monaco.editor.IStandaloneCodeEditor | null = null;

onMounted(async () => {
  await loader.init();

  editor = monaco.editor.create(editorRef.value!, {
    value: props.modelValue,
    language: props.language || 'javascript',
    theme: 'vs-dark',
    readOnly: props.readOnly || false,
    automaticLayout: true,
    minimap: { enabled: true },
    fontSize: 14,
    scrollBeyondLastLine: false,
    roundedSelection: true,
    padding: { top: 16 },
  });

  editor.onDidChangeModelContent(() => {
    const value = editor?.getValue();
    if (value !== undefined) {
      emit('update:modelValue', value);
    }
  });
});

watch(() => props.modelValue, (newValue) => {
  if (editor && newValue !== editor.getValue()) {
    editor.setValue(newValue);
  }
});
</script>

<style scoped>
.monaco-editor-container {
  height: 100%;
  border: 1px solid #dcdfe6;
  border-radius: 4px;
  overflow: hidden;
}
</style>
```

#### 验收标准
- [x] Monaco Editor 正常显示
- [x] 代码高亮正常
- [x] 支持 JavaScript 语法
- [x] 内容双向绑定正常

#### 产物
- `src/components/MonacoEditor.vue` ✓ (263行)
- `package.json` ✓ (添加依赖)
- `docs/verification/P3-01-verification-report.md` ✓

**状态**: ✅ 已完成
- 安装了 monaco-editor@0.55.1 和 @monaco-editor/loader@1.7.0
- 实现了 MonacoEditor 组件（263行）
- 实现了完整的 props API（modelValue, language, theme, readOnly, fontSize, minimap, wordWrap, lineNumbers）
- 实现了事件（update:modelValue, ready）
- 实现了响应式布局（ResizeObserver）
- 实现了暴露方法（getEditor, getValue, setValue, focus, layout）
- 实现了代码高亮和智能提示
- 实现了 v-model 双向绑定
- 构建通过（26.48秒）
- [验证报告](../verification/P3-01-verification-report.md)

---

### P3-02: 实现策略参数编辑表单

**依赖**: P3-01

#### 实施步骤

创建 `src/components/ParameterEditor.vue`：
```vue
<template>
  <div class="parameter-editor">
    <el-table :data="parameters" border>
      <el-table-column prop="name" label="参数名" width="150" />

      <el-table-column prop="type" label="类型" width="100">
        <template #default="{ row }">
          <el-tag :type="getTypeColor(row.type)">{{ row.type }}</el-tag>
        </template>
      </el-table-column>

      <el-table-column prop="default" label="默认值" width="120" />

      <el-table-column label="当前值" width="150">
        <template #default="{ row }">
          <el-input-number
            v-if="row.type === 'number'"
            v-model="values[row.name]"
            :min="row.min"
            :max="row.max"
            :step="row.step"
            size="small"
          />
          <el-switch
            v-else-if="row.type === 'boolean'"
            v-model="values[row.name]"
          />
          <span v-else>--</span>
        </template>
      </el-table-column>

      <el-table-column prop="description" label="说明" show-overflow-tooltip />
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { StrategyParameter } from '@/types';

const props = defineProps<{
  parameters: StrategyParameter[];
  modelValue: Record<string, any>;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: Record<string, any>];
}>();

const values = ref<Record<string, any>>({ ...props.modelValue });

function getTypeColor(type: string) {
  const colors: Record<string, string> = {
    number: 'primary',
    string: 'success',
    boolean: 'warning',
    select: 'info',
  };
  return colors[type] || '';
}

watch(values, (newValues) => {
  emit('update:modelValue', newValues);
}, { deep: true });
</script>
```

#### 验收标准
- [x] 可编辑策略参数
- [x] 参数类型正确显示
- [x] 默认值显示正确
- [x] 编译无错误

#### 产物
- `src/components/ParameterEditor.vue` ✓ (340行)
- `docs/verification/P3-02-verification-report.md` ✓

**状态**: ✅ 已完成
- 实现了 ParameterEditor 组件（340行）
- 实现了 4 种参数类型支持（number, string, boolean, select）
- 实现了类型专用输入控件（InputNumber, Switch, Select, Input）
- 实现了 min/max/step 约束（number 类型）
- 实现了默认值显示
- 实现了单个和全部重置功能
- 实现了变更追踪（changedCount）
- 实现了底部统计摘要
- 实现了禁用状态支持
- 实现了 v-model 双向绑定
- 实现了类型颜色编码
- 构建通过（20.98秒）
- [验证报告](../verification/P3-02-verification-report.md)

---

### P3-03: 实现 StrategyEditor 页面

**依赖**: P3-01, P3-02

#### 验收标准
- [x] 策略编辑器页面正常显示
- [x] Monaco Editor 集成正常
- [x] Parameter Editor 集成正常
- [x] 参数管理功能正常
- [x] 保存/取消功能正常
- [x] 编译无错误

#### 产物
- `src/views/Strategy/StrategyEditor.vue` ✓ (645行)
- `docs/verification/P3-03-verification-report.md` ✓

**状态**: ✅ 已完成
- 实现了 StrategyEditor 页面（645行）
- 实现了策略基本信息表单（名称、分类、描述、标签）
- 实现了 Monaco Editor 代码编辑区集成
- 实现了代码格式化功能
- 实现了参数管理表格（添加/删除参数）
- 实现了参数值编辑器集成
- 实现了参数预览区域
- 实现了保存/取消处理
- 实现了未保存更改检测
- 实现了默认策略模板（onInit, onBar, onStop + MA交叉示例）
- 实现了路由参数支持（编辑/新建模式）
- 构建通过（23.96秒）
- [验证报告](../verification/P3-03-verification-report.md)

---

### P3-04: 实现策略保存/加载

**依赖**: P3-03

#### 验收标准
- [x] 策略可保存到数据库
- [x] 策略可从数据库加载
- [x] 策略列表可获取
- [x] 前端 API 正确调用
- [x] 后端编译无错误
- [x] 前端编译无错误

#### 产物
- `src-tauri/src/models/strategy.rs` ✓ (200行)
- `src-tauri/src/repository/strategy_repo.rs` ✓ (133行)
- `src-tauri/src/commands/strategy.rs` ✓ (144行)
- `src/api/tauri.ts` ✓ (更新 strategyApi)
- `src/views/Strategy/StrategyEditor.vue` ✓ (使用实际 API)
- `docs/verification/P3-04-verification-report.md` ✓

**状态**: ✅ 已完成
- 创建了 Strategy 相关数据模型（Strategy, StrategyDto, StrategyParameter, SaveStrategyRequest, StrategyListItem）
- 实现了 StrategyRepository（find_by_user, find_by_id_dto, save, delete, name_exists）
- 实现了 strategy Tauri commands（strategy_list, strategy_get, strategy_save, strategy_delete）
- 实现了前后端类型映射（JSON 序列化/反序列化）
- 更新了前端 strategyApi（list, get, save, delete）
- 更新了 StrategyEditor 使用实际 API 调用
- 实现了编辑模式加载策略数据
- 实现了保存时的用户 ID 验证
- 后端编译通过（53.50秒，仅警告）
- 前端构建通过（21.97秒）
- [验证报告](../verification/P3-04-verification-report.md)

---

### P3-05: 实现策略脚本执行

**依赖**: P3-04

#### 验收标准
- [x] JavaScript 引擎可执行用户代码
- [x] 支持策略生命周期回调 (onInit, onBar, onStop)
- [x] 可传递参数给策略
- [x] 可传递K线数据给策略
- [x] 可返回交易信号
- [x] 错误处理正确
- [x] 后端编译无错误
- [x] 单元测试通过

#### 产物
- `src-tauri/src/core/strategy/script.rs` ✓ (250行)
- `src-tauri/src/core/strategy/mod.rs` ✓ (3行)
- `src-tauri/src/commands/strategy_test.rs` ✓ (85行)
- `src-tauri/Cargo.toml` ✓ (添加 rquickjs 依赖)
- `docs/verification/P3-05-verification-report.md` ✓

**状态**: ✅ 已完成
- 集成了 rquickjs (QuickJS) JavaScript 引擎
- 实现了 ScriptExecutor (onInit, onBar, onStop 方法)
- 实现了策略测试命令 (strategy_test_execute, strategy_validate_code)
- 实现了参数传递 (context.parameters)
- 实现了 K线数据传递 (kline 对象)
- 实现了信号返回值处理 (JSON 序列化/反序列化)
- 实现了错误处理 (anyhow::Error 传播)
- 后端编译通过 (18.27秒，仅警告)
- 单元测试通过 (4/4 tests passed)
- [验证报告](../verification/P3-05-verification-report.md)

---

### P3-06: 实现策略上下文 API

**依赖**: P3-05

#### 验收标准
- [x] Storage API 功能完整 (set, get, has, keys, remove, clear)
- [x] Storage 可跨回调持久化
- [x] getHistory API 返回历史数据
- [x] getHistory 支持过滤 (symbol, timeframe, count)
- [x] 后端编译无错误
- [x] 单元测试通过

#### 产物
- `src-tauri/src/core/strategy/script.rs` ✓ (545行，+295行)
- `docs/verification/P3-06-verification-report.md` ✓

**状态**: ✅ 已完成
- 实现了 Storage API (set, get, has, keys, remove, clear)
- 实现了 getHistory API (支持 symbol/timeframe/count 过滤)
- 实现了存储数据注入 (prepare_storage_js 方法)
- 实现了历史数据序列化注入
- 实现了移动平均策略示例
- 后端编译通过 (36.62秒，仅警告)
- 单元测试通过 (7/7 tests passed)
- [验证报告](../verification/P3-06-verification-report.md)

---

### P3-07: 实现 StrategyEngine

**依赖**: P3-05, P3-06

#### 验收标准
- [x] 策略引擎可启动实例
- [x] 策略引擎可停止实例
- [x] 策略引擎可列出实例
- [x] 策略引擎可获取实例信息
- [x] 订阅市场事件
- [x] 执行策略回调 (onInit, onBar, onStop)
- [x] 处理策略信号
- [x] 后端编译无错误
- [x] 单元测试通过

#### 产物
- `src-tauri/src/core/strategy/engine.rs` ✓ (~350行)
  - StrategyEngine, RunningInstance, StrategyConfig
  - InstanceStatus, InstanceInfo
- `src-tauri/src/commands/strategy_engine.rs` ✓ (48行)
  - strategy_engine_start, strategy_engine_stop
  - strategy_engine_list, strategy_engine_get
- `src-tauri/src/infrastructure/database.rs` ✓
  - 添加 EventBus 和 StrategyEngine 单例
- `docs/verification/P3-07-verification-report.md` ✓

**状态**: ✅ 已完成
- 实现了 StrategyEngine (管理多个运行实例)
- 实现了 RunningInstance (策略生命周期管理)
- 实现了策略配置 (StrategyConfig)
- 实现了实例状态 (InstanceStatus: Starting/Running/Stopping/Stopped/Error)
- 实现了 Tauri 命令 (start, stop, list, get)
- 实现了市场事件订阅和过滤 (symbol, timeframe)
- 实现了历史数据缓冲 (每symbol 1000根K线)
- 实现了优雅关闭 (broadcast channel)
- 后端编译通过 (37.26秒，仅警告)
- 单元测试通过 (2/2 tests passed)
- [验证报告](../verification/P3-07-verification-report.md)

---

### P3-08: 实现策略实例管理

**依赖**: P3-07

#### 验收标准
- [x] 实例列表展示
- [x] 启动策略功能
- [x] 停止策略功能
- [x] 状态显示
- [x] 前端编译无错误
- [x] 后端编译无错误
- [x] 导航菜单集成

#### 产物
- `src/views/Strategy/StrategyInstances.vue` ✓ (~250行)
  - 实例列表表格
  - 启动策略对话框
  - 停止确认对话框
  - 自动刷新 (5秒间隔)
- `src/types/index.ts` ✓
  - 新增: InstanceStatus, StrategyConfig, InstanceInfo
- `src/api/tauri.ts` ✓
  - 新增: strategyEngineApi (start, stop, list, get)
- `src/router/index.ts` ✓
  - 新增: /strategy/instances 路由
- `src/views/Layout.vue` ✓
  - 新增: "运行实例" 菜单项
- `docs/verification/P3-08-verification-report.md` ✓

**状态**: ✅ 已完成
- 实现了策略实例管理页面 (StrategyInstances.vue)
- 实现了实例列表显示 (表格视图)
- 实现了启动策略对话框 (表单验证)
- 实现了停止策略功能 (确认对话框)
- 实现了状态指示器 (颜色编码标签)
- 实现了自动刷新 (每5秒)
- 实现了路由配置
- 实现了导航菜单集成
- 前端编译通过 (26.56秒)
- 后端编译通过 (3.27秒)
- [验证报告](../verification/P3-08-verification-report.md)

---

### P3-09 ~ P3-13: 回测引擎

**核心实现**：

1. 创建 `src-tauri/src/core/strategy/engine.rs`：
```rust
use crate::core::event::{EventBus, StrategyEvent, MarketEvent};
use crate::core::trade::types::*;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast, oneshot};

pub struct StrategyEngine {
    instances: Arc<RwLock<HashMap<String, RunningInstance>>>,
    event_bus: Arc<EventBus>,
}

impl StrategyEngine {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
        }
    }

    pub async fn start_instance(
        &self,
        config: StrategyConfig,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let instance = RunningInstance::new(
            id.clone(),
            config,
            self.event_bus.clone(),
        );

        // 启动策略循环
        let instance_clone = instance.clone();
        tokio::spawn(async move {
            instance_clone.run().await;
        });

        let mut instances = self.instances.write().await;
        instances.insert(id.clone(), instance);

        Ok(id)
    }

    pub async fn stop_instance(&self, id: &str) -> Result<()> {
        let mut instances = self.instances.write().await;
        if let Some(instance) = instances.remove(id) {
            instance.stop().await;
        }
        Ok(())
    }
}

pub struct RunningInstance {
    id: String,
    config: StrategyConfig,
    event_bus: Arc<EventBus>,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl RunningInstance {
    pub fn new(
        id: String,
        config: StrategyConfig,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            id,
            config,
            event_bus,
            shutdown_tx: None,
        }
    }

    pub async fn run(mut self) {
        let mut kline_stream = self.event_bus.subscribe_market();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        // 执行策略初始化
        self.execute_callback("onInit", &[]).await;

        loop {
            tokio::select! {
                // 接收行情事件
                Ok(event) = kline_stream.recv() => {
                    if let MarketEvent::Kline(kline) = event {
                        if let Ok(signal) = self.on_bar(&kline).await {
                            self.handle_signal(signal).await;
                        }
                    }
                }

                // 接收停止信号
                Ok(()) = &mut shutdown_rx => {
                    break;
                }
            }
        }

        // 执行策略清理
        let _ = self.execute_callback("onStop", &[]).await;
    }

    pub async fn stop(self) {
        if let Some(tx) = self.shutdown_tx {
            let _ = tx.send(());
        }
    }
}
```

---

### P3-09 ~ P3-13: 回测引擎

**核心实现**：

1. 创建 `src-tauri/src/core/backtest/engine.rs`：
```rust
use crate::core::trade::types::*;
use anyhow::Result;

pub struct BacktestEngine {
    datafeed: HistoricalDatafeed,
    broker: Broker,
    exchange: SimulatedExchange,
}

impl BacktestEngine {
    pub async fn run(
        &mut self,
        strategy_code: &str,
        params: BacktestParams,
    ) -> Result<BacktestReport> {
        // 1. 加载历史数据
        let klines = self.datafeed.load_klines(
            &params.symbol,
            &params.timeframe,
            params.start_time,
            params.end_time,
        ).await?;

        // 2. 初始化策略
        let mut strategy = StrategyScript::new(strategy_code);
        strategy.on_init().await;

        // 3. 逐根K线回放
        for kline in &klines {
            // 更新模拟交易所
            self.exchange.update_kline(kline.clone());

            // 执行策略
            if let Some(signal) = strategy.on_bar(kline).await {
                self.broker.execute(signal, &mut self.exchange).await?;
            }

            // 更新持仓
            self.exchange.update_positions();
        }

        // 4. 平仓所有持仓
        self.broker.close_all(&mut self.exchange).await?;

        // 5. 生成报告
        let report = self.generate_report()?;

        strategy.on_stop().await;

        Ok(report)
    }
}
```

---

## Phase 3 验收标准

### 功能验收
- [ ] 可创建和编辑策略代码
- [ ] 策略参数可配置
- [ ] 回测可正常运行
- [ ] 回测报告正确展示

### 技术验收
- [ ] JavaScript 引擎可执行用户代码
- [ ] 模拟交易所撮合逻辑正确
- [ ] 性能指标计算准确
