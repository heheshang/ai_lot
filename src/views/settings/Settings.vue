<template>
  <div class="settings-container">
    <el-page-header @back="goBack" class="header">
      <template #content>
        <span class="title">系统设置</span>
      </template>
    </el-page-header>

    <el-card v-loading="loading" class="settings-card">
      <el-form
        ref="formRef"
        :model="formData"
        label-width="180px"
        label-position="left"
        class="settings-form"
      >
        <!-- 主题与语言 -->
        <div class="section">
          <div class="section-title">
            <el-icon><Setting /></el-icon>
            <span>界面设置</span>
          </div>

          <el-form-item label="主题" prop="theme">
            <el-select v-model="formData.theme" placeholder="请选择主题">
              <el-option label="浅色" value="light" />
              <el-option label="深色" value="dark" />
              <el-option label="跟随系统" value="auto" />
            </el-select>
          </el-form-item>

          <el-form-item label="语言" prop="language">
            <el-select v-model="formData.language" placeholder="请选择语言">
              <el-option label="简体中文" value="zh-CN" />
              <el-option label="English" value="en-US" />
            </el-select>
          </el-form-item>

          <el-form-item label="时区" prop="timezone">
            <el-input
              v-model="formData.timezone"
              placeholder="Asia/Shanghai"
              clearable
            />
            <div class="form-tip">IANA 时区标识符，如 Asia/Shanghai</div>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 通知设置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><Bell /></el-icon>
            <span>通知设置</span>
          </div>

          <el-form-item label="启用通知" prop="notifications.enabled">
            <el-switch
              v-model="formData.notifications.enabled"
              active-text="已启用"
              inactive-text="未启用"
            />
          </el-form-item>

          <el-form-item label="通知方式" prop="notifications.methods">
            <el-checkbox-group v-model="formData.notifications.methods">
              <el-checkbox label="log">日志</el-checkbox>
              <el-checkbox label="dingtalk">钉钉</el-checkbox>
              <el-checkbox label="email">邮件</el-checkbox>
            </el-checkbox-group>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 交易设置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><DataBoard /></el-icon>
            <span>交易设置</span>
          </div>

          <el-form-item label="最大持仓数" prop="trading.max_positions">
            <el-input-number
              v-model="formData.trading.max_positions"
              :min="1"
              :max="100"
              :step="1"
              controls-position="right"
            />
            <div class="form-tip">同时持有的最大仓位数量</div>
          </el-form-item>

          <el-form-item label="最大持仓比例" prop="trading.max_position_ratio">
            <el-input-number
              v-model="formData.trading.max_position_ratio"
              :min="0.01"
              :max="1"
              :step="0.01"
              controls-position="right"
            />
            <div class="form-tip">单个仓位占账户净值的最大比例</div>
          </el-form-item>

          <el-form-item label="默认杠杆" prop="trading.default_leverage">
            <el-input-number
              v-model="formData.trading.default_leverage"
              :min="1"
              :max="125"
              :step="1"
              controls-position="right"
            />
            <div class="form-tip">新开仓位的默认杠杆倍数</div>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 风控设置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><DataBoard /></el-icon>
            <span>风控设置</span>
          </div>

          <el-form-item label="每日亏损限制" prop="risk.daily_loss_limit">
            <el-input-number
              v-model="formData.risk.daily_loss_limit"
              :min="0"
              :step="100"
              controls-position="right"
            />
            <span class="unit">USDT</span>
            <div class="form-tip">单日最大允许亏损金额</div>
          </el-form-item>

          <el-form-item label="最大回撤比例" prop="risk.max_drawdown_percent">
            <el-input-number
              v-model="formData.risk.max_drawdown_percent"
              :min="1"
              :max="100"
              :step="1"
              controls-position="right"
            />
            <span class="unit">%</span>
            <div class="form-tip">触发风控的最大回撤百分比</div>
          </el-form-item>
        </div>

        <!-- 操作按钮 -->
        <div class="actions">
          <el-button type="primary" @click="handleSave" :loading="saving">
            <el-icon><Check /></el-icon>
            保存设置
          </el-button>
          <el-button @click="handleReset" :loading="resetting">
            <el-icon><RefreshLeft /></el-icon>
            重置为默认
          </el-button>
          <el-button @click="handleReload" :loading="loading">
            <el-icon><Refresh /></el-icon>
            重新加载
          </el-button>
        </div>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus';
import {
  Setting,
  DataBoard,
  Bell,
  Check,
  RefreshLeft,
  Refresh,
} from '@element-plus/icons-vue';
import { configApi, type SystemConfig } from '@/api/tauri';

const router = useRouter();
const formRef = ref<FormInstance>();
const loading = ref(false);
const saving = ref(false);
const resetting = ref(false);

// 表单数据
const formData = reactive<SystemConfig>({
  theme: 'dark',
  language: 'zh-CN',
  timezone: 'Asia/Shanghai',
  notifications: {
    enabled: true,
    methods: ['log'],
  },
  trading: {
    max_positions: 5,
    max_position_ratio: 0.2,
    default_leverage: 1,
  },
  risk: {
    daily_loss_limit: 1000,
    max_drawdown_percent: 20,
  },
});

// 返回上一页
const goBack = () => {
  router.back();
};

// 加载配置
const loadConfig = async () => {
  loading.value = true;
  try {
    const config = await configApi.get();
    Object.assign(formData, config);
    ElMessage.success('配置加载成功');
  } catch (error) {
    ElMessage.error(`加载配置失败: ${error}`);
  } finally {
    loading.value = false;
  }
};

// 保存配置
const handleSave = async () => {
  saving.value = true;
  try {
    const result = await configApi.update(formData);
    Object.assign(formData, result);
    ElMessage.success('配置保存成功');
  } catch (error) {
    ElMessage.error(`保存配置失败: ${error}`);
  } finally {
    saving.value = false;
  }
};

// 重置为默认值
const handleReset = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要将所有配置重置为默认值吗？此操作不可撤销。',
      '确认重置',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    resetting.value = true;
    try {
      const result = await configApi.reset();
      Object.assign(formData, result);
      ElMessage.success('配置已重置为默认值');
    } catch (error) {
      ElMessage.error(`重置配置失败: ${error}`);
    } finally {
      resetting.value = false;
    }
  } catch {
    // 用户取消
  }
};

// 重新加载
const handleReload = () => {
  loadConfig();
};

// 组件挂载时加载配置
onMounted(() => {
  loadConfig();
});
</script>

<style scoped>
.settings-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
  overflow: auto;
}

.header {
  margin-bottom: 20px;
}

.title {
  font-size: 20px;
  font-weight: 600;
}

.settings-card {
  flex: 1;
  overflow: auto;
}

.settings-form {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px 0;
}

.section {
  margin-bottom: 20px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 20px;
  color: #303133;
}

.section-title .el-icon {
  font-size: 18px;
}

:deep(.el-form-item) {
  margin-bottom: 22px;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #606266;
}

.unit {
  margin-left: 10px;
  color: #909399;
  font-size: 14px;
}

.form-tip {
  margin-top: 4px;
  font-size: 12px;
  color: #909399;
  line-height: 1.5;
}

.actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid #ebeef5;
}

:deep(.el-input-number) {
  width: 200px;
}

:deep(.el-select) {
  width: 200px;
}

:deep(.el-input__inner) {
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
}

:deep(.el-textarea__inner) {
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
}

/* 暗色主题适配 */
.dark .section-title {
  color: #e5eaf3;
}

.dark .form-tip {
  color: #a3a6ad;
}

.dark :deep(.el-form-item__label) {
  color: #cfd3dc;
}
</style>
