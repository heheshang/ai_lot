<template>
  <div class="empty-state" :class="[`empty-state--${type}`, { 'empty-state--compact': compact }]">
    <!-- 图标 -->
    <div class="empty-state__icon" :class="`empty-state__icon--${size}`">
      <slot name="icon">
        <!-- 默认 SVG 图标 -->
        <svg v-if="type === 'default'" class="default-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="32" cy="32" r="28" fill="#E8F5E9"/>
          <path d="M20 32H44M32 20V44" stroke="#4CAF50" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <svg v-else-if="type === 'search'" class="search-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="28" cy="28" r="20" fill="#E3F2FD"/>
          <path d="M40 40L52 52M24 20V36M36 24H20" stroke="#2196F3" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <svg v-else-if="type === 'error'" class="error-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="32" cy="32" r="28" fill="#FFEBEE"/>
          <path d="M24 24L40 40M40 24L24 40" stroke="#F44336" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <svg v-else-if="type === 'warning'" class="warning-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="32" cy="32" r="28" fill="#FFF3E0"/>
          <path d="M32 18V34M32 42V44" stroke="#FF9800" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <svg v-else-if="type === 'folder'" class="folder-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="8" y="16" width="48" height="36" rx="4" fill="#E8F5E9"/>
          <path d="M16 12H28L32 16H48C50.2091 16 52 17.7909 52 20V48C52 50.2091 50.2091 52 48 52H16C13.7909 52 12 50.2091 12 48V16C12 13.7909 13.7909 12 16 12Z" stroke="#4CAF50" stroke-width="2"/>
        </svg>
        <svg v-else-if="type === 'network'" class="network-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="32" cy="32" r="28" fill="#E3F2FD"/>
          <circle cx="32" cy="20" r="4" fill="#2196F3"/>
          <circle cx="20" cy="44" r="4" fill="#2196F3"/>
          <circle cx="44" cy="44" r="4" fill="#2196F3"/>
          <path d="M32 20V44M20 44L44 44" stroke="#2196F3" stroke-width="2"/>
        </svg>
        <svg v-else-if="type === 'data'" class="data-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="8" y="8" width="48" height="48" rx="8" fill="#E8F5E9"/>
          <path d="M16 20H48M16 32H40M16 44H32" stroke="#4CAF50" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </slot>
    </div>

    <!-- 标题 -->
    <div v-if="title || $slots.title" class="empty-state__title" :class="`empty-state__title--${size}`">
      <slot name="title">{{ title || defaultTitle }}</slot>
    </div>

    <!-- 描述 -->
    <div v-if="description || $slots.description" class="empty-state__description" :class="`empty-state__description--${size}`">
      <slot name="description">{{ description || defaultDescription }}</slot>
    </div>

    <!-- 操作按钮 -->
    <div v-if="showAction || $slots.action" class="empty-state__action">
      <slot name="action">
        <el-button v-if="actionText" :type="actionType" :icon="actionIcon" @click="handleAction">
          {{ actionText }}
        </el-button>
      </slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Plus } from '@element-plus/icons-vue';

interface Props {
  type?: 'default' | 'search' | 'error' | 'warning' | 'folder' | 'network' | 'data' | 'custom';
  title?: string;
  description?: string;
  size?: 'small' | 'medium' | 'large';
  compact?: boolean;
  showAction?: boolean;
  actionText?: string;
  actionType?: 'primary' | 'success' | 'warning' | 'danger' | 'info';
  actionIcon?: any;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'default',
  size: 'medium',
  compact: false,
  showAction: false,
  actionType: 'primary',
  actionIcon: Plus,
});

const emit = defineEmits<{
  action: [];
}>();

const defaultTitle = computed(() => {
  const titles: Record<string, string> = {
    default: '暂无数据',
    search: '未找到相关内容',
    error: '加载失败',
    warning: '注意',
    folder: '文件夹为空',
    network: '网络错误',
    data: '暂无数据',
  };
  return titles[props.type] || '暂无数据';
});

const defaultDescription = computed(() => {
  const descriptions: Record<string, string> = {
    default: '当前没有任何数据可显示',
    search: '请尝试使用其他关键词搜索',
    error: '数据加载失败，请稍后重试',
    warning: '请注意以下事项',
    folder: '此文件夹中没有任何文件',
    network: '网络连接失败，请检查网络设置',
    data: '请添加数据或更改筛选条件',
  };
  return descriptions[props.type] || '';
});

function handleAction() {
  emit('action');
}
</script>

<style scoped lang="scss">
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  text-align: center;
  background: #fff;
  border-radius: 12px;
  transition: all 0.3s ease;

  &--compact {
    padding: 32px 20px;
  }

  // 类型变体
  &--default {
    .empty-state__icon {
      color: #4CAF50;
    }
  }

  &--search {
    .empty-state__icon {
      color: #2196F3;
    }
  }

  &--error {
    .empty-state__icon {
      color: #F44336;
    }
  }

  &--warning {
    .empty-state__icon {
      color: #FF9800;
    }
  }

  // 图标大小
  &__icon {
    margin-bottom: 20px;
    animation: float 3s ease-in-out infinite;

    svg {
      display: block;
    }

    &--small svg {
      width: 64px;
      height: 64px;
    }

    &--medium svg {
      width: 96px;
      height: 96px;
    }

    &--large svg {
      width: 128px;
      height: 128px;
    }
  }

  // 标题
  &__title {
    font-weight: 600;
    color: #303133;
    margin: 0 0 8px 0;

    &--small {
      font-size: 14px;
    }

    &--medium {
      font-size: 16px;
    }

    &--large {
      font-size: 18px;
    }
  }

  // 描述
  &__description {
    color: #909399;
    max-width: 400px;
    margin: 0 0 24px 0;
    line-height: 1.6;

    &--small {
      font-size: 12px;
    }

    &--medium {
      font-size: 13px;
    }

    &--large {
      font-size: 14px;
    }
  }

  // 操作按钮
  &__action {
    display: flex;
    gap: 12px;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8px);
  }
}

// SVG 图标样式
.default-icon,
.search-icon,
.error-icon,
.warning-icon,
.folder-icon,
.network-icon,
.data-icon {
  display: block;
  width: 100%;
  height: 100%;
}
</style>