<template>
  <div class="dashboard">
    <!-- 顶部统计卡片 -->
    <el-row :gutter="20">
      <el-col :span="6" v-for="stat in stats" :key="stat.title">
        <div class="stat-card" :class="stat.type">
          <div class="stat-card__bg"></div>
          <div class="stat-card__content">
            <div class="stat-card__icon">
              <el-icon :size="28">
                <component :is="stat.icon" />
              </el-icon>
            </div>
            <div class="stat-card__info">
              <div class="stat-card__value">
                <span class="stat-card__value-text">{{ stat.value }}</span>
                <span v-if="stat.change" class="stat-card__change" :class="stat.changeClass">
                  {{ stat.change }}
                </span>
              </div>
              <div class="stat-card__title">{{ stat.title }}</div>
            </div>
          </div>
          <div class="stat-card__trend" v-if="stat.trend">
            <svg viewBox="0 0 120 40" xmlns="http://www.w3.org/2000/svg">
              <path :d="stat.trend" fill="none" stroke="currentColor" stroke-width="2" />
            </svg>
          </div>
        </div>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <!-- 收益曲线图 -->
      <el-col :span="16">
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <span class="card-title">收益曲线</span>
              <div class="card-actions">
                <el-button-group size="small">
                  <el-button :type="period === 'day' ? 'primary' : ''" @click="period = 'day'">日</el-button>
                  <el-button :type="period === 'week' ? 'primary' : ''" @click="period = 'week'">周</el-button>
                  <el-button :type="period === 'month' ? 'primary' : ''" @click="period = 'month'">月</el-button>
                  <el-button :type="period === 'year' ? 'primary' : ''" @click="period = 'year'">年</el-button>
                </el-button-group>
              </div>
            </div>
          </template>
          <div ref="profitChartRef" class="chart-wrapper" style="height: 300px;"></div>
        </el-card>
      </el-col>

      <!-- 持仓分布 -->
      <el-col :span="8">
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <span class="card-title">持仓分布</span>
          </template>
          <div ref="positionChartRef" class="chart-wrapper" style="height: 300px;"></div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <!-- 交易统计 -->
      <el-col :span="8">
        <el-card class="stat-detail-card" shadow="hover">
          <template #header>
            <span class="card-title">今日交易</span>
          </template>
          <div class="trade-stats">
            <div class="trade-stat-item">
              <span class="trade-stat-label">交易次数</span>
              <span class="trade-stat-value">128</span>
            </div>
            <div class="trade-stat-item">
              <span class="trade-stat-label">买入次数</span>
              <span class="trade-stat-value trade-stat-buy">67</span>
            </div>
            <div class="trade-stat-item">
              <span class="trade-stat-label">卖出次数</span>
              <span class="trade-stat-value trade-stat-sell">61</span>
            </div>
            <div class="trade-stat-item">
              <span class="trade-stat-label">成交金额</span>
              <span class="trade-stat-value">$1,234,567</span>
            </div>
            <div class="trade-stat-item">
              <span class="trade-stat-label">手续费</span>
              <span class="trade-stat-value text-regular">$123.45</span>
            </div>
          </div>
        </el-card>
      </el-col>

      <!-- 策略运行状态 -->
      <el-col :span="8">
        <el-card class="strategy-card" shadow="hover">
          <template #header>
            <div class="card-header">
              <span class="card-title">策略运行状态</span>
              <el-tag size="small" type="success">2 个运行中</el-tag>
            </div>
          </template>
          <div class="strategy-list">
            <div v-for="strategy in strategies" :key="strategy.id" class="strategy-item">
              <div class="strategy-info">
                <div class="strategy-name">{{ strategy.name }}</div>
                <div class="strategy-time">{{ strategy.time }}</div>
              </div>
              <div class="strategy-status">
                <span class="status-badge" :class="`status-${strategy.status}`">
                  {{ strategy.statusText }}
                </span>
              </div>
            </div>
            <el-empty v-if="strategies.length === 0" description="暂无运行中的策略" :image-size="80" />
          </div>
        </el-card>
      </el-col>

      <!-- 系统活动 -->
      <el-col :span="8">
        <el-card class="activity-card" shadow="hover">
          <template #header>
            <span class="card-title">系统活动</span>
          </template>
          <div class="activity-timeline">
            <div v-for="(activity, index) in activities" :key="index" class="activity-item">
              <div class="activity-dot" :class="`activity-dot-${activity.type}`"></div>
              <div class="activity-content">
                <div class="activity-text">{{ activity.text }}</div>
                <div class="activity-time">{{ activity.time }}</div>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 快捷操作 -->
    <el-row :gutter="20" class="mt-4">
      <el-col :span="24">
        <el-card class="quick-action-card" shadow="hover">
          <template #header>
            <span class="card-title">快捷操作</span>
          </template>
          <div class="quick-actions">
            <div class="quick-action-item" @click="$router.push('/strategy/editor')">
              <el-icon :size="24" color="#409eff"><Edit /></el-icon>
              <span>创建策略</span>
            </div>
            <div class="quick-action-item" @click="$router.push('/market')">
              <el-icon :size="24" color="#67c23a"><TrendCharts /></el-icon>
              <span>查看行情</span>
            </div>
            <div class="quick-action-item" @click="$router.push('/backtest')">
              <el-icon :size="24" color="#e6a23c"><DataAnalysis /></el-icon>
              <span>回测策略</span>
            </div>
            <div class="quick-action-item" @click="$router.push('/trade')">
              <el-icon :size="24" color="#f56c6c"><ShoppingCart /></el-icon>
              <span>交易控制台</span>
            </div>
            <div class="quick-action-item" @click="$router.push('/risk')">
              <el-icon :size="24" color="#909399"><Monitor /></el-icon>
              <span>风险监控</span>
            </div>
            <div class="quick-action-item" @click="$router.push('/settings')">
              <el-icon :size="24" color="#606266"><Setting /></el-icon>
              <span>系统设置</span>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import * as echarts from 'echarts';
import {
  Wallet,
  TrendCharts,
  Document,
  DataLine,
  Edit,
  DataAnalysis,
  ShoppingCart,
  Monitor,
  Setting,
} from '@element-plus/icons-vue';

const period = ref('day');
const profitChartRef = ref<HTMLElement>();
const positionChartRef = ref<HTMLElement>();
let profitChart: echarts.ECharts | null = null;
let positionChart: echarts.ECharts | null = null;

const stats = ref([
  {
    title: '总资产',
    value: '$125,678.90',
    icon: Wallet,
    type: 'primary',
    change: '+2.5%',
    changeClass: 'up',
    trend: 'M0,20 Q30,25 60,15 T120,20',
  },
  {
    title: '运行策略',
    value: '2',
    icon: TrendCharts,
    type: 'success',
    change: '+1',
    changeClass: 'up',
    trend: 'M0,30 Q30,20 60,25 T120,10',
  },
  {
    title: '策略总数',
    value: '5',
    icon: Document,
    type: 'danger',
    change: '+2',
    changeClass: 'up',
    trend: 'M0,25 Q30,30 60,20 T120,15',
  },
  {
    title: '今日盈亏',
    value: '+$1,234.56',
    icon: DataLine,
    type: 'warning',
    change: '+1.2%',
    changeClass: 'up',
    trend: 'M0,35 Q30,15 60,25 T120,5',
  },
]);

const strategies = ref([
  {
    id: 1,
    name: '双均线突破策略',
    status: 'running',
    statusText: '运行中',
    time: '运行 2h 35m',
  },
  {
    id: 2,
    name: '网格交易策略',
    status: 'running',
    statusText: '运行中',
    time: '运行 5h 12m',
  },
  {
    id: 3,
    name: 'MACD 策略',
    status: 'stopped',
    statusText: '已停止',
    time: '已停止',
  },
]);

const activities = ref([
  { text: '双均线策略开仓 BTC/USDT', time: '2分钟前', type: 'success' },
  { text: '网格策略平仓 ETH/USDT', time: '5分钟前', type: 'info' },
  { text: '系统自动备份完成', time: '15分钟前', type: 'default' },
  { text: '新增策略 "RSI交易"', time: '1小时前', type: 'warning' },
  { text: '风险预警触发，仓位已调整', time: '2小时前', type: 'danger' },
]);

const initProfitChart = () => {
  if (!profitChartRef.value) return;

  profitChart = echarts.init(profitChartRef.value);

  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      borderColor: '#409eff',
      textStyle: { color: '#fff' },
    },
    grid: {
      left: '3%',
      right: '4%',
      bottom: '3%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      data: ['00:00', '04:00', '08:00', '12:00', '16:00', '20:00', '24:00'],
      axisLine: { lineStyle: { color: '#dcdfe6' } },
      axisLabel: { color: '#606266' },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: { color: '#606266' },
      splitLine: { lineStyle: { color: '#ebeef5', type: 'dashed' } },
    },
    series: [
      {
        name: '收益',
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        data: [120000, 121500, 122800, 124200, 123800, 125100, 125679],
        itemStyle: { color: '#409eff' },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
            { offset: 1, color: 'rgba(64, 158, 255, 0.05)' },
          ]),
        },
      },
    ],
  };

  profitChart.setOption(option);
};

const initPositionChart = () => {
  if (!positionChartRef.value) return;

  positionChart = echarts.init(positionChartRef.value);

  const option = {
    tooltip: {
      trigger: 'item',
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      textStyle: { color: '#fff' },
    },
    legend: {
      bottom: '0',
      left: 'center',
      itemWidth: 12,
      itemHeight: 12,
      textStyle: { fontSize: 12, color: '#606266' },
    },
    series: [
      {
        name: '持仓分布',
        type: 'pie',
        radius: ['40%', '65%'],
        center: ['50%', '45%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 8,
          borderColor: '#fff',
          borderWidth: 2,
        },
        label: { show: false },
        emphasis: {
          label: { show: true, fontSize: 14, fontWeight: 'bold' },
        },
        data: [
          { value: 45, name: 'BTC', itemStyle: { color: '#f7931a' } },
          { value: 30, name: 'ETH', itemStyle: { color: '#627eea' } },
          { value: 15, name: 'BNB', itemStyle: { color: '#f3ba2f' } },
          { value: 10, name: '其他', itemStyle: { color: '#909399' } },
        ],
      },
    ],
  };

  positionChart.setOption(option);
};

const handleResize = () => {
  profitChart?.resize();
  positionChart?.resize();
};

onMounted(() => {
  initProfitChart();
  initPositionChart();
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  profitChart?.dispose();
  positionChart?.dispose();
  window.removeEventListener('resize', handleResize);
});
</script>

<style scoped lang="scss">
.dashboard {
  padding: 0;
}

.mt-4 {
  margin-top: 20px;
}

// 统计卡片样式
.stat-card {
  position: relative;
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  cursor: pointer;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid #ebeef5;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.12);

    .stat-card__trend {
      opacity: 1;
    }
  }

  &__bg {
    position: absolute;
    top: 0;
    right: 0;
    width: 120px;
    height: 120px;
    border-radius: 50%;
    opacity: 0.1;
    transform: translate(30%, -30%);
  }

  &.primary &__bg {
    background: #409eff;
  }

  &.success &__bg {
    background: #67c23a;
  }

  &.danger &__bg {
    background: #f56c6c;
  }

  &.warning &__bg {
    background: #e6a23c;
  }

  &__content {
    display: flex;
    align-items: center;
    gap: 16px;
    position: relative;
    z-index: 1;
  }

  &__icon {
    width: 56px;
    height: 56px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s;
  }

  &.primary &__icon {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    color: #409eff;
  }

  &.success &__icon {
    background: linear-gradient(135deg, #f0f9ff 0%, #e1f3d8 100%);
    color: #67c23a;
  }

  &.danger &__icon {
    background: linear-gradient(135deg, #fef0f0 0%, #fde2e2 100%);
    color: #f56c6c;
  }

  &.warning &__icon {
    background: linear-gradient(135deg, #fdf6ec 0%, #faecd8 100%);
    color: #e6a23c;
  }

  &:hover &__icon {
    transform: scale(1.1) rotate(5deg);
  }

  &__info {
    flex: 1;
  }

  &__value {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 4px;

    &-text {
      font-size: 24px;
      font-weight: 700;
      color: #303133;
      line-height: 1;
    }
  }

  &__change {
    font-size: 12px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 12px;

    &.up {
      background: rgba(103, 194, 58, 0.1);
      color: #67c23a;
    }

    &.down {
      background: rgba(245, 108, 108, 0.1);
      color: #f56c6c;
    }
  }

  &__title {
    font-size: 13px;
    color: #909399;
  }

  &__trend {
    position: absolute;
    bottom: 10px;
    right: 10px;
    width: 60px;
    height: 20px;
    opacity: 0;
    transition: opacity 0.3s;

    svg {
      width: 100%;
      height: 100%;
    }

    &.primary svg {
      color: #409eff;
    }

    &.success svg {
      color: #67c23a;
    }
  }
}

// 卡片通用样式
.chart-card,
.stat-detail-card,
.strategy-card,
.activity-card,
.quick-action-card {
  :deep(.el-card__header) {
    padding: 16px 20px;
    border-bottom: 1px solid #ebeef5;
    background: #fafbfc;
  }

  :deep(.el-card__body) {
    padding: 20px;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #303133;
}

.chart-wrapper {
  width: 100%;
}

// 交易统计
.trade-stats {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.trade-stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.3s;

  &:hover {
    background: #ecf5ff;
    transform: translateX(4px);
  }
}

.trade-stat-label {
  font-size: 13px;
  color: #606266;
}

.trade-stat-value {
  font-size: 16px;
  font-weight: 600;
  color: #303133;

  &.trade-stat-buy {
    color: #ef5350;
  }

  &.trade-stat-sell {
    color: #26a69a;
  }
}

// 策略列表
.strategy-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.strategy-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  transition: all 0.3s;

  &:hover {
    background: #ecf5ff;
    box-shadow: 0 2px 8px rgba(64, 158, 255, 0.15);
  }
}

.strategy-name {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
}

.strategy-time {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

// 活动时间线
.activity-timeline {
  position: relative;
  padding-left: 20px;
}

.activity-item {
  position: relative;
  padding-bottom: 20px;
  padding-left: 24px;

  &:last-child {
    padding-bottom: 0;

    &::before {
      display: none;
    }
  }

  &::before {
    content: '';
    position: absolute;
    left: 5px;
    top: 20px;
    bottom: 0;
    width: 2px;
    background: #ebeef5;
  }
}

.activity-dot {
  position: absolute;
  left: 0;
  top: 2px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: 2px solid #fff;
  box-shadow: 0 0 0 2px #ebeef5;

  &.activity-dot-success {
    background: #67c23a;
    box-shadow: 0 0 0 2px #e1f3d8;
  }

  &.activity-dot-info {
    background: #409eff;
    box-shadow: 0 0 0 2px #d9ecff;
  }

  &.activity-dot-warning {
    background: #e6a23c;
    box-shadow: 0 0 0 2px #faecd8;
  }

  &.activity-dot-danger {
    background: #f56c6c;
    box-shadow: 0 0 0 2px #fde2e2;
  }

  &.activity-dot-default {
    background: #909399;
    box-shadow: 0 0 0 2px #ebeef5;
  }
}

.activity-text {
  font-size: 13px;
  color: #303133;
  line-height: 1.5;
}

.activity-time {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

// 快捷操作
.quick-actions {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.quick-action-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px 24px;
  background: #f5f7fa;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
  flex: 1;
  min-width: 120px;

  &:hover {
    background: #fff;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);

    .el-icon {
      transform: scale(1.1);
    }
  }

  span {
    font-size: 13px;
    color: #606266;
    font-weight: 500;
  }

  .el-icon {
    transition: transform 0.3s;
  }
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

  &.status-error {
    background: rgba(245, 108, 108, 0.1);
    color: #f56c6c;
  }

  &.status-paused {
    background: rgba(230, 162, 60, 0.1);
    color: #e6a23c;
  }
}
</style>
