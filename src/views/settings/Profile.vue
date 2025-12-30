<template>
  <div class="profile-container">
    <!-- 页面头部 -->
    <div class="profile-header">
      <h1 class="page-title">个人资料</h1>
      <p class="page-subtitle">管理您的个人信息和偏好设置</p>
    </div>

    <el-row :gutter="24">
      <!-- 左侧：头像卡片 -->
      <el-col :xs="24" :sm="24" :md="8" :lg="6">
        <el-card shadow="never" class="avatar-card">
          <div class="avatar-section">
            <div class="avatar-wrapper">
              <el-avatar :size="120" :src="avatarUrl" :icon="UserFilled" />
              <el-button
                class="avatar-edit-btn"
                type="primary"
                circle
                size="small"
                @click="showAvatarDialog = true"
              >
                <el-icon><Edit /></el-icon>
              </el-button>
            </div>
            <h3 class="user-display-name">{{ displayName || userStore.username }}</h3>
            <p class="user-role-badge">
              <el-tag :type="getRoleTagType(userStore.roleName)" size="small">
                {{ userStore.roleName }}
              </el-tag>
            </p>
            <p class="user-status">
              <el-badge
                :value="statusText"
                :type="userStore.user?.status === UserStatus.ACTIVE ? 'success' : 'danger'"
                class="status-badge"
              />
            </p>
          </div>

          <!-- 账户信息 -->
          <div class="account-info">
            <div class="info-item">
              <span class="info-label">用户ID</span>
              <span class="info-value">{{ userStore.user?.id || '-' }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">用户名</span>
              <span class="info-value">{{ userStore.username }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">注册时间</span>
              <span class="info-value">{{ formatDate(userStore.user?.createdAt) }}</span>
            </div>
          </div>
        </el-card>
      </el-col>

      <!-- 右侧：信息编辑 -->
      <el-col :xs="24" :sm="24" :md="16" :lg="18">
        <!-- 基本信息 -->
        <el-card shadow="never" class="info-card">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><User /></el-icon>
              <span>基本信息</span>
            </div>
          </template>

          <el-form
            ref="profileFormRef"
            :model="profileForm"
            :rules="profileRules"
            label-width="100px"
            class="profile-form"
          >
            <el-form-item label="用户名">
              <el-input v-model="userStore.username" disabled />
              <template #error>
                <span></span>
              </template>
            </el-form-item>

            <el-form-item label="显示名称" prop="displayName">
              <el-input
                v-model="profileForm.displayName"
                placeholder="请输入显示名称"
                clearable
                maxlength="50"
                show-word-limit
              />
              <template #error>
                <span></span>
              </template>
            </el-form-item>

            <el-form-item label="个人简介" prop="bio">
              <el-input
                v-model="profileForm.bio"
                type="textarea"
                :rows="4"
                placeholder="介绍一下自己..."
                maxlength="200"
                show-word-limit
              />
            </el-form-item>

            <el-form-item>
              <el-button type="primary" :loading="saving" @click="saveProfile">
                保存更改
              </el-button>
              <el-button @click="resetForm">重置</el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <!-- 偏好设置 -->
        <el-card shadow="never" class="info-card">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Setting /></el-icon>
              <span>偏好设置</span>
            </div>
          </template>

          <el-form label-width="100px" class="preferences-form">
            <el-form-item label="界面语言">
              <el-select v-model="preferences.language" placeholder="选择语言">
                <el-option label="简体中文" value="zh-CN" />
                <el-option label="English" value="en-US" />
              </el-select>
            </el-form-item>

            <el-form-item label="时区设置">
              <el-select v-model="preferences.timezone" placeholder="选择时区">
                <el-option label="UTC+8 北京时间" value="Asia/Shanghai" />
                <el-option label="UTC+0 伦敦时间" value="Europe/London" />
                <el-option label="UTC-5 纽约时间" value="America/New_York" />
              </el-select>
            </el-form-item>

            <el-form-item label="通知设置">
              <div class="notification-settings">
                <el-checkbox v-model="preferences.emailNotification">邮件通知</el-checkbox>
                <el-checkbox v-model="preferences.tradeNotification">交易通知</el-checkbox>
                <el-checkbox v-model="preferences.riskAlert">风险预警</el-checkbox>
              </div>
            </el-form-item>
          </el-form>
        </el-card>

        <!-- 安全设置 -->
        <el-card shadow="never" class="info-card security-card">
          <template #header>
            <div class="card-header">
              <el-icon class="header-icon"><Lock /></el-icon>
              <span>安全设置</span>
            </div>
          </template>

          <div class="security-section">
            <div class="security-item">
              <div class="security-info">
                <h4>修改密码</h4>
                <p class="security-desc">定期修改密码可以提高账户安全性</p>
              </div>
              <el-button type="primary" plain @click="showPasswordDialog = true">
                修改密码
              </el-button>
            </div>

            <el-divider />

            <div class="security-item">
              <div class="security-info">
                <h4>最近登录</h4>
                <p class="security-desc">{{ formatDate(userStore.user?.updatedAt) }}</p>
              </div>
              <el-tag type="success">当前设备</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 头像编辑对话框 -->
    <el-dialog
      v-model="showAvatarDialog"
      title="更换头像"
      width="400px"
      :close-on-click-modal="false"
    >
      <div class="avatar-dialog-content">
        <el-upload
          class="avatar-uploader"
          :show-file-list="false"
          :before-upload="beforeAvatarUpload"
          :http-request="handleAvatarUpload"
          accept="image/*"
        >
          <img v-if="avatarPreview" :src="avatarPreview" class="avatar-preview" />
          <el-icon v-else class="avatar-uploader-icon"><Plus /></el-icon>
        </el-upload>
        <div class="avatar-tips">
          <p>支持 JPG、PNG 格式</p>
          <p>建议尺寸 200x200 像素</p>
          <p>文件大小不超过 2MB</p>
        </div>
      </div>
      <template #footer>
        <el-button @click="showAvatarDialog = false">取消</el-button>
        <el-button type="primary" :disabled="!avatarPreview" @click="confirmAvatar">
          确认
        </el-button>
      </template>
    </el-dialog>

    <!-- 修改密码对话框 -->
    <el-dialog
      v-model="showPasswordDialog"
      title="修改密码"
      width="450px"
      :close-on-click-modal="false"
    >
      <el-form
        ref="passwordFormRef"
        :model="passwordForm"
        :rules="passwordRules"
        label-width="100px"
      >
        <el-form-item label="当前密码" prop="oldPassword">
          <el-input
            v-model="passwordForm.oldPassword"
            type="password"
            placeholder="请输入当前密码"
            show-password
          />
        </el-form-item>
        <el-form-item label="新密码" prop="newPassword">
          <el-input
            v-model="passwordForm.newPassword"
            type="password"
            placeholder="请输入新密码"
            show-password
          />
        </el-form-item>
        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input
            v-model="passwordForm.confirmPassword"
            type="password"
            placeholder="请再次输入新密码"
            show-password
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showPasswordDialog = false">取消</el-button>
        <el-button type="primary" :loading="changingPassword" @click="changePassword">
          确认修改
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue';
import { ElMessage, type FormInstance, type FormRules, type UploadRequestOptions } from 'element-plus';
import {
  UserFilled,
  User,
  Setting,
  Lock,
  Edit,
  Plus,
} from '@element-plus/icons-vue';
import { useUserStore } from '@/store';
import { UserStatus } from '@/types';
import type { UserProfile, UserPreferences } from '@/types';

const userStore = useUserStore();

// 表单引用
const profileFormRef = ref<FormInstance>();
const passwordFormRef = ref<FormInstance>();

// 状态
const saving = ref(false);
const changingPassword = ref(false);
const showAvatarDialog = ref(false);
const showPasswordDialog = ref(false);
const avatarUrl = ref('');
const avatarPreview = ref('');
const tempAvatarFile = ref<File | null>(null);

// 个人信息表单
const profileForm = reactive<UserProfile>({
  displayName: userStore.user?.displayName || '',
  bio: '',
});

// 偏好设置
const preferences = reactive<UserPreferences>({
  language: 'zh-CN',
  timezone: 'Asia/Shanghai',
  emailNotification: true,
  tradeNotification: true,
  riskAlert: true,
});

// 密码表单
const passwordForm = reactive({
  oldPassword: '',
  newPassword: '',
  confirmPassword: '',
});

// 计算属性
const displayName = computed(() => profileForm.displayName || userStore.user?.displayName);

const statusText = computed(() => {
  const status = userStore.user?.status;
  if (status === UserStatus.ACTIVE) return '正常';
  if (status === UserStatus.DISABLED) return '已禁用';
  if (status === UserStatus.LOCKED) return '已锁定';
  return '未知';
});

// 表单验证规则
const profileRules: FormRules = {
  displayName: [
    { min: 2, max: 50, message: '显示名称长度在 2 到 50 个字符', trigger: 'blur' },
  ],
};

const validateConfirmPassword = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('请再次输入密码'));
  } else if (value !== passwordForm.newPassword) {
    callback(new Error('两次输入密码不一致'));
  } else {
    callback();
  }
};

const passwordRules: FormRules = {
  oldPassword: [
    { required: true, message: '请输入当前密码', trigger: 'blur' },
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, max: 20, message: '密码长度在 6 到 20 个字符', trigger: 'blur' },
  ],
  confirmPassword: [
    { required: true, validator: validateConfirmPassword, trigger: 'blur' },
  ],
};

// 方法
function getRoleTagType(roleName: string): 'success' | 'warning' | 'info' | 'danger' | '' {
  const roleMap: Record<string, 'success' | 'warning' | 'info' | 'danger' | ''> = {
    '管理员': 'danger',
    '开发者': 'warning',
    '交易员': 'success',
    '审计员': 'info',
  };
  return roleMap[roleName] || '';
}

function formatDate(timestamp?: number): string {
  if (!timestamp) return '-';
  const date = new Date(timestamp);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

async function saveProfile() {
  if (!profileFormRef.value) return;

  try {
    await profileFormRef.value.validate();
    saving.value = true;

    // TODO: 调用 API 保存用户资料
    // await api.userApi.updateProfile(profileForm);

    // 模拟保存
    await new Promise(resolve => setTimeout(resolve, 500));

    ElMessage.success('保存成功');
  } catch (error) {
    console.error('Save profile failed:', error);
  } finally {
    saving.value = false;
  }
}

function resetForm() {
  profileForm.displayName = userStore.user?.displayName || '';
  profileForm.bio = '';
  profileFormRef.value?.clearValidate();
}

function beforeAvatarUpload(file: File): boolean {
  const isImage = file.type.startsWith('image/');
  const isLt2M = file.size / 1024 / 1024 < 2;

  if (!isImage) {
    ElMessage.error('只能上传图片文件！');
    return false;
  }
  if (!isLt2M) {
    ElMessage.error('图片大小不能超过 2MB！');
    return false;
  }

  return true;
}

function handleAvatarUpload(options: UploadRequestOptions): void {
  const file = options.file;
  tempAvatarFile.value = file;

  // 创建预览
  const reader = new FileReader();
  reader.onload = (e) => {
    avatarPreview.value = e.target?.result as string;
  };
  reader.readAsDataURL(file);
}

function confirmAvatar() {
  if (!tempAvatarFile.value) return;

  // TODO: 调用 API 上传头像
  // const formData = new FormData();
  // formData.append('avatar', tempAvatarFile.value);
  // await api.userApi.uploadAvatar(formData);

  avatarUrl.value = avatarPreview.value;
  showAvatarDialog.value = false;
  tempAvatarFile.value = null;
  ElMessage.success('头像更新成功');
}

async function changePassword() {
  if (!passwordFormRef.value) return;

  try {
    await passwordFormRef.value.validate();
    changingPassword.value = true;

    // TODO: 调用 API 修改密码
    // await api.userApi.changePassword(passwordForm);

    // 模拟修改
    await new Promise(resolve => setTimeout(resolve, 500));

    ElMessage.success('密码修改成功，请重新登录');
    showPasswordDialog.value = false;

    // 重置表单
    passwordForm.oldPassword = '';
    passwordForm.newPassword = '';
    passwordForm.confirmPassword = '';
    passwordFormRef.value.clearValidate();
  } catch (error) {
    console.error('Change password failed:', error);
  } finally {
    changingPassword.value = false;
  }
}

onMounted(() => {
  // 初始化表单数据
  if (userStore.user) {
    profileForm.displayName = userStore.user.displayName || '';
  }
});
</script>

<style scoped lang="scss">
.profile-container {
  max-width: 1200px;
  margin: 0 auto;
}

.profile-header {
  margin-bottom: 24px;

  .page-title {
    font-size: 24px;
    font-weight: 700;
    color: #303133;
    margin: 0 0 8px 0;
  }

  .page-subtitle {
    font-size: 14px;
    color: #909399;
    margin: 0;
  }
}

// 头像卡片
.avatar-card {
  margin-bottom: 24px;
  position: sticky;
  top: 24px;

  :deep(.el-card__body) {
    padding: 24px;
  }
}

.avatar-section {
  text-align: center;

  .avatar-wrapper {
    position: relative;
    display: inline-block;
    margin-bottom: 16px;

    .avatar-edit-btn {
      position: absolute;
      bottom: 0;
      right: 0;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    }
  }

  .user-display-name {
    font-size: 18px;
    font-weight: 600;
    color: #303133;
    margin: 0 0 8px 0;
  }

  .user-role-badge {
    margin: 8px 0;
  }

  .user-status {
    margin-top: 12px;
  }
}

.account-info {
  margin-top: 24px;
  padding-top: 24px;
  border-top: 1px solid #ebeef5;

  .info-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 0;

    .info-label {
      font-size: 13px;
      color: #909399;
    }

    .info-value {
      font-size: 14px;
      color: #303133;
      font-weight: 500;
    }
  }
}

// 信息卡片
.info-card {
  margin-bottom: 24px;

  :deep(.el-card__header) {
    padding: 16px 20px;
    background: #fafbfc;
    border-bottom: 1px solid #ebeef5;
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: #303133;

    .header-icon {
      font-size: 18px;
      color: #409eff;
    }
  }

  :deep(.el-card__body) {
    padding: 24px;
  }
}

.profile-form,
.preferences-form {
  max-width: 600px;
}

.notification-settings {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

// 安全设置卡片
.security-card {
  .security-section {
    .security-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 8px 0;

      .security-info {
        h4 {
          font-size: 15px;
          font-weight: 600;
          color: #303133;
          margin: 0 0 4px 0;
        }

        .security-desc {
          font-size: 13px;
          color: #909399;
          margin: 0;
        }
      }
    }
  }
}

// 头像上传对话框
.avatar-dialog-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;

  .avatar-uploader {
    :deep(.el-upload) {
      border: 2px dashed #d9d9d9;
      border-radius: 12px;
      cursor: pointer;
      position: relative;
      overflow: hidden;
      transition: all 0.3s;
      width: 200px;
      height: 200px;
      display: flex;
      align-items: center;
      justify-content: center;

      &:hover {
        border-color: #409eff;
      }
    }
  }

  .avatar-preview {
    width: 200px;
    height: 200px;
    object-fit: cover;
    display: block;
  }

  .avatar-uploader-icon {
    font-size: 48px;
    color: #8c939d;
  }

  .avatar-tips {
    text-align: center;
    color: #909399;
    font-size: 13px;
    line-height: 1.8;
  }
}

// 响应式
@media (max-width: 768px) {
  .profile-container {
    .profile-header {
      .page-title {
        font-size: 20px;
      }
    }
  }

  .avatar-card {
    position: static;
  }

  .profile-form,
  .preferences-form {
    max-width: 100%;
  }

  .profile-form :deep(.el-form-item__label) {
    width: 80px !important;
  }
}
</style>
