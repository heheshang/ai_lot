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
        <el-button type="primary" @click="createStrategy">
          <el-icon><Plus /></el-icon>
          新建策略
        </el-button>
      </div>
    </div>

    <!-- 筛选和搜索栏 -->
    <el-card class="filter-card" shadow="never">
      <el-row :gutter="16">
        <el-col :span="6">
          <el-input
            v-model="searchText"
            placeholder="搜索策略名称或描述"
            clearable
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
          <el-select v-model="filterStatus" placeholder="运行状态" clearable @change="handleFilter">
            <el-option label="全部" value="" />
            <el-option label="运行中" value="running" />
            <el-option label="已停止" value="stopped" />
            <el-option label="已暂停" value="paused" />
            <el-option label="错误" value="error" />
          </el-select>
        </el-col>
        <el-col :span="6">
          <el-select v-model="sortBy" placeholder="排序方式" @change="handleSort">
            <el-option label="创建时间" value="created_at" />
            <el-option label="更新时间" value="updated_at" />
            <el-option label="策略名称" value="name" />
            <el-option label="收益率" value="return_rate" />
          </el-select>
        </el-col>
        <el-col :span="4">
          <div class="view-toggle">
            <el-radio-group v-model="viewMode" size="default">
              <el-radio-button label="card">
                <el-icon><Grid /></el-icon>
                卡片
              </el-radio-button>
              <el-radio-button label="list">
                <el-icon><List /></el-icon>
                列表
              </el-radio-button>
            </el-radio-group>
          </div>
        </el-col>
      </el-row>
    </el-card>

    <!-- 加载状态 -->
    <div v-if="loading" class="loading-container">
      <el-icon class="is-loading" :size="40"><Loading /></el-icon>
      <p>加载策略列表中...</p>
    </div>

    <!-- 空状态 -->
    <div v-else-if="filteredStrategies.length === 0" class="empty-container">
      <el-empty :description="searchText || filterCategory || filterStatus ? '未找到符合条件的策略' : '暂无策略'">
        <el-button v-if="!searchText && !filterCategory && !filterStatus" type="primary" @click="createStrategy">
          创建第一个策略
        </el-button>
      </el-empty>
    </div>

    <!-- 卡片视图 -->
    <div v-else-if="viewMode === 'card'" class="strategy-cards">
      <div
        v-for="strategy in filteredStrategies"
        :key="strategy.id"
        class="strategy-card"
        @click="viewStrategy(strategy.id)"
      >
        <div class="card-header">
          <div class="strategy-icon" :style="{ background: getStrategyColor(strategy.category) }">
            <el-icon :size="24"><component :is="getStrategyIcon(strategy.category)" /></el-icon>
          </div>
          <div class="strategy-status">
            <span class="status-badge" :class="`status-${strategy.status}`">
              {{ getStatusText(strategy.status) }}
            </span>
          </div>
        </div>

        <div class="card-body">
          <h3 class="strategy-name">{{ strategy.name }}</h3>
          <p class="strategy-description">{{ strategy.description || '暂无描述' }}</p>

          <div class="strategy-meta">
            <el-tag size="small" type="info">{{ getCategoryText(strategy.category) }}</el-tag>
            <div class="meta-item">
              <el-icon><Calendar /></el-icon>
              <span>{{ formatDate(strategy.created_at) }}</span>
            </div>
          </div>

          <div class="strategy-stats" v-if="strategy.stats">
            <div class="stat-item">
              <span class="stat-label">总收益率</span>
              <span class="stat-value" :class="strategy.stats.return_rate >= 0 ? 'text-up' : 'text-down'">
                {{ strategy.stats.return_rate >= 0 ? '+' : '' }}{{ strategy.stats.return_rate }}%
              </span>
            </div>
            <div class="stat-item">
              <span class="stat-label">运行次数</span>
              <span class="stat-value">{{ strategy.stats.run_count }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">胜率</span>
              <span class="stat-value">{{ strategy.stats.win_rate }}%</span>
            </div>
          </div>

          <div class="strategy-tags" v-if="strategy.tags && strategy.tags.length > 0">
            <el-tag
              v-for="tag in strategy.tags.slice(0, 3)"
              :key="tag"
              size="small"
              type="warning"
              effect="plain"
            >
              {{ tag }}
            </el-tag>
            <span v-if="strategy.tags.length > 3" class="more-tags">
              +{{ strategy.tags.length - 3 }}
            </span>
          </div>
        </div>

        <div class="card-footer">
          <el-button-group>
            <el-button size="small" @click.stop="editStrategy(strategy.id)">
              <el-icon><Edit /></el-icon>
              编辑
            </el-button>
            <el-button size="small" @click.stop="copyStrategy(strategy.id)">
              <el-icon><CopyDocument /></el-icon>
              复制
            </el-button>
            <el-dropdown trigger="click" @command="(cmd) => handleAction(cmd, strategy.id)">
              <el-button size="small">
                <el-icon><MoreFilled /></el-icon>
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="run">
                    <el-icon><VideoPlay /></el-icon>
                    运行
                  </el-dropdown-item>
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
      </div>
    </div>

    <!-- 列表视图 -->
    <el-card v-else class="table-card" shadow="never">
      <el-table :data="filteredStrategies" stripe style="width: 100%">
        <el-table-column prop="name" label="策略名称" min-width="180">
          <template #default="{ row }">
            <div class="table-name">
              <div
                class="strategy-icon-small"
                :style="{ background: getStrategyColor(row.category) }"
              >
                <el-icon :size="16">
                  <component :is="getStrategyIcon(row.category)" />
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
            <el-tag size="small" type="info">{{ getCategoryText(row.category) }}</el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <span class="status-badge" :class="`status-${row.status}`">
              {{ getStatusText(row.status) }}
            </span>
          </template>
        </el-table-column>

        <el-table-column label="收益率" width="120" v-if="showStats">
          <template #default="{ row }">
            <span v-if="row.stats" :class="row.stats.return_rate >= 0 ? 'text-up' : 'text-down'">
              {{ row.stats.return_rate >= 0 ? '+' : '' }}{{ row.stats.return_rate }}%
            </span>
            <span v-else class="text-regular">--</span>
          </template>
        </el-table-column>

        <el-table-column label="运行次数" width="100" v-if="showStats">
          <template #default="{ row }">
            <span v-if="row.stats">{{ row.stats.run_count }}</span>
            <span v-else class="text-regular">--</span>
          </template>
        </el-table-column>

        <el-table-column prop="updated_at" label="更新时间" width="160">
          <template #default="{ row }">
            {{ formatDateTime(row.updated_at) }}
          </template>
        </el-table-column>

        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button-group size="small">
              <el-button type="primary" link @click="editStrategy(row.id)">编辑</el-button>
              <el-button type="primary" link @click="copyStrategy(row.id)">复制</el-button>
              <el-dropdown trigger="click" @command="(cmd) => handleAction(cmd, row.id)">
                <el-button type="primary" link>
                  更多<el-icon class="el-icon--right"><ArrowDown /></el-icon>
                </el-button>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item command="run">运行</el-dropdown-item>
                    <el-dropdown-item command="backtest">回测</el-dropdown-item>
                    <el-dropdown-item command="export">导出</el-dropdown-item>
                    <el-dropdown-item command="delete" divided>删除</el-dropdown-item>
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
        :total="totalStrategies"
        layout="total, sizes, prev, pager, next, jumper"
        @size-change="handleSizeChange"
        @current-change="handlePageChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  Plus,
  Search,
  Grid,
  List,
  Edit,
  CopyDocument,
  Delete,
  MoreFilled,
  VideoPlay,
  DataAnalysis,
  Download,
  Calendar,
  ArrowDown,
  Loading,
} from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';

interface StrategyStats {
  return_rate: number;
  run_count: number;
  win_rate: number;
}

interface Strategy {
  id: number;
  name: string;
  description: string;
  category: string;
  status: string;
  tags: string[];
  created_at: string;
  updated_at: string;
  stats?: StrategyStats;
}

const router = useRouter();

// 状态
const loading = ref(false);
const viewMode = ref<'card' | 'list'>('card');
const searchText = ref('');
const filterCategory = ref('');
const filterStatus = ref('');
const sortBy = ref('created_at');
const currentPage = ref(1);
const pageSize = ref(12);

// 模拟数据
const strategies = ref<Strategy[]>([
  {
    id: 1,
    name: '双均线突破策略',
    description: '基于5日和20日均线的趋势跟踪策略，当短期均线上穿长期均线时开仓。',
    category: 'trend',
    status: 'running',
    tags: ['趋势', '均线', '突破'],
    created_at: '2024-01-15T10:30:00Z',
    updated_at: '2024-01-20T14:25:00Z',
    stats: { return_rate: 12.5, run_count: 156, win_rate: 68 },
  },
  {
    id: 2,
    name: '网格交易策略',
    description: '在指定价格区间内设置网格，自动低买高卖，适合震荡行情。',
    category: 'grid',
    status: 'running',
    tags: ['网格', '震荡', '高频'],
    created_at: '2024-01-10T08:00:00Z',
    updated_at: '2024-01-19T16:40:00Z',
    stats: { return_rate: 8.3, run_count: 342, win_rate: 72 },
  },
  {
    id: 3,
    name: 'MACD策略',
    description: '基于MACD指标的趋势跟踪策略，结合DIF和DEA交叉信号。',
    category: 'trend',
    status: 'stopped',
    tags: ['MACD', '趋势', '指标'],
    created_at: '2024-01-05T12:00:00Z',
    updated_at: '2024-01-18T09:15:00Z',
    stats: { return_rate: -2.1, run_count: 89, win_rate: 45 },
  },
  {
    id: 4,
    name: 'RSI均值回归',
    description: '当RSI超买超卖时进行反向交易，适合震荡行情。',
    category: 'mean_reversion',
    status: 'paused',
    tags: ['RSI', '均值回归', '震荡'],
    created_at: '2024-01-03T15:20:00Z',
    updated_at: '2024-01-17T11:30:00Z',
    stats: { return_rate: 5.7, run_count: 201, win_rate: 58 },
  },
  {
    id: 5,
    name: '跨期套利',
    description: '同时买卖不同到期日的期货合约，赚取价差收敛收益。',
    category: 'arbitrage',
    status: 'stopped',
    tags: ['套利', '期货', '低风险'],
    created_at: '2024-01-01T09:00:00Z',
    updated_at: '2024-01-16T13:45:00Z',
    stats: { return_rate: 3.2, run_count: 67, win_rate: 82 },
  },
  {
    id: 6,
    name: '布林带策略',
    description: '基于布林带通道的突破策略，价格触及上下轨时产生信号。',
    category: 'trend',
    status: 'error',
    tags: ['布林带', '突破', '波动率'],
    created_at: '2023-12-28T10:00:00Z',
    updated_at: '2024-01-15T16:20:00Z',
    stats: { return_rate: -5.4, run_count: 45, win_rate: 38 },
  },
]);

// 计算属性
const filteredStrategies = computed(() => {
  let result = [...strategies.value];

  // 搜索过滤
  if (searchText.value) {
    const keyword = searchText.value.toLowerCase();
    result = result.filter(s =>
      s.name.toLowerCase().includes(keyword) ||
      (s.description && s.description.toLowerCase().includes(keyword))
    );
  }

  // 分类过滤
  if (filterCategory.value) {
    result = result.filter(s => s.category === filterCategory.value);
  }

  // 状态过滤
  if (filterStatus.value) {
    result = result.filter(s => s.status === filterStatus.value);
  }

  // 排序
  result.sort((a, b) => {
    if (sortBy.value === 'name') {
      return a.name.localeCompare(b.name, 'zh-CN');
    } else if (sortBy.value === 'return_rate') {
      return (b.stats?.return_rate || 0) - (a.stats?.return_rate || 0);
    } else {
      return new Date(b[sortBy.value]).getTime() - new Date(a[sortBy.value]).getTime();
    }
  });

  // 分页
  const start = (currentPage.value - 1) * pageSize.value;
  return result.slice(start, start + pageSize.value);
});

const totalStrategies = computed(() => {
  let result = strategies.value.length;
  return result;
});

const showStats = computed(() => {
  return strategies.value.some(s => s.stats);
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

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    running: '运行中',
    stopped: '已停止',
    paused: '已暂停',
    error: '错误',
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

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (days === 0) return '今天';
  if (days === 1) return '昨天';
  if (days < 7) return `${days}天前`;
  if (days < 30) return `${Math.floor(days / 7)}周前`;
  return date.toLocaleDateString('zh-CN');
};

const formatDateTime = (dateStr: string) => {
  return new Date(dateStr).toLocaleString('zh-CN');
};

const handleSearch = () => {
  currentPage.value = 1;
};

const handleFilter = () => {
  currentPage.value = 1;
};

const handleSort = () => {
  currentPage.value = 1;
};

const handleSizeChange = () => {
  currentPage.value = 1;
};

const handlePageChange = () => {
  // 页面变化处理
};

const createStrategy = () => {
  router.push('/strategy/editor');
};

const viewStrategy = (id: number) => {
  router.push(`/strategy/editor/${id}`);
};

const editStrategy = (id: number) => {
  router.push(`/strategy/editor/${id}`);
};

const copyStrategy = async (id: number) => {
  try {
    // TODO: 调用复制策略API
    ElMessage.success('策略复制成功');
  } catch (error) {
    ElMessage.error('策略复制失败');
  }
};

const handleAction = async (command: string, id: number) => {
  switch (command) {
    case 'run':
      ElMessage.success('策略已启动运行');
      break;
    case 'backtest':
      router.push(`/backtest?strategyId=${id}`);
      break;
    case 'export':
      ElMessage.success('策略导出成功');
      break;
    case 'delete':
      try {
        await ElMessageBox.confirm('确定要删除此策略吗？此操作不可撤销。', '确认删除', {
          confirmButtonText: '删除',
          cancelButtonText: '取消',
          type: 'warning',
        });
        ElMessage.success('策略已删除');
      } catch {
        // 用户取消
      }
      break;
  }
};

onMounted(() => {
  // TODO: 加载策略列表
});
</script>

<style scoped lang="scss">
.strategy-list {
  padding: 20px;
  min-height: calc(100vh - 60px);
  background: #f5f7fa;
}

// 页面头部
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #303133;
  margin: 0;
}

// 筛选卡片
.filter-card {
  margin-bottom: 20px;

  :deep(.el-card__body) {
    padding: 16px 20px;
  }
}

.view-toggle {
  display: flex;
  justify-content: flex-end;
}

// 加载和空状态
.loading-container,
.empty-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  color: #909399;

  .el-icon {
    color: #409eff;
    margin-bottom: 16px;
  }
}

// 卡片视图
.strategy-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.strategy-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid #ebeef5;
  display: flex;
  flex-direction: column;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);
    border-color: #c6e2ff;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 16px;
}

.strategy-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.strategy-name {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.strategy-description {
  font-size: 13px;
  color: #606266;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  min-height: 38px;
}

.strategy-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: #909399;

  .el-icon {
    font-size: 14px;
  }
}

.strategy-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.stat-label {
  font-size: 12px;
  color: #909399;
}

.stat-value {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.strategy-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;

  .more-tags {
    font-size: 12px;
    color: #909399;
  }
}

.card-footer {
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}

// 状态徽章
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;

  &.status-running {
    background: rgba(103, 194, 58, 0.1);
    color: #67c23a;
  }

  &.status-stopped {
    background: rgba(144, 147, 153, 0.1);
    color: #909399;
  }

  &.status-paused {
    background: rgba(230, 162, 60, 0.1);
    color: #e6a23c;
  }

  &.status-error {
    background: rgba(245, 108, 108, 0.1);
    color: #f56c6c;
  }
}

// 列表视图
.table-card {
  :deep(.el-table) {
    .table-name {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .strategy-icon-small {
      width: 36px;
      height: 36px;
      border-radius: 8px;
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
  }
}

// 文本颜色
.text-up {
  color: #ef5350;
}

.text-down {
  color: #26a69a;
}

.text-regular {
  color: #909399;
}

// 分页
.pagination-container {
  display: flex;
  justify-content: center;
  padding: 20px 0;
}
</style>
