<template>
  <div class="strategy-instances">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">策略实例</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>策略实例</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button type="primary" @click="showStartDialog = true">
          <el-icon><Plus /></el-icon>
          启动策略
        </el-button>
      </div>
    </div>

    <!-- 统计概览 -->
    <el-row :gutter="16" class="stats-row">
      <el-col :span="6">
        <div class="stat-card stat-running">
          <div class="stat-icon">
            <el-icon :size="28"><VideoPlay /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-value">{{ runningCount }}</div>
            <div class="stat-label">运行中</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card stat-stopped">
          <div class="stat-icon">
            <el-icon :size="28"><VideoPause /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-value">{{ stoppedCount }}</div>
            <div class="stat-label">已停止</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card stat-error">
          <div class="stat-icon">
            <el-icon :size="28"><WarningFilled /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-value">{{ errorCount }}</div>
            <div class="stat-label">异常</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="stat-card stat-total">
          <div class="stat-icon">
            <el-icon :size="28"><DataLine /></el-icon>
          </div>
          <div class="stat-info">
            <div class="stat-value">{{ instances.length }}</div>
            <div class="stat-label">总实例</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- 实例列表 -->
    <el-card class="instances-card" shadow="never">
      <template #header>
        <div class="card-header">
          <span>运行实例</span>
          <el-radio-group v-model="viewMode" size="small">
            <el-radio-button label="card">卡片</el-radio-button>
            <el-radio-button label="list">列表</el-radio-button>
          </el-radio-group>
        </div>
      </template>

      <!-- 加载状态 -->
      <div v-if="loading" class="loading-container">
        <el-icon class="is-loading" :size="40"><Loading /></el-icon>
        <p>加载实例列表中...</p>
      </div>

      <!-- 空状态 -->
      <el-empty v-else-if="instances.length === 0" description="暂无策略实例">
        <el-button type="primary" @click="showStartDialog = true">启动第一个策略</el-button>
      </el-empty>

      <!-- 卡片视图 -->
      <div v-else-if="viewMode === 'card'" class="instances-grid">
        <div
          v-for="instance in instances"
          :key="instance.id"
          class="instance-card"
          :class="`instance-${instance.status.toLowerCase()}`"
        >
          <div class="instance-header">
            <div class="instance-name">{{ instance.name }}</div>
            <div class="instance-status">
              <span class="status-indicator" :class="`status-${instance.status.toLowerCase()}`"></span>
              <span class="status-text">{{ getStatusText(instance.status) }}</span>
            </div>
          </div>

          <div class="instance-id">
            <el-icon><Tickets /></el-icon>
            <span>{{ instance.id.slice(0, 16) }}...</span>
          </div>

          <div class="instance-body">
            <div class="instance-row">
              <span class="row-label">交易对</span>
              <div class="row-value symbols-wrapper">
                <el-tag
                  v-for="symbol in instance.symbols.slice(0, 3)"
                  :key="symbol"
                  size="small"
                  type="info"
                >
                  {{ symbol }}
                </el-tag>
                <span v-if="instance.symbols.length > 3" class="more-tags">
                  +{{ instance.symbols.length - 3 }}
                </span>
              </div>
            </div>

            <div class="instance-row">
              <span class="row-label">周期</span>
              <div class="row-value">
                <el-tag
                  v-for="timeframe in instance.timeframes.slice(0, 3)"
                  :key="timeframe"
                  size="small"
                  type="success"
                >
                  {{ timeframe }}
                </el-tag>
                <span v-if="instance.timeframes.length > 3" class="more-tags">
                  +{{ instance.timeframes.length - 3 }}
                </span>
              </div>
            </div>

            <div class="instance-row">
              <span class="row-label">运行时长</span>
              <span class="row-value">{{ formatRuntime(instance.startTime) }}</span>
            </div>

            <div class="instance-stats" v-if="instance.stats">
              <div class="mini-stat">
                <span class="mini-stat-label">交易次数</span>
                <span class="mini-stat-value">{{ instance.stats.tradeCount || 0 }}</span>
              </div>
              <div class="mini-stat">
                <span class="mini-stat-label">盈亏</span>
                <span class="mini-stat-value" :class="(instance.stats.pnl || 0) >= 0 ? 'profit' : 'loss'">
                  {{ (instance.stats.pnl || 0) >= 0 ? '+' : '' }}{{ instance.stats.pnl || 0 }}
                </span>
              </div>
            </div>
          </div>

          <div class="instance-footer">
            <el-button-group>
              <el-button size="small" @click="viewInstance(instance.id)">
                <el-icon><View /></el-icon>
                详情
              </el-button>
              <el-button
                v-if="instance.status === 'Running'"
                size="small"
                type="warning"
                @click="pauseInstance(instance.id)"
              >
                <el-icon><VideoPause /></el-icon>
                暂停
              </el-button>
              <el-button
                v-if="instance.status === 'Running' || instance.status === 'Paused'"
                size="small"
                type="danger"
                @click="stopInstance(instance.id)"
              >
                <el-icon><CircleCloseFilled /></el-icon>
                停止
              </el-button>
            </el-button-group>
          </div>
        </div>
      </div>

      <!-- 列表视图 -->
      <el-table v-else :data="instances" stripe class="instances-table">
        <el-table-column prop="name" label="策略名称" min-width="150" />
        <el-table-column prop="id" label="实例ID" min-width="200">
          <template #default="{ row }">
            <code class="instance-id-text">{{ row.id.slice(0, 20) }}...</code>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="120">
          <template #default="{ row }">
            <span class="status-badge" :class="`status-${row.status.toLowerCase()}`">
              <span class="status-dot"></span>
              {{ getStatusText(row.status) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="交易对" width="200">
          <template #default="{ row }">
            <el-tag
              v-for="symbol in row.symbols.slice(0, 2)"
              :key="symbol"
              size="small"
              style="margin-right: 4px"
            >
              {{ symbol }}
            </el-tag>
            <span v-if="row.symbols.length > 2" class="more-text">
              +{{ row.symbols.length - 2 }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="周期" width="150">
          <template #default="{ row }">
            <el-tag
              v-for="timeframe in row.timeframes.slice(0, 2)"
              :key="timeframe"
              size="small"
              type="success"
              style="margin-right: 4px"
            >
              {{ timeframe }}
            </el-tag>
            <span v-if="row.timeframes.length > 2" class="more-text">
              +{{ row.timeframes.length - 2 }}
            </span>
          </template>
        </el-table-column>
        <el-table-column label="运行时长" width="120">
          <template #default="{ row }">
            {{ formatRuntime(row.startTime) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button-group size="small">
              <el-button type="primary" link @click="viewInstance(row.id)">详情</el-button>
              <el-button
                v-if="row.status === 'Running'"
                type="warning"
                link
                @click="pauseInstance(row.id)"
              >
                暂停
              </el-button>
              <el-button
                v-if="row.status === 'Running' || row.status === 'Paused'"
                type="danger"
                link
                @click="stopInstance(row.id)"
              >
                停止
              </el-button>
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 启动策略对话框 -->
    <el-dialog
      v-model="showStartDialog"
      title="启动策略实例"
      width="600px"
      @close="resetForm"
    >
      <el-form :model="form" :rules="rules" ref="formRef" label-width="120px">
        <el-form-item label="策略名称" prop="name">
          <el-input v-model="form.name" placeholder="输入策略名称" />
        </el-form-item>

        <el-form-item label="策略代码" prop="code">
          <el-input
            v-model="form.code"
            type="textarea"
            :rows="8"
            placeholder="输入JavaScript策略代码"
          />
          <div class="form-hint">
            示例: function onBar(context, kline) { if (kline.close > kline.open) { return { action: 'buy', symbol: kline.symbol, quantity: 0.1, price: kline.close }; } return null; }
          </div>
        </el-form-item>

        <el-form-item label="策略参数" prop="parameters">
          <el-input
            v-model="form.parametersJson"
            type="textarea"
            :rows="3"
            placeholder='{"param1": "value1"}'
          />
        </el-form-item>

        <el-form-item label="订阅交易对" prop="symbols">
          <el-select
            v-model="form.symbols"
            multiple
            filterable
            allow-create
            placeholder="选择或输入交易对"
            style="width: 100%"
          >
            <el-option label="BTCUSDT" value="BTCUSDT" />
            <el-option label="ETHUSDT" value="ETHUSDT" />
            <el-option label="BNBUSDT" value="BNBUSDT" />
            <el-option label="SOLUSDT" value="SOLUSDT" />
          </el-select>
        </el-form-item>

        <el-form-item label="订阅周期" prop="timeframes">
          <el-select
            v-model="form.timeframes"
            multiple
            placeholder="选择K线周期"
            style="width: 100%"
          >
            <el-option label="1分钟" value="1m" />
            <el-option label="5分钟" value="5m" />
            <el-option label="15分钟" value="15m" />
            <el-option label="1小时" value="1h" />
            <el-option label="4小时" value="4h" />
            <el-option label="1天" value="1d" />
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="showStartDialog = false">取消</el-button>
        <el-button type="primary" @click="startInstance" :loading="starting">
          启动
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import {
  Plus,
  VideoPlay,
  VideoPause,
  CircleCloseFilled,
  WarningFilled,
  DataLine,
  Loading,
  View,
  Tickets,
} from '@element-plus/icons-vue';
import { strategyEngineApi } from '@/api/tauri';
import type { InstanceInfo, StrategyConfig } from '@/types';

const instances = ref<InstanceInfo[]>([]);
const loading = ref(false);
const showStartDialog = ref(false);
const starting = ref(false);
const viewMode = ref<'card' | 'list'>('card');
const formRef = ref<FormInstance>();

const form = reactive({
  name: '',
  code: '',
  parametersJson: '{}',
  symbols: [] as string[],
  timeframes: [] as string[],
});

const rules: FormRules = {
  name: [{ required: true, message: '请输入策略名称', trigger: 'blur' }],
  code: [{ required: true, message: '请输入策略代码', trigger: 'blur' }],
  symbols: [{ required: true, message: '请选择订阅交易对', trigger: 'change' }],
  timeframes: [{ required: true, message: '请选择订阅周期', trigger: 'change' }],
};

// 计算统计数据
const runningCount = computed(() => instances.value.filter(i => i.status === 'Running').length);
const stoppedCount = computed(() => instances.value.filter(i => i.status === 'Stopped').length);
const errorCount = computed(() => instances.value.filter(i => i.status === 'Error').length);

let refreshInterval: number | null = null;

// 获取实例列表
const loadInstances = async () => {
  loading.value = true;
  try {
    instances.value = await strategyEngineApi.list();
  } catch (error: any) {
    ElMessage.error('加载实例列表失败: ' + error.message);
  } finally {
    loading.value = false;
  }
};

// 启动策略实例
const startInstance = async () => {
  if (!formRef.value) return;

  await formRef.value.validate(async (valid) => {
    if (!valid) return;

    starting.value = true;
    try {
      let parameters: Record<string, any> = {};
      try {
        parameters = JSON.parse(form.parametersJson);
      } catch {
        ElMessage.error('策略参数JSON格式错误');
        starting.value = false;
        return;
      }

      const config: StrategyConfig = {
        name: form.name,
        code: form.code,
        parameters,
        symbols: form.symbols,
        timeframes: form.timeframes,
      };

      const instanceId = await strategyEngineApi.start(config);
      ElMessage.success(`策略实例已启动: ${instanceId}`);
      showStartDialog.value = false;
      resetForm();
      await loadInstances();
    } catch (error: any) {
      ElMessage.error('启动策略失败: ' + error.message);
    } finally {
      starting.value = false;
    }
  });
};

// 暂停策略实例
const pauseInstance = async (id: string) => {
  try {
    await strategyEngineApi.pause(id);
    ElMessage.success('策略实例已暂停');
    await loadInstances();
  } catch (error: any) {
    ElMessage.error('暂停策略失败: ' + error.message);
  }
};

// 停止策略实例
const stopInstance = async (id: string) => {
  try {
    await ElMessageBox.confirm('确认停止该策略实例？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });

    await strategyEngineApi.stop(id);
    ElMessage.success('策略实例已停止');
    await loadInstances();
  } catch (error: any) {
    if (error !== 'cancel') {
      ElMessage.error('停止策略失败: ' + error.message);
    }
  }
};

// 查看实例详情
const viewInstance = (id: string) => {
  ElMessage.info('实例详情功能开发中');
};

// 重置表单
const resetForm = () => {
  form.name = '';
  form.code = '';
  form.parametersJson = '{}';
  form.symbols = [];
  form.timeframes = [];
  formRef.value?.resetFields();
};

// 获取状态文本
const getStatusText = (status: string) => {
  const textMap: Record<string, string> = {
    Starting: '启动中',
    Running: '运行中',
    Paused: '已暂停',
    Stopping: '停止中',
    Stopped: '已停止',
    Error: '异常',
  };
  return textMap[status] || status;
};

// 格式化运行时长
const formatRuntime = (startTime?: string) => {
  if (!startTime) return '--';
  const start = new Date(startTime);
  const now = new Date();
  const diff = now.getTime() - start.getTime();

  const hours = Math.floor(diff / (1000 * 60 * 60));
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

  if (hours > 24) {
    const days = Math.floor(hours / 24);
    return `${days}天${hours % 24}小时`;
  }
  if (hours > 0) {
    return `${hours}小时${minutes}分钟`;
  }
  return `${minutes}分钟`;
};

onMounted(() => {
  loadInstances();
  // 每5秒刷新一次实例列表
  refreshInterval = window.setInterval(() => {
    loadInstances();
  }, 5000);
});

onUnmounted(() => {
  if (refreshInterval !== null) {
    clearInterval(refreshInterval);
  }
});
</script>

<style scoped lang="scss">
.strategy-instances {
  padding: 20px;
  min-height: calc(100vh - 60px);
  background: #f5f7fa;
}

// 页面头部
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
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

// 统计卡片
.stats-row {
  margin-bottom: 20px;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.3s;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
  }

  .stat-icon {
    width: 56px;
    height: 56px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #fff;
  }

  &.stat-running .stat-icon {
    background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
  }

  &.stat-stopped .stat-icon {
    background: linear-gradient(135deg, #909399 0%, #b3b8bd 100%);
  }

  &.stat-error .stat-icon {
    background: linear-gradient(135deg, #f56c6c 0%, #f78989 100%);
  }

  &.stat-total .stat-icon {
    background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%);
  }

  .stat-info {
    flex: 1;
  }

  .stat-value {
    font-size: 28px;
    font-weight: 700;
    color: #303133;
    line-height: 1;
    margin-bottom: 8px;
  }

  .stat-label {
    font-size: 13px;
    color: #909399;
  }
}

// 实例卡片
.instances-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: #909399;

  .el-icon {
    color: #409eff;
    margin-bottom: 16px;
  }
}

.instances-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
}

.instance-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  border: 2px solid transparent;
  transition: all 0.3s;

  &.instance-running {
    border-color: rgba(103, 194, 58, 0.2);
    background: linear-gradient(to bottom, #f0f9ff, #fff);
  }

  &.instance-error {
    border-color: rgba(245, 108, 108, 0.2);
    background: linear-gradient(to bottom, #fef0f0, #fff);
  }

  &.instance-stopped {
    border-color: #ebeef5;
  }
}

.instance-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.instance-name {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
}

.instance-status {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  animation: pulse 2s infinite;

  &.status-running {
    background: #67c23a;
  }

  &.status-stopped {
    background: #909399;
    animation: none;
  }

  &.status-error {
    background: #f56c6c;
  }

  &.status-paused {
    background: #e6a23c;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.status-text {
  font-size: 12px;
  font-weight: 500;
}

.instance-id {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: #f5f7fa;
  border-radius: 6px;
  margin-bottom: 16px;
  font-size: 12px;
  color: #606266;
  font-family: monospace;
}

.instance-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.instance-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.row-label {
  font-size: 13px;
  color: #909399;
}

.row-value {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: #303133;
}

.symbols-wrapper {
  flex-wrap: wrap;
  justify-content: flex-end;
}

.more-tags {
  font-size: 12px;
  color: #909399;
}

.instance-stats {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
}

.mini-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.mini-stat-label {
  font-size: 11px;
  color: #909399;
}

.mini-stat-value {
  font-size: 14px;
  font-weight: 600;
  color: #303133;

  &.profit {
    color: #ef5350;
  }

  &.loss {
    color: #26a69a;
  }
}

.instance-footer {
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}

// 状态徽章
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
  }

  &.status-running {
    background: rgba(103, 194, 58, 0.1);
    color: #67c23a;

    .status-dot {
      background: #67c23a;
    }
  }

  &.status-stopped {
    background: rgba(144, 147, 153, 0.1);
    color: #909399;

    .status-dot {
      background: #909399;
    }
  }

  &.status-error {
    background: rgba(245, 108, 108, 0.1);
    color: #f56c6c;

    .status-dot {
      background: #f56c6c;
    }
  }
}

.instance-id-text {
  font-family: monospace;
  font-size: 12px;
  color: #606266;
  background: #f5f7fa;
  padding: 2px 6px;
  border-radius: 4px;
}

.more-text {
  font-size: 12px;
  color: #909399;
}

.form-hint {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
  line-height: 1.4;
}
</style>
