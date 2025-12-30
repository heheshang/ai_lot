<template>
  <div class="loading-skeleton" :class="[`loading-skeleton--${variant}`, { 'loading-skeleton--pulse': pulse }]">
    <!-- 卡片骨架屏 -->
    <template v-if="variant === 'card'">
      <div v-for="i in rows" :key="i" class="skeleton-card">
        <div class="skeleton-card__header">
          <div class="skeleton-avatar" :class="{ 'skeleton-avatar--circle': avatarCircle }"></div>
          <div class="skeleton-card__title"></div>
        </div>
        <div class="skeleton-card__body">
          <div v-for="j in linesPerRow" :key="j" class="skeleton-line" :class="{ 'skeleton-line--short': j === linesPerRow }"></div>
        </div>
      </div>
    </template>

    <!-- 列表骨架屏 -->
    <template v-else-if="variant === 'list'">
      <div v-for="i in rows" :key="i" class="skeleton-list-item">
        <div v-if="showAvatar" class="skeleton-avatar" :class="{ 'skeleton-avatar--circle': avatarCircle }"></div>
        <div class="skeleton-list-item__content">
          <div class="skeleton-line skeleton-line--title"></div>
          <div class="skeleton-line skeleton-line--subtitle"></div>
        </div>
        <div v-if="showAction" class="skeleton-action"></div>
      </div>
    </template>

    <!-- 表格骨架屏 -->
    <template v-else-if="variant === 'table'">
      <div class="skeleton-table">
        <div class="skeleton-table__header">
          <div v-for="col in columns" :key="col" class="skeleton-table__th"></div>
        </div>
        <div v-for="i in rows" :key="i" class="skeleton-table__row">
          <div v-for="col in columns" :key="col" class="skeleton-table__td">
            <div class="skeleton-line"></div>
          </div>
        </div>
      </div>
    </template>

    <!-- 图表骨架屏 -->
    <template v-else-if="variant === 'chart'">
      <div class="skeleton-chart">
        <div class="skeleton-chart__axis skeleton-chart__axis--y">
          <div v-for="i in 5" :key="i" class="skeleton-chart__tick"></div>
        </div>
        <div class="skeleton-chart__content">
          <div v-for="i in chartBars" :key="i" class="skeleton-chart__bar" :style="{ height: `${30 + Math.random() * 60}%` }"></div>
        </div>
        <div class="skeleton-chart__axis skeleton-chart__axis--x">
          <div v-for="i in chartBars" :key="i" class="skeleton-chart__tick"></div>
        </div>
      </div>
    </template>

    <!-- 自定义骨架屏 -->
    <template v-else>
      <slot></slot>
    </template>
  </div>
</template>

<script setup lang="ts">
import { defineProps, withDefaults } from 'vue';

interface Props {
  variant?: 'card' | 'list' | 'table' | 'chart' | 'custom';
  rows?: number;
  linesPerRow?: number;
  columns?: number;
  chartBars?: number;
  pulse?: boolean;
  showAvatar?: boolean;
  avatarCircle?: boolean;
  showAction?: boolean;
}

withDefaults(defineProps<Props>(), {
  variant: 'list',
  rows: 3,
  linesPerRow: 3,
  columns: 4,
  chartBars: 7,
  pulse: true,
  showAvatar: true,
  avatarCircle: true,
  showAction: false,
});
</script>

<style scoped lang="scss">
.loading-skeleton {
  --skeleton-base: #f2f2f2;
  --skeleton-highlight: #e6e6e6;

  &--pulse {
    .skeleton-line,
    .skeleton-avatar,
    .skeleton-action,
    .skeleton-card__title,
    .skeleton-table__th,
    .skeleton-table__td,
    .skeleton-chart__bar,
    .skeleton-chart__tick {
      background: linear-gradient(90deg, var(--skeleton-base) 25%, var(--skeleton-highlight) 50%, var(--skeleton-base) 75%);
      background-size: 200% 100%;
      animation: shimmer 1.5s infinite;
    }
  }
}

@keyframes shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

// 卡片骨架屏
.skeleton-card {
  background: #fff;
  border: 1px solid #ebeef5;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 16px;

  &:last-child {
    margin-bottom: 0;
  }

  &__header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  &__title {
    flex: 1;
    height: 20px;
    border-radius: 4px;
  }

  &__body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
}

// 列表骨架屏
.skeleton-list-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 0;
  border-bottom: 1px solid #f5f7fa;

  &:last-child {
    border-bottom: none;
  }

  &__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
}

// 表格骨架屏
.skeleton-table {
  background: #fff;
  border-radius: 12px;
  overflow: hidden;

  &__header {
    display: flex;
    background: #fafbfc;
    border-bottom: 1px solid #ebeef5;
  }

  &__th {
    flex: 1;
    height: 44px;
    margin: 8px;
    border-radius: 4px;
  }

  &__row {
    display: flex;
    border-bottom: 1px solid #f5f7fa;

    &:last-child {
      border-bottom: none;
    }
  }

  &__td {
    flex: 1;
    padding: 16px 8px;

    .skeleton-line {
      height: 14px;
    }
  }
}

// 图表骨架屏
.skeleton-chart {
  display: flex;
  height: 240px;
  padding: 20px;
  background: #fff;
  border-radius: 12px;
  border: 1px solid #ebeef5;

  &__axis {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    &--y {
      padding-right: 12px;
      .skeleton-chart__tick {
        width: 40px;
        height: 12px;
      }
    }

    &--x {
      flex-direction: row;
      padding-top: 12px;

      .skeleton-chart__tick {
        width: 12px;
        height: 12px;
      }
    }
  }

  &__content {
    flex: 1;
    display: flex;
    align-items: flex-end;
    justify-content: space-around;
    gap: 8px;
    padding: 0 16px;
  }

  &__bar {
    flex: 1;
    max-width: 40px;
    border-radius: 4px 4px 0 0;
    transition: height 0.3s ease;
  }
}

// 通用元素
.skeleton {
  &-line {
    height: 14px;
    border-radius: 4px;
    background: var(--skeleton-base);

    &--title {
      height: 18px;
      width: 60%;
    }

    &--subtitle {
      width: 80%;
    }

    &--short {
      width: 40%;
    }
  }

  &-avatar {
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    border-radius: 8px;
    background: var(--skeleton-base);

    &--circle {
      border-radius: 50%;
    }
  }

  &-action {
    flex-shrink: 0;
    width: 60px;
    height: 32px;
    border-radius: 6px;
    background: var(--skeleton-base);
  }
}
</style>
