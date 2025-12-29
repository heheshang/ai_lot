<template>
  <div class="exchange-container">
    <el-page-header @back="goBack" class="header">
      <template #content>
        <span class="title">交易所设置</span>
      </template>
    </el-page-header>

    <!-- Statistics Cards -->
    <div class="stats-row">
      <div class="stat-card stat-primary">
        <div class="stat-icon">
          <el-icon><Coin /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-label">交易所总数</div>
          <div class="stat-value">{{ exchanges.length }}</div>
        </div>
      </div>

      <div class="stat-card stat-success">
        <div class="stat-icon">
          <el-icon><CircleCheck /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-label">活跃中</div>
          <div class="stat-value">{{ activeCount }}</div>
        </div>
      </div>

      <div class="stat-card stat-warning">
        <div class="stat-icon">
          <el-icon><Clock /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-label">已禁用</div>
          <div class="stat-value">{{ inactiveCount }}</div>
        </div>
      </div>

      <div class="stat-card stat-info">
        <div class="stat-icon">
          <el-icon><Warning /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-label">测试网</div>
          <div class="stat-value">{{ testnetCount }}</div>
        </div>
      </div>
    </div>

    <!-- Exchange List -->
    <el-card v-loading="loading" class="exchange-card">
      <template #header>
        <div class="card-header">
          <span>交易所列表</span>
          <el-button
            type="primary"
            :icon="Plus"
            @click="handleAdd"
          >
            添加交易所
          </el-button>
        </div>
      </template>

      <!-- Search and Filter -->
      <div class="filter-bar">
        <el-input
          v-model="searchText"
          placeholder="搜索交易所名称..."
          :prefix-icon="Search"
          clearable
          class="search-input"
        />
        <el-select
          v-model="filterType"
          placeholder="全部类型"
          clearable
          class="filter-select"
        >
          <el-option label="Binance" value="binance" />
          <el-option label="OKX" value="okx" />
          <el-option label="Bybit" value="bybit" />
        </el-select>
        <el-select
          v-model="filterStatus"
          placeholder="全部状态"
          clearable
          class="filter-select"
        >
          <el-option label="活跃" value="active" />
          <el-option label="已禁用" value="inactive" />
          <el-option label="停用" value="disabled" />
        </el-select>
        <el-select
          v-model="filterNetwork"
          placeholder="全部网络"
          clearable
          class="filter-select"
        >
          <el-option label="主网" value="mainnet" />
          <el-option label="测试网" value="testnet" />
        </el-select>
      </div>

      <!-- Table -->
      <el-table
        :data="filteredExchanges"
        stripe
        style="width: 100%"
        :empty-text="!loading && filteredExchanges.length === 0 ? (hasFilters ? '未找到匹配的交易所' : '暂无交易所配置') : '加载中...'"
      >
        <el-table-column prop="displayName" label="名称" min-width="160">
          <template #default="{ row }">
            <div class="name-cell">
              <el-icon class="exchange-icon" :class="`icon-${row.exchangeName}`">
                <component :is="getExchangeIcon(row.exchangeName)" />
              </el-icon>
              <div class="name-info">
                <div class="name">{{ row.displayName }}</div>
                <div class="api-key-masked">{{ row.apiKeyMasked || '未配置' }}</div>
              </div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="exchangeName" label="类型" width="110">
          <template #default="{ row }">
            <el-tag :type="getExchangeTypeColor(row.exchangeName)" size="small">
              {{ getExchangeTypeLabel(row.exchangeName) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="isTestnet" label="网络" width="90">
          <template #default="{ row }">
            <el-tag :type="row.isTestnet ? 'warning' : 'success'" size="small">
              {{ row.isTestnet ? '测试网' : '主网' }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusColor(row.status)" size="small">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="createdAt" label="创建时间" width="110">
          <template #default="{ row }">
            <span class="time-text">{{ formatDate(row.createdAt) }}</span>
          </template>
        </el-table-column>

        <el-table-column prop="updatedAt" label="更新时间" width="110">
          <template #default="{ row }">
            <span class="time-text">{{ formatDate(row.updatedAt) }}</span>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="220" fixed="right">
          <template #default="{ row }">
            <el-button-group>
              <el-button
                type="primary"
                size="small"
                :icon="Edit"
                @click="handleEdit(row)"
              >
                编辑
              </el-button>
              <el-dropdown trigger="click" @command="(cmd: string) => handleCommand(cmd, row)">
                <el-button size="small">
                  更多<el-icon class="el-icon--right"><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item :command="'clone'" :icon="DocumentCopy">
                      克隆配置
                    </el-dropdown-item>
                    <el-dropdown-item
                      :command="'toggle'"
                      :icon="row.status === 'active' ? Lock : Unlock"
                      :divided="true"
                    >
                      {{ row.status === 'active' ? '禁用' : '启用' }}
                    </el-dropdown-item>
                    <el-dropdown-item
                      :command="'delete'"
                      :icon="Delete"
                      class="danger-item"
                    >
                      删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>

      <!-- Empty State with Guide -->
      <el-empty
        v-if="!loading && exchanges.length === 0"
        description="暂无交易所配置"
      >
        <template #image>
          <div class="empty-icon">
            <el-icon :size="80"><Coin /></el-icon>
          </div>
        </template>
        <el-button type="primary" :icon="Plus" @click="handleAdd">
          添加第一个交易所
        </el-button>
      </el-empty>
    </el-card>

    <!-- Add/Edit Dialog with Multi-Step Form -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEditing ? '编辑交易所配置' : '添加交易所配置'"
      width="680px"
      @close="handleDialogClose"
      :close-on-click-modal="false"
      class="exchange-dialog"
      destroy-on-close
    >
      <!-- Loading State for Edit -->
      <div v-if="editLoading" class="edit-loading">
        <el-icon class="is-loading" :size="32"><Loading /></el-icon>
        <p>正在加载交易所配置...</p>
      </div>

      <!-- Dialog Content -->
      <div v-else class="dialog-content-wrapper">
        <!-- Progress Steps -->
        <div class="form-progress">
          <el-steps :active="currentStep" align-center finish-status="success">
            <el-step title="基本信息" />
            <el-step title="API 配置" />
            <el-step title="网络设置" />
            <el-step title="确认" />
          </el-steps>
        </div>

        <!-- Dialog Header Description -->
        <div class="dialog-description">
          <el-alert
            :type="isEditing ? 'info' : 'success'"
            :closable="false"
            show-icon
          >
            <template #title>
              <span>{{ isEditing ? '修改现有交易所的API配置' : '添加新的交易所API配置' }}</span>
            </template>
          </el-alert>
        </div>

        <el-form
          ref="formRef"
          :model="formData"
          :rules="formRules"
          label-width="115px"
          class="exchange-form"
        >
        <!-- Step 1: Basic Info -->
        <div v-show="currentStep === 0" class="step-content">
          <div class="form-section">
            <div class="section-title">
              <el-icon><Coin /></el-icon>
              <span>基本信息</span>
              <el-tag size="small" type="info">步骤 1/3</el-tag>
            </div>

            <el-form-item label="显示名称" prop="displayName">
              <el-input
                v-model="formData.displayName"
                placeholder="例如：Binance 主账号、OKX 现货账户"
                clearable
                maxlength="50"
                show-word-limit
                size="large"
              >
                <template #prefix>
                  <el-icon><Edit /></el-icon>
                </template>
              </el-input>
              <div class="form-help">用于区分不同的交易所账户，支持中英文</div>
            </el-form-item>

            <el-form-item label="交易所" prop="exchangeName">
              <div class="exchange-selector">
                <el-radio-group v-model="formData.exchangeName" class="exchange-radio-group" @change="handleExchangeChange">
                  <el-radio-button label="binance" class="exchange-radio-button">
                    <div class="radio-content">
                      <div class="exchange-logo binance-logo">B</div>
                      <span class="exchange-label">Binance</span>
                      <el-tag size="small" type="success">推荐</el-tag>
                    </div>
                  </el-radio-button>
                  <el-radio-button label="okx" class="exchange-radio-button">
                    <div class="radio-content">
                      <div class="exchange-logo okx-logo">O</div>
                      <span class="exchange-label">OKX</span>
                      <el-tag size="small" type="primary">常用</el-tag>
                    </div>
                  </el-radio-button>
                  <el-radio-button label="bybit" class="exchange-radio-button">
                    <div class="radio-content">
                      <div class="exchange-logo bybit-logo">B</div>
                      <span class="exchange-label">Bybit</span>
                      <el-tag size="small" type="warning">新兴</el-tag>
                    </div>
                  </el-radio-button>
                </el-radio-group>
              </div>
              <div class="form-help">选择您使用的交易所平台</div>
            </el-form-item>

            <!-- Exchange Info Card -->
            <div class="exchange-info-card">
              <div class="info-card-header">
                <el-icon><InfoFilled /></el-icon>
                <span>交易所说明</span>
              </div>
              <div class="info-card-content">
                <div v-if="formData.exchangeName === 'binance'">
                  <p><strong>Binance（币安）</strong> - 全球最大的加密货币交易所之一</p>
                  <ul class="feature-list">
                    <li>✓ 支持 spot、margin、期货交易</li>
                    <li>✓ API 稳定性好，文档完善</li>
                    <li>✓ 支持 WebSocket 实时数据</li>
                  </ul>
                </div>
                <div v-else-if="formData.exchangeName === 'okx'">
                  <p><strong>OKX（欧易）</strong> - 一站式加密货币交易平台</p>
                  <ul class="feature-list">
                    <li>✓ 需要 Passphrase 进行 API 认证</li>
                    <li>✓ 支持多种交易模式</li>
                    <li>✓ 低手续费，高性能</li>
                  </ul>
                </div>
                <div v-else-if="formData.exchangeName === 'bybit'">
                  <p><strong>Bybit</strong> - 专业衍生品交易平台</p>
                  <ul class="feature-list">
                    <li>✓ 专注于合约交易</li>
                    <li>✓ API 响应速度快</li>
                    <li>✓ 支持反向合约</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 2: API Credentials -->
        <div v-show="currentStep === 1" class="step-content">
          <div class="form-section">
            <div class="section-title">
              <el-icon><Lock /></el-icon>
              <span>API 凭证</span>
              <el-tag size="small" type="danger" effect="plain">加密存储</el-tag>
              <el-tag size="small" type="info">步骤 2/3</el-tag>
            </div>

            <!-- Password Visibility Toggle -->
            <div class="password-toggle-bar">
              <el-switch
                v-model="showPasswords"
                active-text="显示密码"
                inactive-text="隐藏密码"
                size="small"
              />
              <el-tooltip content="显示所有密码字段（仅用于确认，请注意周围环境）" placement="top">
                <el-icon class="help-icon"><QuestionFilled /></el-icon>
              </el-tooltip>
            </div>

            <el-form-item label="API Key" prop="apiKey">
              <el-input
                v-model="formData.apiKey"
                :type="showPasswords ? 'text' : 'password'"
                :show-password="!showPasswords"
                placeholder="粘贴您的 API Key"
                clearable
                autocomplete="off"
                size="large"
              >
                <template #prefix>
                  <el-icon><Key /></el-icon>
                </template>
                <template #suffix>
                  <el-tooltip content="从剪贴板粘贴" placement="top">
                    <el-icon class="paste-icon" @click="pasteToField('apiKey')"><DocumentCopy /></el-icon>
                  </el-tooltip>
                </template>
              </el-input>
              <div class="form-help">
                <span>在交易所后台创建 API Key，需要开启现货交易权限</span>
                <el-link
                  :href="getApiHelpUrl(formData.exchangeName)"
                  target="_blank"
                  type="primary"
                  :underline="false"
                  class="help-link"
                >
                  <el-icon><QuestionFilled /></el-icon>
                  如何获取？
                </el-link>
              </div>
            </el-form-item>

            <el-form-item label="API Secret" prop="apiSecret">
              <el-input
                v-model="formData.apiSecret"
                :type="showPasswords ? 'text' : 'password'"
                :show-password="!showPasswords"
                placeholder="粘贴您的 API Secret"
                clearable
                autocomplete="off"
                size="large"
              >
                <template #prefix>
                  <el-icon><Key /></el-icon>
                </template>
                <template #suffix>
                  <el-tooltip content="从剪贴板粘贴" placement="top">
                    <el-icon class="paste-icon" @click="pasteToField('apiSecret')"><DocumentCopy /></el-icon>
                  </el-tooltip>
                </template>
              </el-input>
              <div class="form-help">
                创建 API Key 时生成的 Secret，仅在创建时显示一次
              </div>
            </el-form-item>

            <el-form-item
              v-if="formData.exchangeName === 'okx'"
              label="Passphrase"
              prop="passphrase"
            >
              <el-input
                v-model="formData.passphrase"
                :type="showPasswords ? 'text' : 'password'"
                :show-password="!showPasswords"
                placeholder="粘贴您的 Passphrase"
                clearable
                autocomplete="off"
                size="large"
              >
                <template #prefix>
                  <el-icon><Key /></el-icon>
                </template>
                <template #suffix>
                  <el-tooltip content="从剪贴板粘贴" placement="top">
                    <el-icon class="paste-icon" @click="pasteToField('passphrase')"><DocumentCopy /></el-icon>
                  </el-tooltip>
                </template>
              </el-input>
              <div class="form-help">
                <el-alert type="warning" :closable="false" show-icon class="inline-alert">
                  OKX 交易所创建 API Key 时需要设置 Passphrase
                </el-alert>
              </div>
            </el-form-item>

            <!-- Quick Fill Template -->
            <div class="quick-fill-section">
              <div class="quick-fill-header">
                <el-icon><DocumentCopy /></el-icon>
                <span>快速填充</span>
              </div>
              <el-button
                type="primary"
                plain
                size="small"
                @click="fillTestCredentials"
              >
                填充测试凭证（演示用）
              </el-button>
            </div>
          </div>
        </div>

        <!-- Step 3: Network Settings -->
        <div v-show="currentStep === 2" class="step-content">
          <div class="form-section">
            <div class="section-title">
              <el-icon><Setting /></el-icon>
              <span>网络设置</span>
              <el-tag size="small" type="info">步骤 3/3</el-tag>
            </div>

            <el-form-item label="网络类型" prop="isTestnet">
              <div class="network-switch">
                <el-switch
                  v-model="formData.isTestnet"
                  active-text="测试网"
                  inactive-text="主网"
                  size="large"
                  inline-prompt
                  class="network-toggle"
                  @change="handleNetworkChange"
                />
              </div>
              <div class="form-help network-help">
                <el-alert
                  :type="formData.isTestnet ? 'warning' : 'success'"
                  :closable="false"
                  show-icon
                  class="network-alert"
                >
                  <template v-if="formData.isTestnet">
                    <strong>测试网模式</strong> - 用于测试策略，不产生真实交易，使用测试资金
                  </template>
                  <template v-else>
                    <strong>主网模式</strong> - 真实交易环境，使用真实资金，请谨慎操作
                  </template>
                </el-alert>
              </div>
            </el-form-item>

            <!-- Network Warning for Mainnet -->
            <div v-if="!formData.isTestnet" class="mainnet-warning">
              <el-alert type="error" :closable="false" show-icon>
                <template #title>
                  ⚠️ 警告：主网模式使用真实资金
                </template>
                <template #default>
                  <ul class="warning-list">
                    <li>所有交易将使用真实资金</li>
                    <li>请确保已充分测试策略</li>
                    <li>建议从小额开始逐步增加投入</li>
                    <li>请妥善保管 API 密钥</li>
                  </ul>
                </template>
              </el-alert>
            </div>
          </div>
        </div>

        <!-- Step 4: Confirmation -->
        <div v-show="currentStep === 3" class="step-content">
          <div class="form-section">
            <div class="section-title">
              <el-icon><CircleCheck /></el-icon>
              <span>确认配置</span>
              <el-tag size="small" type="info">最后检查</el-tag>
            </div>

            <div class="confirmation-summary">
              <div class="summary-item">
                <span class="summary-label">交易所类型：</span>
                <el-tag :type="getExchangeTypeColor(formData.exchangeName)">
                  {{ getExchangeTypeLabel(formData.exchangeName) }}
                </el-tag>
              </div>
              <div class="summary-item">
                <span class="summary-label">显示名称：</span>
                <span class="summary-value">{{ formData.displayName || '-' }}</span>
              </div>
              <div class="summary-item">
                <span class="summary-label">API Key：</span>
                <span class="summary-value masked">{{ maskApiKey(formData.apiKey) }}</span>
              </div>
              <div class="summary-item">
                <span class="summary-label">API Secret：</span>
                <span class="summary-value masked">•••••••••••••</span>
              </div>
              <div v-if="formData.exchangeName === 'okx'" class="summary-item">
                <span class="summary-label">Passphrase：</span>
                <span class="summary-value masked">•••••••••••••</span>
              </div>
              <div class="summary-item">
                <span class="summary-label">网络类型：</span>
                <el-tag :type="formData.isTestnet ? 'warning' : 'success'">
                  {{ formData.isTestnet ? '测试网' : '主网' }}
                </el-tag>
              </div>
            </div>

            <!-- Final Security Notice -->
            <div class="security-notice">
              <el-alert
                type="info"
                :closable="false"
                show-icon
              >
                <template #default>
                  <div class="notice-content">
                    <p><strong>安全提示：</strong></p>
                    <ul>
                      <li>API 密钥将使用 AES-256 加密存储在本地</li>
                      <li>建议创建仅用于交易的 API Key，不要开启提现权限</li>
                      <li>定期更换 API Key 以确保账户安全</li>
                      <li>请妥善保管您的 API 密钥，不要泄露给他人</li>
                    </ul>
                  </div>
                </template>
              </el-alert>
            </div>
          </div>
        </div>
        </el-form>
      </div>

      <template #footer>
        <div class="dialog-footer">
          <div class="footer-navigation">
            <el-button
              v-if="currentStep > 0"
              size="large"
              @click="previousStep"
            >
              <el-icon><ArrowLeft /></el-icon>
              上一步
            </el-button>
            <span v-else></span>
          </div>

          <div class="footer-center" v-if="showValidationSummary">
            <el-alert
              v-if="validationErrors.length > 0"
              type="error"
              :closable="false"
              show-icon
              class="validation-alert"
            >
              请完善必填项：{{ validationErrors.join('、') }}
            </el-alert>
          </div>

          <div class="footer-actions">
            <el-button size="large" @click="dialogVisible = false">
              <el-icon><Close /></el-icon>
              取消
            </el-button>
            <el-button
              v-if="currentStep < 3"
              type="primary"
              size="large"
              @click="nextStep"
            >
              下一步<el-icon><ArrowRight /></el-icon>
            </el-button>
            <el-button
              v-else
              type="primary"
              size="large"
              :loading="saving"
              @click="handleSave"
            >
              <el-icon v-if="!saving"><Check /></el-icon>
              {{ isEditing ? '保存修改' : '添加交易所' }}
            </el-button>
          </div>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import {
  Plus,
  Edit,
  Delete,
  Coin,
  CircleCheck,
  Clock,
  Warning,
  Search,
  DocumentCopy,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Unlock,
  Lock,
  InfoFilled,
  Key,
  Setting,
  QuestionFilled,
  Close,
  Check,
  Loading,
} from '@element-plus/icons-vue';
import { exchangeApi, type ExchangeConfig } from '@/api/tauri';
import { useUserStore } from '@/store/modules/user';

const router = useRouter();
const userStore = useUserStore();
const formRef = ref<FormInstance>();
const loading = ref(false);
const editLoading = ref(false);
const saving = ref(false);
const dialogVisible = ref(false);
const isEditing = ref(false);
const exchanges = ref<(ExchangeConfig & { _toggling?: boolean })[]>([]);

// Multi-step form state
const currentStep = ref(0);
const showPasswords = ref(false);
const showValidationSummary = ref(false);
const validationErrors = ref<string[]>([]);

// Search and filter
const searchText = ref('');
const filterType = ref('');
const filterStatus = ref('');
const filterNetwork = ref('');

// Form data
const formData = reactive<ExchangeConfig>({
  exchangeName: 'binance',
  displayName: '',
  apiKey: '',
  apiSecret: '',
  passphrase: '',
  isTestnet: false,
});

// Form validation rules
const formRules: FormRules = {
  displayName: [
    { required: true, message: '请输入交易所名称', trigger: 'blur' },
    { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' },
  ],
  exchangeName: [
    { required: true, message: '请选择交易所类型', trigger: 'change' },
  ],
  apiKey: [
    { required: true, message: '请输入 API Key', trigger: 'blur' },
    { min: 10, message: 'API Key 长度不正确', trigger: 'blur' },
  ],
  apiSecret: [
    { required: true, message: '请输入 API Secret', trigger: 'blur' },
    { min: 10, message: 'API Secret 长度不正确', trigger: 'blur' },
  ],
  passphrase: [
    {
      validator: (_rule, value, callback) => {
        if (formData.exchangeName === 'okx' && !value) {
          callback(new Error('OKX 交易所需要 Passphrase'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    },
  ],
};

// Computed properties
const activeCount = computed(() =>
  exchanges.value.filter(e => e.status === 'active').length
);

const inactiveCount = computed(() =>
  exchanges.value.filter(e => e.status === 'inactive' || e.status === 'disabled').length
);

const testnetCount = computed(() =>
  exchanges.value.filter(e => e.isTestnet).length
);

const hasFilters = computed(() =>
  searchText.value || filterType.value || filterStatus.value || filterNetwork.value
);

const filteredExchanges = computed(() => {
  return exchanges.value.filter(exchange => {
    // Search by name
    if (searchText.value) {
      const search = searchText.value.toLowerCase();
      if (!exchange.displayName?.toLowerCase().includes(search)) {
        return false;
      }
    }

    // Filter by type
    if (filterType.value && exchange.exchangeName !== filterType.value) {
      return false;
    }

    // Filter by status
    if (filterStatus.value && exchange.status !== filterStatus.value) {
      return false;
    }

    // Filter by network
    if (filterNetwork.value === 'testnet' && !exchange.isTestnet) {
      return false;
    }
    if (filterNetwork.value === 'mainnet' && exchange.isTestnet) {
      return false;
    }

    return true;
  });
});

// Helper functions
const getExchangeTypeLabel = (type: string): string => {
  const labels: Record<string, string> = {
    binance: 'Binance',
    okx: 'OKX',
    bybit: 'Bybit',
  };
  return labels[type] || type;
};

const getExchangeTypeColor = (type: string): string => {
  const colors: Record<string, string> = {
    binance: 'success',
    okx: 'primary',
    bybit: 'warning',
  };
  return colors[type] || 'info';
};

const getExchangeIcon = (_type: string) => {
  return Coin;
};

const getStatusLabel = (status: string): string => {
  const labels: Record<string, string> = {
    active: '活跃',
    inactive: '已禁用',
    disabled: '停用',
  };
  return labels[status] || status;
};

const getStatusColor = (status: string): string => {
  const colors: Record<string, string> = {
    active: 'success',
    inactive: 'info',
    disabled: 'danger',
  };
  return colors[status] || 'info';
};

const formatDate = (timestamp?: number): string => {
  if (!timestamp) return '-';
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (days === 0) return '今天';
  if (days === 1) return '昨天';
  if (days < 7) return `${days}天前`;
  if (days < 30) return `${Math.floor(days / 7)}周前`;
  return date.toLocaleDateString('zh-CN');
};

const getApiHelpUrl = (exchange: string): string => {
  const urls: Record<string, string> = {
    binance: 'https://www.binance.com/zh-CN/support/faq/how-to-create-api-keys-on-binance-360002502072',
    okx: 'https://www.okx.com/zh-hans/help/iii-create-an-api-key',
    bybit: 'https://help.bybit.com/hc/en-us/articles/360010425273-How-to-create-API-key',
  };
  return urls[exchange] || '#';
};

const maskApiKey = (key?: string): string => {
  if (!key) return '-';
  if (key.length <= 8) return '****';
  return `${key.substring(0, 4)}${'*'.repeat(Math.min(key.length - 4, 12))}`;
};

// Multi-step form functions
const nextStep = async () => {
  // Validate current step fields
  const stepFields = getStepFields(currentStep.value);

  if (stepFields.length > 0) {
    try {
      // Validate only the specified fields
      for (const field of stepFields) {
        await formRef.value?.validateField(field);
      }
      validationErrors.value = [];
      showValidationSummary.value = false;
    } catch (error: any) {
      // Extract field name from error message
      const fieldMap: Record<string, string> = {
        displayName: '显示名称',
        exchangeName: '交易所',
        apiKey: 'API Key',
        apiSecret: 'API Secret',
        passphrase: 'Passphrase',
      };
      const fieldName = error?.field || Object.keys(formRef.value?.fields || {})[0] || '';
      validationErrors.value = fieldName ? [fieldMap[fieldName] || fieldName] : ['请完善必填项'];
      showValidationSummary.value = true;
      return;
    }
  }

  if (currentStep.value < 3) {
    currentStep.value++;
  }
};

const previousStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--;
    validationErrors.value = [];
    showValidationSummary.value = false;
  }
};

const getStepFields = (step: number): string[] => {
  const stepFields: Record<number, string[]> = {
    0: ['displayName', 'exchangeName'],
    1: ['apiKey', 'apiSecret', 'passphrase'],
    2: [],
    3: [],
  };
  return stepFields[step] || [];
};

const handleExchangeChange = () => {
  // Reset passphrase when switching away from OKX
  if (formData.exchangeName !== 'okx') {
    formData.passphrase = '';
  }
};

const handleNetworkChange = () => {
  // Can add special handling for network type changes
};

const pasteToField = async (field: 'apiKey' | 'apiSecret' | 'passphrase') => {
  try {
    const text = await navigator.clipboard.readText();
    if (field === 'apiKey') {
      formData.apiKey = text;
    } else if (field === 'apiSecret') {
      formData.apiSecret = text;
    } else if (field === 'passphrase') {
      formData.passphrase = text;
    }
    ElMessage.success('已从剪贴板粘贴');
  } catch {
    ElMessage.warning('无法访问剪贴板，请手动粘贴');
  }
};

const fillTestCredentials = () => {
  formData.apiKey = 'test_api_key_12345678';
  formData.apiSecret = 'test_secret_12345678';
  if (formData.exchangeName === 'okx') {
    formData.passphrase = 'test_passphrase';
  }
  ElMessage.success('已填充测试凭证（仅供演示）');
};

// Actions
const goBack = () => {
  router.back();
};

const loadExchanges = async () => {
  loading.value = true;
  try {
    const userId = userStore.user?.id || 'default';
    const result = await exchangeApi.list(userId);
    exchanges.value = result;
  } catch (error) {
    ElMessage.error(`加载交易所列表失败: ${error}`);
  } finally {
    loading.value = false;
  }
};

const handleAdd = () => {
  isEditing.value = false;
  currentStep.value = 0;
  showPasswords.value = false;
  showValidationSummary.value = false;
  validationErrors.value = [];
  Object.assign(formData, {
    exchangeName: 'binance',
    displayName: '',
    apiKey: '',
    apiSecret: '',
    passphrase: '',
    isTestnet: false,
  });
  dialogVisible.value = true;
};

const handleEdit = async (row: ExchangeConfig) => {
  if (!row.id) return;

  // Show loading indicator for edit operation
  editLoading.value = true;
  dialogVisible.value = true;
  isEditing.value = true;
  currentStep.value = 0;
  showPasswords.value = false;
  showValidationSummary.value = false;
  validationErrors.value = [];

  try {
    // Fetch full config with decrypted keys
    const detail = await exchangeApi.getDetail(row.id);

    Object.assign(formData, {
      id: detail.id,
      exchangeName: detail.exchangeName,
      displayName: detail.displayName,
      apiKey: detail.apiKey,
      apiSecret: detail.apiSecret,
      passphrase: detail.passphrase || '',
      isTestnet: detail.isTestnet,
    });
  } catch (error) {
    ElMessage.error(`获取交易所配置失败: ${error}`);
    dialogVisible.value = false;
  } finally {
    editLoading.value = false;
  }
};

const handleCommand = async (command: string, row: ExchangeConfig & { _toggling?: boolean }) => {
  switch (command) {
    case 'clone':
      await handleClone(row);
      break;
    case 'toggle':
      await handleToggleStatus(row);
      break;
    case 'delete':
      await handleDelete(row);
      break;
  }
};

const handleClone = async (row: ExchangeConfig) => {
  if (!row.id) return;

  try {
    // Fetch full config with decrypted keys
    const detail = await exchangeApi.getDetail(row.id);

    // Reset for adding as new
    isEditing.value = false;
    currentStep.value = 0;
    showPasswords.value = false;
    showValidationSummary.value = false;
    validationErrors.value = [];

    // Clone the data without ID, and modify display name
    Object.assign(formData, {
      id: undefined, // Remove ID to create new record
      exchangeName: detail.exchangeName,
      displayName: `${detail.displayName} - 副本`, // Append "副本" suffix
      apiKey: detail.apiKey,
      apiSecret: detail.apiSecret,
      passphrase: detail.passphrase || '',
      isTestnet: detail.isTestnet,
    });

    dialogVisible.value = true;
    ElMessage.success('已克隆配置，请确认后保存');
  } catch (error) {
    ElMessage.error(`克隆配置失败: ${error}`);
  }
};

const handleSave = async () => {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
  } catch {
    return;
  }

  saving.value = true;
  try {
    const userId = userStore.user?.id || 'default';

    if (isEditing.value && formData.id) {
      await exchangeApi.update(formData.id, formData);
      ElMessage.success('交易所配置已更新');
    } else {
      const id = await exchangeApi.add(userId, formData);
      ElMessage.success(`交易所配置已添加，ID: ${id}`);
    }

    dialogVisible.value = false;
    await loadExchanges();
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`);
  } finally {
    saving.value = false;
  }
};

const handleDelete = async (row: ExchangeConfig) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除交易所 "${row.displayName}" 吗？此操作不可撤销。`,
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    await exchangeApi.delete(row.id!);
    ElMessage.success('交易所配置已删除');
    await loadExchanges();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`删除失败: ${error}`);
    }
  }
};

const handleToggleStatus = async (row: ExchangeConfig & { _toggling?: boolean }) => {
  row._toggling = true;
  const newStatus = row.status === 'active' ? 'inactive' : 'active';

  try {
    await exchangeApi.updateStatus(row.id!, newStatus);
    ElMessage.success(newStatus === 'active' ? '已启用' : '已禁用');
    row.status = newStatus;
  } catch (error) {
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    row._toggling = false;
  }
};

const handleDialogClose = () => {
  currentStep.value = 0;
  showPasswords.value = false;
  showValidationSummary.value = false;
  validationErrors.value = [];
  formRef.value?.resetFields();
};

// Lifecycle
onMounted(() => {
  loadExchanges();
});
</script>

<style scoped>
.exchange-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px;
  gap: 20px;
  overflow: auto;
}

.header {
  flex-shrink: 0;
}

.title {
  font-size: 20px;
  font-weight: 600;
}

/* Statistics Cards */
.stats-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  flex-shrink: 0;
}

.stat-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  color: #fff;
  flex-shrink: 0;
}

.stat-primary .stat-icon {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
}

.stat-success .stat-icon {
  background: linear-gradient(135deg, #26a69a 0%, #00897b 100%);
}

.stat-warning .stat-icon {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
}

.stat-info .stat-icon {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
}

.stat-content {
  flex: 1;
}

.stat-label {
  font-size: 13px;
  color: #909399;
  margin-bottom: 4px;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: #303133;
}

/* Exchange Card */
.exchange-card {
  flex: 1;
  overflow: auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

/* Filter Bar */
.filter-bar {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.search-input {
  flex: 1;
  min-width: 200px;
  max-width: 300px;
}

.filter-select {
  width: 140px;
}

/* Table Styles */
.name-cell {
  display: flex;
  align-items: center;
  gap: 12px;
}

.exchange-icon {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: #fff;
}

.icon-binance {
  background: linear-gradient(135deg, #f0b90b 0%, #d4a20a 100%);
}

.icon-okx {
  background: linear-gradient(135deg, #2f85f8 0%, #1a6dd9 100%);
}

.icon-bybit {
  background: linear-gradient(135deg, #f7931a 0%, #e07e0d 100%);
}

.name-info {
  flex: 1;
  min-width: 0;
}

.name {
  font-weight: 500;
  color: #303133;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.api-key-masked {
  font-size: 12px;
  color: #909399;
  margin-top: 2px;
}

.time-text {
  font-size: 13px;
  color: #909399;
}

/* Empty State */
.empty-icon {
  color: #c0c4cc;
  margin-bottom: 20px;
}

/* Enhanced Dialog Styles */
:deep(.exchange-dialog) {
  border-radius: 16px;
}

:deep(.exchange-dialog .el-dialog__header) {
  padding: 20px 24px 12px;
  margin: 0;
}

:deep(.exchange-dialog .el-dialog__body) {
  padding: 0 24px 24px;
  max-height: 75vh;
  overflow-y: auto;
}

:deep(.exchange-dialog .el-dialog__footer) {
  padding: 16px 24px;
  border-top: 1px solid #ebeef5;
  background: #fafafa;
}

.dialog-description {
  margin-bottom: 20px;
}

.dialog-description :deep(.el-alert) {
  border-radius: 8px;
}

.dialog-description :deep(.el-alert__title) {
  font-size: 14px;
}

/* Progress Steps */
.form-progress {
  margin-bottom: 24px;
  padding: 0 20px;
}

/* Form Sections */
.exchange-form {
  padding: 0;
}

.form-section {
  margin-bottom: 24px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 12px;
  border: 1px solid #ebeef5;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 16px;
  padding-bottom: 12px;
  border-bottom: 2px solid #e4e7ed;
}

.section-title .el-icon {
  font-size: 18px;
  color: #409eff;
}

.section-title .el-tag {
  margin-left: auto;
}

/* Step Content */
.step-content {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Form Items */
:deep(.exchange-form .el-form-item) {
  margin-bottom: 20px;
}

:deep(.exchange-form .el-form-item__label) {
  font-weight: 500;
  color: #606266;
  font-size: 14px;
}

:deep(.exchange-form .el-input__wrapper) {
  border-radius: 8px;
  transition: all 0.3s ease;
}

:deep(.exchange-form .el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px var(--el-color-primary) inset;
}

:deep(.exchange-form .el-input__prefix) {
  color: #909399;
}

/* Form Help Text */
.form-help {
  margin-top: 6px;
  font-size: 12px;
  color: #606266;
  line-height: 1.6;
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.help-link {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
}

.help-link:hover {
  opacity: 0.8;
}

.paste-icon {
  cursor: pointer;
  color: #909399;
  transition: color 0.3s;
}

.paste-icon:hover {
  color: #409eff;
}

/* Exchange Selector */
.exchange-radio-group {
  display: flex;
  width: 100%;
  gap: 8px;
}

.exchange-radio-button {
  flex: 1;
}

.exchange-radio-button :deep(.el-radio-button__inner) {
  width: 100%;
  padding: 12px;
  border-radius: 8px;
  height: auto;
  border: 2px solid #dcdfe6;
  background: #fff;
  transition: all 0.3s ease;
}

.exchange-radio-button :deep(.el-radio-button__inner:hover) {
  border-color: #409eff;
  background: #ecf5ff;
}

.exchange-radio-button.is-active :deep(.el-radio-button__inner) {
  border-color: #409eff;
  background: #ecf5ff;
  color: #303133;
}

.radio-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.exchange-logo {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: 700;
  color: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.binance-logo {
  background: linear-gradient(135deg, #f0b90b 0%, #d4a20a 100%);
}

.okx-logo {
  background: linear-gradient(135deg, #2f85f8 0%, #1a6dd9 100%);
}

.bybit-logo {
  background: linear-gradient(135deg, #f7931a 0%, #e07e0d 100%);
}

.exchange-label {
  font-size: 14px;
  font-weight: 500;
}

/* Exchange Info Card */
.exchange-info-card {
  margin-top: 20px;
  padding: 16px;
  background: #e7f3ff;
  border-radius: 8px;
  border: 1px solid #b3d8ff;
}

.info-card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: #0056b3;
  margin-bottom: 12px;
}

.info-card-content p {
  margin: 0 0 8px 0;
  font-weight: 500;
}

.feature-list {
  margin: 0;
  padding-left: 20px;
  list-style: none;
}

.feature-list li {
  margin-bottom: 4px;
  font-size: 13px;
  color: #0056b3;
}

/* Password Toggle Bar */
.password-toggle-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  padding: 12px;
  background: #fff7e6;
  border-radius: 8px;
  border: 1px solid #ffe7ba;
}

.help-icon {
  color: #e6a23c;
  cursor: help;
}

/* Quick Fill Section */
.quick-fill-section {
  margin-top: 20px;
  padding: 16px;
  background: #f0f9ff;
  border-radius: 8px;
  border: 1px solid #c3feff;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.quick-fill-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: #0369a1;
}

/* Network Switch */
.network-switch {
  display: flex;
  justify-content: center;
  padding: 12px 0;
}

.network-toggle :deep(.el-switch__core) {
  height: 28px;
  min-width: 80px;
  border-radius: 14px;
}

.network-toggle :deep(.el-switch__action) {
  height: 22px;
  width: 22px;
}

.network-help {
  margin-top: 12px;
}

.network-alert :deep(.el-alert__content) {
  font-size: 13px;
}

/* Mainnet Warning */
.mainnet-warning {
  margin-top: 16px;
}

.warning-list {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.warning-list li {
  margin-bottom: 4px;
  font-size: 13px;
}

/* Inline Alert */
.inline-alert {
  padding: 8px 12px;
  border-radius: 6px;
}

.inline-alert :deep(.el-alert__content) {
  font-size: 12px;
}

/* Confirmation Summary */
.confirmation-summary {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  border: 1px solid #ebeef5;
}

.summary-item {
  display: flex;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid #f5f5f5;
}

.summary-item:last-child {
  border-bottom: none;
}

.summary-label {
  font-weight: 500;
  color: #606266;
  min-width: 120px;
}

.summary-value {
  flex: 1;
  color: #303133;
}

.summary-value.masked {
  font-family: monospace;
  color: #909399;
}

/* Security Notice */
.security-notice {
  margin-top: 16px;
}

.notice-content p {
  margin: 0 0 8px 0;
}

.notice-content ul {
  margin: 0;
  padding-left: 20px;
}

.notice-content li {
  margin-bottom: 6px;
  line-height: 1.6;
}

/* Edit Loading State */
.edit-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #909399;
}

.edit-loading .el-icon {
  color: #409eff;
  margin-bottom: 16px;
}

.edit-loading p {
  margin: 0;
  font-size: 14px;
}

/* Dialog Footer */
.dialog-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-navigation {
  display: flex;
  gap: 12px;
}

.footer-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.footer-actions {
  display: flex;
  gap: 12px;
}

.footer-actions .el-button {
  min-width: 120px;
}

.validation-alert {
  max-width: 400px;
}

/* Dropdown Menu */
:deep(.danger-item) {
  color: #ef5350;
}

/* Dark Theme */
.dark .stat-value,
.dark .name {
  color: #e5e7eb;
}

.dark .api-key-masked,
.dark .time-text,
.dark .stat-label {
  color: #9ca3af;
}

.dark :deep(.el-form-item__label) {
  color: #cfd3dc;
}

.dark .stat-card {
  background: #1f2937;
}

/* Dark Theme - Dialog */
.dark :deep(.exchange-dialog .el-dialog__footer) {
  background: #1f2937;
  border-top-color: #374151;
}

.dark .form-section {
  background: #1f2937;
  border-color: #374151;
}

.dark .section-title {
  color: #e5e7eb;
  border-bottom-color: #374151;
}

.dark .form-help {
  color: #9ca3af;
}

.dark :deep(.exchange-form .el-input__wrapper) {
  background: #374151;
}

.dark .exchange-radio-button :deep(.el-radio-button__inner) {
  background: #374151;
  border-color: #4b5563;
}

.dark .exchange-radio-button.is-active :deep(.el-radio-button__inner) {
  background: #1e3a8a;
  border-color: #3b82f6;
}

.dark .exchange-info-card {
  background: #1e3a5f;
  border-color: #1e40af;
}

.dark .info-card-header {
  color: #93c5fd;
}

.dark .feature-list li {
  color: #93c5fd;
}

.dark .confirmation-summary {
  background: #374151;
  border-color: #4b5563;
}

.dark .summary-label {
  color: #9ca3af;
}

.dark .summary-value {
  color: #e5e7eb;
}
</style>
