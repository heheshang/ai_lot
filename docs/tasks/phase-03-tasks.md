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

**实施步骤**:

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
- [ ] Monaco Editor 正常显示
- [ ] 代码高亮正常
- [ ] 支持 JavaScript 语法
- [ ] 内容双向绑定正常

---

### P3-02: 实现策略参数编辑表单

**依赖**: P3-01

**实施步骤**:

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

---

### P3-03: 实现 StrategyEditor 页面

**依赖**: P3-01, P3-02

**实施步骤**:

创建 `src/views/Strategy/StrategyEditor.vue`：
```vue
<template>
  <div class="strategy-editor">
    <el-card>
      <template #header>
        <div class="header">
          <h2>{{ isEdit ? '编辑策略' : '新建策略' }}</h2>
          <div class="actions">
            <el-button @click="$router.back()">取消</el-button>
            <el-button type="primary" @click="handleSave" :loading="saving">
              保存
            </el-button>
          </div>
        </div>
      </template>

      <el-form :model="form" label-width="100px">
        <el-form-item label="策略名称" required>
          <el-input v-model="form.name" placeholder="请输入策略名称" />
        </el-form-item>

        <el-form-item label="策略描述">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="2"
            placeholder="请输入策略描述"
          />
        </el-form-item>

        <el-form-item label="策略代码" required>
          <MonacoEditor
            v-model="form.code"
            language="javascript"
            :style="{ height: '400px' }"
          />
        </el-form-item>

        <el-form-item label="策略参数">
          <ParameterEditor
            v-model="form.parameterValues"
            :parameters="form.parameters"
          />
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage } from 'element-plus';
import MonacoEditor from '@/components/MonacoEditor.vue';
import ParameterEditor from '@/components/ParameterEditor.vue';
import type { Strategy } from '@/types';

const route = useRoute();
const router = useRouter();

const isEdit = ref(false);
const saving = ref(false);

const form = ref<Strategy>({
  id: '',
  userId: '',
  name: '',
  description: '',
  code: `// 策略初始化
function onInit() {
  // 在这里编写初始化逻辑
}

// K线更新时调用
function onBar(kline) {
  // 在这里编写交易逻辑
  // 返回交易信号或 null

  // 示例: 简单的均线策略
  if (kline.close > kline.open) {
    return {
      action: 'buy',
      quantity: 0.1,
      price: kline.close
    };
  }

  return null;
}

// 策略停止时调用
function onStop() {
  // 在这里编写清理逻辑
}
`,
  language: 'javascript',
  parameters: [
    {
      name: 'fastPeriod',
      type: 'number',
      default: 5,
      min: 1,
      max: 100,
      step: 1,
      description: '快速均线周期',
    },
    {
      name: 'slowPeriod',
      type: 'number',
      default: 20,
      min: 1,
      max: 200,
      step: 1,
      description: '慢速均线周期',
    },
  ],
  parameterValues: {
    fastPeriod: 5,
    slowPeriod: 20,
  },
  category: 'trend',
  tags: ['ma', 'trend'],
  version: 1,
  status: 'draft',
  createdAt: 0,
  updatedAt: 0,
});

async function handleSave() {
  if (!form.value.name) {
    ElMessage.warning('请输入策略名称');
    return;
  }

  saving.value = true;
  try {
    // 调用保存 API
    ElMessage.success(isEdit.value ? '策略已更新' : '策略已创建');
    router.push('/strategy');
  } catch (error) {
    ElMessage.error('保存失败：' + (error as Error).message);
  } finally {
    saving.value = false;
  }
}

onMounted(() => {
  const id = route.params.id as string;
  if (id) {
    isEdit.value = true;
    // 加载策略数据
  }
});
</script>

<style scoped>
.strategy-editor {
  padding: 0;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header h2 {
  margin: 0;
}

.actions {
  display: flex;
  gap: 8px;
}
</style>
```

---

### P3-05 ~ P3-08: 策略引擎

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
