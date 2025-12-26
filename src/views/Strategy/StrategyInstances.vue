<template>
  <div class="strategy-instances">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>策略实例管理</span>
          <el-button type="primary" @click="showStartDialog = true">
            <el-icon><Plus /></el-icon>
            启动策略
          </el-button>
        </div>
      </template>

      <!-- 实例列表 -->
      <el-table :data="instances" v-loading="loading" stripe>
        <el-table-column prop="id" label="实例ID" width="280" />
        <el-table-column prop="name" label="策略名称" width="200" />
        <el-table-column label="状态" width="120">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="订阅交易对" width="200">
          <template #default="{ row }">
            <el-tag
              v-for="symbol in row.symbols"
              :key="symbol"
              size="small"
              style="margin-right: 4px"
            >
              {{ symbol }}
            </el-tag>
            <span v-if="row.symbols.length === 0" class="text-muted">全部</span>
          </template>
        </el-table-column>
        <el-table-column label="订阅周期" width="200">
          <template #default="{ row }">
            <el-tag
              v-for="timeframe in row.timeframes"
              :key="timeframe"
              size="small"
              type="success"
              style="margin-right: 4px"
            >
              {{ timeframe }}
            </el-tag>
            <span v-if="row.timeframes.length === 0" class="text-muted">全部</span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'Running'"
              type="danger"
              size="small"
              @click="stopInstance(row.id)"
            >
              停止
            </el-button>
            <el-button
              v-else
              type="info"
              size="small"
              :disabled="row.status === 'Stopping' || row.status === 'Starting'"
            >
              {{ row.status === 'Stopped' ? '已停止' : '停止中' }}
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty v-if="!loading && instances.length === 0" description="暂无运行中的策略实例" />
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
          <div class="text-muted" style="margin-top: 4px">
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
import { ref, reactive, onMounted, onUnmounted } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { Plus } from '@element-plus/icons-vue';
import { strategyEngineApi } from '@/api/tauri';
import type { InstanceInfo, StrategyConfig } from '@/types';

const instances = ref<InstanceInfo[]>([]);
const loading = ref(false);
const showStartDialog = ref(false);
const starting = ref(false);
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

// 重置表单
const resetForm = () => {
  form.name = '';
  form.code = '';
  form.parametersJson = '{}';
  form.symbols = [];
  form.timeframes = [];
  formRef.value?.resetFields();
};

// 获取状态类型
const getStatusType = (status: string) => {
  const typeMap: Record<string, any> = {
    Starting: 'info',
    Running: 'success',
    Stopping: 'warning',
    Stopped: 'info',
    Error: 'danger',
  };
  return typeMap[status] || '';
};

// 获取状态文本
const getStatusText = (status: string) => {
  const textMap: Record<string, string> = {
    Starting: '启动中',
    Running: '运行中',
    Stopping: '停止中',
    Stopped: '已停止',
    Error: '错误',
  };
  return textMap[status] || status;
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

<style scoped>
.strategy-instances {
  height: 100%;
  padding: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.text-muted {
  color: #999;
  font-size: 12px;
}

.el-table {
  margin-top: 16px;
}

.el-form-item {
  margin-bottom: 18px;
}
</style>
