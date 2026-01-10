<template>
  <div class="dashboard">
    <!-- 顶部统计卡片 -->
    <el-row :gutter="20">
      <el-col :span="6" v-for="stat in stats" :key="stat.title">
        <div
          class="stat-card"
          :class="[stat.type, { 'stat-card-enter': cardEnter }]"
          @click="handleStatClick(stat)"
        >
          <div class="stat-card__bg"></div>
          <div class="stat-card__content">
            <div class="stat-card__icon">
              <el-icon :size="28">
                <component :is="stat.icon" />
              </el-icon>
            </div>
            <div class="stat-card__info">
              <div class="stat-card__value">
                <span class="stat-card__value-text" :data-value="stat.rawValue">{{
                  stat.value
                }}</span>
                <span v-if="stat.change" class="stat-card__change" :class="stat.changeClass">
                  <el-icon
                    ><CaretTop v-if="stat.changeClass === 'up'" /><CaretBottom v-else
                  /></el-icon>
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
                  <el-button :type="period === 'day' ? 'primary' : ''" @click="updatePeriod('day')"
                    >日</el-button
                  >
                  <el-button
                    :type="period === 'week' ? 'primary' : ''"
                    @click="updatePeriod('week')"
                    >周</el-button
                  >
                  <el-button
                    :type="period === 'month' ? 'primary' : ''"
                    @click="updatePeriod('month')"
                    >月</el-button
                  >
                  <el-button
                    :type="period === 'year' ? 'primary' : ''"
                    @click="updatePeriod('year')"
                    >年</el-button
                  >
                </el-button-group>
              </div>
            </div>
          </template>
          <div ref="profitChartRef" class="chart-wrapper" style="height: 320px"></div>
        </el-card>
      </el-col>

      <!-- 持仓分布 -->
      <el-col :span="8">
        <el-card class="chart-card" shadow="hover">
          <template #header>
            <span class="card-title">持仓分布</span>
          </template>
          <div ref="positionChartRef" class="chart-wrapper" style="height: 320px"></div>
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
            <div class="trade-stat-item" v-for="item in tradeStats" :key="item.label">
              <span class="trade-stat-label">{{ item.label }}</span>
              <span class="trade-stat-value" :class="item.valueClass">{{ item.value }}</span>
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
              <el-tag size="small" type="success">{{ runningCount }} 个运行中</el-tag>
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
                  <span v-if="strategy.status === 'running'" class="status-dot"></span>
                  {{ strategy.statusText }}
                </span>
              </div>
            </div>
            <el-empty
              v-if="strategies.length === 0"
              description="暂无运行中的策略"
              :image-size="80"
            />
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
            <div
              class="quick-action-item"
              v-for="action in quickActions"
              :key="action.name"
              @click="action.onClick"
            >
              <div class="action-icon" :style="{ background: action.bg }">
                <el-icon :size="24" :color="action.color">
                  <component :is="action.icon" />
                </el-icon>
              </div>
              <span>{{ action.name }}</span>
              <div class="action-hint">{{ action.hint }}</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import * as echarts from 'echarts'
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
  CaretTop,
  CaretBottom,
} from '@element-plus/icons-vue'

const router = useRouter()
const period = ref('day')
const cardEnter = ref(false)
const profitChartRef = ref<HTMLElement>()
const positionChartRef = ref<HTMLElement>()
let profitChart: echarts.ECharts | null = null
let positionChart: echarts.ECharts | null = null

// 统计数据
const stats = ref([
  {
    title: '总资产',
    value: '$125,678.90',
    rawValue: 125678.9,
    icon: Wallet,
    type: 'primary',
    change: '+2.5%',
    changeClass: 'up',
    trend: 'M0,20 Q30,25 60,15 T120,20',
  },
  {
    title: '运行策略',
    value: '2',
    rawValue: 2,
    icon: TrendCharts,
    type: 'success',
    change: '+1',
    changeClass: 'up',
    trend: 'M0,30 Q30,20 60,25 T120,10',
  },
  {
    title: '策略总数',
    value: '5',
    rawValue: 5,
    icon: Document,
    type: 'danger',
    change: '+2',
    changeClass: 'up',
    trend: 'M0,25 Q30,30 60,20 T120,15',
  },
  {
    title: '今日盈亏',
    value: '+$1,234.56',
    rawValue: 1234.56,
    icon: DataLine,
    type: 'warning',
    change: '+1.2%',
    changeClass: 'up',
    trend: 'M0,35 Q30,15 60,25 T120,5',
  },
])

// 交易统计
const tradeStats = ref([
  { label: '交易次数', value: '128', valueClass: '' },
  { label: '买入次数', value: '67', valueClass: 'trade-stat-buy' },
  { label: '卖出次数', value: '61', valueClass: 'trade-stat-sell' },
  { label: '成交金额', value: '$1,234,567', valueClass: '' },
  { label: '手续费', value: '$123.45', valueClass: 'text-regular' },
])

// 运行中策略数量
const runningCount = computed(() => strategies.value.filter((s) => s.status === 'running').length)

// 策略列表
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
])

// 系统活动
const activities = ref([
  { text: '双均线策略开仓 BTC/USDT', time: '2分钟前', type: 'success' },
  { text: '网格策略平仓 ETH/USDT', time: '5分钟前', type: 'info' },
  { text: '系统自动备份完成', time: '15分钟前', type: 'default' },
  { text: '新增策略 "RSI交易"', time: '1小时前', type: 'warning' },
  { text: '风险预警触发，仓位已调整', time: '2小时前', type: 'danger' },
])

// 快捷操作
const quickActions = ref([
  {
    name: '创建策略',
    hint: '编写新策略',
    icon: Edit,
    color: '#409eff',
    bg: 'linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%)',
    onClick: () => router.push('/strategy/editor'),
  },
  {
    name: '查看行情',
    hint: '市场数据',
    icon: TrendCharts,
    color: '#67c23a',
    bg: 'linear-gradient(135deg, #f0f9ff 0%, #e1f3d8 100%)',
    onClick: () => router.push('/market'),
  },
  {
    name: '回测策略',
    hint: '历史数据',
    icon: DataAnalysis,
    color: '#e6a23c',
    bg: 'linear-gradient(135deg, #fdf6ec 0%, #faecd8 100%)',
    onClick: () => router.push('/backtest'),
  },
  {
    name: '交易控制台',
    hint: '手动交易',
    icon: ShoppingCart,
    color: '#f56c6c',
    bg: 'linear-gradient(135deg, #fef0f0 0%, #fde2e2 100%)',
    onClick: () => router.push('/trade'),
  },
  {
    name: '风险监控',
    hint: '仓位管理',
    icon: Monitor,
    color: '#909399',
    bg: 'linear-gradient(135deg, #f4f4f5 0%, #e9e9eb 100%)',
    onClick: () => router.push('/risk'),
  },
  {
    name: '系统设置',
    hint: '配置',
    icon: Setting,
    color: '#606266',
    bg: 'linear-gradient(135deg, #f5f7fa 0%, #e4e7ed 100%)',
    onClick: () => router.push('/settings'),
  },
])

// 数字滚动动画
function animateNumber(element: HTMLElement, targetValue: number, duration: number = 1500) {
  const startValue = 0
  const startTime = performance.now()
  const isCurrency = element.textContent?.includes('$')
  const hasPlus = element.textContent?.startsWith('+')

  function update(currentTime: number) {
    const elapsed = currentTime - startTime
    const progress = Math.min(elapsed / duration, 1)
    // 使用 easeOutExpo 缓动
    const eased = progress === 1 ? 1 : 1 - Math.pow(2, -10 * progress)
    const currentValue = startValue + (targetValue - startValue) * eased

    if (isCurrency) {
      const formatted = Math.floor(currentValue).toLocaleString('en-US')
      element.textContent = `${hasPlus ? '+' : ''}$${formatted}.${(currentValue % 1).toFixed(2).slice(2)}`
    } else {
      element.textContent = Math.floor(currentValue).toString()
    }

    if (progress < 1) {
      requestAnimationFrame(update)
    }
  }

  requestAnimationFrame(update)
}

// 初始化数字动画
function initNumberAnimations() {
  setTimeout(() => {
    cardEnter.value = true
    const valueElements = document.querySelectorAll('.stat-card__value-text')
    valueElements.forEach((el, index) => {
      const rawValue = parseFloat(el.getAttribute('data-value') || '0')
      setTimeout(() => {
        animateNumber(el as HTMLElement, rawValue, 1500)
      }, index * 100)
    })
  }, 100)
}

// 更新时间周期
function updatePeriod(newPeriod: string) {
  period.value = newPeriod
  updateProfitChart()
}

// 初始化收益图表
function initProfitChart() {
  if (!profitChartRef.value) return

  profitChart = echarts.init(profitChartRef.value)
  updateProfitChart()
}

// 更新收益图表
function updateProfitChart() {
  if (!profitChart) return

  const dataMap: Record<string, number[]> = {
    day: [120000, 121500, 122800, 124200, 123800, 125100, 125679],
    week: [115000, 118000, 121500, 120000, 123000, 124500, 125679],
    month: [100000, 105000, 110000, 108000, 115000, 120000, 125679],
    year: [50000, 70000, 65000, 85000, 90000, 110000, 125679],
  }

  const labelMap: Record<string, string[]> = {
    day: ['00:00', '04:00', '08:00', '12:00', '16:00', '20:00', '24:00'],
    week: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'],
    month: ['第1周', '第2周', '第3周', '第4周', '第5周', '第6周', '第7周'],
    year: ['1月', '3月', '5月', '7月', '9月', '11月', '12月'],
  }

  const option = {
    tooltip: {
      trigger: 'axis',
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      borderColor: '#409eff',
      textStyle: { color: '#fff' },
      formatter: (params: any) => {
        const param = params[0]
        return `${param.axisValue}<br/>收益: <span style="color: #409eff; font-weight: bold">$${param.data.toLocaleString()}</span>`
      },
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
      data: labelMap[period.value],
      axisLine: { lineStyle: { color: '#dcdfe6' } },
      axisLabel: { color: '#606266' },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: {
        color: '#606266',
        formatter: (value: number) => '$' + (value / 1000).toFixed(0) + 'k',
      },
      splitLine: { lineStyle: { color: '#ebeef5', type: 'dashed' } },
    },
    series: [
      {
        name: '收益',
        type: 'line',
        smooth: true,
        symbol: 'circle',
        symbolSize: 8,
        showSymbol: false,
        data: dataMap[period.value],
        itemStyle: { color: '#409eff' },
        lineStyle: { width: 3 },
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
            { offset: 1, color: 'rgba(64, 158, 255, 0.05)' },
          ]),
        },
        emphasis: {
          focus: 'series',
          scale: true,
        },
      },
    ],
    animationDuration: 1000,
    animationEasing: 'cubicOut' as const,
  }

  profitChart.setOption(option, true)
}

// 初始化持仓图表
function initPositionChart() {
  if (!positionChartRef.value) return

  positionChart = echarts.init(positionChartRef.value)

  const option = {
    tooltip: {
      trigger: 'item',
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      textStyle: { color: '#fff' },
      formatter: '{a} <br/>{b}: {c}% ({d}%)',
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
          label: { show: true, fontSize: 14, fontWeight: 'bold', color: '#303133' },
          itemStyle: {
            shadowBlur: 10,
            shadowOffsetX: 0,
            shadowColor: 'rgba(0, 0, 0, 0.3)',
          },
        },
        data: [
          { value: 45, name: 'BTC', itemStyle: { color: '#f7931a' } },
          { value: 30, name: 'ETH', itemStyle: { color: '#627eea' } },
          { value: 15, name: 'BNB', itemStyle: { color: '#f3ba2f' } },
          { value: 10, name: '其他', itemStyle: { color: '#909399' } },
        ],
        animationType: 'scale',
        animationEasing: 'elasticOut',
        animationDelay: () => Math.random() * 200,
      },
    ],
  }

  positionChart.setOption(option)
}

// 处理统计卡片点击
function handleStatClick(stat: any) {
  // 可以根据点击的卡片跳转到相应页面
  console.log('Clicked stat:', stat.title)
}

// 处理窗口大小变化
const handleResize = () => {
  profitChart?.resize()
  positionChart?.resize()
}

onMounted(() => {
  initProfitChart()
  initPositionChart()
  initNumberAnimations()
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  profitChart?.dispose()
  positionChart?.dispose()
  window.removeEventListener('resize', handleResize)
})
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
  border-radius: 16px;
  padding: 24px;
  cursor: pointer;
  overflow: hidden;
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
  border: 1px solid #ebeef5;
  opacity: 0;
  transform: translateY(20px) scale(0.95);

  &.stat-card-enter {
    opacity: 1;
    transform: translateY(0) scale(1);
  }

  &:hover {
    transform: translateY(-6px);
    box-shadow: 0 16px 32px rgba(0, 0, 0, 0.12);

    .stat-card__trend {
      opacity: 1;
      transform: translateX(0);
    }

    .stat-card__icon {
      transform: scale(1.1) rotate(5deg);
    }

    &__bg {
      transform: translate(25%, -25%) scale(1.2);
    }
  }

  &__bg {
    position: absolute;
    top: 0;
    right: 0;
    width: 140px;
    height: 140px;
    border-radius: 50%;
    opacity: 0.08;
    transform: translate(30%, -30%);
    transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  &.primary &__bg {
    background: linear-gradient(135deg, #409eff, #66b1ff);
  }
  &.success &__bg {
    background: linear-gradient(135deg, #67c23a, #85ce61);
  }
  &.danger &__bg {
    background: linear-gradient(135deg, #f56c6c, #f78989);
  }
  &.warning &__bg {
    background: linear-gradient(135deg, #e6a23c, #ebb563);
  }

  &__content {
    display: flex;
    align-items: center;
    gap: 16px;
    position: relative;
    z-index: 1;
  }

  &__icon {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
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

  &__info {
    flex: 1;
  }

  &__value {
    display: flex;
    align-items: baseline;
    gap: 8px;
    margin-bottom: 6px;

    &-text {
      font-size: 26px;
      font-weight: 700;
      color: #303133;
      line-height: 1;
      letter-spacing: -0.5px;
    }
  }

  &__change {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    font-size: 13px;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 12px;

    &.up {
      background: rgba(103, 194, 58, 0.12);
      color: #67c23a;
    }

    &.down {
      background: rgba(245, 108, 108, 0.12);
      color: #f56c6c;
    }

    .el-icon {
      font-size: 14px;
    }
  }

  &__title {
    font-size: 13px;
    color: #909399;
    font-weight: 500;
  }

  &__trend {
    position: absolute;
    bottom: 12px;
    right: 12px;
    width: 70px;
    height: 25px;
    opacity: 0;
    transform: translateX(10px);
    transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);

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
    &.danger svg {
      color: #f56c6c;
    }
    &.warning svg {
      color: #e6a23c;
    }
  }
}

// 卡片通用样式
.chart-card,
.stat-detail-card,
.strategy-card,
.activity-card,
.quick-action-card {
  border-radius: 16px;
  overflow: hidden;
  transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

  &:hover {
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  }

  :deep(.el-card__header) {
    padding: 18px 24px;
    border-bottom: 1px solid #ebeef5;
    background: linear-gradient(180deg, #fafbfc 0%, #f5f7fa 100%);
  }

  :deep(.el-card__body) {
    padding: 24px;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  font-size: 16px;
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
  gap: 12px;
}

.trade-stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);
  border: 1px solid transparent;

  &:hover {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: translateX(6px);
    border-color: rgba(64, 158, 255, 0.2);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);
  }
}

.trade-stat-label {
  font-size: 13px;
  color: #606266;
  font-weight: 500;
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
  padding: 14px 16px;
  background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
  border-radius: 12px;
  transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);
  border: 1px solid transparent;

  &:hover {
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    border-color: rgba(64, 158, 255, 0.2);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);
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
    background: linear-gradient(180deg, #ebeef5 0%, transparent 100%);
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
  box-shadow: 0 0 0 3px #ebeef5;
  transition: all 0.3s;

  &.activity-dot-success {
    background: #67c23a;
    box-shadow: 0 0 0 3px #e1f3d8;
  }
  &.activity-dot-info {
    background: #409eff;
    box-shadow: 0 0 0 3px #d9ecff;
  }
  &.activity-dot-warning {
    background: #e6a23c;
    box-shadow: 0 0 0 3px #faecd8;
  }
  &.activity-dot-danger {
    background: #f56c6c;
    box-shadow: 0 0 0 3px #fde2e2;
  }
  &.activity-dot-default {
    background: #909399;
    box-shadow: 0 0 0 3px #ebeef5;
  }
}

.activity-item:hover .activity-dot {
  transform: scale(1.2);
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
  gap: 10px;
  padding: 24px;
  background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
  border-radius: 16px;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
  flex: 1;
  min-width: 140px;
  border: 1px solid transparent;
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.8) 0%, rgba(255, 255, 255, 0.4) 100%);
    opacity: 0;
    transition: opacity 0.3s;
  }

  &:hover {
    background: #fff;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
    transform: translateY(-4px);
    border-color: rgba(64, 158, 255, 0.2);

    &::before {
      opacity: 1;
    }

    .action-icon {
      transform: scale(1.1) rotate(-5deg);
    }

    .action-hint {
      opacity: 1;
      transform: translateY(0);
    }
  }

  span {
    font-size: 14px;
    color: #303133;
    font-weight: 600;
    position: relative;
    z-index: 1;
  }

  .action-icon {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
    position: relative;
    z-index: 1;
  }

  .action-hint {
    font-size: 12px;
    color: #909399;
    opacity: 0;
    transform: translateY(10px);
    transition: all 0.3s;
    position: relative;
    z-index: 1;
  }
}

// 状态徽章
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    animation: pulse-dot 2s ease-in-out infinite;
  }

  &.status-running {
    background: rgba(103, 194, 58, 0.12);
    color: #67c23a;
  }

  &.status-stopped {
    background: rgba(144, 147, 153, 0.12);
    color: #909399;
  }

  &.status-error {
    background: rgba(245, 108, 108, 0.12);
    color: #f56c6c;
  }

  &.status-paused {
    background: rgba(230, 162, 60, 0.12);
    color: #e6a23c;
  }
}

@keyframes pulse-dot {
  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.5;
    transform: scale(0.8);
  }
}

// 响应式
@media (max-width: 1200px) {
  .stat-card {
    margin-bottom: 16px;
  }

  .quick-action-item {
    min-width: calc(50% - 8px);
  }
}

@media (max-width: 768px) {
  .stat-card {
    margin-bottom: 12px;
  }

  .quick-action-item {
    min-width: calc(33.333% - 11px);
  }
}
</style>
