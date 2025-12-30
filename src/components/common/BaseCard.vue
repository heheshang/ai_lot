<template>
  <div
    class="base-card"
    :class="[
      `base-card--${variant}`,
      { 'base-card--hoverable': hoverable },
      { 'base-card--clickable': clickable },
      { 'base-card--bordered': bordered },
      { 'base-card--shadow': shadow },
      { 'is-loading': loading }
    ]"
    @click="handleClick"
  >
    <!-- 骨架屏加载状态 -->
    <div v-if="loading" class="base-card__skeleton">
      <slot name="skeleton">
        <div class="skeleton-header">
          <div class="skeleton-title"></div>
        </div>
        <div class="skeleton-body">
          <div class="skeleton-line"></div>
          <div class="skeleton-line short"></div>
        </div>
      </slot>
    </div>

    <!-- 正常内容 -->
    <template v-else>
      <!-- 卡片头部 -->
      <div v-if="$slots.header || title" class="base-card__header">
        <div class="base-card__header-content">
          <div v-if="icon" class="base-card__icon">
            <el-icon :size="iconSize">
              <component :is="icon" />
            </el-icon>
          </div>
          <div class="base-card__title">
            <slot name="header">{{ title }}</slot>
          </div>
        </div>
        <div v-if="$slots.extra" class="base-card__extra">
          <slot name="extra"></slot>
        </div>
      </div>

      <!-- 卡片内容 -->
      <div class="base-card__body" :class="bodyClass">
        <slot></slot>
      </div>

      <!-- 卡片底部 -->
      <div v-if="$slots.footer" class="base-card__footer">
        <slot name="footer"></slot>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

interface Props {
  title?: string;
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'info';
  hoverable?: boolean;
  clickable?: boolean;
  bordered?: boolean;
  shadow?: boolean | 'never' | 'hover' | 'always';
  loading?: boolean;
  bodyClass?: string;
  icon?: any;
  iconSize?: number | string;
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  hoverable: false,
  clickable: false,
  bordered: true,
  shadow: 'hover',
  loading: false,
  iconSize: 20,
});

const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

function handleClick(event: MouseEvent) {
  if (props.clickable) {
    emit('click', event);
  }
}
</script>

<style scoped lang="scss">
.base-card {
  background: #fff;
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  position: relative;

  // 边框变体
  &--bordered {
    border: 1px solid #ebeef5;
  }

  // 阴影变体
  &--shadow {
    box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));
  }

  // Hover 状态
  &--hoverable:hover,
  &--clickable:hover {
    box-shadow: var(--shadow-hover, 0 6px 20px rgba(0, 0, 0, 0.12));
    transform: translateY(-2px);
  }

  &--clickable {
    cursor: pointer;
  }

  // 颜色变体
  &--primary {
    border-color: #409eff;
    .base-card__header {
      background: linear-gradient(135deg, #ecf5ff 0%, #ffffff 100%);
    }
  }

  &--success {
    border-color: #26a69a;
    .base-card__header {
      background: linear-gradient(135deg, #e6f7f7 0%, #ffffff 100%);
    }
  }

  &--warning {
    border-color: #e6a23c;
    .base-card__header {
      background: linear-gradient(135deg, #fef8f0 0%, #ffffff 100%);
    }
  }

  &--danger {
    border-color: #ef5350;
    .base-card__header {
      background: linear-gradient(135deg, #fef2f2 0%, #ffffff 100%);
    }
  }

  &--info {
    border-color: #909399;
    .base-card__header {
      background: linear-gradient(135deg, #f4f4f5 0%, #ffffff 100%);
    }
  }

  // 头部
  &__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid #ebeef5;
  }

  &__header-content {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  &__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 8px;
    background: #f5f7fa;
    color: #409eff;
  }

  &__title {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
  }

  &__extra {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  // 内容区域
  &__body {
    padding: 20px;
  }

  // 底部
  &__footer {
    padding: 16px 20px;
    border-top: 1px solid #ebeef5;
    background: #fafbfc;
  }

  // 骨架屏
  &__skeleton {
    padding: 20px;
  }
}

.skeleton {
  &-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  &-title {
    width: 40%;
    height: 20px;
    background: linear-gradient(90deg, #f2f2f2 25%, #e6e6e6 50%, #f2f2f2 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: 4px;
  }

  &-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  &-line {
    width: 100%;
    height: 14px;
    background: linear-gradient(90deg, #f2f2f2 25%, #e6e6e6 50%, #f2f2f2 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: 4px;

    &.short {
      width: 60%;
    }
  }
}

@keyframes shimmer {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}
</style>
