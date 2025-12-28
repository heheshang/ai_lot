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
        <!-- 应用设置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><Setting /></el-icon>
            <span>应用设置</span>
          </div>

          <el-form-item label="语言" prop="app.language">
            <el-select v-model="formData.app.language" placeholder="请选择语言">
              <el-option label="简体中文" value="zh-CN" />
              <el-option label="English" value="en-US" />
            </el-select>
          </el-form-item>

          <el-form-item label="主题" prop="app.theme">
            <el-select v-model="formData.app.theme" placeholder="请选择主题">
              <el-option label="浅色" value="light" />
              <el-option label="深色" value="dark" />
              <el-option label="跟随系统" value="auto" />
            </el-select>
          </el-form-item>

          <el-form-item label="自动保存间隔" prop="app.auto_save_interval">
            <el-input-number
              v-model="formData.app.auto_save_interval"
              :min="10"
              :max="3600"
              :step="10"
              controls-position="right"
            />
            <span class="unit">秒</span>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 数据库配置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><DataBoard /></el-icon>
            <span>数据库配置</span>
          </div>

          <el-form-item label="数据库路径" prop="database.path">
            <el-input
              v-model="formData.database.path"
              placeholder="ai-lot.db"
              :disabled="true"
            >
              <template #append>
                <el-icon><InfoFilled /></el-icon>
              </template>
            </el-input>
            <div class="form-tip">数据库文件存储在应用数据目录中</div>
          </el-form-item>

          <el-form-item label="备份间隔" prop="database.backup_interval_hours">
            <el-input-number
              v-model="formData.database.backup_interval_hours"
              :min="1"
              :max="168"
              :step="1"
              controls-position="right"
            />
            <span class="unit">小时</span>
          </el-form-item>

          <el-form-item label="备份保留天数" prop="database.backup_retention_days">
            <el-input-number
              v-model="formData.database.backup_retention_days"
              :min="1"
              :max="365"
              :step="1"
              controls-position="right"
            />
            <span class="unit">天</span>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 风控配置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><Shield /></el-icon>
            <span>风控配置</span>
          </div>

          <el-form-item label="启用风控" prop="risk.enabled">
            <el-switch
              v-model="formData.risk.enabled"
              active-text="已启用"
              inactive-text="未启用"
            />
          </el-form-item>

          <el-form-item label="默认风控动作" prop="risk.default_action">
            <el-select v-model="formData.risk.default_action" placeholder="请选择默认动作">
              <el-option label="仅告警" value="alert" />
              <el-option label="平仓" value="close_position" />
              <el-option label="停止策略" value="stop_strategy" />
            </el-select>
            <div class="form-tip">触发风控规则时的默认处理方式</div>
          </el-form-item>
        </div>

        <el-divider />

        <!-- 通知配置 -->
        <div class="section">
          <div class="section-title">
            <el-icon><Bell /></el-icon>
            <span>通知配置</span>
          </div>

          <!-- 钉钉通知 -->
          <el-form-item label="钉钉 Webhook" prop="notifications.dingtalk_webhook">
            <el-input
              v-model="formData.notifications.dingtalk_webhook"
              placeholder="https://oapi.dingtalk.com/robot/send?access_token=xxx"
              type="password"
              show-password
              clearable
            />
            <div class="form-tip">留空则不启用钉钉通知</div>
          </el-form-item>

          <!-- 邮件通知 -->
          <el-form-item label="SMTP 服务器" prop="notifications.smtp_server">
            <el-input
              v-model="formData.notifications.smtp_server"
              placeholder="smtp.example.com"
              clearable
            />
          </el-form-item>

          <el-form-item label="SMTP 端口" prop="notifications.smtp_port">
            <el-input-number
              v-model="formData.notifications.smtp_port"
              :min="1"
              :max="65535"
              :step="1"
              controls-position="right"
              placeholder="587"
            />
          </el-form-item>

          <el-form-item label="SMTP 用户名" prop="notifications.smtp_username">
            <el-input
              v-model="formData.notifications.smtp_username"
              placeholder="user@example.com"
              clearable
            />
          </el-form-item>

          <el-form-item label="SMTP 密码" prop="notifications.smtp_password">
            <el-input
              v-model="formData.notifications.smtp_password"
              type="password"
              show-password
              placeholder="请输入SMTP密码"
              clearable
            />
          </el-form-item>

          <el-form-item label="通知邮箱" prop="notifications.notification_emails">
            <el-input
              v-model="formData.notifications.notification_emails"
              type="textarea"
              :rows="2"
              placeholder="email1@example.com, email2@example.com"
              clearable
            />
            <div class="form-tip">多个邮箱用逗号分隔，留空则不启用邮件通知</div>
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
  InfoFilled,
  Check,
  RefreshLeft,
  Refresh,
} from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import type { AppConfig } from '@/types/config';
import { validateConfig } from '@/types/config';

const router = useRouter();
const formRef = ref<FormInstance>();
const loading = ref(false);
const saving = ref(false);
const resetting = ref(false);

// 表单数据
const formData = reactive<AppConfig>({
  app: {
    language: 'zh-CN',
    theme: 'dark',
    auto_save_interval: 60,
  },
  database: {
    path: 'ai-lot.db',
    backup_interval_hours: 24,
    backup_retention_days: 30,
  },
  risk: {
    enabled: true,
    default_action: 'alert',
  },
  notifications: {
    dingtalk_webhook: '',
    smtp_server: '',
    smtp_port: undefined,
    smtp_username: '',
    smtp_password: '',
    notification_emails: '',
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
    const config = await invoke<AppConfig>('config_get');
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
  // 验证配置
  const errors: string[] = [];
  errors.push(...validateConfig('app', formData.app));
  errors.push(...validateConfig('database', formData.database));
  errors.push(...validateConfig('risk', formData.risk));
  errors.push(...validateConfig('notifications', formData.notifications));

  if (errors.length > 0) {
    ElMessage.error({
      message: `配置验证失败:\n${errors.join('\n')}`,
      duration: 5000,
    });
    return;
  }

  saving.value = true;
  try {
    const result = await invoke<AppConfig>('config_update', {
      updater: formData,
    });
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
      const result = await invoke<AppConfig>('config_reset');
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
