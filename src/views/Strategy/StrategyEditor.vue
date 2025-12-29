<template>
  <div class="strategy-editor">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">{{ isEdit ? '编辑策略' : '新建策略' }}</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/strategy' }">策略列表</el-breadcrumb-item>
          <el-breadcrumb-item>{{ isEdit ? '编辑策略' : '新建策略' }}</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button @click="handleCancel">
          <el-icon><Back /></el-icon>
          取消
        </el-button>
        <el-button type="primary" @click="handleSave" :loading="saving">
          <el-icon><Check /></el-icon>
          保存策略
        </el-button>
      </div>
    </div>

    <!-- 基本信息卡片 -->
    <el-card class="info-card" shadow="never">
      <div class="card-title">基本信息</div>
      <el-form :model="form" label-width="100px" class="basic-form">
        <el-row :gutter="20">
          <el-col :span="8">
            <el-form-item label="策略名称" required>
              <el-input
                v-model="form.name"
                placeholder="请输入策略名称"
                maxlength="50"
                show-word-limit
                clearable
              >
                <template #prefix>
                  <el-icon><Edit /></el-icon>
                </template>
              </el-input>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="策略分类">
              <el-select v-model="form.category" placeholder="选择分类" style="width: 100%">
                <el-option label="趋势跟踪" value="trend">
                  <div class="option-item">
                    <el-icon><TrendCharts /></el-icon>
                    <span>趋势跟踪</span>
                  </div>
                </el-option>
                <el-option label="均值回归" value="mean_reversion">
                  <div class="option-item">
                    <el-icon><RefreshLeft /></el-icon>
                    <span>均值回归</span>
                  </div>
                </el-option>
                <el-option label="套利" value="arbitrage">
                  <div class="option-item">
                    <el-icon><Switch /></el-icon>
                    <span>套利</span>
                  </div>
                </el-option>
                <el-option label="网格交易" value="grid">
                  <div class="option-item">
                    <el-icon><Grid /></el-icon>
                    <span>网格交易</span>
                  </div>
                </el-option>
                <el-option label="高频交易" value="high_frequency">
                  <div class="option-item">
                    <el-icon><Lightning /></el-icon>
                    <span>高频交易</span>
                  </div>
                </el-option>
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="8">
            <el-form-item label="代码语言">
              <el-select v-model="form.language" style="width: 100%">
                <el-option label="JavaScript" value="javascript" />
                <el-option label="TypeScript" value="typescript" />
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item label="策略描述">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="2"
            placeholder="请输入策略描述，简要说明策略的核心逻辑和适用场景"
            maxlength="200"
            show-word-limit
          />
        </el-form-item>

        <el-form-item label="策略标签">
          <el-select
            v-model="form.tags"
            multiple
            filterable
            allow-create
            placeholder="选择或输入标签，按回车确认"
            style="width: 100%"
          >
            <el-option label="均线" value="ma" />
            <el-option label="MACD" value="macd" />
            <el-option label="RSI" value="rsi" />
            <el-option label="布林带" value="boll" />
            <el-option label="突破" value="breakout" />
            <el-option label="跟踪止损" value="trailing" />
            <el-option label="趋势" value="trend" />
            <el-option label="震荡" value="oscillator" />
          </el-select>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 主内容区域 -->
    <div class="content-area">
      <!-- 代码编辑器 -->
      <el-card class="editor-card" shadow="never">
        <template #header>
          <div class="card-header">
            <div class="header-left">
              <el-icon class="card-icon"><Document /></el-icon>
              <span class="card-title">策略代码</span>
              <el-tag size="small" :type="form.language === 'javascript' ? 'warning' : 'primary'">
                {{ form.language === 'javascript' ? 'JS' : 'TS' }}
              </el-tag>
            </div>
            <div class="header-actions">
              <el-button-group>
                <el-button size="small" @click="formatCode">
                  <el-icon><MagicStick /></el-icon>
                  格式化
                </el-button>
                <el-button size="small" @click="resetCode">
                  <el-icon><RefreshLeft /></el-icon>
                  重置
                </el-button>
                <el-button size="small" @click="showTemplates = true">
                  <el-icon><Collection /></el-icon>
                  模板
                </el-button>
              </el-button-group>
            </div>
          </div>
        </template>
        <div class="editor-wrapper">
          <MonacoEditor
            ref="monacoRef"
            v-model="form.code"
            :language="form.language"
            theme="vs-dark"
            :font-size="14"
            height="100%"
            @ready="onEditorReady"
          />
        </div>
      </el-card>

      <!-- 参数面板 -->
      <el-card class="params-card" shadow="never">
        <template #header>
          <div class="card-header">
            <div class="header-left">
              <el-icon class="card-icon"><Setting /></el-icon>
              <span class="card-title">策略参数</span>
              <el-badge :value="form.parameters.length" :max="99" class="param-badge" />
            </div>
            <el-button size="small" type="primary" @click="addParameter">
              <el-icon><Plus /></el-icon>
              添加参数
            </el-button>
          </div>
        </template>
        <div class="params-content">
          <!-- 参数列表 -->
          <div v-if="form.parameters.length > 0" class="params-list">
            <div
              v-for="(param, index) in form.parameters"
              :key="param.name"
              class="param-item"
            >
              <div class="param-main">
                <div class="param-info">
                  <span class="param-name">{{ param.name }}</span>
                  <el-tag :type="getParamTypeColor(param.type)" size="small">
                    {{ getParamTypeLabel(param.type) }}
                  </el-tag>
                </div>
                <div class="param-desc">{{ param.description || '无说明' }}</div>
              </div>
              <div class="param-value">
                <span class="param-default">默认: {{ param.default }}</span>
              </div>
              <div class="param-actions">
                <el-button
                  size="small"
                  :icon="Edit"
                  @click="editParameter(index)"
                />
                <el-button
                  size="small"
                  type="danger"
                  :icon="Delete"
                  @click="removeParameter(index)"
                />
              </div>
            </div>
          </div>

          <el-empty v-else description="暂无参数，点击上方按钮添加" :image-size="80" />

          <!-- 参数值编辑器 -->
          <div v-if="form.parameters.length > 0" class="param-values">
            <el-divider content-position="left">
              <span class="divider-title">参数值配置</span>
            </el-divider>
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

    <!-- 预览卡片 -->
    <el-card v-if="form.parameters.length > 0" class="preview-card" shadow="never">
      <template #header>
        <div class="card-header">
          <el-icon class="card-icon"><View /></el-icon>
          <span class="card-title">参数预览</span>
        </div>
      </template>
      <div class="preview-content">
        <pre class="preview-code">{{ formatParametersPreview() }}</pre>
      </div>
    </el-card>

    <!-- 添加参数对话框 -->
    <el-dialog
      v-model="paramDialogVisible"
      :title="editingParamIndex >= 0 ? '编辑参数' : '添加参数'"
      width="500px"
      @close="resetNewParam"
    >
      <el-form :model="newParam" label-width="90px" class="param-form">
        <el-form-item label="参数名" required>
          <el-input v-model="newParam.name" placeholder="例如: fastPeriod">
            <template #prefix>
              <el-icon><Key /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="类型" required>
          <el-select v-model="newParam.type" style="width: 100%" @change="onParamTypeChange">
            <el-option label="数字" value="number">
              <div class="option-item">
                <el-icon><Histogram /></el-icon>
                <span>数字</span>
              </div>
            </el-option>
            <el-option label="文本" value="string">
              <div class="option-item">
                <el-icon><Notebook /></el-icon>
                <span>文本</span>
              </div>
            </el-option>
            <el-option label="布尔" value="boolean">
              <div class="option-item">
                <el-icon><Toggle /></el-icon>
                <span>布尔</span>
              </div>
            </el-option>
            <el-option label="选择" value="select">
              <div class="option-item">
                <el-icon><List /></el-icon>
                <span>选择</span>
              </div>
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="默认值">
          <el-input v-model="newParam.default" placeholder="参数默认值">
            <template #prefix>
              <el-icon><PriceTag /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item v-if="newParam.type === 'number'" label="取值范围">
          <div class="range-inputs">
            <el-input-number
              v-model="newParam.min"
              placeholder="最小值"
              :controls="false"
              style="flex: 1"
            />
            <span class="range-separator">~</span>
            <el-input-number
              v-model="newParam.max"
              placeholder="最大值"
              :controls="false"
              style="flex: 1"
            />
          </div>
        </el-form-item>
        <el-form-item v-if="newParam.type === 'number'" label="步长">
          <el-input-number v-model="newParam.step" :min="0.001" :step="0.001" :precision="3" />
        </el-form-item>
        <el-form-item v-if="newParam.type === 'select'" label="选项">
          <el-select
            v-model="newParam.options"
            multiple
            filterable
            allow-create
            placeholder="输入可选项，按回车添加"
            style="width: 100%"
          >
            <el-option
              v-for="opt in newParam.options"
              :key="opt"
              :label="opt"
              :value="opt"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="说明">
          <el-input v-model="newParam.description" type="textarea" :rows="2" placeholder="参数说明，例如：快速均线周期" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="paramDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmAddParameter">
          {{ editingParamIndex >= 0 ? '保存' : '添加' }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 代码模板对话框 -->
    <el-dialog v-model="showTemplates" title="策略模板" width="600px">
      <div class="templates-list">
        <div
          v-for="template in strategyTemplates"
          :key="template.id"
          class="template-item"
          @click="applyTemplate(template)"
        >
          <div class="template-icon">
            <el-icon :size="24">
              <component :is="template.icon" />
            </el-icon>
          </div>
          <div class="template-content">
            <div class="template-name">{{ template.name }}</div>
            <div class="template-desc">{{ template.description }}</div>
          </div>
          <el-icon class="template-arrow"><ArrowRight /></el-icon>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  Back,
  Check,
  Edit,
  TrendCharts,
  RefreshLeft,
  Grid,
  Lightning,
  Document,
  Setting,
  Plus,
  Delete,
  View,
  MagicStick,
  Collection,
  Key,
  Histogram,
  Notebook,
  List,
  PriceTag,
  ArrowRight,
} from '@element-plus/icons-vue';
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
const monacoRef = ref();
const paramDialogVisible = ref(false);
const showTemplates = ref(false);
const editingParamIndex = ref(-1);

// Form
const form = ref<Strategy>({
  id: '',
  userId: '',
  name: '',
  description: '',
  code: `// 策略初始化 - 在策略启动时调用一次
function onInit(context) {
  const params = context.parameters;
  context.storage.set('initialized', true);
  console.log('策略初始化完成，参数:', params);
}

// K线更新回调 - 每根K线收盘时调用
function onBar(context, kline) {
  const params = context.parameters;
  const history = context.getHistory(kline.symbol, kline.timeframe, 100);

  // 数据不足检查
  if (history.length < params.slowPeriod) {
    return null;
  }

  // 计算均线
  const fastMA = calculateMA(history, params.fastPeriod);
  const slowMA = calculateMA(history, params.slowPeriod);

  // 金叉买入
  const prevFastMA = fastMA[fastMA.length - 2];
  const prevSlowMA = slowMA[slowMA.length - 2];

  if (prevFastMA <= prevSlowMA && fastMA[fastMA.length - 1] > slowMA[slowMA.length - 1]) {
    return {
      action: 'buy',
      quantity: params.quantity || 0.1,
      price: kline.close,
      reason: '金叉买入'
    };
  }

  // 死叉卖出
  if (prevFastMA >= prevSlowMA && fastMA[fastMA.length - 1] < slowMA[slowMA.length - 1]) {
    return {
      action: 'sell',
      quantity: params.quantity || 0.1,
      price: kline.close,
      reason: '死叉卖出'
    };
  }

  return null;
}

// 策略停止时调用
function onStop(context) {
  console.log('策略已停止');
  // 清理资源
}

// 辅助函数：计算移动平均线
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

// 新参数表单
const newParam = ref<StrategyParameter>({
  name: '',
  type: 'number',
  default: null,
  description: '',
  min: undefined,
  max: undefined,
  step: undefined,
  options: [],
});

// 策略模板
const strategyTemplates = ref([
  {
    id: 'ma_cross',
    name: '双均线交叉策略',
    description: '基于快慢均线的金叉死叉信号进行交易',
    icon: 'TrendCharts',
    code: `// 双均线交叉策略
function onInit(context) {
  console.log('双均线策略初始化');
}

function onBar(context, kline) {
  const params = context.parameters;
  const history = context.getHistory(kline.symbol, kline.timeframe, 100);

  if (history.length < params.slowPeriod) return null;

  const fastMA = calculateMA(history, params.fastPeriod);
  const slowMA = calculateMA(history, params.slowPeriod);

  const prevFastMA = fastMA[fastMA.length - 2];
  const prevSlowMA = slowMA[slowMA.length - 2];

  if (prevFastMA <= prevSlowMA && fastMA[fastMA.length - 1] > slowMA[slowMA.length - 1]) {
    return { action: 'buy', quantity: params.quantity, price: kline.close };
  }

  if (prevFastMA >= prevSlowMA && fastMA[fastMA.length - 1] < slowMA[slowMA.length - 1]) {
    return { action: 'sell', quantity: params.quantity, price: kline.close };
  }

  return null;
}

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
    parameters: [
      { name: 'fastPeriod', type: 'number', default: 5, min: 1, max: 50, description: '快速均线周期' },
      { name: 'slowPeriod', type: 'number', default: 20, min: 1, max: 200, description: '慢速均线周期' },
      { name: 'quantity', type: 'number', default: 0.1, description: '交易数量' },
    ],
  },
  {
    id: 'grid',
    name: '网格交易策略',
    description: '在指定价格区间内设置网格，自动低买高卖',
    icon: 'Grid',
    code: `// 网格交易策略
function onInit(context) {
  context.storage.set('grid', []);
}

function onBar(context, kline) {
  const params = context.parameters;
  const grids = context.storage.get('grid') || [];

  // 检查是否触发网格
  for (const grid of grids) {
    if (kline.close <= grid.buyPrice) {
      return { action: 'buy', quantity: grid.quantity, price: kline.close };
    }
    if (kline.close >= grid.sellPrice) {
      return { action: 'sell', quantity: grid.quantity, price: kline.close };
    }
  }

  return null;
}
`,
    parameters: [
      { name: 'gridCount', type: 'number', default: 10, min: 2, max: 50, description: '网格数量' },
      { name: 'gridSpacing', type: 'number', default: 0.01, min: 0.001, description: '网格间距' },
      { name: 'quantity', type: 'number', default: 0.1, description: '每格数量' },
    ],
  },
  {
    id: 'macd',
    name: 'MACD策略',
    description: '基于MACD指标的趋势跟踪策略',
    icon: 'DataAnalysis',
    code: `// MACD策略
function onBar(context, kline) {
  const params = context.parameters;
  const history = context.getHistory(kline.symbol, kline.timeframe, 100);

  if (history.length < 26) return null;

  const macd = calculateMACD(history, 12, 26, 9);
  const dif = macd.dif;
  const dea = macd.dea;

  const prevDif = dif[dif.length - 2];
  const prevDea = dea[dea.length - 2];

  if (prevDif <= prevDea && dif[dif.length - 1] > dea[dea.length - 1]) {
    return { action: 'buy', quantity: params.quantity, price: kline.close };
  }

  if (prevDif >= prevDea && dif[dif.length - 1] < dea[dea.length - 1]) {
    return { action: 'sell', quantity: params.quantity, price: kline.close };
  }

  return null;
}

function calculateMACD(data, short, long, signal) {
  // MACD 计算逻辑
  // 返回 { dif, dea, histogram }
  const emaShort = calculateEMA(data.map(d => d.close), short);
  const emaLong = calculateEMA(data.map(d => d.close), long);

  const dif = emaShort.map((v, i) => v - emaLong[i]);
  const dea = calculateEMA(dif, signal);

  return { dif, dea };
}
`,
    parameters: [
      { name: 'quantity', type: 'number', default: 0.1, description: '交易数量' },
    ],
  },
]);

// Editor instance
let editorInstance: any = null;

function onEditorReady(editor: any) {
  editorInstance = editor;
}

function formatCode() {
  if (editorInstance) {
    const action = editorInstance.getAction('editor.action.formatDocument');
    if (action) {
      action.run();
      ElMessage.success('代码格式化完成');
    }
  }
}

function resetCode() {
  ElMessageBox.confirm('确定要重置代码吗？所有修改将丢失。', '确认重置', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning',
  }).then(() => {
    // 重置为初始代码
    ElMessage.success('代码已重置');
  }).catch(() => {});
}

function addParameter() {
  editingParamIndex.value = -1;
  newParam.value = {
    name: '',
    type: 'number',
    default: null,
    description: '',
    options: [],
  };
  paramDialogVisible.value = true;
}

function editParameter(index: number) {
  editingParamIndex.value = index;
  const param = form.value.parameters[index];
  newParam.value = { ...param };
  paramDialogVisible.value = true;
}

function confirmAddParameter() {
  if (!newParam.value.name) {
    ElMessage.warning('请输入参数名');
    return;
  }

  // 检查重复（编辑时排除自己）
  const existingIndex = form.value.parameters.findIndex((p, i) =>
    p.name === newParam.value.name && i !== editingParamIndex.value
  );

  if (existingIndex >= 0) {
    ElMessage.warning('参数名已存在');
    return;
  }

  const paramData = { ...newParam.value };

  if (editingParamIndex.value >= 0) {
    // 编辑模式
    const oldName = form.value.parameters[editingParamIndex.value].name;
    form.value.parameters[editingParamIndex.value] = paramData;

    // 更新 parameterValues
    if (oldName !== paramData.name) {
      form.value.parameterValues[paramData.name] = form.value.parameterValues[oldName];
      delete form.value.parameterValues[oldName];
    }

    ElMessage.success('参数已更新');
  } else {
    // 新增模式
    form.value.parameters.push(paramData);
    form.value.parameterValues[paramData.name] = paramData.default;
    ElMessage.success('参数添加成功');
  }

  paramDialogVisible.value = false;
}

function removeParameter(index: number) {
  const param = form.value.parameters[index];
  form.value.parameters.splice(index, 1);
  delete form.value.parameterValues[param.name];
  ElMessage.success('参数已删除');
}

function resetNewParam() {
  newParam.value = {
    name: '',
    type: 'number',
    default: null,
    description: '',
    options: [],
  };
}

function onParamTypeChange() {
  // 类型切换时重置相关字段
  if (newParam.value.type !== 'number') {
    newParam.value.min = undefined;
    newParam.value.max = undefined;
    newParam.value.step = undefined;
  }
  if (newParam.value.type !== 'select') {
    newParam.value.options = [];
  }
}

function getParamTypeColor(type: string): string {
  const colors: Record<string, string> = {
    number: 'warning',
    string: 'success',
    boolean: 'info',
    select: 'primary',
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

  return `const parameters = {
${params}
};`;
}

function applyTemplate(tpl: any) {
  ElMessageBox.confirm(
    `应用模板 "${tpl.name}" 将覆盖当前代码，是否继续？`,
    '确认应用模板',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    }
  ).then(() => {
    form.value.code = tpl.code;
    form.value.parameters = [...tpl.parameters];
    form.value.parameterValues = {};
    tpl.parameters.forEach((p: StrategyParameter) => {
      form.value.parameterValues[p.name] = p.default;
    });
    showTemplates.value = false;
    ElMessage.success('模板已应用');
  }).catch(() => {});
}

async function handleSave() {
  if (!form.value.name) {
    ElMessage.warning('请输入策略名称');
    return;
  }

  if (!form.value.code || form.value.code.trim().length === 0) {
    ElMessage.warning('请输入策略代码');
    return;
  }

  const userId = userStore.user?.id;
  if (!userId) {
    ElMessage.error('请先登录');
    return;
  }

  saving.value = true;
  try {
    const strategyToSave: Strategy = {
      ...form.value,
      userId,
    };

    // 打印发送的数据用于调试
    console.log('Saving strategy:', strategyToSave);

    const result = await strategyApi.save(strategyToSave);

    console.log('Strategy saved successfully:', result);

    ElMessage.success(isEdit.value ? '策略已更新' : '策略已创建');

    setTimeout(() => {
      router.push('/strategy');
    }, 500);
  } catch (error) {
    console.error('Failed to save strategy:', error);

    // 更详细的错误信息
    let errorMessage = '保存失败';
    if (error instanceof Error) {
      errorMessage = error.message || errorMessage;
    } else if (typeof error === 'string') {
      errorMessage = error;
    } else {
      errorMessage = JSON.stringify(error);
    }

    ElMessage.error(errorMessage);
  } finally {
    saving.value = false;
  }
}

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
      .catch(() => {});
  } else {
    router.back();
  }
}

function hasUnsavedChanges(): boolean {
  return form.value.name !== '' || form.value.code.trim().length > 0;
}

onMounted(async () => {
  const id = route.params.id as string;
  if (id) {
    try {
      const strategy = await strategyApi.get(id);
      if (strategy) {
        // 转换参数格式：将 options JSON 字符串解析为数组
        const parameters = strategy.parameters.map(param => ({
          ...param,
          options: param.options ? (JSON.parse(param.options as string) as string[]) : undefined,
        }));

        form.value = {
          ...form.value,
          ...strategy,
          parameters,
          parameterValues: strategy.parameterValues || {},
        };
      }
    } catch (error) {
      ElMessage.error('加载策略失败');
    }
  }
});
</script>

<style scoped lang="scss">
.strategy-editor {
  padding: 0;
  min-height: calc(100vh - 60px);
  background: #f5f7fa;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

// 页面头部
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
}

// 卡片通用样式
.info-card,
.editor-card,
.params-card,
.preview-card {
  :deep(.el-card__header) {
    padding: 16px 20px;
    background: #fafbfc;
    border-bottom: 1px solid #ebeef5;
  }

  :deep(.el-card__body) {
    padding: 20px;
  }
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
}

// 基本信息表单
.basic-form {
  :deep(.el-form-item__label) {
    font-weight: 500;
  }

  .option-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }
}

// 内容区域
.content-area {
  display: flex;
  gap: 20px;
  flex: 1;
  min-height: 0;
}

.editor-card {
  flex: 1;
  min-width: 0;
}

.params-card {
  flex: 0 0 360px;
  min-width: 360px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;

  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .card-icon {
    font-size: 18px;
    color: #409eff;
  }

  .card-title {
    font-size: 14px;
    font-weight: 500;
  }

  .param-badge {
    margin-left: 4px;
  }
}

.editor-wrapper {
  height: 500px;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #dcdfe6;
}

// 参数列表
.params-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

.params-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.param-item {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 12px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  border: 1px solid #ebeef5;
  transition: all 0.2s;

  &:hover {
    border-color: #c6e2ff;
    box-shadow: 0 2px 8px rgba(64, 158, 255, 0.1);
  }
}

.param-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.param-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.param-name {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  font-family: monospace;
}

.param-desc {
  font-size: 12px;
  color: #909399;
}

.param-value {
  display: flex;
  align-items: center;
}

.param-default {
  font-size: 12px;
  color: #606266;
  font-family: monospace;
  background: #fff;
  padding: 2px 8px;
  border-radius: 4px;
}

.param-actions {
  display: flex;
  gap: 4px;
}

// 参数值编辑器
.param-values {
  margin-top: 16px;

  .divider-title {
    font-size: 13px;
    font-weight: 500;
    color: #606266;
  }

  :deep(.parameter-editor) {
    .param-input {
      margin-bottom: 12px;
    }
  }
}

// 预览
.preview-card {
  flex-shrink: 0;
}

.preview-content {
  background: #1e1e1e;
  border-radius: 8px;
  overflow: hidden;
}

.preview-code {
  margin: 0;
  padding: 16px;
  font-family: 'Fira Code', 'Monaco', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.6;
  color: #d4d4d4;
  overflow-x: auto;
}

// 对话框样式
.param-form {
  .option-item {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .range-inputs {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .range-separator {
    color: #909399;
  }
}

// 模板列表
.templates-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.template-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background: #ecf5ff;
    transform: translateX(4px);
  }
}

.template-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.template-content {
  flex: 1;
}

.template-name {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
}

.template-desc {
  font-size: 12px;
  color: #909399;
  margin-top: 2px;
}

.template-arrow {
  color: #c0c4cc;
}
</style>
