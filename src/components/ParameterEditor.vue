<template>
  <div class="parameter-editor">
    <el-table
      :data="parameters"
      border
      stripe
      :empty-text="emptyText"
      max-height="400"
    >
      <!-- 参数名列 -->
      <el-table-column
        prop="name"
        label="参数名"
        width="150"
        fixed
      >
        <template #default="{ row }">
          <span class="parameter-name">{{ row.name }}</span>
        </template>
      </el-table-column>

      <!-- 类型列 -->
      <el-table-column
        prop="type"
        label="类型"
        width="100"
        align="center"
      >
        <template #default="{ row }">
          <el-tag :type="getTypeColor(row.type)" size="small">
            {{ getTypeLabel(row.type) }}
          </el-tag>
        </template>
      </el-table-column>

      <!-- 默认值列 -->
      <el-table-column
        prop="default"
        label="默认值"
        width="120"
        align="center"
      >
        <template #default="{ row }">
          <span class="default-value">{{ formatValue(row.default, row.type) }}</span>
        </template>
      </el-table-column>

      <!-- 当前值编辑列 -->
      <el-table-column
        label="当前值"
        width="200"
      >
        <template #default="{ row }">
          <!-- Number type: InputNumber with min/max/step -->
          <el-input-number
            v-if="row.type === 'number'"
            :model-value="values[row.name]"
            @update:model-value="(val: number | undefined) => updateValue(row.name, val)"
            :min="row.min"
            :max="row.max"
            :step="row.step"
            :precision="getPrecision(row)"
            size="small"
            controls-position="right"
            :disabled="disabled"
            style="width: 100%"
          />

          <!-- Boolean type: Switch -->
          <el-switch
            v-else-if="row.type === 'boolean'"
            :model-value="values[row.name]"
            @update:model-value="(val: boolean) => updateValue(row.name, val)"
            :disabled="disabled"
          />

          <!-- Select type: Select dropdown -->
          <el-select
            v-else-if="row.type === 'select'"
            :model-value="values[row.name]"
            @update:model-value="(val: any) => updateValue(row.name, val)"
            size="small"
            :disabled="disabled"
            style="width: 100%"
          >
            <el-option
              v-for="option in row.options"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>

          <!-- String type: Input -->
          <el-input
            v-else-if="row.type === 'string'"
            :model-value="values[row.name]"
            @update:model-value="(val: string) => updateValue(row.name, val)"
            size="small"
            :disabled="disabled"
          />

          <!-- Unknown type -->
          <span v-else class="unknown-type">不支持</span>
        </template>
      </el-table-column>

      <!-- 范围约束列 (仅 number 类型显示) -->
      <el-table-column
        v-if="hasNumberType"
        label="范围"
        width="140"
        align="center"
      >
        <template #default="{ row }">
          <span v-if="row.type === 'number'" class="range-text">
            {{ formatRange(row) }}
          </span>
          <span v-else class="range-text">-</span>
        </template>
      </el-table-column>

      <!-- 说明列 -->
      <el-table-column
        prop="description"
        label="说明"
        min-width="200"
        show-overflow-tooltip
      >
        <template #default="{ row }">
          <span class="description">{{ row.description || '-' }}</span>
        </template>
      </el-table-column>

      <!-- 操作列 -->
      <el-table-column
        v-if="showReset"
        label="操作"
        width="80"
        align="center"
        fixed="right"
      >
        <template #default="{ row }">
          <el-button
            type="primary"
            link
            size="small"
            @click="resetValue(row)"
            :disabled="disabled"
          >
            重置
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 底部统计信息 -->
    <div v-if="showSummary" class="summary">
      <el-space>
        <el-tag size="small" type="info">共 {{ parameters.length }} 个参数</el-tag>
        <el-tag v-if="changedCount > 0" size="small" type="warning">
          已修改 {{ changedCount }} 个
        </el-tag>
        <el-button
          v-if="changedCount > 0"
          type="primary"
          link
          size="small"
          @click="resetAll"
        >
          全部重置
        </el-button>
      </el-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { StrategyParameter } from '@/types';

/**
 * ParameterEditor component props
 */
interface Props {
  parameters: StrategyParameter[];
  modelValue: Record<string, any>;
  disabled?: boolean;
  showReset?: boolean;
  showSummary?: boolean;
  emptyText?: string;
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
  showReset: true,
  showSummary: true,
  emptyText: '暂无参数',
});

const emit = defineEmits<{
  'update:modelValue': [value: Record<string, any>];
  'change': [name: string, value: any];
}>();

// Local values state
const values = ref<Record<string, any>>({});

// Initialize values from props
function initValues() {
  const newValues: Record<string, any> = {};

  for (const param of props.parameters) {
    // Use modelValue if provided, otherwise use default
    if (props.modelValue && param.name in props.modelValue) {
      newValues[param.name] = props.modelValue[param.name];
    } else {
      newValues[param.name] = param.default;
    }
  }

  values.value = newValues;
}

// Initialize on mount
initValues();

// Watch for external modelValue changes
watch(() => props.modelValue, () => {
  initValues();
}, { deep: true });

// Watch for parameters changes
watch(() => props.parameters, () => {
  initValues();
}, { deep: true });

// Watch for value changes and emit
watch(values, (newValues) => {
  emit('update:modelValue', { ...newValues });
}, { deep: true });

/**
 * Update a single parameter value
 */
function updateValue(name: string, value: any) {
  values.value[name] = value;
  emit('change', name, value);
}

/**
 * Reset a single parameter to its default value
 */
function resetValue(param: StrategyParameter) {
  values.value[param.name] = param.default;
}

/**
 * Reset all parameters to their default values
 */
function resetAll() {
  initValues();
}

/**
 * Get color tag for parameter type
 */
function getTypeColor(type: string): 'success' | 'warning' | 'primary' | 'info' | 'danger' {
  const colors: Record<string, 'success' | 'warning' | 'primary' | 'info' | 'danger'> = {
    number: 'primary',
    string: 'success',
    boolean: 'warning',
    select: 'info',
  };
  return colors[type] || 'danger';
}

/**
 * Get display label for parameter type
 */
function getTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    number: '数字',
    string: '文本',
    boolean: '布尔',
    select: '选择',
  };
  return labels[type] || type;
}

/**
 * Format value for display
 */
function formatValue(value: any, type: string): string {
  if (value === null || value === undefined) {
    return '-';
  }

  switch (type) {
    case 'boolean':
      return value ? '是' : '否';
    case 'number':
      return String(value);
    default:
      return String(value);
  }
}

/**
 * Format range for number type
 */
function formatRange(param: StrategyParameter): string {
  if (param.type !== 'number') return '-';

  const parts: string[] = [];
  if (param.min !== undefined) {
    parts.push(`最小: ${param.min}`);
  }
  if (param.max !== undefined) {
    parts.push(`最大: ${param.max}`);
  }
  if (param.step !== undefined) {
    parts.push(`步长: ${param.step}`);
  }

  return parts.length > 0 ? parts.join(' | ') : '无限制';
}

/**
 * Get precision for number input
 */
function getPrecision(param: StrategyParameter): number | undefined {
  if (param.type !== 'number') return undefined;

  // Determine precision based on step value
  if (param.step !== undefined) {
    const stepStr = param.step.toString();
    if (stepStr.includes('.')) {
      return stepStr.split('.')[1].length;
    }
    return 0;
  }

  return undefined;
}

/**
 * Check if any parameter is number type
 */
const hasNumberType = computed(() => {
  return props.parameters.some(p => p.type === 'number');
});

/**
 * Count changed parameters
 */
const changedCount = computed(() => {
  return props.parameters.filter(param => {
    const current = values.value[param.name];
    const def = param.default;
    return current !== def;
  }).length;
});

// Expose methods
defineExpose({
  resetValue,
  resetAll,
  values,
  initValues,
});
</script>

<style scoped>
.parameter-editor {
  width: 100%;
}

.parameter-name {
  font-weight: 600;
  color: var(--el-text-color-primary);
  font-family: 'Courier New', monospace;
}

.default-value {
  color: var(--el-text-color-secondary);
  font-size: 0.9em;
}

.range-text {
  color: var(--el-text-color-secondary);
  font-size: 0.85em;
}

.description {
  color: var(--el-text-color-regular);
  font-size: 0.9em;
}

.unknown-type {
  color: var(--el-color-danger);
  font-size: 0.9em;
}

.summary {
  margin-top: 16px;
  padding: 12px 16px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  display: flex;
  align-items: center;
}

:deep(.el-input-number) {
  width: 100%;
}

:deep(.el-input-number .el-input__inner) {
  text-align: left;
}
</style>
