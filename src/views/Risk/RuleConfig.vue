<template>
  <div class="rule-config">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">风控规则配置</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>风控管理</el-breadcrumb-item>
          <el-breadcrumb-item>规则配置</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <el-icon class="is-loading" :size="40"><Loading /></el-icon>
      <p>加载风控规则中...</p>
    </div>

    <!-- 规则列表 -->
    <div v-else class="rules-container">
      <el-row :gutter="20">
        <el-col
          v-for="rule in rules"
          :key="rule.name"
          :span="24"
        >
          <el-card class="rule-card" shadow="hover">
            <!-- 规则头部 -->
            <template #header>
              <div class="rule-card-header">
                <div class="rule-title">
                  <el-icon :size="20" class="rule-icon">
                    <Lock />
                  </el-icon>
                  <span class="rule-name">{{ rule.display_name }}</span>
                  <el-tag
                    :type="rule.config.enabled ? 'success' : 'info'"
                    size="small"
                    class="rule-status-tag"
                  >
                    {{ rule.config.enabled ? '已启用' : '已禁用' }}
                  </el-tag>
                </div>
                <el-switch
                  v-model="rule.config.enabled"
                  @change="handleToggleRule(rule)"
                  :loading="rule.updating"
                />
              </div>
            </template>

            <!-- 规则描述 -->
            <div class="rule-description">
              <el-text type="info">{{ rule.description }}</el-text>
            </div>

            <!-- 参数配置表单 -->
            <el-form
              :model="rule.config"
              :rules="getValidationRules(rule.rule_type)"
              ref="ruleFormsRef"
              label-width="180px"
              class="rule-form"
            >
              <!-- 仓位限制规则参数 -->
              <template v-if="rule.rule_type === 'position_limit'">
                <el-divider content-position="left">参数配置</el-divider>
                <el-row :gutter="16">
                  <el-col :span="8">
                    <el-form-item label="单个仓位最大价值" prop="max_position_value">
                      <el-input-number
                        v-model="rule.config.params.max_position_value"
                        :min="100"
                        :max="1000000"
                        :step="100"
                        :precision="2"
                        controls-position="right"
                        class="full-width"
                      />
                      <template #suffix>
                        <span class="input-suffix">USDT</span>
                      </template>
                    </el-form-item>
                  </el-col>
                  <el-col :span="8">
                    <el-form-item label="总仓位最大价值" prop="max_total_value">
                      <el-input-number
                        v-model="rule.config.params.max_total_value"
                        :min="1000"
                        :max="5000000"
                        :step="1000"
                        :precision="2"
                        controls-position="right"
                        class="full-width"
                      />
                      <template #suffix>
                        <span class="input-suffix">USDT</span>
                      </template>
                    </el-form-item>
                  </el-col>
                  <el-col :span="8">
                    <el-form-item label="单方向最大比例" prop="max_direction_ratio">
                      <el-input-number
                        v-model="rule.config.params.max_direction_ratio"
                        :min="0.1"
                        :max="1.0"
                        :step="0.05"
                        :precision="2"
                        controls-position="right"
                        class="full-width"
                      />
                      <template #suffix>
                        <span class="input-suffix">%</span>
                      </template>
                    </el-form-item>
                  </el-col>
                </el-row>
              </template>

              <!-- 回撤限制规则参数 -->
              <template v-if="rule.rule_type === 'drawdown_limit'">
                <el-divider content-position="left">参数配置</el-divider>
                <el-row :gutter="16">
                  <el-col :span="8">
                    <el-form-item label="最大回撤百分比" prop="max_drawdown_pct">
                      <el-input-number
                        v-model="rule.config.params.max_drawdown_pct"
                        :min="1"
                        :max="100"
                        :step="1"
                        :precision="2"
                        controls-position="right"
                        class="full-width"
                      />
                      <template #suffix>
                        <span class="input-suffix">%</span>
                      </template>
                    </el-form-item>
                  </el-col>
                </el-row>
              </template>

              <!-- 触发动作配置 -->
              <el-divider content-position="left">触发动作</el-divider>
              <el-row :gutter="16">
                <el-col :span="12">
                  <el-form-item label="触发后动作">
                    <el-select
                      v-model="rule.config.action"
                      placeholder="选择触发后的动作"
                      class="full-width"
                    >
                      <el-option
                        label="警告"
                        value="warning"
                      >
                        <div class="action-option">
                          <el-icon><Warning /></el-icon>
                          <span>警告 - 仅发送通知提醒</span>
                        </div>
                      </el-option>
                      <el-option
                        label="停止策略"
                        value="stop_strategy"
                      >
                        <div class="action-option">
                          <el-icon><VideoPause /></el-icon>
                          <span>停止策略 - 暂停策略运行，保留仓位</span>
                        </div>
                      </el-option>
                      <el-option
                        label="紧急平仓"
                        value="emergency_close"
                      >
                        <div class="action-option">
                          <el-icon><CircleClose /></el-icon>
                          <span>紧急平仓 - 立即平掉所有仓位</span>
                        </div>
                      </el-option>
                    </el-select>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="通知方式">
                    <el-checkbox-group v-model="rule.config.notify_methods">
                      <el-checkbox label="dingtalk">
                        <div class="notify-option">
                          <el-icon><ChatDotRound /></el-icon>
                          <span>钉钉通知</span>
                        </div>
                      </el-checkbox>
                      <el-checkbox label="email">
                        <div class="notify-option">
                          <el-icon><Message /></el-icon>
                          <span>邮件通知</span>
                        </div>
                      </el-checkbox>
                    </el-checkbox-group>
                  </el-form-item>
                </el-col>
              </el-row>

              <!-- 操作按钮 -->
              <el-row>
                <el-col :span="24" class="form-actions">
                  <el-button
                    type="primary"
                    @click="handleSaveRule(rule)"
                    :loading="rule.saving"
                  >
                    <el-icon><Check /></el-icon>
                    保存配置
                  </el-button>
                  <el-button @click="handleResetRule(rule)">
                    <el-icon><RefreshLeft /></el-icon>
                    重置
                  </el-button>
                </el-col>
              </el-row>
            </el-form>
          </el-card>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance } from 'element-plus';
import {
  Loading,
  Warning,
  VideoPause,
  CircleClose,
  Check,
  RefreshLeft,
  ChatDotRound,
  Message,
  Lock,
} from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import type {
  RiskRuleListItem,
  RiskRuleConfig,
} from '@/types/risk';

// 扩展的类型以支持 UI 状态
interface RuleWithState extends RiskRuleListItem {
  updating?: boolean;
  saving?: boolean;
  originalConfig?: RiskRuleConfig;
}

// 响应式状态
const loading = ref(true);
const rules = ref<RuleWithState[]>([]);
const ruleFormsRef = ref<FormInstance[]>([]);

// 表单验证规则
const getValidationRules = (ruleType: string) => {
  const rules: Record<string, any> = {};

  if (ruleType === 'position_limit') {
    rules.max_position_value = [
      { required: true, message: '请输入单个仓位最大价值', trigger: 'blur' },
      { type: 'number', min: 100, message: '最小值为 100 USDT', trigger: 'blur' },
    ];
    rules.max_total_value = [
      { required: true, message: '请输入总仓位最大价值', trigger: 'blur' },
      { type: 'number', min: 1000, message: '最小值为 1000 USDT', trigger: 'blur' },
    ];
    rules.max_direction_ratio = [
      { required: true, message: '请输入单方向最大比例', trigger: 'blur' },
      { type: 'number', min: 0.1, max: 1.0, message: '比例必须在 0.1 - 1.0 之间', trigger: 'blur' },
    ];
  } else if (ruleType === 'drawdown_limit') {
    rules.max_drawdown_pct = [
      { required: true, message: '请输入最大回撤百分比', trigger: 'blur' },
      { type: 'number', min: 1, max: 100, message: '回撤必须在 1 - 100 之间', trigger: 'blur' },
    ];
  }

  return rules;
};

// 加载风控规则列表
const loadRules = async () => {
  try {
    loading.value = true;
    const result = await invoke<RiskRuleListItem[]>('get_risk_rules');

    // 为每个规则添加 UI 状态并保存原始配置
    rules.value = result.map((rule) => ({
      ...rule,
      updating: false,
      saving: false,
      originalConfig: JSON.parse(JSON.stringify(rule.config)),
    }));
  } catch (error) {
    console.error('Failed to load risk rules:', error);
    ElMessage.error('加载风控规则失败');
  } finally {
    loading.value = false;
  }
};

// 切换规则启用状态
const handleToggleRule = async (rule: RuleWithState) => {
  try {
    rule.updating = true;
    await updateRule(rule.name, rule.config);
    ElMessage.success(`规则 ${rule.display_name} 已${rule.config.enabled ? '启用' : '禁用'}`);
  } catch (error: any) {
    console.error('Failed to toggle rule:', error);
    // 回滚状态
    rule.config.enabled = !rule.config.enabled;
    ElMessage.error(error || '更新规则状态失败');
  } finally {
    rule.updating = false;
  }
};

// 保存规则配置
const handleSaveRule = async (rule: RuleWithState) => {
  try {
    await ElMessageBox.confirm(
      `确认保存 ${rule.display_name} 的配置吗？`,
      '保存确认',
      {
        confirmButtonText: '保存',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    rule.saving = true;
    await updateRule(rule.name, rule.config);

    // 更新原始配置
    rule.originalConfig = JSON.parse(JSON.stringify(rule.config));

    ElMessage.success(`规则 ${rule.display_name} 配置已保存`);
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Failed to save rule:', error);
      ElMessage.error(error || '保存规则配置失败');
    }
  } finally {
    rule.saving = false;
  }
};

// 重置规则配置
const handleResetRule = async (rule: RuleWithState) => {
  try {
    await ElMessageBox.confirm(
      '确认重置为上次保存的配置吗？当前修改将丢失。',
      '重置确认',
      {
        confirmButtonText: '重置',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    // 恢复原始配置
    rule.config = JSON.parse(JSON.stringify(rule.originalConfig));
    ElMessage.success('规则配置已重置');
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to reset rule:', error);
      ElMessage.error('重置规则配置失败');
    }
  }
};

// 调用 Tauri 命令更新规则
const updateRule = async (ruleName: string, config: RiskRuleConfig) => {
  await invoke('update_risk_rule', {
    ruleName,
    config: {
      enabled: config.enabled,
      action: config.action,
      notifyMethods: config.notify_methods,
      params: config.params,
    },
  });
};

// 组件挂载时加载数据
onMounted(() => {
  loadRules();
});
</script>

<style scoped lang="scss">
.rule-config {
  padding: 20px;
  background: #f5f7fa;
  min-height: 100vh;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 20px;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .page-title {
    margin: 0;
    font-size: 24px;
    font-weight: 600;
    color: #303133;
  }
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100px 0;
  color: #909399;

  p {
    margin-top: 16px;
    font-size: 14px;
  }
}

.rules-container {
  .el-row {
    margin-bottom: 20px;
  }
}

.rule-card {
  margin-bottom: 20px;

  :deep(.el-card__header) {
    padding: 16px 20px;
    background: #fafafa;
  }

  :deep(.el-card__body) {
    padding: 20px;
  }
}

.rule-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;

  .rule-title {
    display: flex;
    align-items: center;
    gap: 12px;

    .rule-icon {
      color: #409eff;
    }

    .rule-name {
      font-size: 16px;
      font-weight: 600;
      color: #303133;
    }

    .rule-status-tag {
      margin-left: 8px;
    }
  }
}

.rule-description {
  margin-bottom: 24px;
  padding: 12px;
  background: #f0f9ff;
  border-left: 3px solid #409eff;
  border-radius: 4px;
}

.rule-form {
  .el-divider {
    margin: 24px 0;
    font-weight: 600;
    color: #606266;

    :deep(.el-divider__text) {
      background: transparent;
      padding: 0;
      font-size: 14px;
    }
  }

  .full-width {
    width: 100%;
  }

  .input-suffix {
    margin-left: 8px;
    color: #909399;
    font-size: 12px;
  }

  .action-option {
    display: flex;
    align-items: center;
    gap: 8px;

    .el-icon {
      color: #606266;
    }
  }

  .notify-option {
    display: flex;
    align-items: center;
    gap: 6px;

    .el-icon {
      color: #409eff;
    }
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding-top: 16px;
    border-top: 1px solid #ebeef5;
    margin-top: 8px;
  }
}

// 响应式设计
@media (max-width: 1200px) {
  .rule-form {
    .el-col {
      margin-bottom: 16px;
    }
  }
}

@media (max-width: 768px) {
  .rule-config {
    padding: 12px;
  }

  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }

  .rule-card-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .rule-form {
    .el-col {
      span: 24 !important;
    }
  }
}
</style>
