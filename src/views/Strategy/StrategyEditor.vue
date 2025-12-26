<template>
  <div class="strategy-editor">
    <!-- Header Card -->
    <el-card class="header-card">
      <template #header>
        <div class="header">
          <h2>{{ isEdit ? '编辑策略' : '新建策略' }}</h2>
          <div class="actions">
            <el-button @click="handleCancel">取消</el-button>
            <el-button type="primary" @click="handleSave" :loading="saving">
              保存策略
            </el-button>
          </div>
        </div>
      </template>

      <!-- Basic Info Form -->
      <el-form :model="form" label-width="100px" class="basic-form">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="策略名称" required>
              <el-input
                v-model="form.name"
                placeholder="请输入策略名称"
                maxlength="50"
                show-word-limit
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="策略分类">
              <el-select v-model="form.category" placeholder="选择分类" style="width: 100%">
                <el-option label="趋势策略" value="trend" />
                <el-option label="震荡策略" value="oscillator" />
                <el-option label="套利策略" value="arbitrage" />
                <el-option label="网格策略" value="grid" />
                <el-option label="其他" value="other" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item label="策略描述">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="2"
            placeholder="请输入策略描述"
            maxlength="200"
            show-word-limit
          />
        </el-form-item>

        <el-form-item label="标签">
          <el-select
            v-model="form.tags"
            multiple
            filterable
            allow-create
            placeholder="选择或输入标签"
            style="width: 100%"
          >
            <el-option label="均线" value="ma" />
            <el-option label="MACD" value="macd" />
            <el-option label="RSI" value="rsi" />
            <el-option label="布林带" value="boll" />
            <el-option label="突破" value="breakout" />
            <el-option label="跟踪止损" value="trailing" />
          </el-select>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- Main Content Area -->
    <div class="content-area">
      <!-- Code Editor -->
      <el-card class="editor-card">
        <template #header>
          <div class="card-header">
            <span>策略代码</span>
            <el-space>
              <el-tag size="small">{{ form.language }}</el-tag>
              <el-button
                type="primary"
                link
                size="small"
                @click="formatCode"
              >
                格式化代码
              </el-button>
            </el-space>
          </div>
        </template>
        <div class="editor-wrapper">
          <MonacoEditor
            ref="monacoRef"
            v-model="form.code"
            language="javascript"
            theme="vs-dark"
            :font-size="14"
            height="100%"
            @ready="onEditorReady"
          />
        </div>
      </el-card>

      <!-- Parameters Panel -->
      <el-card class="params-card">
        <template #header>
          <div class="card-header">
            <span>策略参数</span>
            <el-button
              type="primary"
              link
              size="small"
              @click="addParameter"
            >
              + 添加参数
            </el-button>
          </div>
        </template>
        <div class="params-content">
          <!-- Parameters Table -->
          <el-table
            :data="form.parameters"
            border
            size="small"
            max-height="300"
          >
            <el-table-column prop="name" label="参数名" width="120" />
            <el-table-column prop="type" label="类型" width="80" align="center">
              <template #default="{ row }">
                <el-tag :type="getParamTypeColor(row.type)" size="small">
                  {{ getParamTypeLabel(row.type) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="default" label="默认值" width="80" />
            <el-table-column prop="description" label="说明" min-width="120" />
            <el-table-column label="操作" width="80" align="center" fixed="right">
              <template #default="{ $index }">
                <el-button
                  type="danger"
                  link
                  size="small"
                  @click="removeParameter($index)"
                >
                  删除
                </el-button>
              </template>
            </el-table-column>
          </el-table>

          <!-- Add Parameter Dialog -->
          <el-dialog
            v-model="paramDialogVisible"
            title="添加参数"
            width="500px"
          >
            <el-form :model="newParam" label-width="80px">
              <el-form-item label="参数名" required>
                <el-input v-model="newParam.name" placeholder="例如: fastPeriod" />
              </el-form-item>
              <el-form-item label="类型" required>
                <el-select v-model="newParam.type" style="width: 100%">
                  <el-option label="数字" value="number" />
                  <el-option label="文本" value="string" />
                  <el-option label="布尔" value="boolean" />
                  <el-option label="选择" value="select" />
                </el-select>
              </el-form-item>
              <el-form-item label="默认值">
                <el-input v-model="newParam.default" placeholder="默认值" />
              </el-form-item>
              <el-form-item v-if="newParam.type === 'number'" label="范围">
                <el-input-number v-model="newParam.min" placeholder="最小" :controls="false" />
                <span style="margin: 0 8px">-</span>
                <el-input-number v-model="newParam.max" placeholder="最大" :controls="false" />
              </el-form-item>
              <el-form-item v-if="newParam.type === 'number'" label="步长">
                <el-input-number v-model="newParam.step" :min="0.001" :step="0.001" />
              </el-form-item>
              <el-form-item label="说明">
                <el-input v-model="newParam.description" placeholder="参数说明" />
              </el-form-item>
            </el-form>
            <template #footer>
              <el-button @click="paramDialogVisible = false">取消</el-button>
              <el-button type="primary" @click="confirmAddParameter">确定</el-button>
            </template>
          </el-dialog>

          <!-- Parameter Values Editor -->
          <div v-if="form.parameters.length > 0" class="param-values">
            <el-divider content-position="left">参数值配置</el-divider>
            <ParameterEditor
              v-model="form.parameterValues"
              :parameters="form.parameters"
              :show-reset="false"
              :show-summary="false"
            />
          </div>
        </div>
      </el-card>
    </div>

    <!-- Test Preview -->
    <el-card v-if="form.parameters.length > 0" class="preview-card">
      <template #header>
        <span>参数预览</span>
      </template>
      <pre class="preview-code">{{ formatParametersPreview() }}</pre>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import MonacoEditor from '@/components/MonacoEditor.vue';
import ParameterEditor from '@/components/ParameterEditor.vue';
import { strategyApi } from '@/api/tauri';
import { useUserStore } from '@/store/modules/user';
import type { Strategy, StrategyParameter } from '@/types';

// Route & Router
const route = useRoute();
const router = useRouter();

// User Store
const userStore = useUserStore();

// State
const isEdit = computed(() => !!route.params.id);
const saving = ref(false);
const loading = ref(false);
const monacoRef = ref();
const paramDialogVisible = ref(false);

// Form
const form = ref<Strategy>({
  id: '',
  userId: '',
  name: '',
  description: '',
  code: `// 策略初始化
// 此函数在策略启动时调用一次
function onInit(context) {
  // 获取策略参数
  const params = context.parameters;

  // 初始化变量
  context.storage.set('initialized', true);

  console.log('Strategy initialized with params:', params);
}

// K线更新时调用
// 此函数在每根K线收盘时调用
function onBar(context, kline) {
  // 获取参数
  const params = context.parameters;

  // 获取历史K线
  const history = context.getHistory(kline.symbol, kline.timeframe, 100);

  // 计算指标
  // 示例: 简单的均线策略
  if (history.length < params.slowPeriod) {
    return null; // 数据不足
  }

  // 计算快速均线
  const fastMA = calculateMA(history, params.fastPeriod);

  // 计算慢速均线
  const slowMA = calculateMA(history, params.slowPeriod);

  // 金叉买入，死叉卖出
  const prevFastMA = fastMA[fastMA.length - 2];
  const prevSlowMA = slowMA[slowMA.length - 2];

  // 金叉: 快线上穿慢线
  if (prevFastMA <= prevSlowMA && fastMA[fastMA.length - 1] > slowMA[slowMA.length - 1]) {
    return {
      action: 'buy',
      quantity: params.quantity || 0.1,
      price: kline.close,
      reason: '金叉买入信号'
    };
  }

  // 死叉: 快线下穿慢线
  if (prevFastMA >= prevSlowMA && fastMA[fastMA.length - 1] < slowMA[slowMA.length - 1]) {
    return {
      action: 'sell',
      quantity: params.quantity || 0.1,
      price: kline.close,
      reason: '死叉卖出信号'
    };
  }

  return null; // 无信号
}

// 策略停止时调用
// 此函数在策略停止时调用一次
function onStop(context) {
  console.log('Strategy stopped');
  // 清理资源
}

// 辅助函数: 计算移动平均线
function calculateMA(data, period) {
  const result = [];
  for (let i = period - 1; i < data.length; i++) {
    let sum = 0;
    for (let j = 0; j < period; j++) {
      sum += data[i - j].close;
    }
    result.push(sum / period);
  }
  return result;
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
    {
      name: 'quantity',
      type: 'number',
      default: 0.1,
      min: 0.001,
      max: 10,
      step: 0.001,
      description: '交易数量',
    },
  ],
  parameterValues: {
    fastPeriod: 5,
    slowPeriod: 20,
    quantity: 0.1,
  },
  category: 'trend',
  tags: ['ma', 'trend'],
  version: 1,
  status: 'draft',
  createdAt: 0,
  updatedAt: 0,
});

// New parameter form
const newParam = ref<StrategyParameter>({
  name: '',
  type: 'number',
  default: null,
  description: '',
});

// Editor instance
let editorInstance: any = null;

function onEditorReady(editor: any) {
  editorInstance = editor;
  console.log('Monaco Editor ready');
}

// Format code using Monaco's format action
function formatCode() {
  if (editorInstance) {
    const action = editorInstance.getAction('editor.action.formatDocument');
    if (action) {
      action.run();
      ElMessage.success('代码格式化完成');
    } else {
      ElMessage.warning('格式化功能不可用');
    }
  }
}

// Add parameter
function addParameter() {
  newParam.value = {
    name: '',
    type: 'number',
    default: null,
    description: '',
  };
  paramDialogVisible.value = true;
}

function confirmAddParameter() {
  if (!newParam.value.name) {
    ElMessage.warning('请输入参数名');
    return;
  }

  // Check for duplicate names
  if (form.value.parameters.some(p => p.name === newParam.value.name)) {
    ElMessage.warning('参数名已存在');
    return;
  }

  form.value.parameters.push({ ...newParam.value });

  // Initialize value
  form.value.parameterValues[newParam.value.name] = newParam.value.default;

  paramDialogVisible.value = false;
  ElMessage.success('参数添加成功');
}

function removeParameter(index: number) {
  const param = form.value.parameters[index];
  form.value.parameters.splice(index, 1);
  delete form.value.parameterValues[param.name];
}

function getParamTypeColor(type: string): string {
  const colors: Record<string, string> = {
    number: 'primary',
    string: 'success',
    boolean: 'warning',
    select: 'info',
  };
  return colors[type] || '';
}

function getParamTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    number: '数字',
    string: '文本',
    boolean: '布尔',
    select: '选择',
  };
  return labels[type] || type;
}

function formatParametersPreview(): string {
  const params = form.value.parameters.map(p => {
    const value = form.value.parameterValues[p.name];
    return `  ${p.name}: ${JSON.stringify(value)} // ${p.description || ''}`;
  }).join('\n');

  return `{
${params}
}`;
}

// Save strategy
async function handleSave() {
  if (!form.value.name) {
    ElMessage.warning('请输入策略名称');
    return;
  }

  if (!form.value.code || form.value.code.trim().length === 0) {
    ElMessage.warning('请输入策略代码');
    return;
  }

  // Get current user ID
  const userId = userStore.user?.id;
  if (!userId) {
    ElMessage.error('请先登录');
    return;
  }

  saving.value = true;
  try {
    // Prepare strategy data
    const strategyToSave: Strategy = {
      ...form.value,
      userId,
    };

    // Call API to save strategy
    await strategyApi.save(strategyToSave);

    ElMessage.success(isEdit.value ? '策略已更新' : '策略已创建');

    // Navigate back to list
    setTimeout(() => {
      router.push('/strategy');
    }, 500);
  } catch (error) {
    ElMessage.error('保存失败：' + (error as Error).message);
  } finally {
    saving.value = false;
  }
}

// Cancel and go back
function handleCancel() {
  if (hasUnsavedChanges()) {
    ElMessageBox.confirm(
      '您有未保存的更改，确定要离开吗？',
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
      .then(() => {
        router.back();
      })
      .catch(() => {
        // User cancelled
      });
  } else {
    router.back();
  }
}

// Check for unsaved changes
function hasUnsavedChanges(): boolean {
  // Simple check - in real app, compare with original data
  return form.value.name !== '' || form.value.code.trim().length > 0;
}

// Load strategy on edit mode
onMounted(async () => {
  const id = route.params.id as string;
  if (id) {
    loading.value = true;
    try {
      // Load strategy from API
      const strategy = await strategyApi.get(id);

      if (strategy) {
        // Merge loaded data with form
        form.value = {
          ...form.value,
          ...strategy,
          // Ensure parameterValues is initialized
          parameterValues: strategy.parameterValues || {},
        };
        ElMessage.success('策略加载成功');
      } else {
        ElMessage.error('策略不存在');
        router.back();
      }
    } catch (error) {
      ElMessage.error('加载策略失败：' + (error as Error).message);
      router.back();
    } finally {
      loading.value = false;
    }
  }
});
</script>

<style scoped>
.strategy-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: hidden;
}

.header-card {
  flex-shrink: 0;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.actions {
  display: flex;
  gap: 8px;
}

.basic-form {
  margin-top: 16px;
}

.content-area {
  flex: 1;
  display: flex;
  gap: 16px;
  min-height: 0;
}

.editor-card,
.params-card {
  display: flex;
  flex-direction: column;
}

.editor-card {
  flex: 1;
  min-width: 0;
}

.params-card {
  flex: 0 0 400px;
}

:deep(.editor-card .el-card__body),
:deep(.params-card .el-card__body) {
  flex: 1;
  padding: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.editor-wrapper {
  height: 500px;
  width: 100%;
}

.params-content {
  flex: 1;
  overflow: auto;
  padding: 16px;
}

.param-values {
  margin-top: 16px;
}

.preview-card {
  flex-shrink: 0;
}

.preview-code {
  background: var(--el-fill-color-light);
  padding: 12px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.6;
  color: var(--el-text-color-primary);
  margin: 0;
  overflow-x: auto;
}
</style>
