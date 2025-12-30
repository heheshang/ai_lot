<template>
  <div class="login-container">
    <!-- 粒子背景 -->
    <canvas ref="particleCanvas" class="particle-background"></canvas>

    <!-- 背景装饰 -->
    <div class="bg-decoration">
      <div class="decoration-circle circle-1"></div>
      <div class="decoration-circle circle-2"></div>
      <div class="decoration-circle circle-3"></div>
    </div>

    <!-- 登录卡片 -->
    <el-card class="login-card" :class="{ 'card-enter': cardLoaded }">
      <div class="login-brand">
        <div class="brand-logo">
          <div class="logo-icon">
            <svg viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="20" cy="20" r="18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <path d="M20 8 L20 20 L28 24" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <circle cx="20" cy="20" r="3" fill="currentColor"/>
            </svg>
          </div>
        </div>
        <h1 class="brand-title">AI-LOT</h1>
        <p class="brand-subtitle">量化交易终端</p>
      </div>

      <div class="login-divider">
        <span></span>
        <span class="divider-text">登录账户</span>
        <span></span>
      </div>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="0"
        size="large"
        class="login-form"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="username">
          <div class="input-wrapper" :class="{ 'input-focused': focusedField === 'username' }">
            <el-icon class="input-icon"><User /></el-icon>
            <el-input
              v-model="form.username"
              placeholder="用户名"
              clearable
              @focus="focusedField = 'username'"
              @blur="focusedField = ''"
            />
            <div class="input-line"></div>
          </div>
        </el-form-item>

        <el-form-item prop="password">
          <div class="input-wrapper" :class="{ 'input-focused': focusedField === 'password' }">
            <el-icon class="input-icon"><Lock /></el-icon>
            <el-input
              v-model="form.password"
              :type="passwordVisible ? 'text' : 'password'"
              placeholder="密码"
              clearable
              @focus="focusedField = 'password'"
              @blur="focusedField = ''"
              @keyup.enter="handleLogin"
            >
              <template #suffix>
                <el-icon
                  class="password-toggle"
                  @click="passwordVisible = !passwordVisible"
                >
                  <View v-if="passwordVisible" />
                  <Hide v-else />
                </el-icon>
              </template>
            </el-input>
            <div class="input-line"></div>
          </div>
        </el-form-item>

        <div class="login-options">
          <el-checkbox v-model="form.rememberMe" class="remember-checkbox">
            <span>记住我</span>
          </el-checkbox>
          <el-link type="primary" :underline="false">忘记密码?</el-link>
        </div>

        <el-form-item>
          <el-button
            type="primary"
            :loading="loading"
            class="login-button"
            @click="handleLogin"
          >
            <span v-if="!loading">登录</span>
            <span v-else>登录中...</span>
          </el-button>
        </el-form-item>
      </el-form>

      <div class="login-footer">
        <div class="footer-divider">
          <span></span>
          <span class="text">默认账户</span>
          <span></span>
        </div>
        <div class="default-account">
          <el-tag type="info" size="small" effect="plain">admin / admin123</el-tag>
        </div>
      </div>
    </el-card>

    <!-- 底部版权 -->
    <div class="login-copyright">
      <p>&copy; 2025 AI-LOT. All rights reserved.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted, onUnmounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { ElMessage, type FormInstance, type FormRules } from 'element-plus';
import { User, Lock, View, Hide } from '@element-plus/icons-vue';
import { useUserStore } from '@/store';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const formRef = ref<FormInstance>();
const loading = ref(false);
const cardLoaded = ref(false);
const passwordVisible = ref(false);
const focusedField = ref('');
const particleCanvas = ref<HTMLCanvasElement>();

const form = reactive({
  username: '',
  password: '',
  rememberMe: false,
});

const rules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, max: 50, message: '密码长度在 6 到 50 个字符', trigger: 'blur' },
  ],
};

// 粒子动画
class ParticleSystem {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private particles: Particle[] = [];
  private animationId: number | null = null;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');
    if (!ctx) throw new Error('无法获取 Canvas 上下文');
    this.ctx = ctx;
    this.resize();
    this.initParticles();
    window.addEventListener('resize', () => this.resize());
  }

  private resize() {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  private initParticles() {
    const count = Math.floor((this.canvas.width * this.canvas.height) / 15000);
    this.particles = [];
    for (let i = 0; i < count; i++) {
      this.particles.push(new Particle(this.canvas.width, this.canvas.height));
    }
  }

  public animate() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    this.particles.forEach(particle => {
      particle.update();
      particle.draw(this.ctx);
    });

    // 绘制连线
    this.drawConnections();

    this.animationId = requestAnimationFrame(() => this.animate());
  }

  private drawConnections() {
    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        const dx = this.particles[i].x - this.particles[j].x;
        const dy = this.particles[i].y - this.particles[j].y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < 120) {
          this.ctx.beginPath();
          this.ctx.strokeStyle = `rgba(102, 126, 234, ${0.15 * (1 - distance / 120)})`;
          this.ctx.lineWidth = 1;
          this.ctx.moveTo(this.particles[i].x, this.particles[i].y);
          this.ctx.lineTo(this.particles[j].x, this.particles[j].y);
          this.ctx.stroke();
        }
      }
    }
  }

  public destroy() {
    if (this.animationId) {
      cancelAnimationFrame(this.animationId);
    }
    window.removeEventListener('resize', () => this.resize());
  }
}

class Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  opacity: number;

  constructor(canvasWidth: number, canvasHeight: number) {
    this.x = Math.random() * canvasWidth;
    this.y = Math.random() * canvasHeight;
    this.vx = (Math.random() - 0.5) * 0.5;
    this.vy = (Math.random() - 0.5) * 0.5;
    this.radius = Math.random() * 2 + 1;
    this.opacity = Math.random() * 0.5 + 0.2;
  }

  update() {
    this.x += this.vx;
    this.y += this.vy;

    // 边界反弹
    if (this.x < 0 || this.x > window.innerWidth) this.vx *= -1;
    if (this.y < 0 || this.y > window.innerHeight) this.vy *= -1;
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.beginPath();
    ctx.fillStyle = `rgba(102, 126, 234, ${this.opacity})`;
    ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
    ctx.fill();
  }
}

let particleSystem: ParticleSystem | null = null;

onMounted(() => {
  // 卡片入场动画
  setTimeout(() => {
    cardLoaded.value = true;
  }, 100);

  // 初始化粒子系统
  if (particleCanvas.value) {
    particleSystem = new ParticleSystem(particleCanvas.value);
    particleSystem.animate();
  }
});

onUnmounted(() => {
  if (particleSystem) {
    particleSystem.destroy();
  }
});

async function handleLogin() {
  if (!formRef.value) return;

  const valid = await formRef.value.validate().catch(() => false);
  if (!valid) return;

  loading.value = true;

  try {
    const success = await userStore.login({
      username: form.username,
      password: form.password,
    });

    if (success) {
      ElMessage.success('登录成功');
      const redirect = (route.query.redirect as string) || '/dashboard';
      router.push(redirect);
    } else {
      ElMessage.error('登录失败，请检查用户名和密码');
    }
  } catch (error) {
    ElMessage.error('登录失败：' + (error as Error).message);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped lang="scss">
.login-container {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f093fb 100%);
}

// 粒子背景
.particle-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
}

// 背景装饰
.bg-decoration {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
  pointer-events: none;

  .decoration-circle {
    position: absolute;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.03);
    animation: float 20s ease-in-out infinite;

    &.circle-1 {
      width: 400px;
      height: 400px;
      top: -100px;
      right: -100px;
      animation-delay: 0s;
    }

    &.circle-2 {
      width: 300px;
      height: 300px;
      bottom: -50px;
      left: -50px;
      animation-delay: -7s;
    }

    &.circle-3 {
      width: 200px;
      height: 200px;
      top: 50%;
      left: 10%;
      animation-delay: -14s;
    }
  }
}

@keyframes float {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
  }
  25% {
    transform: translate(30px, -30px) rotate(5deg);
  }
  50% {
    transform: translate(0, -50px) rotate(0deg);
  }
  75% {
    transform: translate(-30px, -30px) rotate(-5deg);
  }
}

// 登录卡片
.login-card {
  width: 420px;
  position: relative;
  z-index: 10;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2),
              0 0 0 1px rgba(255, 255, 255, 0.1) inset;
  padding: 16px;
  opacity: 0;
  transform: translateY(30px) scale(0.95);
  transition: all 0.6s cubic-bezier(0.25, 1, 0.5, 1);

  &.card-enter {
    opacity: 1;
    transform: translateY(0) scale(1);
  }

  :deep(.el-card__body) {
    padding: 0;
  }

  :deep(.el-form-item) {
    margin-bottom: 20px;
  }
}

// 品牌区域
.login-brand {
  text-align: center;
  padding: 24px 0;

  .brand-logo {
    margin-bottom: 16px;

    .logo-icon {
      width: 64px;
      height: 64px;
      margin: 0 auto;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border-radius: 16px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      box-shadow: 0 8px 24px rgba(102, 126, 234, 0.4);
      animation: logoFloat 3s ease-in-out infinite;

      svg {
        width: 32px;
        height: 32px;
      }
    }
  }

  .brand-title {
    font-size: 28px;
    font-weight: 700;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0 0 4px 0;
    letter-spacing: 1px;
  }

  .brand-subtitle {
    font-size: 14px;
    color: #909399;
    margin: 0;
  }
}

@keyframes logoFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-8px) rotate(5deg);
  }
}

// 分隔线
.login-divider {
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 0 24px 24px;

  span:first-child,
  span:last-child {
    flex: 1;
    height: 1px;
    background: linear-gradient(90deg, transparent, #e4e7ed, transparent);
  }

  .divider-text {
    font-size: 13px;
    color: #909399;
    white-space: nowrap;
  }
}

// 表单
.login-form {
  padding: 0 24px;

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    background: #f5f7fa;
    border-radius: 12px;
    padding: 0 16px;
    transition: all 0.3s ease;
    border: 2px solid transparent;

    &.input-focused {
      background: white;
      border-color: #667eea;
      box-shadow: 0 0 0 4px rgba(102, 126, 234, 0.1);

      .input-icon {
        color: #667eea;
        transform: scale(1.1);
      }

      .input-line {
        width: 100%;
      }
    }

    :deep(.el-input) {
      flex: 1;

      .el-input__wrapper {
        background: transparent;
        box-shadow: none;
        padding: 12px 0;
      }
    }

    .input-icon {
      font-size: 18px;
      color: #909399;
      margin-right: 12px;
      transition: all 0.3s ease;
    }

    .password-toggle {
      cursor: pointer;
      color: #909399;
      transition: color 0.3s ease;

      &:hover {
        color: #667eea;
      }
    }

    .input-line {
      position: absolute;
      bottom: 0;
      left: 0;
      height: 2px;
      width: 0;
      background: linear-gradient(90deg, #667eea, #764ba2);
      border-radius: 1px;
      transition: width 0.3s ease;
    }
  }

  .login-options {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;

    .remember-checkbox {
      :deep(.el-checkbox__label) {
        color: #606266;
        font-size: 14px;
      }
    }
  }

  .login-button {
    width: 100%;
    height: 48px;
    font-size: 16px;
    font-weight: 600;
    border-radius: 12px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    box-shadow: 0 8px 24px rgba(102, 126, 234, 0.4);
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 12px 32px rgba(102, 126, 234, 0.5);
    }

    &:active {
      transform: translateY(0);
    }

    &.is-loading {
      opacity: 0.8;
    }
  }
}

// 底部
.login-footer {
  padding: 16px 24px 0;

  .footer-divider {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;

    span:first-child,
    span:last-child {
      flex: 1;
      height: 1px;
      background: #f0f0f0;
    }

    .text {
      font-size: 12px;
      color: #c0c4cc;
    }
  }

  .default-account {
    text-align: center;
    padding-bottom: 16px;
  }
}

// 版权
.login-copyright {
  position: absolute;
  bottom: 20px;
  left: 0;
  width: 100%;
  text-align: center;
  z-index: 10;

  p {
    margin: 0;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.7);
  }
}

// 响应式
@media (max-width: 480px) {
  .login-card {
    width: calc(100% - 32px);
    margin: 16px;
  }

  .brand-title {
    font-size: 24px !important;
  }
}
</style>
