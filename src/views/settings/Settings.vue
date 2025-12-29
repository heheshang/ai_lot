<template>
  <div class="settings-container">
    <!-- Page Header -->
    <div class="page-header">
      <div class="header-content">
        <el-button @click="goBack" :icon="ArrowLeft" circle text />
        <div class="header-info">
          <h1 class="page-title">系统设置</h1>
          <p class="page-subtitle">应用程序全局配置</p>
        </div>
      </div>
      <div class="header-actions">
        <el-button :icon="Refresh" @click="handleReload" :loading="loading" circle />
        <el-button type="primary" :icon="Check" @click="handleSave" :loading="saving">
          保存设置
        </el-button>
      </div>
    </div>

    <!-- Theme Transition Overlay -->
    <transition name="theme-transition">
      <div v-if="isTransitioning" class="theme-transition-overlay" :class="`to-${nextTheme}`">
        <div class="transition-content">
          <el-icon class="transition-icon" :size="48">
            <Sunny v-if="nextTheme === 'light'" />
            <Moon v-else-if="nextTheme === 'dark'" />
            <Monitor v-else />
          </el-icon>
          <p class="transition-text">{{ getThemeLabel(nextTheme) }}模式</p>
        </div>
      </div>
    </transition>

    <!-- Settings Content -->
    <div v-loading="loading" class="settings-content" :class="{ 'is-transitioning': isTransitioning }">
      <!-- Appearance Settings -->
      <div class="settings-section">
        <div class="section-header">
          <div class="section-icon appearance-icon">
            <el-icon><Sunny /></el-icon>
          </div>
          <div class="section-info">
            <h3 class="section-title">界面设置</h3>
            <p class="section-description">主题和语言配置</p>
          </div>
        </div>

        <div class="settings-grid">
          <!-- Theme Selection -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Brush /></el-icon>
              <div class="label-text">
                <span class="label-name">主题模式</span>
              </div>
            </div>
            <div class="setting-control">
              <div class="theme-selector">
                <el-segmented v-model="formData.app.theme" :options="themeOptions" @change="handleThemeChange" :class="`theme-segmented-${formData.app.theme}`">
                  <template #default="{ item }">
                    <div class="theme-option" :class="`theme-option-${item.value}`">
                      <el-icon class="theme-icon">
                        <Sunny v-if="item.value === 'light'" />
                        <Moon v-else-if="item.value === 'dark'" />
                        <Monitor v-else />
                      </el-icon>
                      <span class="theme-label">{{ item.label }}</span>
                      <span v-if="formData.app.theme === item.value" class="theme-indicator" />
                    </div>
                  </template>
                </el-segmented>
                <div class="theme-preview" :class="`preview-${currentTheme}`">
                  <div class="preview-dot" />
                  <span class="preview-text">{{ getThemeLabel(currentTheme) }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Language Selection -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Location /></el-icon>
              <div class="label-text">
                <span class="label-name">语言</span>
              </div>
            </div>
            <div class="setting-control">
              <el-select v-model="formData.app.language" class="full-width-select">
                <el-option
                  v-for="lang in languageOptions"
                  :key="lang.value"
                  :label="lang.label"
                  :value="lang.value"
                >
                  <div class="language-option">
                    <span>{{ lang.icon }}</span>
                    <span>{{ lang.label }}</span>
                  </div>
                </el-option>
              </el-select>
            </div>
          </div>

          <!-- Auto Save Interval -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Timer /></el-icon>
              <div class="label-text">
                <span class="label-name">自动保存间隔</span>
                <span class="label-desc">数据自动保存的时间间隔（秒）</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input-number
                v-model="formData.app.auto_save_interval"
                :min="10"
                :max="600"
                :step="10"
                controls-position="right"
              />
              <span class="input-unit">秒</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Database Settings -->
      <div class="settings-section">
        <div class="section-header">
          <div class="section-icon database-icon">
            <el-icon><Files /></el-icon>
          </div>
          <div class="section-info">
            <h3 class="section-title">数据库设置</h3>
            <p class="section-description">数据存储和备份配置</p>
          </div>
        </div>

        <div class="settings-grid">
          <!-- Database Path -->
          <div class="setting-item full-width">
            <div class="setting-label">
              <el-icon><FolderOpened /></el-icon>
              <div class="label-text">
                <span class="label-name">数据库路径</span>
                <span class="label-desc">数据库文件存储路径</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input v-model="formData.database.path" readonly>
                <template #suffix>
                  <el-icon><Document /></el-icon>
                </template>
              </el-input>
            </div>
          </div>

          <!-- Backup Interval -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Clock /></el-icon>
              <div class="label-text">
                <span class="label-name">备份间隔</span>
                <span class="label-desc">自动备份的时间间隔</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input-number
                v-model="formData.database.backup_interval_hours"
                :min="1"
                :max="168"
                :step="1"
                controls-position="right"
              />
              <span class="input-unit">小时</span>
            </div>
          </div>

          <!-- Backup Retention -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Delete /></el-icon>
              <div class="label-text">
                <span class="label-name">备份保留</span>
                <span class="label-desc">备份文件保留天数</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input-number
                v-model="formData.database.backup_retention_days"
                :min="1"
                :max="365"
                :step="1"
                controls-position="right"
              />
              <span class="input-unit">天</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Risk Settings -->
      <div class="settings-section risk-section">
        <div class="section-header">
          <div class="section-icon risk-icon">
            <el-icon><Lock /></el-icon>
          </div>
          <div class="section-info">
            <h3 class="section-title">风控设置</h3>
            <p class="section-description">风险控制全局配置</p>
          </div>
        </div>

        <div class="settings-grid">
          <!-- Enable Risk -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Lock /></el-icon>
              <div class="label-text">
                <span class="label-name">启用风控</span>
                <span class="label-desc">全局风险控制系统</span>
              </div>
            </div>
            <div class="setting-control">
              <el-switch
                v-model="formData.risk.enabled"
                size="large"
                :active-icon="Check"
                :inactive-icon="Close"
                inline-prompt
                active-text="开"
                inactive-text="关"
              />
            </div>
          </div>

          <!-- Default Action -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Operation /></el-icon>
              <div class="label-text">
                <span class="label-name">默认动作</span>
                <span class="label-desc">触发风控后的默认行为</span>
              </div>
            </div>
            <div class="setting-control">
              <el-select v-model="formData.risk.default_action" class="full-width-select">
                <el-option label="仅警告" value="alert" />
                <el-option label="停止策略" value="stop_strategy" />
                <el-option label="平仓" value="close_positions" />
              </el-select>
            </div>
          </div>
        </div>
      </div>

      <!-- Notification Settings -->
      <div class="settings-section">
        <div class="section-header">
          <div class="section-icon notification-icon">
            <el-icon><Bell /></el-icon>
          </div>
          <div class="section-info">
            <h3 class="section-title">通知设置</h3>
            <p class="section-description">消息推送服务配置</p>
          </div>
        </div>

        <div class="settings-grid">
          <!-- DingTalk Webhook -->
          <div class="setting-item full-width">
            <div class="setting-label">
              <el-icon><ChatDotRound /></el-icon>
              <div class="label-text">
                <span class="label-name">钉钉 Webhook</span>
                <span class="label-desc">钉钉机器人推送地址</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input
                v-model="formData.notifications.dingtalk_webhook"
                placeholder="https://oapi.dingtalk.com/robot/send..."
                clearable
              />
            </div>
          </div>

          <!-- SMTP Server -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Message /></el-icon>
              <div class="label-text">
                <span class="label-name">SMTP 服务器</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input
                v-model="formData.notifications.smtp_server"
                placeholder="smtp.example.com"
                clearable
              />
            </div>
          </div>

          <!-- SMTP Port -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Connection /></el-icon>
              <div class="label-text">
                <span class="label-name">SMTP 端口</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input-number
                v-model="formData.notifications.smtp_port"
                :min="1"
                :max="65535"
                controls-position="right"
              />
            </div>
          </div>

          <!-- SMTP Username -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><User /></el-icon>
              <div class="label-text">
                <span class="label-name">SMTP 用户名</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input
                v-model="formData.notifications.smtp_username"
                placeholder="user@example.com"
                clearable
              />
            </div>
          </div>

          <!-- SMTP Password -->
          <div class="setting-item">
            <div class="setting-label">
              <el-icon><Lock /></el-icon>
              <div class="label-text">
                <span class="label-name">SMTP 密码</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input
                v-model="formData.notifications.smtp_password"
                type="password"
                placeholder="••••••••"
                show-password
                clearable
              />
            </div>
          </div>

          <!-- Notification Emails -->
          <div class="setting-item full-width">
            <div class="setting-label">
              <el-icon><Message /></el-icon>
              <div class="label-text">
                <span class="label-name">通知邮箱</span>
                <span class="label-desc">接收通知的邮箱列表（逗号分隔）</span>
              </div>
            </div>
            <div class="setting-control">
              <el-input
                v-model="formData.notifications.notification_emails"
                placeholder="user1@example.com, user2@example.com"
                clearable
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Save Indicator -->
      <transition name="fade">
        <div v-if="hasChanges" class="save-indicator">
          <el-icon class="pulse-icon"><Warning /></el-icon>
          <span>您有未保存的更改</span>
          <el-button type="primary" size="small" @click="handleSave" :loading="saving">
            立即保存
          </el-button>
        </div>
      </transition>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  ArrowLeft,
  Check,
  Close,
  Refresh,
  Sunny,
  Moon,
  Monitor,
  Brush,
  Location,
  Timer,
  Files,
  FolderOpened,
  Document,
  Clock,
  Delete,
  Lock,
  Operation,
  Bell,
  ChatDotRound,
  Message,
  Connection,
  User,
  Warning,
} from '@element-plus/icons-vue';
import { configApi, type AppConfig } from '@/api/tauri';

const router = useRouter();
const loading = ref(false);
const saving = ref(false);

// Theme transition state
const isTransitioning = ref(false);
const nextTheme = ref('');

// Current theme for preview
const currentTheme = computed(() => formData.app.theme);

// Get theme label for display
const getThemeLabel = (theme: string) => {
  const option = themeOptions.find(t => t.value === theme);
  return option ? option.label : theme;
};

// Original config for change detection
const originalConfig = ref<AppConfig | null>(null);

// Theme options
const themeOptions = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
  { label: '跟随系统', value: 'auto' },
];

// Language options
const languageOptions = [
  { label: '简体中文', value: 'zh-CN', icon: '🇨🇳' },
  { label: 'English', value: 'en-US', icon: '🇺🇸' },
];

// Form data
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
    smtp_port: 587,
    smtp_username: '',
    smtp_password: '',
    notification_emails: '',
  },
});

// Apply theme to HTML element with smooth transition
const applyTheme = (theme: string, withTransition = false) => {
  const html = document.documentElement;
  const body = document.body;

  // Add transition class for smooth theme change
  if (withTransition) {
    html.style.setProperty('--theme-transition', 'all 0.4s cubic-bezier(0.4, 0, 0.2, 1)');
    body.classList.add('theme-transitioning');
  }

  // Apply dark class
  if (theme === 'dark') {
    html.classList.add('dark');
  } else if (theme === 'light') {
    html.classList.remove('dark');
  } else {
    // Auto - follow system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    if (prefersDark) {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
  }

  // Remove transition class after animation
  if (withTransition) {
    setTimeout(() => {
      html.style.removeProperty('--theme-transition');
      body.classList.remove('theme-transitioning');
    }, 400);
  }
};

// Check if there are unsaved changes
const hasChanges = computed(() => {
  if (!originalConfig.value) return false;
  return JSON.stringify(formData) !== JSON.stringify(originalConfig.value);
});

// Handle theme change with beautiful transition
const handleThemeChange = async (theme: string) => {
  const previousTheme = formData.app.theme;

  // Skip if same theme
  if (previousTheme === theme) return;

  // Start transition animation
  isTransitioning.value = true;
  nextTheme.value = theme;

  // Wait for the overlay to appear (fade in)
  await new Promise(resolve => setTimeout(resolve, 200));

  // Apply the new theme
  applyTheme(theme, true);

  // Wait a bit for the theme to apply
  await new Promise(resolve => setTimeout(resolve, 300));

  // Fade out the overlay
  isTransitioning.value = false;

  // Show success message after transition
  setTimeout(() => {
    ElMessage.success({
      message: `主题已切换为${getThemeLabel(theme)}`,
      icon: theme === 'dark' ? Moon : (theme === 'light' ? Sunny : Monitor),
      duration: 2000,
    });
  }, 200);
};

// Return to previous page
const goBack = () => {
  if (hasChanges.value) {
    ElMessageBox.confirm(
      '您有未保存的更改，确定要离开吗？',
      '确认',
      {
        confirmButtonText: '离开',
        cancelButtonText: '取消',
        type: 'warning',
      }
    ).then(() => {
      router.back();
    }).catch(() => {
      // User cancelled
    });
  } else {
    router.back();
  }
};

// Load configuration
const loadConfig = async () => {
  loading.value = true;
  try {
    const config = await configApi.get();
    Object.assign(formData, config);
    originalConfig.value = JSON.parse(JSON.stringify(config));

    // Apply theme after loading
    applyTheme(config.app.theme);
  } catch (error) {
    ElMessage.error(`加载配置失败: ${error}`);
  } finally {
    loading.value = false;
  }
};

// Save configuration
const handleSave = async () => {
  saving.value = true;
  try {
    // Build updater object with only changed fields
    const updater: Record<string, any> = {};

    // Only include sections that have changes
    if (formData.app.language !== originalConfig.value?.app.language ||
        formData.app.theme !== originalConfig.value?.app.theme ||
        formData.app.auto_save_interval !== originalConfig.value?.app.auto_save_interval) {
      updater.app = {
        language: formData.app.language,
        theme: formData.app.theme,
        auto_save_interval: formData.app.auto_save_interval,
      };
    }

    if (formData.database.path !== originalConfig.value?.database.path ||
        formData.database.backup_interval_hours !== originalConfig.value?.database.backup_interval_hours ||
        formData.database.backup_retention_days !== originalConfig.value?.database.backup_retention_days) {
      updater.database = {
        path: formData.database.path,
        backup_interval_hours: formData.database.backup_interval_hours,
        backup_retention_days: formData.database.backup_retention_days,
      };
    }

    if (formData.risk.enabled !== originalConfig.value?.risk.enabled ||
        formData.risk.default_action !== originalConfig.value?.risk.default_action) {
      updater.risk = {
        enabled: formData.risk.enabled,
        default_action: formData.risk.default_action,
      };
    }

    // Handle notifications (with optional fields)
    const notif = formData.notifications;
    const origNotif = originalConfig.value?.notifications || {};
    if (JSON.stringify(notif) !== JSON.stringify(origNotif)) {
      updater.notifications = {};
      if (notif.dingtalk_webhook !== undefined) updater.notifications.dingtalk_webhook = notif.dingtalk_webhook || '';
      if (notif.smtp_server !== undefined) updater.notifications.smtp_server = notif.smtp_server || '';
      if (notif.smtp_port !== undefined) updater.notifications.smtp_port = notif.smtp_port;
      if (notif.smtp_username !== undefined) updater.notifications.smtp_username = notif.smtp_username || '';
      if (notif.smtp_password !== undefined) updater.notifications.smtp_password = notif.smtp_password || '';
      if (notif.notification_emails !== undefined) updater.notifications.notification_emails = notif.notification_emails || '';
    }

    const result = await configApi.update(updater);
    Object.assign(formData, result);
    originalConfig.value = JSON.parse(JSON.stringify(result));
    ElMessage.success('配置保存成功');
  } catch (error) {
    ElMessage.error(`保存配置失败: ${error}`);
  } finally {
    saving.value = false;
  }
};

// Reload configuration
const handleReload = () => {
  loadConfig();
};

// Lifecycle
onMounted(() => {
  loadConfig();
});
</script>

<style scoped lang="scss">
.settings-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Page Header */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  background: white;
  border-bottom: 1px solid #ebeef5;
  flex-shrink: 0;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.page-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #303133;
}

.page-subtitle {
  margin: 0;
  font-size: 13px;
  color: #909399;
}

.header-actions {
  display: flex;
  gap: 12px;
}

/* Settings Content */
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* Settings Section */
.settings-section {
  background: white;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;

  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  }
}

.section-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid #f5f7fa;
}

.section-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  color: white;
}

.appearance-icon {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.database-icon {
  background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
}

.risk-icon {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.notification-icon {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.section-info {
  flex: 1;
}

.section-title {
  margin: 0 0 4px 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
}

.section-description {
  margin: 0;
  font-size: 13px;
  color: #909399;
}

/* Settings Grid */
.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #ebeef5;

  &.full-width {
    grid-column: 1 / -1;
  }
}

.setting-label {
  display: flex;
  align-items: center;
  gap: 10px;

  .el-icon {
    font-size: 18px;
    color: #409eff;
  }
}

.label-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.label-name {
  font-weight: 500;
  color: #303133;
  font-size: 14px;
}

.label-desc {
  font-size: 12px;
  color: #909399;
}

.setting-control {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* Theme Option */
.theme-option {
  display: flex;
  align-items: center;
  gap: 6px;
}

/* Language Option */
.language-option {
  display: flex;
  align-items: center;
  gap: 8px;
}

.full-width-select {
  width: 100%;
}

.input-unit {
  margin-left: 8px;
  font-size: 14px;
  color: #606266;
  font-weight: 500;
}

/* Risk Section */
.risk-section {
  border-left: 4px solid #fa709a;
}

/* Save Indicator */
.save-indicator {
  position: fixed;
  bottom: 24px;
  right: 24px;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}

.pulse-icon {
  color: #e6a23c;
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

/* ============================================
   Theme Selector & Preview Styles
   ============================================ */
.theme-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
  width: 100%;

  :deep(.el-segmented) {
    width: 100%;
    padding: 4px;
    background: linear-gradient(135deg, #f5f7fa 0%, #e8ecf1 100%);
    border-radius: 12px;
    transition: all 0.3s ease;

    &.theme-segmented-light {
      background: linear-gradient(135deg, #fff9e6 0%, #fff3cd 100%);
      box-shadow: 0 2px 8px rgba(255, 193, 7, 0.2);
    }

    &.theme-segmented-dark {
      background: linear-gradient(135deg, #e8eaf6 0%, #c5cae9 100%);
      box-shadow: 0 2px 8px rgba(103, 58, 183, 0.2);
    }

    &.theme-segmented-auto {
      background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%);
      box-shadow: 0 2px 8px rgba(33, 150, 243, 0.2);
    }
  }

  :deep(.el-segmented__item) {
    padding: 8px 16px;
    border-radius: 8px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;

    &.is-selected {
      background: white;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    &:hover:not(.is-selected) {
      background: rgba(255, 255, 255, 0.6);
    }
  }
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 8px;
  position: relative;

  .theme-icon {
    font-size: 18px;
    transition: all 0.3s ease;

    .theme-option-light & {
      color: #f59e0b;
      animation: sun-glow 2s ease-in-out infinite;
    }

    .theme-option-dark & {
      color: #8b5cf6;
      animation: moon-glow 3s ease-in-out infinite;
    }

    .theme-option-auto & {
      color: #3b82f6;
      animation: monitor-pulse 2s ease-in-out infinite;
    }
  }

  .theme-label {
    font-weight: 500;
    font-size: 14px;
  }

  .theme-indicator {
    position: absolute;
    right: -4px;
    top: 50%;
    transform: translateY(-50%);
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
    animation: indicator-pulse 1.5s ease-in-out infinite;
  }
}

@keyframes sun-glow {
  0%, 100% {
    filter: drop-shadow(0 0 2px rgba(245, 158, 11, 0.5));
    transform: scale(1);
  }
  50% {
    filter: drop-shadow(0 0 8px rgba(245, 158, 11, 0.8));
    transform: scale(1.1);
  }
}

@keyframes moon-glow {
  0%, 100% {
    filter: drop-shadow(0 0 2px rgba(139, 92, 246, 0.5));
    transform: rotate(0deg);
  }
  50% {
    filter: drop-shadow(0 0 8px rgba(139, 92, 246, 0.8));
    transform: rotate(15deg);
  }
}

@keyframes monitor-pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}

@keyframes indicator-pulse {
  0%, 100% {
    transform: translateY(-50%) scale(1);
    opacity: 1;
  }
  50% {
    transform: translateY(-50%) scale(1.2);
    opacity: 0.8;
  }
}

.theme-preview {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 500;
  transition: all 0.3s ease;
  background: white;
  border: 1px solid #e5e7eb;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);

  &.preview-light {
    background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
    border-color: #fbbf24;
    color: #92400e;

    .preview-dot {
      background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
      box-shadow: 0 0 8px rgba(251, 191, 36, 0.5);
    }
  }

  &.preview-dark {
    background: linear-gradient(135deg, #ede9fe 0%, #ddd6fe 100%);
    border-color: #8b5cf6;
    color: #5b21b6;

    .preview-dot {
      background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
      box-shadow: 0 0 8px rgba(139, 92, 246, 0.5);
    }
  }

  &.preview-auto {
    background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
    border-color: #3b82f6;
    color: #1e40af;

    .preview-dot {
      background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
      box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
    }
  }
}

.preview-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.preview-text {
  flex: 1;
}

/* ============================================
   Theme Transition Overlay
   ============================================ */
.theme-transition-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;

  &.to-light {
    background: radial-gradient(circle at center, rgba(255, 255, 255, 0.95) 0%, rgba(255, 249, 230, 0.9) 100%);
    animation: light-sweep 0.6s ease-out forwards;
  }

  &.to-dark {
    background: radial-gradient(circle at center, rgba(17, 24, 39, 0.95) 0%, rgba(31, 41, 55, 0.9) 100%);
    animation: dark-sweep 0.6s ease-out forwards;
  }

  &.to-auto {
    background: radial-gradient(circle at center, rgba(219, 234, 254, 0.95) 0%, rgba(191, 219, 254, 0.9) 100%);
    animation: auto-sweep 0.6s ease-out forwards;
  }
}

@keyframes light-sweep {
  0% {
    opacity: 0;
    clip-path: circle(0% at center);
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0;
    clip-path: circle(150% at center);
  }
}

@keyframes dark-sweep {
  0% {
    opacity: 0;
    clip-path: circle(0% at center);
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0;
    clip-path: circle(150% at center);
  }
}

@keyframes auto-sweep {
  0% {
    opacity: 0;
    clip-path: circle(0% at center);
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0;
    clip-path: circle(150% at center);
  }
}

.transition-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  animation: content-scale 0.6s ease-out forwards;
}

@keyframes content-scale {
  0% {
    transform: scale(0.5);
    opacity: 0;
  }
  50% {
    transform: scale(1.1);
    opacity: 1;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.transition-icon {
  animation: icon-rotate 0.8s ease-in-out forwards;

  .to-light & {
    color: #fbbf24;
  }

  .to-dark & {
    color: #8b5cf6;
  }

  .to-auto & {
    color: #3b82f6;
  }
}

@keyframes icon-rotate {
  0% {
    transform: rotate(-180deg) scale(0);
    opacity: 0;
  }
  60% {
    transform: rotate(10deg) scale(1.2);
    opacity: 1;
  }
  80% {
    transform: rotate(-5deg) scale(0.95);
  }
  100% {
    transform: rotate(0deg) scale(1);
    opacity: 1;
  }
}

.transition-text {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: 1px;
  animation: text-fade 0.6s ease-out 0.2s forwards;
  opacity: 0;

  .to-light & {
    color: #92400e;
  }

  .to-dark & {
    color: #e5e7eb;
  }

  .to-auto & {
    color: #1e40af;
  }
}

@keyframes text-fade {
  to {
    opacity: 1;
  }
}

.theme-transition-enter-active,
.theme-transition-leave-active {
  transition: all 0.3s ease;
}

.theme-transition-enter-from,
.theme-transition-leave-to {
  opacity: 0;
}

/* Theme transition for all elements */
body.theme-transitioning,
body.theme-transitioning * {
  transition: background-color 0.4s cubic-bezier(0.4, 0, 0.2, 1) !important,
              color 0.4s cubic-bezier(0.4, 0, 0.2, 1) !important,
              border-color 0.4s cubic-bezier(0.4, 0, 0.2, 1) !important,
              box-shadow 0.4s cubic-bezier(0.4, 0, 0.2, 1) !important;
}

.settings-content.is-transitioning {
  filter: blur(2px);
  transition: filter 0.3s ease;
}

/* Dark Theme */
.dark {
  .page-header {
    background: #1f2937;
    border-bottom-color: #374151;
  }

  .page-title {
    color: #e5e7eb;
  }

  .page-subtitle {
    color: #9ca3af;
  }

  .settings-section {
    background: #1f2937;
    border: 1px solid #374151;
  }

  .section-header {
    border-bottom-color: #374151;
  }

  .section-title {
    color: #e5e7eb;
  }

  .section-description {
    color: #9ca3af;
  }

  .setting-item {
    background: #374151;
    border-color: #4b5563;
  }

  .label-name {
    color: #e5e7eb;
  }

  .label-desc {
    color: #9ca3af;
  }

  .save-indicator {
    background: #1f2937;
  }

  /* Theme Selector in Dark Mode */
  .theme-selector {
    :deep(.el-segmented) {
      background: linear-gradient(135deg, #374151 0%, #4b5563 100%);

      &.theme-segmented-light {
        background: linear-gradient(135deg, #4b5563 0%, #6b7280 100%);
        box-shadow: 0 2px 8px rgba(255, 193, 7, 0.15);
      }

      &.theme-segmented-dark {
        background: linear-gradient(135deg, #312e81 0%, #3730a3 100%);
        box-shadow: 0 2px 8px rgba(139, 92, 246, 0.3);
      }

      &.theme-segmented-auto {
        background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
        box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
      }
    }

    :deep(.el-segmented__item) {
      &.is-selected {
        background: #4b5563;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
      }

      &:hover:not(.is-selected) {
        background: rgba(75, 85, 99, 0.6);
      }
    }
  }

  .theme-preview {
    background: #374151;
    border-color: #4b5563;
    color: #d1d5db;

    &.preview-light {
      background: linear-gradient(135deg, #78716c 0%, #a8a29e 100%);
      border-color: #fbbf24;
      color: #fef3c7;
    }

    &.preview-dark {
      background: linear-gradient(135deg, #312e81 0%, #3730a3 100%);
      border-color: #8b5cf6;
      color: #ddd6fe;
    }

    &.preview-auto {
      background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
      border-color: #3b82f6;
      color: #dbeafe;
    }
  }
}

/* Responsive */
@media (max-width: 768px) {
  .settings-content {
    padding: 16px;
  }

  .settings-grid {
    grid-template-columns: 1fr;
  }

  .page-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    justify-content: flex-end;
  }
}
</style>
