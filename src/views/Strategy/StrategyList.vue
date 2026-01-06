<template>
  <div class="strategy-list">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">策略管理</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>策略列表</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button type="primary" @click="createStrategy" class="create-btn">
          <el-icon><Plus /></el-icon>
          新建策略
        </el-button>
      </div>
    </div>

    <!-- 统计卡片 -->
    <el-row :gutter="16" class="stats-row">
      <el-col :span="6" v-for="(stat, index) in statCards" :key="stat.key">
        <div
          class="stat-card"
          :class="[stat.type, { 'stat-enter': statsEnter }]"
          :style="{ transitionDelay: `${index * 80}ms` }"
        >
          <div class="stat-bg"></div>
          <div class="stat-content">
            <div class="stat-icon" :class="stat.type">
              <el-icon :size="26"><component :is="stat.icon" /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stat.value }}</div>
              <div class="stat-label">{{ stat.label }}</div>
            </div>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- 筛选和搜索栏 -->
    <el-card class="filter-card" shadow="never">
      <el-row :gutter="16">
        <el-col :span="6">
          <el-input
            v-model="searchText"
            placeholder="搜索策略名称或描述"
            clearable
            class="search-input"
            @input="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </el-col>
        <el-col :span="4">
          <el-select v-model="filterCategory" placeholder="策略分类" clearable @change="handleFilter">
            <el-option label="全部" value="" />
            <el-option label="趋势跟踪" value="trend" />
            <el-option label="均值回归" value="mean_reversion" />
            <el-option label="套利" value="arbitrage" />
            <el-option label="网格交易" value="grid" />
            <el-option label="高频交易" value="high_frequency" />
          </el-select>
        </el-col>
        <el-col :span="4">
          <el-select v-model="filterStatus" placeholder="策略状态" clearable @change="handleFilter">
            <el-option label="全部" value="" />
            <el-option label="草稿" value="draft" />
            <el-option label="测试中" value="testing" />
            <el-option label="已激活" value="active" />
            <el-option label="已归档" value="archived" />
          </el-select>
        </el-col>
        <el-col :span="6">
          <el-select v-model="sortBy" placeholder="排序方式" @change="handleSort">
            <el-option label="创建时间" value="created_at" />
            <el-option label="更新时间" value="updated_at" />
            <el-option label="策略名称" value="name" />
            <el-option label="版本号" value="version" />
          </el-select>
        </el-col>
        <el-col :span="4">
          <div class="view-toggle">
            <el-radio-group v-model="viewMode" size="default">
              <el-radio-button label="card">
                <el-icon><Grid /></el-icon>
              </el-radio-button>
              <el-radio-button label="list">
                <el-icon><List /></el-icon>
              </el-radio-button>
            </el-radio-group>
          </div>
        </el-col>
      </el-row>
    </el-card>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <div class="loading-spinner">
        <div class="spinner"></div>
        <div class="spinner-ring"></div>
      </div>
      <p>加载策略列表中...</p>
    </div>

    <!-- 空状态 -->
    <div v-else-if="filteredStrategies.length === 0" class="empty-container">
      <div class="empty-illustration">
        <svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
          <circle cx="100" cy="100" r="80" fill="url(#grad1)" opacity="0.1"/>
          <circle cx="100" cy="100" r="60" fill="url(#grad1)" opacity="0.15"/>
          <path d="M70 85 L100 65 L130 85 V115 H70 V85Z" fill="url(#grad1)" opacity="0.3"/>
          <rect x="85" y="95" width="30" height="35" rx="2" fill="url(#grad1)"/>
          <defs>
            <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
              <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
            </linearGradient>
          </defs>
        </svg>
      </div>
      <h3 class="empty-title">
        {{ searchText || filterCategory || filterStatus ? '未找到符合条件的策略' : '暂无策略' }}
      </h3>
      <p class="empty-desc">
        {{ searchText || filterCategory || filterStatus ? '试试调整筛选条件' : '创建您的第一个量化交易策略' }}
      </p>
      <el-button v-if="!searchText && !filterCategory && !filterStatus" type="primary" @click="createStrategy" class="empty-btn">
        <el-icon><Plus /></el-icon>
        创建第一个策略
      </el-button>
    </div>

    <!-- 卡片视图 -->
    <div v-else-if="viewMode === 'card'" class="strategy-cards">
      <TransitionGroup name="card">
        <el-card
          v-for="(strategy, index) in paginatedStrategies"
          :key="strategy.id"
          class="strategy-card"
          :class="`card-${index % 4}`"
          shadow="hover"
          @click="viewStrategy(strategy.id)"
        >
          <div class="card-header">
            <div class="strategy-icon" :style="{ background: getStrategyColor(strategy.category || '') }">
              <el-icon :size="24"><component :is="getStrategyIcon(strategy.category || '')" /></el-icon>
            </div>
            <div class="strategy-info">
              <h3 class="strategy-name">{{ strategy.name }}</h3>
              <div class="strategy-meta">
                <el-tag size="small" type="info" effect="plain">{{ getCategoryText(strategy.category || '') }}</el-tag>
                <el-tag size="small" :type="getStatusType(strategy.status)" effect="plain">
                  {{ getStatusText(strategy.status) }}
                </el-tag>
                <span class="version-text">v{{ strategy.version }}</span>
              </div>
            </div>
          </div>

          <div class="card-body">
            <p class="strategy-description">{{ strategy.description || '暂无描述' }}</p>

            <div class="strategy-tags" v-if="strategy.tags && strategy.tags.length > 0">
              <TransitionGroup name="tag">
                <el-tag
                  v-for="tag in strategy.tags.slice(0, 3)"
                  :key="tag"
                  size="small"
                  type="warning"
                  effect="plain"
                  class="tag-item"
                >
                  #{{ tag }}
                </el-tag>
              </TransitionGroup>
              <span v-if="strategy.tags.length > 3" class="more-tags">
                +{{ strategy.tags.length - 3 }}
              </span>
            </div>

            <div class="strategy-dates">
              <div class="date-item">
                <el-icon><Calendar /></el-icon>
                <span>创建: {{ formatDate(strategy.createdAt) }}</span>
              </div>
              <div class="date-item">
                <el-icon><Clock /></el-icon>
                <span>更新: {{ formatDate(strategy.updatedAt) }}</span>
              </div>
            </div>
          </div>

          <div class="card-footer">
            <el-button-group>
              <el-button size="small" type="primary" @click.stop="editStrategy(strategy.id)">
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
              <el-button size="small" @click.stop="copyStrategy(strategy)">
                <el-icon><CopyDocument /></el-icon>
                复制
              </el-button>
              <el-dropdown trigger="click" @command="(cmd: string) => handleAction(cmd, strategy)">
                <el-button size="small">
                  <el-icon><MoreFilled /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="backtest">
                      <el-icon><DataAnalysis /></el-icon>
                      回测
                    </el-dropdown-item>
                    <el-dropdown-item command="export">
                      <el-icon><Download /></el-icon>
                      导出
                    </el-dropdown-item>
                    <el-dropdown-item command="delete" divided>
                      <el-icon><Delete /></el-icon>
                      删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </el-button-group>
          </div>
        </el-card>
      </TransitionGroup>
    </div>

    <!-- 列表视图 -->
    <el-card v-else class="table-card" shadow="never">
      <el-table :data="paginatedStrategies" stripe style="width: 100%" class="strategy-table">
        <el-table-column prop="name" label="策略名称" min-width="180">
          <template #default="{ row }">
            <div class="table-name">
              <div
                class="strategy-icon-small"
                :style="{ background: getStrategyColor(row.category || '') }"
              >
                <el-icon :size="16">
                  <component :is="getStrategyIcon(row.category || '')" />
                </el-icon>
              </div>
              <div>
                <div class="name-text">{{ row.name }}</div>
                <div class="name-desc">{{ row.description || '暂无描述' }}</div>
              </div>
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="category" label="分类" width="120">
          <template #default="{ row }">
            <el-tag size="small" type="info" effect="plain">{{ getCategoryText(row.category) }}</el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag size="small" :type="getStatusType(row.status)" effect="plain">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="version" label="版本" width="80">
          <template #default="{ row }">
            <span class="version-text">v{{ row.version }}</span>
          </template>
        </el-table-column>

        <el-table-column prop="createdAt" label="创建时间" width="160">
          <template #default="{ row }">
            {{ formatDateTime(row.createdAt) }}
          </template>
        </el-table-column>

        <el-table-column prop="updatedAt" label="更新时间" width="160">
          <template #default="{ row }">
            {{ formatDateTime(row.updatedAt) }}
          </template>
        </el-table-column>

        <el-table-column label="操作" width="220" fixed="right">
          <template #default="{ row }">
            <el-button-group size="small">
              <el-button type="primary" link @click="editStrategy(row.id)">
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
              <el-button type="primary" link @click="copyStrategy(row)">
                <el-icon><CopyDocument /></el-icon>
                复制
              </el-button>
              <el-dropdown trigger="click" @command="(cmd: string) => handleAction(cmd, row)">
                <el-button type="primary" link>
                  <el-icon><MoreFilled /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="backtest">
                      <el-icon><DataAnalysis /></el-icon>
                      回测
                    </el-dropdown-item>
                    <el-dropdown-item command="export">
                      <el-icon><Download /></el-icon>
                      导出
                    </el-dropdown-item>
                    <el-dropdown-item command="delete" divided>
                      <el-icon><Delete /></el-icon>
                      删除
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </el-button-group>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 分页 -->
    <div v-if="filteredStrategies.length > 0" class="pagination-container">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="[12, 24, 48, 96]"
        :total="filteredStrategies.length"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handlePageChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import {
  Plus,
  Search,
  Grid,
  List,
  Edit,
  CopyDocument,
  Delete,
  MoreFilled,
  DataAnalysis,
  Download,
  Calendar,
  Clock,
  Document,
  EditPen,
  CircleCheck,
} from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { strategyApi } from '@/api/tauri';
import { useUserStore } from '@/store/modules/user';
import type { Strategy } from '@/types';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 状态
const loading = ref(false);
const viewMode = ref<'card' | 'list'>('card');
const searchText = ref('');
const filterCategory = ref('');
const filterStatus = ref('');
const sortBy = ref('created_at');
const currentPage = ref(1);
const pageSize = ref(12);
const statsEnter = ref(false);

const strategies = ref<Strategy[]>([]);

// 统计卡片配置
const statCards = computed(() => [
  { key: 'total', label: '总策略数', value: stats.value.total, icon: Document, type: 'total' },
  { key: 'draft', label: '草稿', value: stats.value.draft, icon: EditPen, type: 'draft' },
  { key: 'testing', label: '测试中', value: stats.value.testing, icon: DataAnalysis, type: 'testing' },
  { key: 'active', label: '已激活', value: stats.value.active, icon: CircleCheck, type: 'active' },
]);

// 统计数据
const stats = computed(() => {
  const total = strategies.value.length;
  const draft = strategies.value.filter(s => s.status === 'draft').length;
  const testing = strategies.value.filter(s => s.status === 'testing').length;
  const active = strategies.value.filter(s => s.status === 'active').length;
  return { total, draft, testing, active };
});

// 过滤和排序
const filteredStrategies = computed(() => {
  let result = [...strategies.value];

  if (searchText.value) {
    const keyword = searchText.value.toLowerCase();
    result = result.filter(s =>
      s.name.toLowerCase().includes(keyword) ||
      (s.description && s.description.toLowerCase().includes(keyword)) ||
      (s.tags && s.tags.some(tag => tag.toLowerCase().includes(keyword)))
    );
  }

  if (filterCategory.value) {
    result = result.filter(s => s.category === filterCategory.value);
  }

  if (filterStatus.value) {
    result = result.filter(s => s.status === filterStatus.value);
  }

  result.sort((a, b) => {
    if (sortBy.value === 'name') {
      return a.name.localeCompare(b.name, 'zh-CN');
    } else if (sortBy.value === 'version') {
      return b.version - a.version;
    } else {
      const key = sortBy.value as 'createdAt' | 'updatedAt';
      return b[key] - a[key];
    }
  });

  return result;
});

// 分页
const paginatedStrategies = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return filteredStrategies.value.slice(start, start + pageSize.value);
});

// 方法
const getStrategyIcon = (category: string) => {
  const icons: Record<string, any> = {
    trend: 'TrendCharts',
    mean_reversion: 'RefreshLeft',
    arbitrage: 'Switch',
    grid: 'Grid',
    high_frequency: 'Lightning',
  };
  return icons[category] || 'Document';
};

const getStrategyColor = (category: string) => {
  const colors: Record<string, string> = {
    trend: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    mean_reversion: 'linear-gradient(135deg, #84fab0 0%, #8fd3f4 100%)',
    arbitrage: 'linear-gradient(135deg, #fccb90 0%, #d57eeb 100%)',
    grid: 'linear-gradient(135deg, #ff9a9e 0%, #fecfef 100%)',
    high_frequency: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
  };
  return colors[category] || 'linear-gradient(135deg, #a1c4fd 0%, #c2e9fb 100%)';
};

const getStatusType = (status: string) => {
  const typeMap: Record<string, any> = {
    draft: 'info',
    testing: 'warning',
    active: 'success',
    archived: 'info',
  };
  return typeMap[status] || 'info';
};

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    draft: '草稿',
    testing: '测试中',
    active: '已激活',
    archived: '已归档',
  };
  return statusMap[status] || status;
};

const getCategoryText = (category: string) => {
  const categoryMap: Record<string, string> = {
    trend: '趋势跟踪',
    mean_reversion: '均值回归',
    arbitrage: '套利',
    grid: '网格交易',
    high_frequency: '高频交易',
  };
  return categoryMap[category] || category;
};

const formatDate = (timestamp: number) => {
  // 验证时间戳有效性
  if (!timestamp || timestamp <= 0) return '--';

  const date = new Date(timestamp * 1000);

  // 检查日期是否有效
  if (isNaN(date.getTime())) return '--';

  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (days === 0) return '今天';
  if (days === 1) return '昨天';
  if (days < 7) return `${days}天前`;
  if (days < 30) return `${Math.floor(days / 7)}周前`;
  return date.toLocaleDateString('zh-CN');
};

const formatDateTime = (timestamp: number) => {
  // 验证时间戳有效性
  if (!timestamp || timestamp <= 0) return '--';

  const date = new Date(timestamp * 1000);

  // 检查日期是否有效
  if (isNaN(date.getTime())) return '--';

  return date.toLocaleString('zh-CN');
};

const handleSearch = () => { currentPage.value = 1; };
const handleFilter = () => { currentPage.value = 1; };
const handleSort = () => { currentPage.value = 1; };
const handleSizeChange = () => { currentPage.value = 1; };
const handlePageChange = () => { window.scrollTo({ top: 0, behavior: 'smooth' }); };

const createStrategy = () => { router.push('/strategy/editor'); };
const viewStrategy = (id: string) => { router.push(`/strategy/editor/${id}`); };
const editStrategy = (id: string) => { router.push(`/strategy/editor/${id}`); };

const copyStrategy = async (strategy: Strategy) => {
  try {
    const copy: Partial<Strategy> = {
      ...strategy,
      id: '',
      name: `${strategy.name} - 副本`,
      status: 'draft',
      version: 1,
      createdAt: 0,
      updatedAt: 0,
    };
    await strategyApi.save(copy as Strategy);
    ElMessage.success('策略复制成功');
    await loadStrategies();
  } catch (error) {
    console.error('Failed to copy strategy:', error);
    ElMessage.error('策略复制失败');
  }
};

const handleAction = async (command: string, strategy: Strategy) => {
  switch (command) {
    case 'backtest':
      router.push(`/backtest?strategyId=${strategy.id}`);
      break;
    case 'export':
      try {
        const data = JSON.stringify(strategy, null, 2);
        const blob = new Blob([data], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${strategy.name}.json`;
        a.click();
        URL.revokeObjectURL(url);
        ElMessage.success('策略导出成功');
      } catch (error) {
        ElMessage.error('策略导出失败');
      }
      break;
    case 'delete':
      try {
        await ElMessageBox.confirm(
          `确定要删除策略"${strategy.name}"吗？此操作不可撤销。`,
          '确认删除',
          { confirmButtonText: '删除', cancelButtonText: '取消', type: 'warning' }
        );
        await strategyApi.delete(strategy.id);
        ElMessage.success('策略已删除');
        await loadStrategies();
      } catch (error) {
        if (error !== 'cancel') {
          console.error('Failed to delete strategy:', error);
          ElMessage.error('策略删除失败');
        }
      }
      break;
  }
};

const loadStrategies = async () => {
  const userId = userStore.user?.id;
  if (!userId) {
    ElMessage.error('请先登录');
    return;
  }
  loading.value = true;
  try {
    const data = await strategyApi.list(userId);
    strategies.value = data;
    // 触发入场动画
    setTimeout(() => { statsEnter.value = true; }, 100);
  } catch (error) {
    console.error('Failed to load strategies:', error);
    ElMessage.error('加载策略列表失败');
  } finally {
    loading.value = false;
  }
};

onMounted(() => { loadStrategies(); });

// 监听路由变化，当从其他页面返回时刷新列表
const previousPath = ref<string>('');
watch(() => route.path, (newPath) => {
  // 只在路径从其他页面切换回 /strategy 时刷新
  if (newPath === '/strategy' && previousPath.value !== '/strategy') {
    loadStrategies();
  }
  previousPath.value = newPath;
});
</script>

<style scoped lang="scss">
.strategy-list {
  padding: 20px;
  min-height: calc(100vh - 60px);
  background: linear-gradient(180deg, #f5f7fa 0%, #e9eef3 100%);
}

// 页面头部
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 24px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 22px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

.create-btn {
  height: 40px;
  padding: 0 24px;
  font-size: 14px;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.5);
  }
}

// 统计卡片
.stats-row {
  margin-bottom: 20px;
}

.stat-card {
  position: relative;
  border-radius: 16px;
  padding: 20px;
  background: white;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  overflow: hidden;
  cursor: pointer;
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.5s cubic-bezier(0.25, 1, 0.5, 1);

  &.stat-enter {
    opacity: 1;
    transform: translateY(0);
  }

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);

    .stat-bg {
      transform: scale(1.2);
    }
  }

  &__bg {
    position: absolute;
    top: -20px;
    right: -20px;
    width: 100px;
    height: 100px;
    border-radius: 50%;
    opacity: 0.1;
    transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
}

.stat-card.total .stat-bg { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
.stat-card.draft .stat-bg { background: linear-gradient(135deg, #84fab0 0%, #8fd3f4 100%); }
.stat-card.testing .stat-bg { background: linear-gradient(135deg, #fccb90 0%, #d57eeb 100%); }
.stat-card.active .stat-bg { background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
  position: relative;
  z-index: 1;
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);

  &.total { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
  &.draft { background: linear-gradient(135deg, #84fab0 0%, #8fd3f4 100%); }
  &.testing { background: linear-gradient(135deg, #fccb90 0%, #d57eeb 100%); }
  &.active { background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }
}

.stat-card:hover .stat-icon {
  transform: scale(1.1) rotate(5deg);
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 30px;
  font-weight: 700;
  color: #303133;
  line-height: 1;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 13px;
  color: #909399;
  font-weight: 500;
}

// 筛选卡片
.filter-card {
  margin-bottom: 20px;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

  :deep(.el-card__body) {
    padding: 16px 20px;
  }
}

.search-input {
  :deep(.el-input__wrapper) {
    border-radius: 10px;
  }
}

.view-toggle {
  display: flex;
  justify-content: flex-end;

  :deep(.el-radio-button) {
    .el-radio-button__inner {
      border-radius: 8px;
      transition: all 0.3s;
    }

    &:first-child .el-radio-button__inner {
      border-radius: 8px 0 0 8px;
    }

    &:last-child .el-radio-button__inner {
      border-radius: 0 8px 8px 0;
    }
  }
}

// 加载状态
.loading-container,
.empty-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.loading-spinner {
  position: relative;
  width: 60px;
  height: 60px;
  margin-bottom: 20px;
}

.spinner {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 40px;
  height: 40px;
  margin: -20px 0 0 -20px;
  border: 3px solid transparent;
  border-top-color: #667eea;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

.spinner-ring {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 50px;
  height: 50px;
  margin: -25px 0 0 -25px;
  border: 2px solid transparent;
  border-top-color: #764ba2;
  border-radius: 50%;
  animation: spin 1.2s linear infinite reverse;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

// 空状态
.empty-illustration {
  width: 160px;
  height: 160px;
  margin-bottom: 24px;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 8px 0;
}

.empty-desc {
  font-size: 14px;
  color: #909399;
  margin: 0 0 24px 0;
}

.empty-btn {
  height: 40px;
  padding: 0 24px;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
}

// 卡片视图
.strategy-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.strategy-card {
  cursor: pointer;
  border-radius: 16px;
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);

  :deep(.el-card__body) {
    padding: 20px;
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  &:hover {
    transform: translateY(-6px);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.12) !important;
  }
}

.card-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 1, 0.5, 1);
}

.card-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.card-enter-to {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.card-header {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.strategy-icon {
  width: 52px;
  height: 52px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  flex-shrink: 0;
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.strategy-card:hover .strategy-icon {
  transform: scale(1.1) rotate(5deg);
}

.strategy-info {
  flex: 1;
  min-width: 0;
}

.strategy-name {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.strategy-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.version-text {
  font-size: 12px;
  color: #909399;
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.strategy-description {
  font-size: 13px;
  color: #606266;
  line-height: 1.6;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  min-height: 38px;
  margin: 0;
}

.strategy-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;

  .more-tags {
    font-size: 12px;
    color: #909399;
    padding: 2px 8px;
    background: #f5f7fa;
    border-radius: 6px;
  }
}

.tag-item {
  animation: tag-enter 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

@keyframes tag-enter {
  from {
    opacity: 0;
    transform: scale(0.8);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.strategy-dates {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
  border-radius: 10px;
}

.date-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #909399;

  .el-icon {
    font-size: 14px;
  }
}

.card-footer {
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}

// 列表视图
.table-card {
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

  :deep(.el-table) {
    .table-name {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .strategy-icon-small {
      width: 40px;
      height: 40px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      flex-shrink: 0;
    }

    .name-text {
      font-size: 14px;
      font-weight: 500;
      color: #303133;
    }

    .name-desc {
      font-size: 12px;
      color: #909399;
      margin-top: 2px;
    }

    tr:hover {
      .strategy-icon-small {
        transform: scale(1.1);
      }
    }
  }
}

.strategy-table {
  border-radius: 16px;
  overflow: hidden;

  :deep(.el-table__row) {
    transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

    &:hover {
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%) !important;
    }
  }
}

// 分页
.pagination-container {
  display: flex;
  justify-content: center;
  padding: 20px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

// 响应式
@media (max-width: 1200px) {
  .strategy-cards {
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  }
}

@media (max-width: 768px) {
  .strategy-list {
    padding: 12px;
  }

  .page-header {
    flex-direction: column;
    gap: 16px;
    padding: 20px;
  }

  .header-actions {
    width: 100%;

    .create-btn {
      width: 100%;
    }
  }

  .strategy-cards {
    grid-template-columns: 1fr;
  }
}
</style>
