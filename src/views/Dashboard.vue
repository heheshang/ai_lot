<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <!-- 统计卡片 -->
      <el-col :span="6" v-for="stat in stats" :key="stat.title">
        <el-card class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" :style="{ backgroundColor: stat.color }">
              <el-icon :size="32" :color="stat.iconColor">
                <component :is="stat.icon" />
              </el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stat.value }}</div>
              <div class="stat-title">{{ stat.title }}</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px;">
      <!-- 欢迎信息 -->
      <el-col :span="24">
        <el-card>
          <template #header>
            <span>欢迎使用 AI-LOT 量化交易系统</span>
          </template>
          <p>当前系统状态：<el-tag type="success">运行中</el-tag></p>
          <p>请从左侧菜单选择功能模块开始使用。</p>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  Wallet,
  TrendCharts,
  Document,
  DataLine,
} from '@element-plus/icons-vue';

const stats = ref([
  {
    title: '总资产',
    value: '--',
    icon: Wallet,
    color: '#ecf5ff',
    iconColor: '#409eff',
  },
  {
    title: '运行策略',
    value: '0',
    icon: TrendCharts,
    color: '#f0f9ff',
    iconColor: '#67c23a',
  },
  {
    title: '策略总数',
    value: '0',
    icon: Document,
    color: '#fef0f0',
    iconColor: '#f56c6c',
  },
  {
    title: '今日盈亏',
    value: '--',
    icon: DataLine,
    color: '#fdf6ec',
    iconColor: '#e6a23c',
  },
]);
</script>

<style scoped>
.dashboard {
  padding: 0;
}

.stat-card {
  cursor: pointer;
  transition: all 0.3s;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.stat-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.stat-info {
  flex: 1;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #303133;
}

.stat-title {
  font-size: 14px;
  color: #909399;
  margin-top: 4px;
}
</style>
