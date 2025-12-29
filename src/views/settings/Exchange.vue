<template>
  <div class="exchange-container">
    <el-page-header @back="goBack" class="header">
      <template #content>
        <span class="title">交易所设置</span>
      </template>
    </el-page-header>

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

      <el-table :data="exchanges" stripe style="width: 100%">
        <el-table-column prop="display_name" label="名称" width="180" />
        <el-table-column prop="exchange_name" label="类型" width="120">
          <template #default="{ row }">
            <el-tag :type="getExchangeTypeColor(row.exchange_name)">
              {{ getExchangeTypeLabel(row.exchange_name) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="is_testnet" label="网络" width="100">
          <template #default="{ row }">
            <el-tag :type="row.is_testnet ? 'warning' : 'success'" size="small">
              {{ row.is_testnet ? '测试网' : '主网' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-switch
              :model-value="row.status === 'active'"
              @change="handleToggleStatus(row)"
              :loading="row._toggling"
            />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button
              type="primary"
              size="small"
              :icon="Edit"
              @click="handleEdit(row)"
            >
              编辑
            </el-button>
            <el-button
              type="danger"
              size="small"
              :icon="Delete"
              @click="handleDelete(row)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty
        v-if="!loading && exchanges.length === 0"
        description="暂无交易所配置"
      />
    </el-card>

    <!-- Add/Edit Dialog -->
    <el-dialog
      v-model="dialogVisible"
      :title="isEditing ? '编辑交易所' : '添加交易所'"
      width="600px"
      @close="handleDialogClose"
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="120px"
      >
        <el-form-item label="名称" prop="display_name">
          <el-input
            v-model="formData.display_name"
            placeholder="例如: Binance 主账号"
            clearable
          />
        </el-form-item>

        <el-form-item label="交易所类型" prop="exchange_name">
          <el-select
            v-model="formData.exchange_name"
            placeholder="请选择交易所"
            :disabled="isEditing"
            style="width: 100%"
          >
            <el-option label="Binance" value="binance" />
            <el-option label="OKX" value="okx" />
            <el-option label="Bybit" value="bybit" />
          </el-select>
        </el-form-item>

        <el-form-item label="API Key" prop="api_key">
          <el-input
            v-model="formData.api_key"
            type="password"
            show-password
            placeholder="请输入 API Key"
            clearable
          />
        </el-form-item>

        <el-form-item label="API Secret" prop="api_secret">
          <el-input
            v-model="formData.api_secret"
            type="password"
            show-password
            placeholder="请输入 API Secret"
            clearable
          />
        </el-form-item>

        <el-form-item
          v-if="formData.exchange_name === 'okx'"
          label="Passphrase"
          prop="passphrase"
        >
          <el-input
            v-model="formData.passphrase"
            type="password"
            show-password
            placeholder="请输入 Passphrase (OKX 需要)"
            clearable
          />
        </el-form-item>

        <el-form-item label="测试网" prop="is_testnet">
          <el-switch
            v-model="formData.is_testnet"
            active-text="启用"
            inactive-text="禁用"
          />
          <div class="form-tip">测试网用于测试，不会产生真实交易</div>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button
          type="primary"
          :loading="saving"
          @click="handleSave"
        >
          保存
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import {
  Plus,
  Edit,
  Delete,
} from '@element-plus/icons-vue';
import { exchangeApi, type ExchangeConfig } from '@/api/tauri';
import { useUserStore } from '@/store/modules/user';

const router = useRouter();
const userStore = useUserStore();
const formRef = ref<FormInstance>();
const loading = ref(false);
const saving = ref(false);
const dialogVisible = ref(false);
const isEditing = ref(false);
const exchanges = ref<(ExchangeConfig & { _toggling?: boolean })[]>([]);

// Form data
const formData = reactive<ExchangeConfig>({
  exchange_name: 'binance',
  display_name: '',
  api_key: '',
  api_secret: '',
  passphrase: '',
  is_testnet: false,
});

// Form validation rules
const formRules: FormRules = {
  display_name: [
    { required: true, message: '请输入交易所名称', trigger: 'blur' },
  ],
  exchange_name: [
    { required: true, message: '请选择交易所类型', trigger: 'change' },
  ],
  api_key: [
    { required: true, message: '请输入 API Key', trigger: 'blur' },
    { min: 10, message: 'API Key 长度不正确', trigger: 'blur' },
  ],
  api_secret: [
    { required: true, message: '请输入 API Secret', trigger: 'blur' },
    { min: 10, message: 'API Secret 长度不正确', trigger: 'blur' },
  ],
  passphrase: [
    {
      validator: (_rule, value, callback) => {
        if (formData.exchange_name === 'okx' && !value) {
          callback(new Error('OKX 交易所需要 Passphrase'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    },
  ],
};

// Get exchange type label
const getExchangeTypeLabel = (type: string): string => {
  const labels: Record<string, string> = {
    binance: 'Binance',
    okx: 'OKX',
    bybit: 'Bybit',
  };
  return labels[type] || type;
};

// Get exchange type color
const getExchangeTypeColor = (type: string): string => {
  const colors: Record<string, string> = {
    binance: 'success',
    okx: 'primary',
    bybit: 'warning',
  };
  return colors[type] || 'info';
};

// Go back
const goBack = () => {
  router.back();
};

// Load exchanges
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

// Handle add
const handleAdd = () => {
  isEditing.value = false;
  Object.assign(formData, {
    exchange_name: 'binance',
    display_name: '',
    api_key: '',
    api_secret: '',
    passphrase: '',
    is_testnet: false,
  });
  dialogVisible.value = true;
};

// Handle edit
const handleEdit = (row: ExchangeConfig) => {
  isEditing.value = true;
  Object.assign(formData, row);
  dialogVisible.value = true;
};

// Handle save
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
      // Update
      await exchangeApi.update(formData.id, formData);
      ElMessage.success('交易所配置已更新');
    } else {
      // Add
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

// Handle delete
const handleDelete = async (row: ExchangeConfig) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除交易所 "${row.display_name}" 吗？此操作不可撤销。`,
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

// Handle toggle status
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

// Handle dialog close
const handleDialogClose = () => {
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
  overflow: auto;
}

.header {
  margin-bottom: 20px;
}

.title {
  font-size: 20px;
  font-weight: 600;
}

.exchange-card {
  flex: 1;
  overflow: auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.form-tip {
  margin-top: 4px;
  font-size: 12px;
  color: #909399;
  line-height: 1.5;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #606266;
}

/* Dark theme adaptation */
.dark .form-tip {
  color: #a3a6ad;
}

.dark :deep(.el-form-item__label) {
  color: #cfd3dc;
}
</style>
