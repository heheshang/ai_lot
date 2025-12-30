<template>
  <div class="error-state" :class="[`error-state--${type}`, { 'error-state--compact': compact }]">
    <!-- 背景装饰 -->
    <div class="error-state__bg">
      <div class="error-state__circle error-state__circle--1"></div>
      <div class="error-state__circle error-state__circle--2"></div>
      <div class="error-state__circle error-state__circle--3"></div>
    </div>

    <!-- 错误图标/插画 -->
    <div class="error-state__illustration" :class="`error-state__illustration--${size}`">
      <slot name="illustration">
        <!-- 404 错误 -->
        <template v-if="type === '404'">
          <svg class="error-404" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" font-size="64" font-weight="700" fill="#E3F2FD">404</text>
            <path d="M60 70Q100 90 140 70" stroke="#2196F3" stroke-width="3" stroke-linecap="round"/>
            <circle cx="70" cy="50" r="6" fill="#2196F3"/>
            <circle cx="130" cy="50" r="6" fill="#2196F3"/>
          </svg>
        </template>

        <!-- 500 错误 -->
        <template v-else-if="type === '500'">
          <svg class="error-500" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" font-size="64" font-weight="700" fill="#FFEBEE">500</text>
            <path d="M70 90L85 70L100 85L115 65L130 80" stroke="#F44336" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </template>

        <!-- 403 错误 -->
        <template v-else-if="type === '403'">
          <svg class="error-403" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="70" y="30" width="60" height="60" rx="4" fill="#FFF3E0" stroke="#FF9800" stroke-width="2"/>
            <circle cx="100" cy="55" r="12" fill="#FF9800"/>
            <rect x="94" y="70" width="12" height="15" fill="#FF9800"/>
          </svg>
        </template>

        <!-- 网络错误 -->
        <template v-else-if="type === 'network'">
          <svg class="error-network" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M40 60L80 60M120 60L160 60" stroke="#909399" stroke-width="3" stroke-linecap="round" stroke-dasharray="8 8"/>
            <circle cx="100" cy="60" r="8" fill="#F44336"/>
            <path d="M92 52L108 68M108 52L92 68" stroke="#F44336" stroke-width="3" stroke-linecap="round"/>
          </svg>
        </template>

        <!-- 权限错误 -->
        <template v-else-if="type === 'permission'">
          <svg class="error-permission" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <rect x="60" y="25" width="80" height="70" rx="4" fill="#FFEBEE" stroke="#F44336" stroke-width="2"/>
            <circle cx="100" cy="55" r="16" fill="#F44336"/>
            <rect x="96" y="75" width="8" height="12" fill="#F44336"/>
          </svg>
        </template>

        <!-- 通用错误 -->
        <template v-else>
          <svg class="error-generic" viewBox="0 0 200 120" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="100" cy="60" r="40" fill="#E8F5E9"/>
            <path d="M100 40V70M100 80V84" stroke="#4CAF50" stroke-width="4" stroke-linecap="round"/>
          </svg>
        </template>
      </slot>
    </div>

    <!-- 错误代码 -->
    <div v-if="errorCode" class="error-state__code" :class="`error-state__code--${size}`">
      {{ errorCode }}
    </div>

    <!-- 标题 -->
    <div class="error-state__title" :class="`error-state__title--${size}`">
      <slot name="title">{{ title || defaultTitle }}</slot>
    </div>

    <!-- 描述 -->
    <div v-if="description || $slots.description" class="error-state__description" :class="`error-state__description--${size}`">
      <slot name="description">{{ description || defaultDescription }}</slot>
    </div>

    <!-- 详细信息 -->
    <div v-if="showDetails && details" class="error-state__details">
      <el-popover placement="bottom" :width="400" trigger="click">
        <template #reference>
          <el-button text type="info" size="small">
            <el-icon><InfoFilled /></el-icon>
            查看详情
          </el-button>
        </template>
        <div class="error-details">
          <p class="error-details__label">错误信息:</p>
          <pre class="error-details__content">{{ details }}</pre>
        </div>
      </el-popover>
    </div>

    <!-- 操作按钮 -->
    <div class="error-state__actions">
      <slot name="actions">
        <el-button v-if="showRetry" :icon="Refresh" @click="handleRetry">
          重试
        </el-button>
        <el-button v-if="showBack" @click="handleBack">
          返回
        </el-button>
        <el-button v-if="showHome" type="primary" @click="handleHome">
          返回首页
        </el-button>
      </slot>
    </div>

    <!-- 倒计时（自动重试） -->
    <div v-if="autoRetry && countdown > 0" class="error-state__countdown">
      <el-icon><Loading /></el-icon>
      <span>{{ countdown }} 秒后自动重试...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { InfoFilled, Refresh, Loading } from '@element-plus/icons-vue';

interface Props {
  type?: '404' | '500' | '403' | 'network' | 'permission' | 'generic';
  title?: string;
  description?: string;
  details?: string;
  size?: 'small' | 'medium' | 'large';
  compact?: boolean;
  showRetry?: boolean;
  showBack?: boolean;
  showHome?: boolean;
  showDetails?: boolean;
  autoRetry?: boolean;
  autoRetryDelay?: number;
}

const props = withDefaults(defineProps<Props>(), {
  type: 'generic',
  size: 'medium',
  compact: false,
  showRetry: true,
  showBack: false,
  showHome: true,
  showDetails: false,
  autoRetry: false,
  autoRetryDelay: 5,
});

const emit = defineEmits<{
  retry: [];
  back: [];
}>();

const router = useRouter();
const countdown = ref(0);
let countdownTimer: number | null = null;

const errorCode = computed(() => {
  const codes: Record<string, string> = {
    '404': '404',
    '500': '500',
    '403': '403',
  };
  return codes[props.type] || '';
});

const defaultTitle = computed(() => {
  const titles: Record<string, string> = {
    '404': '页面不存在',
    '500': '服务器错误',
    '403': '访问被拒绝',
    'network': '网络连接失败',
    'permission': '权限不足',
    'generic': '出错了',
  };
  return titles[props.type] || '出错了';
});

const defaultDescription = computed(() => {
  const descriptions: Record<string, string> = {
    '404': '抱歉，您访问的页面不存在或已被移除',
    '500': '服务器内部错误，请稍后重试',
    '403': '抱歉，您没有权限访问此页面',
    'network': '网络连接失败，请检查您的网络设置',
    'permission': '您没有权限执行此操作',
    'generic': '发生了未知错误，请稍后重试',
  };
  return descriptions[props.type] || '';
});

function handleRetry() {
  emit('retry');
}

function handleBack() {
  emit('back');
  router.back();
}

function handleHome() {
  router.push('/');
}

function startCountdown() {
  countdown.value = props.autoRetryDelay;
  countdownTimer = window.setInterval(() => {
    countdown.value--;
    if (countdown.value <= 0) {
      clearInterval(countdownTimer!);
      handleRetry();
    }
  }, 1000);
}

onMounted(() => {
  if (props.autoRetry) {
    startCountdown();
  }
});

onUnmounted(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer);
  }
});
</script>

<style scoped lang="scss">
.error-state {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 24px;
  text-align: center;
  background: #fff;
  border-radius: 12px;
  overflow: hidden;

  &--compact {
    padding: 40px 20px;
  }

  // 背景装饰
  &__bg {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow: hidden;
    pointer-events: none;
  }

  &__circle {
    position: absolute;
    border-radius: 50%;
    opacity: 0.1;

    &--1 {
      width: 200px;
      height: 200px;
      background: #409eff;
      top: -50px;
      right: -50px;
      animation: float 6s ease-in-out infinite;
    }

    &--2 {
      width: 150px;
      height: 150px;
      background: #26a69a;
      bottom: -30px;
      left: -30px;
      animation: float 8s ease-in-out infinite reverse;
    }

    &--3 {
      width: 100px;
      height: 100px;
      background: #e6a23c;
      top: 50%;
      left: 10%;
      animation: float 7s ease-in-out infinite;
    }
  }

  // 插画
  &__illustration {
    margin-bottom: 24px;
    position: relative;
    z-index: 1;

    svg {
      display: block;
    }

    &--small svg {
      width: 120px;
      height: auto;
    }

    &--medium svg {
      width: 160px;
      height: auto;
    }

    &--large svg {
      width: 200px;
      height: auto;
    }
  }

  // 错误代码
  &__code {
    font-size: 48px;
    font-weight: 700;
    line-height: 1;
    margin-bottom: 16px;
    background: linear-gradient(135deg, #409eff 0%, #67c23a 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;

    &--small { font-size: 36px; }
    &--medium { font-size: 48px; }
    &--large { font-size: 64px; }
  }

  // 标题
  &__title {
    font-weight: 600;
    color: #303133;
    margin: 0 0 12px 0;
    position: relative;
    z-index: 1;

    &--small { font-size: 16px; }
    &--medium { font-size: 20px; }
    &--large { font-size: 24px; }
  }

  // 描述
  &__description {
    color: #606266;
    max-width: 480px;
    margin: 0 0 24px 0;
    line-height: 1.6;
    position: relative;
    z-index: 1;

    &--small { font-size: 13px; }
    &--medium { font-size: 14px; }
    &--large { font-size: 15px; }
  }

  // 详细信息
  &__details {
    margin-bottom: 24px;
    position: relative;
    z-index: 1;
  }

  // 操作按钮
  &__actions {
    display: flex;
    gap: 12px;
    position: relative;
    z-index: 1;
  }

  // 倒计时
  &__countdown {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 20px;
    color: #909399;
    font-size: 13px;
    position: relative;
    z-index: 1;

    .el-icon {
      animation: spin 1s linear infinite;
    }
  }
}

// 错误详情弹窗
.error-details {
  &__label {
    font-weight: 600;
    color: #303133;
    margin: 0 0 8px 0;
  }

  &__content {
    background: #f5f7fa;
    padding: 12px;
    border-radius: 6px;
    font-size: 12px;
    color: #606266;
    white-space: pre-wrap;
    word-break: break-all;
    margin: 0;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-20px) rotate(5deg);
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

// SVG 错误图标样式
.error-404,
.error-500,
.error-403,
.error-network,
.error-permission,
.error-generic {
  display: block;
  width: 100%;
  height: 100%;
}
</style>
