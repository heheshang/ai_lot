<template>
  <el-container class="app-layout">
    <!-- 侧边栏 -->
    <el-aside :width="isCollapse ? '64px' : '240px'" class="app-aside">
      <!-- Logo区域 -->
      <div class="logo-section">
        <div class="logo-wrapper">
          <div class="logo-icon">
            <el-icon :size="isCollapse ? 24 : 28"><TrendCharts /></el-icon>
          </div>
          <transition name="logo-text">
            <div v-if="!isCollapse" class="logo-content">
              <div class="logo-title">AI-LOT</div>
              <div class="logo-subtitle">量化交易终端</div>
            </div>
          </transition>
        </div>
      </div>

      <!-- 菜单区域 -->
      <div class="menu-section">
        <el-menu
          :default-active="activeMenu"
          :default-openeds="defaultOpeneds"
          :collapse="isCollapse"
          router
          class="app-menu"
        >
          <!-- 分组：概览 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>概览</span>
          </div>

          <el-tooltip v-if="isCollapse" content="仪表盘" placement="right" :show-after="500">
            <el-menu-item index="/dashboard">
              <template #title>
                <el-icon class="menu-icon"><Odometer /></el-icon>
                <span class="menu-text">仪表盘</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/dashboard">
            <template #title>
              <el-icon class="menu-icon"><Odometer /></el-icon>
              <span class="menu-text">仪表盘</span>
            </template>
          </el-menu-item>

          <!-- 分组：策略 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>策略</span>
          </div>

          <el-tooltip v-if="isCollapse" content="策略列表" placement="right" :show-after="500">
            <el-menu-item index="/strategy">
              <template #title>
                <el-icon class="menu-icon"><Grid /></el-icon>
                <span class="menu-text">策略列表</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy">
            <template #title>
              <el-icon class="menu-icon"><Grid /></el-icon>
              <span class="menu-text">策略列表</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="运行实例" placement="right" :show-after="500">
            <el-menu-item index="/strategy/instances">
              <template #title>
                <el-icon class="menu-icon"><VideoPlay /></el-icon>
                <span class="menu-text">运行实例</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy/instances">
            <template #title>
              <el-icon class="menu-icon"><VideoPlay /></el-icon>
              <span class="menu-text">运行实例</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="新建策略" placement="right" :show-after="500">
            <el-menu-item index="/strategy/editor">
              <template #title>
                <el-icon class="menu-icon"><Plus /></el-icon>
                <span class="menu-text">新建策略</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/strategy/editor">
            <template #title>
              <el-icon class="menu-icon"><Plus /></el-icon>
              <span class="menu-text">新建策略</span>
            </template>
          </el-menu-item>

          <!-- 分组：市场 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>市场</span>
          </div>

          <el-tooltip v-if="isCollapse" content="行情" placement="right" :show-after="500">
            <el-menu-item index="/market">
              <template #title>
                <el-icon class="menu-icon"><TrendCharts /></el-icon>
                <span class="menu-text">行情</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/market">
            <template #title>
              <el-icon class="menu-icon"><TrendCharts /></el-icon>
              <span class="menu-text">行情</span>
            </template>
          </el-menu-item>

          <!-- 分组：交易 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>交易</span>
          </div>

          <el-tooltip v-if="isCollapse" content="回测" placement="right" :show-after="500">
            <el-menu-item index="/backtest">
              <template #title>
                <el-icon class="menu-icon"><DataAnalysis /></el-icon>
                <span class="menu-text">回测</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/backtest">
            <template #title>
              <el-icon class="menu-icon"><DataAnalysis /></el-icon>
              <span class="menu-text">回测</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="交易控制台" placement="right" :show-after="500">
            <el-menu-item index="/trade">
              <template #title>
                <el-icon class="menu-icon"><ShoppingCart /></el-icon>
                <span class="menu-text">交易控制台</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/trade">
            <template #title>
              <el-icon class="menu-icon"><ShoppingCart /></el-icon>
              <span class="menu-text">交易控制台</span>
            </template>
          </el-menu-item>

          <!-- 分组：系统 -->
          <div v-if="!isCollapse" class="menu-group-title">
            <span>系统</span>
          </div>

          <el-tooltip v-if="isCollapse" content="风险监控" placement="right" :show-after="500">
            <el-menu-item index="/risk">
              <template #title>
                <el-icon class="menu-icon"><Warning /></el-icon>
                <span class="menu-text">风险监控</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/risk">
            <template #title>
              <el-icon class="menu-icon"><Warning /></el-icon>
              <span class="menu-text">风险监控</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="系统设置" placement="right" :show-after="500">
            <el-menu-item index="/settings">
              <template #title>
                <el-icon class="menu-icon"><Setting /></el-icon>
                <span class="menu-text">系统设置</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/settings">
            <template #title>
              <el-icon class="menu-icon"><Setting /></el-icon>
              <span class="menu-text">系统设置</span>
            </template>
          </el-menu-item>

          <el-tooltip v-if="isCollapse" content="交易所设置" placement="right" :show-after="500">
            <el-menu-item index="/settings/exchange">
              <template #title>
                <el-icon class="menu-icon"><Coin /></el-icon>
                <span class="menu-text">交易所设置</span>
              </template>
            </el-menu-item>
          </el-tooltip>
          <el-menu-item v-else index="/settings/exchange">
            <template #title>
              <el-icon class="menu-icon"><Coin /></el-icon>
              <span class="menu-text">交易所设置</span>
            </template>
          </el-menu-item>
        </el-menu>
      </div>

      <!-- 底部用户信息 -->
      <div class="user-section">
        <transition name="user-info">
          <div v-if="!isCollapse" class="user-card" @click="goToProfile">
            <div class="user-avatar">
              <el-avatar
                :size="36"
                :src="userStore.user?.avatar"
                :icon="UserFilled"
              />
            </div>
            <div class="user-details">
              <div class="user-name">{{ userStore.user?.displayName || userStore.username }}</div>
              <div class="user-role">{{ userStore.roleName }}</div>
            </div>
            <el-icon class="user-arrow"><ArrowRight /></el-icon>
          </div>
          <div v-else class="user-mini" @click="goToProfile">
            <el-avatar
              :size="32"
              :src="userStore.user?.avatar"
              :icon="UserFilled"
            />
          </div>
        </transition>
      </div>
    </el-aside>

    <!-- 主内容区 -->
    <el-container class="main-container">
      <!-- 顶栏 -->
      <el-header class="app-header">
        <div class="header-left">
          <!-- 移动端菜单按钮 -->
          <el-icon class="mobile-menu-btn" @click="mobileMenuOpen = true">
            <Menu />
          </el-icon>

          <!-- 折叠按钮 -->
          <el-icon class="collapse-btn desktop-only" @click="toggleCollapse">
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>

          <!-- 面包屑 -->
          <el-breadcrumb separator="/" class="breadcrumb">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">
              <el-icon><HomeFilled /></el-icon>
              首页
            </el-breadcrumb-item>
            <el-breadcrumb-item v-if="currentRoute.meta.title">
              {{ currentRoute.meta.title }}
            </el-breadcrumb-item>
          </el-breadcrumb>

          <!-- 最近访问下拉 -->
          <el-dropdown
            v-if="navigationStore.history.length > 1"
            class="recent-dropdown"
            trigger="click"
            :hide-on-click="false"
          >
            <div class="recent-trigger">
              <div class="trigger-icon-wrapper">
                <el-icon class="clock-icon"><Clock /></el-icon>
              </div>
              <span class="trigger-text">最近访问</span>
              <el-badge :value="navigationStore.getRecentPages(route.path, 8).length" :max="9" class="history-badge" />
              <el-icon class="dropdown-icon"><ArrowDown /></el-icon>
            </div>
            <template #dropdown>
              <el-dropdown-menu class="recent-dropdown-menu">
                <!-- 头部 -->
                <div class="recent-header">
                  <div class="recent-header-left">
                    <div class="recent-header-icon">
                      <div class="icon-glow"></div>
                      <el-icon><Clock /></el-icon>
                    </div>
                    <div class="recent-header-text">
                      <span class="recent-header-title">最近访问</span>
                      <span class="recent-count">{{ navigationStore.getRecentPages(route.path, 8).length }} 个页面</span>
                    </div>
                  </div>
                  <el-button
                    text
                    size="small"
                    class="clear-btn"
                    @click.stop="navigationStore.clearHistory()"
                  >
                    <el-icon><Delete /></el-icon>
                    清空
                  </el-button>
                </div>

                <!-- 快捷跳转区 -->
                <div class="quick-access">
                  <div
                    class="quick-item"
                    @click="router.push('/dashboard')"
                    :class="{ 'is-active': route.path === '/dashboard' }"
                  >
                    <div class="quick-item-icon" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
                      <el-icon><HomeFilled /></el-icon>
                    </div>
                    <span class="quick-item-text">仪表盘</span>
                  </div>
                  <div
                    class="quick-item"
                    @click="router.push('/market')"
                    :class="{ 'is-active': route.path === '/market' }"
                  >
                    <div class="quick-item-icon" style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);">
                      <el-icon><TrendCharts /></el-icon>
                    </div>
                    <span class="quick-item-text">行情</span>
                  </div>
                  <div
                    class="quick-item"
                    @click="router.push('/strategy')"
                    :class="{ 'is-active': route.path.startsWith('/strategy') }"
                  >
                    <div class="quick-item-icon" style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);">
                      <el-icon><Grid /></el-icon>
                    </div>
                    <span class="quick-item-text">策略</span>
                  </div>
                  <div
                    class="quick-item"
                    @click="router.push('/backtest')"
                    :class="{ 'is-active': route.path === '/backtest' }"
                  >
                    <div class="quick-item-icon" style="background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);">
                      <el-icon><DataAnalysis /></el-icon>
                    </div>
                    <span class="quick-item-text">回测</span>
                  </div>
                </div>

                <!-- 列表 -->
                <div class="recent-list-wrapper">
                  <div class="recent-list-section">
                    <div class="recent-section-title">
                      <span class="section-dot"></span>
                      浏览历史
                    </div>
                    <div class="recent-list">
                      <TransitionGroup name="recent-item" tag="div">
                        <div
                          v-for="(item, index) in navigationStore.getRecentPages(route.path, 8)"
                          :key="item.path"
                          class="recent-item-wrapper"
                          :class="{ 'is-current': item.path === route.path }"
                          :style="{ '--item-index': index }"
                          @click="router.push(item.path)"
                        >
                          <div class="recent-item">
                            <div class="recent-item-rank">{{ index + 1 }}</div>
                            <div class="recent-item-icon">
                              <div class="icon-bg"></div>
                              <el-icon>
                                <component :is="getPageIcon(item.path)" />
                              </el-icon>
                            </div>
                            <div class="recent-item-content">
                              <div class="recent-item-main">
                                <span class="recent-title">{{ item.title }}</span>
                                <div class="recent-item-badges">
                                  <el-tag v-if="item.path === route.path" size="small" type="success" effect="dark" round>当前</el-tag>
                                  <el-tag v-else-if="index === 0" size="small" type="danger" effect="plain" round>最新</el-tag>
                                </div>
                              </div>
                              <div class="recent-item-meta">
                                <span class="recent-time">{{ formatTime(item.timestamp) }}</span>
                              </div>
                            </div>
                            <div class="recent-item-action">
                              <el-icon class="action-icon"><ArrowRight /></el-icon>
                            </div>
                          </div>
                        </div>
                      </TransitionGroup>
                    </div>
                  </div>
                </div>

                <!-- 底部 -->
                <div class="recent-footer">
                  <div class="footer-stats">
                    <div class="stat-item">
                      <div class="stat-value">{{ navigationStore.history.length }}</div>
                      <div class="stat-label">总访问</div>
                    </div>
                    <el-divider direction="vertical" />
                    <div class="stat-item">
                      <div class="stat-value">{{ new Set(navigationStore.history.map(h => h.path)).size }}</div>
                      <div class="stat-label">页面数</div>
                    </div>
                  </div>
                  <el-divider />
                  <el-button text class="footer-btn" @click="router.push('/dashboard')">
                    <el-icon><HomeFilled /></el-icon>
                    返回首页
                  </el-button>
                </div>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>

        <div class="header-right">
          <!-- 搜索框 -->
          <div class="search-box" @click="showCommandPalette = true">
            <el-input
              v-model="searchText"
              placeholder="搜索... (Ctrl+K)"
              :prefix-icon="Search"
              clearable
              class="search-input"
              readonly
            />
            <div class="search-shortcut">
              <kbd>⌘</kbd><kbd>K</kbd>
            </div>
          </div>

          <!-- 快捷操作 -->
          <div class="quick-actions">
            <!-- 通知 -->
            <el-tooltip content="通知" placement="bottom">
              <el-badge :value="notificationCount" :hidden="notificationCount === 0" class="action-item">
                <el-icon class="action-icon" @click="showNotifications">
                  <Bell />
                </el-icon>
              </el-badge>
            </el-tooltip>

            <!-- 全屏 -->
            <el-tooltip :content="isFullscreen ? '退出全屏' : '全屏'" placement="bottom">
              <el-icon class="action-icon" @click="toggleFullscreen">
                <FullScreen />
              </el-icon>
            </el-tooltip>

            <!-- 主题切换 -->
            <el-tooltip :content="isDark ? '浅色模式' : '深色模式'" placement="bottom">
              <el-icon class="action-icon theme-toggle" @click="toggleTheme">
                <Sunny v-if="isDark" />
                <Moon v-else />
              </el-icon>
            </el-tooltip>
          </div>

          <!-- 分隔线 -->
          <el-divider direction="vertical" class="header-divider" />

          <!-- 用户菜单 -->
          <el-dropdown @command="handleCommand" class="user-dropdown" trigger="click" :hide-on-click="true">
            <div class="user-trigger">
              <div class="user-avatar-wrapper">
                <el-avatar
                  :size="38"
                  :src="userStore.user?.avatar"
                  :icon="UserFilled"
                  class="user-avatar-trigger"
                />
                <span class="avatar-status-indicator"></span>
              </div>
              <div class="user-info-section">
                <div class="user-name-display">{{ userStore.user?.displayName || userStore.username }}</div>
                <div class="user-role-display">{{ userStore.roleName }}</div>
              </div>
              <el-icon class="dropdown-arrow"><ArrowDown /></el-icon>
            </div>
            <template #dropdown>
              <el-dropdown-menu class="user-dropdown-menu">
                <transition name="dropdown-fade">
                  <!-- 用户信息头部 -->
                  <div class="user-profile-header">
                    <div class="profile-background">
                      <div class="background-pattern"></div>
                    </div>
                    <div class="profile-content">
                      <div class="profile-avatar-wrapper">
                        <div class="avatar-ring"></div>
                        <el-avatar
                          :size="64"
                          :src="userStore.user?.avatar"
                          :icon="UserFilled"
                        />
                        <span class="avatar-online-badge"></span>
                      </div>
                      <div class="profile-info">
                        <div class="profile-name">{{ userStore.user?.displayName || userStore.username }}</div>
                        <div class="profile-username">@{{ userStore.username }}</div>
                        <div class="profile-meta">
                          <el-tag :type="getRoleTagType(userStore.roleName)" size="small" effect="dark">
                            {{ userStore.roleName }}
                          </el-tag>
                          <span class="profile-status">
                            <el-icon class="status-dot"><SuccessFilled /></el-icon>
                            <span class="status-text">在线</span>
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>

                  <!-- 用户统计信息 -->
                  <div class="user-stats-section">
                    <div class="stat-item">
                      <div class="stat-value">{{ userIdDisplay }}</div>
                      <div class="stat-label">ID</div>
                    </div>
                    <el-divider direction="vertical" />
                    <div class="stat-item">
                      <div class="stat-value">{{ userCreatedAtDisplay }}</div>
                      <div class="stat-label">加入时间</div>
                    </div>
                    <el-divider direction="vertical" />
                    <div class="stat-item">
                      <div class="stat-value">{{ userStatusDisplay }}</div>
                      <div class="stat-label">状态</div>
                    </div>
                  </div>

                  <!-- 菜单项 -->
                  <div class="user-menu-items">
                    <div class="menu-section-title">账户管理</div>
                    <el-dropdown-item command="profile" class="menu-item-profile">
                      <div class="menu-item-content">
                        <div class="menu-item-icon-wrapper">
                          <el-icon class="menu-item-icon"><User /></el-icon>
                        </div>
                        <div class="menu-item-text">
                          <span class="menu-item-title">个人资料</span>
                          <span class="menu-item-desc">编辑头像、昵称等信息</span>
                        </div>
                        <el-icon class="menu-item-arrow"><ArrowRight /></el-icon>
                      </div>
                    </el-dropdown-item>
                    <el-dropdown-item command="settings" class="menu-item-settings">
                      <div class="menu-item-content">
                        <div class="menu-item-icon-wrapper">
                          <el-icon class="menu-item-icon"><Setting /></el-icon>
                        </div>
                        <div class="menu-item-text">
                          <span class="menu-item-title">偏好设置</span>
                          <span class="menu-item-desc">主题、语言、通知等</span>
                        </div>
                        <el-icon class="menu-item-arrow"><ArrowRight /></el-icon>
                      </div>
                    </el-dropdown-item>
                  </div>

                  <!-- 快速操作 -->
                  <div class="user-quick-actions">
                    <div class="quick-action-item" @click.stop>
                      <el-icon><Message /></el-icon>
                      <span>私信</span>
                      <el-badge :value="notificationCount" :hidden="notificationCount === 0" />
                    </div>
                    <div class="quick-action-item" @click.stop>
                      <el-icon><Bell /></el-icon>
                      <span>提醒</span>
                      <el-badge :value="notificationCount" :hidden="notificationCount === 0" />
                    </div>
                    <div class="quick-action-item" @click.stop>
                      <el-icon><Star /></el-icon>
                      <span>收藏</span>
                    </div>
                    <div class="quick-action-item" @click.stop>
                      <el-icon><Clock /></el-icon>
                      <span>动态</span>
                    </div>
                  </div>

                  <!-- 底部退出登录 -->
                  <div class="user-menu-footer">
                    <el-dropdown-item command="logout" class="menu-item-logout">
                      <div class="logout-content">
                        <el-icon><SwitchButton /></el-icon>
                        <span>退出登录</span>
                        <span class="logout-shortcut">⌘Q</span>
                      </div>
                    </el-dropdown-item>
                  </div>
                </transition>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <!-- 主内容 -->
      <el-main class="app-main">
        <router-view v-slot="{ Component }">
          <transition name="page" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </el-main>
    </el-container>

    <!-- 快速跳转面板 -->
    <CommandPalette v-model:visible="showCommandPalette" />

    <!-- 移动端导航抽屉 -->
    <el-drawer
      v-model="mobileMenuOpen"
      direction="ltr"
      :size="280"
      class="mobile-drawer"
      :with-header="false"
    >
      <div class="mobile-menu">
        <!-- 关闭按钮 -->
        <div class="mobile-menu-header">
          <div class="logo-section">
            <div class="logo-wrapper">
              <div class="logo-icon">
                <el-icon :size="24"><TrendCharts /></el-icon>
              </div>
              <div class="logo-content">
                <div class="logo-title">AI-LOT</div>
                <div class="logo-subtitle">量化交易终端</div>
              </div>
            </div>
          </div>
          <el-icon class="close-btn" @click="mobileMenuOpen = false">
            <Close />
          </el-icon>
        </div>

        <!-- 菜单列表 -->
        <div class="mobile-menu-list">
          <div class="menu-group-title">概览</div>
          <div class="menu-item" @click="navigateTo('/dashboard')">
            <el-icon><Odometer /></el-icon>
            <span>仪表盘</span>
          </div>
          <div class="menu-item" @click="navigateTo('/market')">
            <el-icon><TrendCharts /></el-icon>
            <span>行情</span>
          </div>

          <div class="menu-group-title">策略</div>
          <div class="menu-item" @click="navigateTo('/strategy')">
            <el-icon><Grid /></el-icon>
            <span>策略列表</span>
          </div>
          <div class="menu-item" @click="navigateTo('/strategy/instances')">
            <el-icon><VideoPlay /></el-icon>
            <span>运行实例</span>
          </div>
          <div class="menu-item" @click="navigateTo('/strategy/editor')">
            <el-icon><Plus /></el-icon>
            <span>新建策略</span>
          </div>

          <div class="menu-group-title">交易</div>
          <div class="menu-item" @click="navigateTo('/backtest')">
            <el-icon><DataAnalysis /></el-icon>
            <span>回测</span>
          </div>
          <div class="menu-item" @click="navigateTo('/trade')">
            <el-icon><ShoppingCart /></el-icon>
            <span>交易控制台</span>
          </div>

          <div class="menu-group-title">系统</div>
          <div class="menu-item" @click="navigateTo('/risk')">
            <el-icon><Warning /></el-icon>
            <span>风险监控</span>
          </div>
          <div class="menu-item" @click="navigateTo('/settings')">
            <el-icon><Setting /></el-icon>
            <span>系统设置</span>
          </div>
          <div class="menu-item" @click="navigateTo('/settings/exchange')">
            <el-icon><Coin /></el-icon>
            <span>交易所设置</span>
          </div>
        </div>

        <!-- 用户信息 -->
        <div class="mobile-menu-footer">
          <div class="user-info" @click="goToProfile">
            <el-avatar
              :size="40"
              :src="userStore.user?.avatar"
              :icon="UserFilled"
            />
            <div class="user-details">
              <div class="user-name">{{ userStore.user?.displayName || userStore.username }}</div>
              <div class="user-role">{{ userStore.roleName }}</div>
            </div>
          </div>
          <el-button text @click="handleLogout">
            <el-icon><SwitchButton /></el-icon>
            退出登录
          </el-button>
        </div>
      </div>
    </el-drawer>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessageBox, ElMessage } from 'element-plus';
import { UserStatus } from '@/types';
import {
  Odometer,
  TrendCharts,
  DataAnalysis,
  ShoppingCart,
  Warning,
  Setting,
  Coin,
  Fold,
  Expand,
  UserFilled,
  User,
  SwitchButton,
  Search,
  Bell,
  FullScreen,
  Sunny,
  Moon,
  ArrowDown,
  ArrowRight,
  HomeFilled,
  Grid,
  VideoPlay,
  Plus,
  Clock,
  Menu,
  Close,
  SuccessFilled,
  Message,
  Star,
  Delete,
  CircleCheck,
  View,
} from '@element-plus/icons-vue';
import { useUserStore, useNavigationStore } from '@/store';
import CommandPalette from '@/components/CommandPalette.vue';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();
const navigationStore = useNavigationStore();

// 状态
const isCollapse = ref(false);
const searchText = ref('');
const notificationCount = ref(3);
const isDark = ref(false);
const showCommandPalette = ref(false);
const mobileMenuOpen = ref(false);
const isFullscreen = ref(false);

// 计算属性
const currentRoute = computed(() => route);

const activeMenu = computed(() => {
  const path = route.path;
  // 策略相关页面统一使用 strategy 作为父级菜单
  if (path.startsWith('/strategy')) {
    return path;
  }
  return path;
});

// 控制子菜单展开状态（目前没有子菜单，保留备用）
const defaultOpeneds = computed(() => {
  return [];
});

// 用户相关计算属性
const userIdDisplay = computed(() => userStore.user?.id?.slice(-6) || '------');
const userCreatedAtDisplay = computed(() => formatDateShort(userStore.user?.createdAt));
const userStatusDisplay = computed(() => getStatusText(userStore.user?.status));

// 方法
function toggleCollapse() {
  isCollapse.value = !isCollapse.value;
}

// 移动端导航
function navigateTo(path: string) {
  router.push(path);
  mobileMenuOpen.value = false;
}

async function handleLogout() {
  try {
    await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });

    await userStore.logout();
    ElMessage.success('已退出登录');
    router.push('/login');
  } catch {
    // 取消退出
  }
}

// 格式化时间戳
function formatTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp;

  const minute = 60 * 1000;
  const hour = 60 * minute;
  const day = 24 * hour;

  if (diff < minute) {
    return '刚刚';
  } else if (diff < hour) {
    return `${Math.floor(diff / minute)}分钟前`;
  } else if (diff < day) {
    return `${Math.floor(diff / hour)}小时前`;
  } else if (diff < 7 * day) {
    return `${Math.floor(diff / day)}天前`;
  } else {
    const date = new Date(timestamp);
    return `${date.getMonth() + 1}/${date.getDate()}`;
  }
}

// 根据页面路径获取对应图标
function getPageIcon(path: string): any {
  const iconMap: Record<string, any> = {
    '/dashboard': Odometer,
    '/market': TrendCharts,
    '/strategy': Grid,
    '/strategy/editor': Plus,
    '/strategy/instances': VideoPlay,
    '/backtest': DataAnalysis,
    '/trade': ShoppingCart,
    '/risk': Warning,
    '/settings': Setting,
    '/settings/exchange': Coin,
  };

  // 精确匹配
  if (iconMap[path]) {
    return iconMap[path];
  }

  // 前缀匹配
  for (const [key, icon] of Object.entries(iconMap)) {
    if (path.startsWith(key) && key !== '/') {
      return icon;
    }
  }

  return HomeFilled; // 默认图标
}

function showNotifications() {
  ElMessage.info('通知功能开发中');
}

function toggleFullscreen() {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
    isFullscreen.value = true;
  } else {
    document.exitFullscreen();
    isFullscreen.value = false;
  }
}

function getRoleTagType(roleName: string): 'success' | 'warning' | 'info' | 'danger' | '' {
  const roleMap: Record<string, 'success' | 'warning' | 'info' | 'danger' | ''> = {
    '管理员': 'danger',
    '开发者': 'warning',
    '交易员': 'success',
    '审计员': 'info',
  };
  return roleMap[roleName] || '';
}

function formatDateShort(timestamp?: number): string {
  if (!timestamp) return '--';
  const date = new Date(timestamp);
  const now = new Date();
  const diffTime = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return '今天';
  if (diffDays === 1) return '昨天';
  if (diffDays < 7) return `${diffDays}天前`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)}周前`;
  if (diffDays < 365) return `${Math.floor(diffDays / 30)}月前`;
  return `${Math.floor(diffDays / 365)}年前`;
}

function getStatusText(status?: UserStatus): string {
  if (status === UserStatus.ACTIVE) return '正常';
  if (status === UserStatus.DISABLED) return '已禁用';
  if (status === UserStatus.LOCKED) return '已锁定';
  return '未知';
}

function toggleTheme() {
  isDark.value = !isDark.value;
  ElMessage.success(isDark.value ? '已切换到深色模式' : '已切换到浅色模式');
}

function goToProfile() {
  router.push('/profile');
}

async function handleCommand(command: string) {
  switch (command) {
    case 'profile':
      router.push('/profile');
      break;
    case 'settings':
      router.push('/settings');
      break;
    case 'logout':
      try {
        await ElMessageBox.confirm('确定要退出登录吗？', '提示', {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning',
        });

        await userStore.logout();
        ElMessage.success('已退出登录');
        router.push('/login');
      } catch {
        // 取消退出
      }
      break;
  }
}

// 监听路由变化，添加到历史记录
watch(
  () => route.path,
  (newPath, oldPath) => {
    // 只在路径变化且不是登录页时记录
    if (newPath !== oldPath && newPath !== '/login' && oldPath !== '/login') {
      navigationStore.addToHistory(newPath, route.meta.title as string);
    }
  },
  { immediate: true }
);
</script>

<style scoped lang="scss">
.app-layout {
  height: 100vh;
  overflow: hidden;
}

// ========== 侧边栏 ==========
.app-aside {
  display: flex;
  flex-direction: column;
  background: linear-gradient(180deg, #1a1d2d 0%, #141722 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  box-shadow: 4px 0 20px rgba(0, 0, 0, 0.1);
}

// Logo区域
.logo-section {
  flex-shrink: 0;
  padding: 20px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);
}

.logo-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  transition: all 0.3s ease;

  &:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: translateX(4px);
  }
}

.logo-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.5);
  transition: all 0.3s ease;

  .logo-wrapper:hover & {
    transform: rotate(5deg) scale(1.05);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
  }
}

.logo-content {
  flex: 1;
  min-width: 0;
}

.logo-title {
  font-size: 19px;
  font-weight: 700;
  color: #fff;
  line-height: 1.2;
  background: linear-gradient(135deg, #fff 0%, #e0e7ff 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: 0.5px;
}

.logo-subtitle {
  font-size: 11px;
  color: rgba(255, 255, 255, 0.55);
  margin-top: 3px;
  font-weight: 500;
  letter-spacing: 0.3px;
}

// Logo文字过渡动画
.logo-text-enter-active,
.logo-text-leave-active {
  transition: all 0.3s ease;
}

.logo-text-enter-from,
.logo-text-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

// 菜单区域
.menu-section {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 16px 12px;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;

    &:hover {
      background: rgba(255, 255, 255, 0.25);
    }
  }
}

.menu-group-title {
  padding: 12px 12px 8px 12px;
  font-size: 11px;
  font-weight: 700;
  color: rgba(255, 255, 255, 0.4);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 4px;
}

.app-menu {
  border-right: none;
  background: transparent;

  // 工具提示包裹的菜单项需要正确显示
  :deep(.el-tooltip__popper) {
    z-index: 10000;
  }

  :deep(.el-menu-item),
  :deep(.el-sub-menu__title) {
    color: rgba(255, 255, 255, 0.7);
    margin: 0 0 6px 0;
    border-radius: 12px;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    height: 48px;
    line-height: 48px;
    position: relative;
    overflow: hidden;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
      opacity: 0;
      transition: opacity 0.25s ease;
      border-radius: 12px;
    }

    .menu-icon {
      font-size: 19px;
      margin-right: 10px;
      position: relative;
      z-index: 1;
      transition: transform 0.25s ease;
    }

    .menu-text {
      font-size: 14px;
      font-weight: 500;
      position: relative;
      z-index: 1;
    }
  }

  :deep(.el-menu-item:hover),
  :deep(.el-sub-menu__title:hover) {
    background: rgba(255, 255, 255, 0.06) !important;
    color: #fff;
    transform: translateX(4px);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);

    &::before {
      opacity: 1;
    }

    .menu-icon {
      transform: scale(1.1);
    }
  }

  :deep(.el-menu-item.is-active) {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.25) 0%, rgba(118, 75, 162, 0.25) 100%) !important;
    color: #fff !important;
    box-shadow: 0 4px 16px rgba(102, 126, 234, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(102, 126, 234, 0.3);

    &::before {
      opacity: 1;
    }

    .menu-icon {
      color: #a5b4fc;
      filter: drop-shadow(0 0 8px rgba(165, 180, 252, 0.5));
    }
  }

  :deep(.el-sub-menu) {
    .el-sub-menu__title {
      .el-icon {
        font-size: 14px;
        margin-left: auto;
        transition: transform 0.25s ease;
      }
    }

    .el-menu {
      background: rgba(0, 0, 0, 0.15);
      margin: 4px 0 8px 0;
      border-radius: 8px;
      padding: 4px 0;

      .el-menu-item {
        padding-left: 52px !important;
        height: 42px;
        line-height: 42px;
        font-size: 13px;
        margin: 2px 8px;
        border-radius: 8px;

        &.is-active {
          background: rgba(102, 126, 234, 0.2) !important;
        }
      }
    }
  }
}

// 底部用户区域
.user-section {
  flex-shrink: 0;
  padding: 12px 12px 16px 12px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);
}

.user-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.25s ease;
  border: 1px solid rgba(255, 255, 255, 0.05);

  &:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);

    .user-arrow {
      transform: translateX(4px);
      color: #fff;
    }
  }

  .user-avatar {
    flex-shrink: 0;
  }

  .user-details {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 13px;
    font-weight: 600;
    color: #fff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .user-role {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.55);
    margin-top: 2px;
  }

  .user-arrow {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.4);
    transition: all 0.25s ease;
    flex-shrink: 0;
  }
}

.user-mini {
  display: flex;
  justify-content: center;
  padding: 4px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    .el-avatar {
      transform: scale(1.1);
    }
  }

  &:active {
    .el-avatar {
      transform: scale(0.95);
    }
  }

  .el-avatar {
    transition: transform 0.2s ease;
  }
}

.user-info-enter-active,
.user-info-leave-active {
  transition: all 0.3s ease;
}

.user-info-enter-from,
.user-info-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

// ========== 主内容区 ==========
.main-container {
  flex: 1;
  overflow: hidden;
}

// ========== 顶部栏 ==========
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 64px;
  padding: 0 28px;
  background: linear-gradient(180deg, #ffffff 0%, #fafbfc 100%);
  border-bottom: 1px solid #e8ecf0;
  box-shadow: 0 1px 6px rgba(0, 0, 0, 0.04);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.collapse-btn {
  font-size: 19px;
  cursor: pointer;
  color: #606266;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: scale(1.05);
  }

  &:active {
    transform: scale(0.95);
  }
}

.mobile-menu-btn {
  display: none;
  font-size: 20px;
  cursor: pointer;
  color: #606266;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: scale(1.05);
  }
}

.breadcrumb {
  :deep(.el-breadcrumb__item) {
    .el-breadcrumb__inner {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 13px;
      color: #606266;
      font-weight: 500;
      padding: 6px 12px;
      border-radius: 8px;
      transition: all 0.2s ease;

      &:hover {
        color: #409eff;
        background: #f5f7fa;
      }

      .el-icon {
        font-size: 14px;
      }
    }

    &:last-child {
      .el-breadcrumb__inner {
        color: #303133;
        font-weight: 600;

        &:hover {
          background: transparent;
        }
      }
    }

    .el-breadcrumb__separator {
      color: #c0c4cc;
      font-weight: 500;
    }
  }
}

// 最近访问下拉
.recent-dropdown {
  margin-left: 8px;

  .recent-trigger {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 20px;
    border-radius: 16px;
    font-size: 14px;
    color: #606266;
    cursor: pointer;
    transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
    border: 2px solid transparent;
    font-weight: 600;
    position: relative;
    background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
    overflow: hidden;
    backdrop-filter: blur(10px);

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.08) 0%, rgba(118, 75, 162, 0.04) 100%);
      opacity: 0;
      transition: opacity 0.4s ease;
      border-radius: 16px;
    }

    &::after {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      width: 0;
      height: 0;
      background: radial-gradient(circle, rgba(64, 158, 255, 0.1) 0%, transparent 70%);
      border-radius: 50%;
      transform: translate(-50%, -50%);
      transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    .trigger-icon-wrapper {
      width: 32px;
      height: 32px;
      border-radius: 12px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      position: relative;
      z-index: 1;
      box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);

      &::before {
        content: '';
        position: absolute;
        inset: -2px;
        border-radius: 14px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        opacity: 0;
        transition: opacity 0.4s ease;
        z-index: -1;
        filter: blur(8px);
      }

      .clock-icon {
        font-size: 16px;
        color: #fff;
        transition: all 0.4s ease;
        animation: clockPulse 3s ease-in-out infinite;
      }
    }

    .trigger-text {
      position: relative;
      z-index: 1;
      font-weight: 600;
      letter-spacing: 0.3px;
      background: linear-gradient(135deg, #606266 0%, #909399 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
      transition: all 0.4s ease;
    }

    .history-badge {
      position: relative;
      z-index: 1;

      :deep(.el-badge__content) {
        background: linear-gradient(135deg, #f43f5e 0%, #ef4444 100%);
        border: 3px solid #fff;
        font-size: 10px;
        height: 20px;
        line-height: 14px;
        padding: 0 6px;
        box-shadow: 0 4px 12px rgba(244, 63, 94, 0.4);
        font-weight: 700;
        animation: badgePulse 2s ease-in-out infinite;
      }
    }

    .dropdown-icon {
      font-size: 14px;
      color: #c0c4cc;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      position: relative;
      z-index: 1;
    }

    &:hover {
      background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
      color: #0ea5e9;
      border-color: rgba(14, 165, 233, 0.3);
      transform: translateY(-3px);
      box-shadow: 0 12px 32px rgba(14, 165, 233, 0.3), 0 0 0 1px rgba(14, 165, 233, 0.1);

      &::after {
        width: 200px;
        height: 200px;
        opacity: 0.6;
      }

      .trigger-icon-wrapper {
        transform: rotate(20deg) scale(1.15);
        box-shadow: 0 8px 28px rgba(102, 126, 234, 0.5);

        &::before {
          opacity: 0.6;
        }

        .clock-icon {
          transform: rotate(-20deg);
        }
      }

      .trigger-text {
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        transform: scale(1.05);
      }

      .dropdown-icon {
        transform: translateY(4px) rotate(180deg);
        color: #0ea5e9;
      }

      &::before {
        opacity: 1;
      }
    }

    &:active {
      transform: translateY(-1px) scale(0.97);
      box-shadow: 0 8px 24px rgba(14, 165, 233, 0.25);
    }
  }

  &.is-active {
    .recent-trigger {
      background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
      color: #0ea5e9;
      border-color: rgba(14, 165, 233, 0.4);
      box-shadow: 0 8px 28px rgba(14, 165, 233, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.6);

      &::before {
        opacity: 1;
      }

      .trigger-icon-wrapper {
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        box-shadow: 0 6px 24px rgba(14, 165, 233, 0.4);

        &::before {
          opacity: 0.5;
        }
      }

      .trigger-text {
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
      }

      .dropdown-icon {
        transform: rotate(180deg);
        color: #0ea5e9;
      }
    }
  }
}

// 下拉菜单样式
:deep(.recent-dropdown-menu) {
  padding: 0;
  border-radius: 20px;
  box-shadow: 0 25px 80px rgba(0, 0, 0, 0.15), 0 10px 32px rgba(0, 0, 0, 0.1), 0 0 0 1px rgba(14, 165, 233, 0.1);
  border: 2px solid rgba(14, 165, 233, 0.08);
  overflow: hidden;
  min-width: 340px;
  max-width: 400px;
  background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
  backdrop-filter: blur(20px);

  // 头部
  .recent-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px 22px;
    border-bottom: 1px solid rgba(226, 232, 240, 0.8);
    background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
    position: relative;
    overflow: hidden;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, rgba(14, 165, 233, 0.03) 0%, rgba(2, 132, 199, 0.02) 100%);
      z-index: 0;
    }

    &::after {
      content: '';
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      height: 2px;
      background: linear-gradient(90deg, transparent, rgba(14, 165, 233, 0.3) 20%, rgba(2, 132, 199, 0.3) 80%, transparent);
    }

    .recent-header-left {
      display: flex;
      align-items: center;
      gap: 14px;
      position: relative;
      z-index: 1;
    }

    .recent-header-icon {
      width: 44px;
      height: 44px;
      border-radius: 14px;
      background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      box-shadow: 0 8px 24px rgba(14, 165, 233, 0.4), 0 0 0 1px rgba(14, 165, 233, 0.2);
      position: relative;
      transition: all 0.3s ease;

      &::before {
        content: '';
        position: absolute;
        inset: -3px;
        border-radius: 18px;
        background: inherit;
        filter: blur(12px);
        opacity: 0.6;
        animation: headerIconGlow 4s ease-in-out infinite;
        z-index: -1;
      }

      .icon-glow {
        position: absolute;
        inset: -6px;
        border-radius: 20px;
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        filter: blur(16px);
        opacity: 0.4;
        animation: iconGlow 3s ease-in-out infinite;
      }

      .el-icon {
        font-size: 20px;
        position: relative;
        z-index: 2;
        transition: transform 0.3s ease;
      }

      &:hover {
        transform: scale(1.05);
        box-shadow: 0 10px 30px rgba(14, 165, 233, 0.5);

        .el-icon {
          transform: rotate(10deg);
        }
      }
    }

    .recent-header-text {
      display: flex;
      flex-direction: column;
      gap: 4px;
      position: relative;
      z-index: 1;

      .recent-header-title {
        font-size: 16px;
        font-weight: 800;
        background: linear-gradient(135deg, #0f172a 0%, #475569 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        letter-spacing: 0.4px;
        line-height: 1.2;
      }

      .recent-count {
        font-size: 12px;
        color: #64748b;
        font-weight: 600;
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
      }
    }

    .clear-btn {
      color: #64748b;
      font-size: 12px;
      padding: 8px 16px;
      border-radius: 10px;
      transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
      font-weight: 600;
      background: linear-gradient(135deg, #ffffff 0%, #f8fafc 100%);
      border: 1px solid rgba(226, 232, 240, 0.8);
      position: relative;
      z-index: 1;
      backdrop-filter: blur(10px);

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, rgba(244, 63, 94, 0.05) 0%, rgba(239, 68, 68, 0.02) 100%);
        opacity: 0;
        transition: opacity 0.3s ease;
        border-radius: 10px;
      }

      .el-icon {
        font-size: 14px;
        margin-right: 4px;
        transition: transform 0.3s ease;
      }

      &:hover {
        color: #dc2626;
        border-color: rgba(244, 63, 94, 0.3);
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(244, 63, 94, 0.15);

        &::before {
          opacity: 1;
        }

        .el-icon {
          transform: scale(1.2) rotate(-5deg);
        }
      }

      &:active {
        transform: translateY(-1px) scale(0.95);
      }
    }
  }

  // 快捷访问区
  .quick-access {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
    padding: 20px 22px 16px;
    background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%);
    position: relative;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, rgba(14, 165, 233, 0.02) 0%, rgba(2, 132, 199, 0.01) 100%);
    }

    .quick-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 10px;
      padding: 16px 10px;
      border-radius: 16px;
      cursor: pointer;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
      border: 2px solid rgba(226, 232, 240, 0.6);
      position: relative;
      overflow: hidden;
      backdrop-filter: blur(10px);

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, rgba(14, 165, 233, 0.08) 0%, rgba(2, 132, 199, 0.04) 100%);
        opacity: 0;
        transition: opacity 0.4s ease;
        border-radius: 16px;
      }

      &::after {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        width: 0;
        height: 0;
        background: radial-gradient(circle, rgba(14, 165, 233, 0.1) 0%, transparent 70%);
        border-radius: 50%;
        transform: translate(-50%, -50%);
        transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
      }

      .quick-item-icon {
        width: 40px;
        height: 40px;
        border-radius: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(255, 255, 255, 0.1) inset;
        transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
        position: relative;
        z-index: 1;

        &::before {
          content: '';
          position: absolute;
          inset: -2px;
          border-radius: 14px;
          background: inherit;
          opacity: 0;
          transition: opacity 0.4s ease;
          filter: blur(8px);
          z-index: -1;
        }

        .el-icon {
          font-size: 18px;
          transition: transform 0.4s ease;
        }
      }

      .quick-item-text {
        font-size: 12px;
        font-weight: 700;
        color: #475569;
        transition: all 0.3s ease;
        position: relative;
        z-index: 1;
        letter-spacing: 0.3px;
        text-align: center;
        line-height: 1.2;
      }

      &:hover {
        transform: translateY(-4px);
        box-shadow: 0 12px 32px rgba(14, 165, 233, 0.2), 0 0 0 1px rgba(14, 165, 233, 0.2);
        border-color: rgba(14, 165, 233, 0.4);

        &::before {
          opacity: 1;
        }

        &::after {
          width: 150px;
          height: 150px;
          opacity: 0.4;
        }

        .quick-item-icon {
          transform: scale(1.2) rotate(8deg);
          box-shadow: 0 10px 28px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(255, 255, 255, 0.2) inset;

          &::before {
            opacity: 0.6;
          }

          .el-icon {
            transform: rotate(-8deg);
          }
        }

        .quick-item-text {
          color: #0ea5e9;
          transform: scale(1.05);
        }
      }

      &:active {
        transform: translateY(-2px) scale(0.97);
        box-shadow: 0 8px 24px rgba(14, 165, 233, 0.15);
      }

      &.is-active {
        border-color: #0ea5e9;
        background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
        box-shadow: 0 8px 24px rgba(14, 165, 233, 0.15), inset 0 1px 0 rgba(255, 255, 255, 0.4);

        .quick-item-text {
          color: #0ea5e9;
          font-weight: 800;
        }

        &::before {
          opacity: 0.5;
        }

        &::after {
          content: '';
          position: absolute;
          bottom: 0;
          left: 50%;
          transform: translateX(-50%);
          width: 40px;
          height: 4px;
          background: linear-gradient(90deg, #0ea5e9 0%, #0284c7 100%);
          border-radius: 2px 2px 0 0;
          box-shadow: 0 2px 8px rgba(14, 165, 233, 0.4);
        }

        .quick-item-icon {
          transform: scale(1.1);
          box-shadow: 0 8px 24px rgba(14, 165, 233, 0.3);

          &::before {
            opacity: 0.4;
          }
        }
      }
    }
  }

  // 列表包装区
  .recent-list-wrapper {
    background: linear-gradient(180deg, #ffffff 0%, #f8fafc 100%);
    border-top: 1px solid rgba(226, 232, 240, 0.6);
  }

  .recent-list-section {
    .recent-section-title {
      display: flex;
      align-items: center;
      gap: 10px;
      padding: 16px 22px 12px;
      font-size: 12px;
      font-weight: 800;
      color: #64748b;
      text-transform: uppercase;
      letter-spacing: 1px;
      background: linear-gradient(135deg, rgba(14, 165, 233, 0.02) 0%, rgba(2, 132, 199, 0.01) 100%);

      .section-dot {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        animation: sectionPulse 2.5s ease-in-out infinite;
        box-shadow: 0 0 10px rgba(14, 165, 233, 0.4);
      }
    }
  }

  // 列表区域
  .recent-list {
    max-height: 360px;
    overflow-y: auto;
    padding: 10px 0;

    &::-webkit-scrollbar {
      width: 6px;
    }

    &::-webkit-scrollbar-track {
      background: rgba(241, 245, 249, 0.5);
      border-radius: 3px;
    }

    &::-webkit-scrollbar-thumb {
      background: linear-gradient(180deg, rgba(14, 165, 233, 0.2) 0%, rgba(2, 132, 199, 0.3) 100%);
      border-radius: 3px;
      transition: all 0.3s ease;

      &:hover {
        background: linear-gradient(180deg, rgba(14, 165, 233, 0.4) 0%, rgba(2, 132, 199, 0.5) 100%);
      }
    }
  }

  .recent-item-wrapper {
    padding: 0 12px;
    margin: 4px 0;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    animation: itemSlideIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) backwards;
    animation-delay: calc(var(--item-index) * 0.06s);

    &:hover {
      .recent-item {
        background: linear-gradient(90deg, #f0f9ff 0%, #e0f2fe 100%);
        transform: translateX(6px);
        box-shadow: 0 4px 16px rgba(14, 165, 233, 0.15);

        .recent-item-rank {
          color: #0ea5e9;
          background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
          border: 1px solid rgba(14, 165, 233, 0.2);
        }

        .recent-item-icon {
          transform: scale(1.15) rotate(-8deg);

          .icon-bg {
            opacity: 1;
          }
        }

        .recent-item-action {
          .action-icon {
            opacity: 1;
            transform: translateX(0);
            color: #0ea5e9;
          }
        }
      }
    }

    &:active {
      transform: scale(0.98);
    }

    &.is-current {
      .recent-item {
        background: linear-gradient(90deg, #dbeafe 0%, #bfdbfe 100%);
        position: relative;
        border: 1px solid rgba(14, 165, 233, 0.2);

        &::before {
          content: '';
          position: absolute;
          left: 0;
          top: 50%;
          transform: translateY(-50%);
          width: 5px;
          height: 80%;
          background: linear-gradient(180deg, #0ea5e9 0%, #0284c7 100%);
          border-radius: 0 4px 4px 0;
          box-shadow: 0 0 16px rgba(14, 165, 233, 0.5);
        }

        .recent-item-rank {
          background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
          color: #fff;
          box-shadow: 0 6px 16px rgba(14, 165, 233, 0.4);
          border: 1px solid rgba(14, 165, 233, 0.3);
        }

        .recent-item-icon {
          background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
          box-shadow: 0 8px 24px rgba(14, 165, 233, 0.4);
          border: 1px solid rgba(14, 165, 233, 0.3);

          .el-icon {
            color: #fff;
          }

          .icon-bg {
            opacity: 0;
          }
        }

        .recent-item-action .action-icon {
          opacity: 1;
          transform: translateX(0) scale(1.1);
          color: #0ea5e9;
        }
      }
    }
  }

  .recent-item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px 16px;
    width: 100%;
    transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
    border-radius: 12px;
    position: relative;
    overflow: hidden;
    background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
    border: 1px solid rgba(226, 232, 240, 0.8);

    &::after {
      content: '';
      position: absolute;
      top: 0;
      left: -100%;
      width: 100%;
      height: 100%;
      background: linear-gradient(90deg, transparent, rgba(14, 165, 233, 0.1), transparent);
      transition: left 0.8s ease;
    }

    &:hover::after {
      left: 100%;
    }

    .recent-item-rank {
      width: 30px;
      height: 30px;
      border-radius: 10px;
      background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 12px;
      font-weight: 800;
      color: #64748b;
      flex-shrink: 0;
      transition: all 0.4s ease;
      position: relative;
      z-index: 1;
      border: 1px solid rgba(226, 232, 240, 0.6);
    }

    .recent-item-icon {
      width: 42px;
      height: 42px;
      border-radius: 12px;
      background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
      position: relative;
      border: 1px solid rgba(226, 232, 240, 0.6);
      overflow: hidden;

      .icon-bg {
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, rgba(14, 165, 233, 0.08) 0%, rgba(2, 132, 199, 0.04) 100%);
        opacity: 0;
        transition: opacity 0.4s ease;
        border-radius: 12px;
      }

      .el-icon {
        font-size: 18px;
        color: #475569;
        position: relative;
        z-index: 1;
        transition: all 0.4s ease;
      }
    }

    .recent-item-content {
      flex: 1;
      min-width: 0;
    }

    .recent-item-main {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 8px;
      margin-bottom: 5px;

      .recent-title {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        letter-spacing: 0.2px;
      }

      .recent-item-badges {
        display: flex;
        gap: 4px;
        flex-shrink: 0;

        .el-tag {
          font-size: 10px;
          height: 22px;
          line-height: 20px;
          padding: 0 10px;
          font-weight: 700;
          border: none;
          letter-spacing: 0.3px;

          &.el-tag--success {
            background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
            color: #fff;
            box-shadow: 0 2px 8px rgba(16, 185, 129, 0.3);

            &.el-tag--dark {
              background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
            }
          }

          &.el-tag--danger {
            background: linear-gradient(135deg, #f43f5e 0%, #ef4444 100%);
            color: #fff;
            box-shadow: 0 2px 8px rgba(244, 63, 94, 0.3);
          }
        }
      }
    }

    .recent-item-meta {
      display: flex;
      align-items: center;
      gap: 12px;
      font-size: 11px;
      color: #64748b;

      .recent-time {
        display: flex;
        align-items: center;
        font-weight: 600;
        background: linear-gradient(135deg, #64748b 0%, #94a3b8 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
      }
    }

    .recent-item-action {
      .action-icon {
        font-size: 18px;
        color: #cbd5e1;
        opacity: 0;
        transform: translateX(-10px);
        transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      }
    }
  }

  // 底部
  .recent-footer {
    padding: 18px 22px 20px;
    border-top: 1px solid rgba(226, 232, 240, 0.8);
    background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);

    .footer-stats {
      display: flex;
      align-items: center;
      justify-content: center;
      padding-bottom: 16px;

      .stat-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 6px;
        padding: 0 20px;

        .stat-value {
          font-size: 20px;
          font-weight: 800;
          background: linear-gradient(135deg, #0f172a 0%, #475569 100%);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
          background-clip: text;
          line-height: 1;
        }

        .stat-label {
          font-size: 11px;
          color: #64748b;
          font-weight: 700;
          text-transform: uppercase;
          letter-spacing: 0.8px;
        }
      }

      .el-divider--vertical {
        height: 32px;
        margin: 0;
        border-color: rgba(226, 232, 240, 0.8);
      }
    }

    .footer-btn {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
      color: #475569;
      font-size: 14px;
      padding: 12px 16px;
      border-radius: 12px;
      transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
      font-weight: 600;
      background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
      border: 2px solid rgba(226, 232, 240, 0.8);

      .el-icon {
        font-size: 16px;
        transition: transform 0.3s ease;
      }

      &:hover {
        color: #0ea5e9;
        background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
        border-color: rgba(14, 165, 233, 0.4);
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(14, 165, 233, 0.2);

        .el-icon {
          transform: scale(1.1);
        }
      }

      &:active {
        transform: translateY(-1px) scale(0.98);
      }
    }
  }
}

// 列表项动画
.recent-item-enter-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.recent-item-enter-from {
  opacity: 0;
  transform: translateX(-20px) scale(0.95);
}

.recent-item-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 1, 1);
}

.recent-item-leave-to {
  opacity: 0;
  transform: translateX(20px);
  height: 0;
  margin: 0;
  padding: 0;
}

.recent-item-move {
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.header-right {
  display: flex;
  align-items: center;
  gap: 18px;
}

// 搜索框
.search-box {
  position: relative;
  width: 280px;
  cursor: pointer;

  .search-input {
    :deep(.el-input__wrapper) {
      border-radius: 12px;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.04);
      transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
      cursor: pointer;
      border: 1px solid #e8ecf0;

      &:hover {
        box-shadow: 0 3px 10px rgba(0, 0, 0, 0.08);
        border-color: #d9d9d9;
      }
    }

    :deep(.el-input__inner) {
      cursor: pointer;
      font-size: 13px;
    }
  }

  .search-shortcut {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    gap: 2px;
    pointer-events: none;

    kbd {
      padding: 3px 6px;
      font-size: 10px;
      background: linear-gradient(135deg, #f5f7fa 0%, #ebeef5 100%);
      border: 1px solid #e4e7ed;
      border-radius: 6px;
      font-family: inherit;
      color: #606266;
      font-weight: 500;
    }
  }
}

// 快捷操作
.quick-actions {
  display: flex;
  align-items: center;
  gap: 4px;

  .el-badge {
    display: flex;
    align-items: center;
  }
}

.action-icon {
  font-size: 18px;
  color: #606266;
  cursor: pointer;
  padding: 10px;
  border-radius: 10px;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  justify-content: center;

  &:hover {
    color: #409eff;
    background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.2);
  }

  &:active {
    transform: translateY(0);
  }

  &.theme-toggle:hover {
    background: linear-gradient(135deg, #fff7e6 0%, #ffe7ba 100%);
    color: #e6a23c;
    box-shadow: 0 4px 12px rgba(230, 162, 60, 0.2);
  }
}

// 分隔线
.header-divider {
  height: 24px;
  margin: 0 8px;
  border-color: #e8ecf0;
}

// 用户下拉菜单
.user-dropdown {
  .user-trigger {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 12px 6px 6px;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    border: 1px solid transparent;
    position: relative;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      border-radius: 12px;
      background: linear-gradient(135deg, #409eff 0%, #66b1ff 100%);
      opacity: 0;
      transition: opacity 0.3s ease;
      z-index: -1;
    }

    &:hover {
      background: linear-gradient(135deg, #f5f7fa 0%, #ecf5ff 100%);
      border-color: rgba(64, 158, 255, 0.15);
      transform: translateY(-2px);
      box-shadow:
        0 4px 16px rgba(64, 158, 255, 0.15),
        0 0 0 1px rgba(64, 158, 255, 0.05);

      .user-avatar-trigger {
        transform: scale(1.08);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
      }

      .dropdown-arrow {
        transform: rotate(180deg);
        color: #409eff;
      }

      .avatar-status-indicator {
        transform: scale(1.3);
        box-shadow: 0 0 8px rgba(103, 194, 58, 0.6);
      }

      .user-name-display {
        color: #409eff;
      }
    }

    &:active {
      transform: translateY(0);
      box-shadow:
        0 2px 8px rgba(64, 158, 255, 0.1),
        0 0 0 1px rgba(64, 158, 255, 0.05);
    }

    .user-avatar-wrapper {
      position: relative;
      display: inline-block;

      .user-avatar-trigger {
        transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        border: 2px solid transparent;
      }

      .avatar-status-indicator {
        position: absolute;
        bottom: -1px;
        right: -1px;
        width: 11px;
        height: 11px;
        background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
        border: 2.5px solid #fff;
        border-radius: 50%;
        transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
        box-shadow:
          0 2px 6px rgba(0, 0, 0, 0.15),
          0 0 0 2px rgba(103, 194, 58, 0.2);
        z-index: 1;
        animation: pulse 2s ease-in-out infinite;
      }
    }

    .user-info-section {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 2px;
      padding-right: 4px;

      .user-name-display {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
        line-height: 1.2;
        max-width: 120px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        transition: color 0.2s ease;
      }

      .user-role-display {
        font-size: 11px;
        color: #909399;
        font-weight: 500;
        line-height: 1.2;
        transition: color 0.2s ease;
      }
    }

    .dropdown-arrow {
      font-size: 12px;
      color: #c0c4cc;
      transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }
  }

  &.is-active {
    .user-trigger {
      background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
      border-color: rgba(64, 158, 255, 0.25);
      box-shadow:
        0 4px 16px rgba(64, 158, 255, 0.2),
        0 0 0 1px rgba(64, 158, 255, 0.1);

      .user-avatar-trigger {
        border-color: rgba(64, 158, 255, 0.2);
        transform: scale(1.05);
      }

      .dropdown-arrow {
        transform: rotate(180deg);
        color: #409eff;
      }

      .user-name-display {
        color: #409eff;
      }
    }
  }
}

// 用户下拉菜单样式
:deep(.user-dropdown-menu) {
  padding: 0;
  border-radius: 20px;
  box-shadow:
    0 25px 80px rgba(0, 0, 0, 0.15),
    0 10px 32px rgba(0, 0, 0, 0.1),
    0 0 0 1px rgba(14, 165, 233, 0.1);
  border: 2px solid rgba(14, 165, 233, 0.08);
  overflow: hidden;
  min-width: 340px;
  max-width: 400px;
  backdrop-filter: blur(20px);
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.98) 0%, rgba(248, 250, 252, 0.95) 100%);

  .user-profile-header {
    position: relative;
    padding: 0;
    overflow: hidden;

    .profile-background {
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
      z-index: 0;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, rgba(14, 165, 233, 0.9) 0%, rgba(2, 132, 199, 0.8) 100%);
        opacity: 1;
        transition: opacity 0.3s ease;
      }

      .background-pattern {
        position: absolute;
        inset: 0;
        background-image:
          radial-gradient(circle at 20% 50%, rgba(255, 255, 255, 0.12) 0%, transparent 50%),
          radial-gradient(circle at 80% 80%, rgba(255, 255, 255, 0.1) 0%, transparent 40%),
          radial-gradient(circle at 60% 20%, rgba(255, 255, 255, 0.08) 0%, transparent 30%);
        animation: backgroundShift 12s ease-in-out infinite alternate;
      }
    }

    .profile-content {
      position: relative;
      z-index: 1;
      padding: 30px 28px 28px;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 16px;
      text-align: center;

      .profile-avatar-wrapper {
        position: relative;

        .avatar-ring {
          position: absolute;
          inset: -8px;
          border-radius: 50%;
          padding: 4px;
          background: conic-gradient(from 0deg, rgba(255, 255, 255, 0.6), rgba(255, 255, 255, 0.2), rgba(255, 255, 255, 0.6));
          -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
          -webkit-mask-composite: xor;
          mask-composite: exclude;
          animation: ringRotate 10s linear infinite;
          filter: blur(1px);
        }

        .el-avatar {
          background: rgba(255, 255, 255, 0.15);
          color: #fff;
          border: 4px solid rgba(255, 255, 255, 0.4);
          box-shadow:
            0 12px 32px rgba(0, 0, 0, 0.3),
            0 0 0 1px rgba(255, 255, 255, 0.2) inset,
            0 0 20px rgba(14, 165, 233, 0.3);
          position: relative;
          z-index: 1;
          transition: transform 0.3s ease;

          &:hover {
            transform: scale(1.05);
          }
        }

        .avatar-online-badge {
          position: absolute;
          bottom: 4px;
          right: 4px;
          width: 20px;
          height: 20px;
          background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
          border: 3px solid #fff;
          border-radius: 50%;
          box-shadow:
            0 4px 12px rgba(16, 185, 129, 0.6),
            0 0 0 2px rgba(16, 185, 129, 0.3);
          z-index: 2;
          animation: pulse 2.5s ease-in-out infinite;
        }
      }

      .profile-info {
        width: 100%;

        .profile-name {
          font-size: 19px;
          font-weight: 800;
          color: #fff;
          margin-bottom: 4px;
          letter-spacing: 0.4px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          text-shadow: 0 3px 8px rgba(0, 0, 0, 0.2);
        }

        .profile-username {
          font-size: 13px;
          color: rgba(255, 255, 255, 0.8);
          font-weight: 600;
          margin-bottom: 12px;
          letter-spacing: 0.4px;
        }

        .profile-meta {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 10px;
          flex-wrap: wrap;

          .profile-status {
            display: flex;
            align-items: center;
            gap: 4px;
            font-size: 12px;
            color: rgba(255, 255, 255, 0.95);
            font-weight: 600;
            background: rgba(255, 255, 255, 0.2);
            padding: 4px 12px;
            border-radius: 14px;
            backdrop-filter: blur(15px);
            border: 1px solid rgba(255, 255, 255, 0.3);
            transition: all 0.3s ease;

            &:hover {
              background: rgba(255, 255, 255, 0.25);
              transform: translateY(-1px);
            }

            .status-dot {
              font-size: 10px;
              color: #10b981;
            }

            .status-text {
              font-size: 12px;
            }
          }
        }
      }
    }
  }

  .user-stats-section {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 18px 24px;
    background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
    border-bottom: 1px solid rgba(226, 232, 240, 0.8);

    .stat-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 4px;
      flex: 1;
      min-width: 0;

      .stat-value {
        font-size: 15px;
        font-weight: 800;
        background: linear-gradient(135deg, #0f172a 0%, #475569 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        letter-spacing: 0.4px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        max-width: 100%;
        line-height: 1.2;
      }

      .stat-label {
        font-size: 11px;
        color: #64748b;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.8px;
      }
    }

    .el-divider--vertical {
      height: 32px;
      margin: 0;
      border-color: rgba(226, 232, 240, 0.8);
    }
  }

  .user-menu-items {
    padding: 12px 0 6px;

    .menu-section-title {
      padding: 8px 24px 8px;
      font-size: 11px;
      font-weight: 800;
      color: #64748b;
      text-transform: uppercase;
      letter-spacing: 1px;
      background: linear-gradient(135deg, rgba(14, 165, 233, 0.02) 0%, rgba(2, 132, 199, 0.01) 100%);
    }

    .el-dropdown-menu__item {
      padding: 0;
      border: none;
      margin: 0;

      &:hover {
        background: transparent;

        .menu-item-content {
          background: linear-gradient(90deg, #f0f9ff 0%, #e0f2fe 100%);
          transform: translateX(6px);
          box-shadow: 0 4px 16px rgba(14, 165, 233, 0.15);
          border-color: rgba(14, 165, 233, 0.2);
        }
      }

      &:active {
        .menu-item-content {
          transform: translateX(4px) scale(0.97);
        }
      }

      &.menu-item-profile:hover .menu-item-icon-wrapper {
        background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        box-shadow: 0 6px 16px rgba(14, 165, 233, 0.4);
        animation: glow 2s ease-in-out infinite;

        &::after {
          box-shadow: 0 0 16px rgba(14, 165, 233, 0.5);
        }

        .menu-item-icon {
          color: #fff;
          transform: scale(1.15);
        }
      }

      &.menu-item-settings:hover .menu-item-icon-wrapper {
        background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
        box-shadow: 0 6px 16px rgba(245, 158, 11, 0.4);
        animation: glow 2s ease-in-out infinite;

        &::after {
          box-shadow: 0 0 16px rgba(245, 158, 11, 0.5);
        }

        .menu-item-icon {
          color: #fff;
          transform: scale(1.15);
        }
      }
    }

    .menu-item-content {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 12px 16px;
      margin: 4px 12px;
      border-radius: 12px;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      position: relative;
      overflow: hidden;
      background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
      border: 1px solid rgba(226, 232, 240, 0.8);

      &::before {
        content: '';
        position: absolute;
        left: 0;
        top: 50%;
        transform: translateY(-50%);
        width: 4px;
        height: 0;
        background: linear-gradient(180deg, #0ea5e9 0%, #0284c7 100%);
        border-radius: 0 4px 4px 0;
        transition: height 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      }

      &::after {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(
          90deg,
          transparent 0%,
          rgba(14, 165, 233, 0.1) 50%,
          transparent 100%
        );
        background-size: 200% 100%;
        transform: translateX(-100%);
        transition: transform 0.8s ease;
        border-radius: 12px;
      }

      .menu-item-icon-wrapper {
        width: 36px;
        height: 36px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
        border-radius: 10px;
        transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
        flex-shrink: 0;
        box-shadow: 0 3px 8px rgba(0, 0, 0, 0.08);
        position: relative;
        border: 1px solid rgba(226, 232, 240, 0.6);

        &::after {
          content: '';
          position: absolute;
          inset: -3px;
          border-radius: 12px;
          background: transparent;
          transition: box-shadow 0.4s ease;
          z-index: -1;
        }

        .menu-item-icon {
          font-size: 17px;
          color: #475569;
          transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
        }
      }

      .menu-item-text {
        display: flex;
        flex-direction: column;
        gap: 2px;
        flex: 1;
        min-width: 0;

        .menu-item-title {
          font-size: 14px;
          font-weight: 700;
          color: #1e293b;
          transition: color 0.3s ease;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          letter-spacing: 0.2px;
        }

        .menu-item-desc {
          font-size: 11px;
          color: #64748b;
          font-weight: 500;
          transition: color 0.3s ease;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          letter-spacing: 0.1px;
        }
      }

      .menu-item-arrow {
        font-size: 14px;
        color: #cbd5e1;
        transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
        flex-shrink: 0;
      }
    }

    .el-dropdown-menu__item:hover {
      .menu-item-content::before {
        height: 24px;
      }

      .menu-item-content::after {
        transform: translateX(100%);
        animation: shimmer 0.8s ease forwards;
      }

      .menu-item-arrow {
        transform: translateX(6px);
        color: #0ea5e9;
      }

      .menu-item-title {
        color: #0ea5e9;
      }
    }

    .menu-item-settings:hover .menu-item-content::before {
      background: linear-gradient(180deg, #f59e0b 0%, #d97706 100%);
    }

    .menu-item-settings:hover .menu-item-arrow {
      color: #f59e0b;
    }

    .menu-item-settings:hover .menu-item-title {
      color: #f59e0b;
    }
  }

  .user-quick-actions {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 4px;
    padding: 12px 16px;
    background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
    border-top: 1px solid rgba(226, 232, 240, 0.8);
    border-bottom: 1px solid rgba(226, 232, 240, 0.8);

    .quick-action-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 6px;
      padding: 12px 8px;
      border-radius: 12px;
      cursor: pointer;
      transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
      position: relative;
      background: linear-gradient(135deg, #ffffff 0%, #f9fafb 100%);
      border: 1px solid rgba(226, 232, 240, 0.8);

      .el-icon {
        font-size: 18px;
        color: #475569;
        transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
      }

      span {
        font-size: 11px;
        color: #64748b;
        font-weight: 700;
        transition: all 0.3s ease;
        letter-spacing: 0.4px;
      }

      :deep(.el-badge) {
        position: absolute;
        top: 6px;
        right: 6px;
        transition: all 0.3s ease;
      }

      &:hover {
        background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
        transform: translateY(-2px);
        box-shadow: 0 6px 20px rgba(14, 165, 233, 0.15);
        border-color: rgba(14, 165, 233, 0.3);
        animation: float 2s ease-in-out infinite;

        .el-icon {
          color: #0ea5e9;
          transform: scale(1.15);
        }

        span {
          color: #0ea5e9;
          font-weight: 800;
        }

        :deep(.el-badge__content) {
          transform: scale(1.1);
          background: linear-gradient(135deg, #0ea5e9 0%, #0284c7 100%);
        }
      }

      &:active {
        transform: translateY(-1px) scale(0.96);
        box-shadow: 0 3px 12px rgba(14, 165, 233, 0.1);
      }
    }
  }

  .user-menu-footer {
    padding: 10px 0;

    .el-dropdown-menu__item {
      padding: 0;
      border: none;
      margin: 0;

      .logout-content {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 10px;
        padding: 14px 24px;
        margin: 6px 14px;
        border-radius: 12px;
        background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
        border: 1px solid rgba(239, 68, 68, 0.2);
        color: #dc2626;
        font-size: 15px;
        font-weight: 700;
        transition: all 0.3s ease;

        .el-icon {
          font-size: 20px;
          transition: transform 0.3s ease;
        }

        .logout-shortcut {
          margin-left: auto;
          font-size: 11px;
          color: #ef4444;
          background: rgba(239, 68, 68, 0.1);
          padding: 3px 8px;
          border-radius: 6px;
          font-weight: 600;
          border: 1px solid rgba(239, 68, 68, 0.2);
          transition: all 0.3s ease;
        }
      }

      &:hover .logout-content {
        background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
        border-color: rgba(239, 68, 68, 0.3);
        transform: translateY(-1px);
        box-shadow: 0 4px 16px rgba(239, 68, 68, 0.15);

        .el-icon {
          transform: scale(1.1) rotate(-5deg);
        }

        .logout-shortcut {
          background: rgba(239, 68, 68, 0.15);
          border-color: rgba(239, 68, 68, 0.3);
          transform: scale(1.05);
        }
      }

      &:active .logout-content {
        transform: translateY(0) scale(0.97);
        box-shadow: 0 2px 8px rgba(239, 68, 68, 0.1);
      }
    }
  }
}

// ========== 主内容区 ==========
.app-main {
  background: linear-gradient(180deg, #f5f7fa 0%, #f0f2f5 100%);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px;

  // 自定义滚动条
  &::-webkit-scrollbar {
    width: 6px;
    height: 6px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.1);
    border-radius: 3px;

    &:hover {
      background: rgba(0, 0, 0, 0.2);
    }
  }
}

// ========== 页面过渡动画 ==========
.page-enter-active,
.page-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-enter-from {
  opacity: 0;
  transform: translateY(10px);
}

.page-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

// ========== 下拉菜单过渡动画 ==========
.dropdown-fade-enter-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.dropdown-fade-leave-active {
  transition: all 0.2s cubic-bezier(0.4, 0, 1, 1);
}

.dropdown-fade-enter-from {
  opacity: 0;
  transform: translateY(-12px) scale(0.95);
  filter: blur(8px);
}

.dropdown-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
  filter: blur(4px);
}

// ========== 响应式 ==========
@media (max-width: 768px) {
  .app-aside {
    display: none;
  }

  .collapse-btn {
    display: none;
  }

  .mobile-menu-btn {
    display: flex !important;
  }

  .search-box {
    display: none;
  }

  .quick-actions {
    padding: 0;
    gap: 2px;
  }

  .header-divider {
    display: none;
  }

  .user-info-section {
    display: none;
  }

  .dropdown-arrow {
    display: none;
  }

  .user-trigger {
    padding: 6px !important;

    &:hover {
      transform: none !important;
    }
  }

  .breadcrumb {
    display: none;
  }

  .recent-dropdown {
    display: none;
  }

  .app-header {
    padding: 0 16px;
    height: 56px;
  }

  .app-main {
    padding: 16px;
  }
}

// ========== 移动端菜单 ==========
.mobile-menu-btn {
  display: none;
}

.desktop-only {
  @media (max-width: 768px) {
    display: none;
  }
}

:deep(.mobile-drawer) {
  .el-drawer__body {
    padding: 0;
  }
}

.mobile-menu {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: linear-gradient(180deg, #1a1d2d 0%, #141722 100%);
}

.mobile-menu-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(255, 255, 255, 0.02);

  .logo-section {
    .logo-wrapper {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 6px;
      border-radius: 10px;
      background: rgba(255, 255, 255, 0.05);
    }

    .logo-icon {
      width: 42px;
      height: 42px;
      border-radius: 12px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      box-shadow: 0 4px 16px rgba(102, 126, 234, 0.5);
    }

    .logo-title {
      font-size: 18px;
      font-weight: 700;
      color: #fff;
      line-height: 1.2;
    }

    .logo-subtitle {
      font-size: 11px;
      color: rgba(255, 255, 255, 0.55);
      margin-top: 2px;
      font-weight: 500;
    }
  }

  .close-btn {
    font-size: 20px;
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    padding: 10px;
    border-radius: 10px;
    transition: all 0.25s ease;

    &:hover {
      color: #fff;
      background: rgba(255, 255, 255, 0.1);
    }

    &:active {
      transform: scale(0.95);
    }
  }
}

.mobile-menu-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 14px;

  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-track {
    background: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;
  }

  .menu-group-title {
    padding: 14px 14px 10px;
    font-size: 11px;
    font-weight: 700;
    color: rgba(255, 255, 255, 0.4);
    text-transform: uppercase;
    letter-spacing: 0.8px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px 16px;
    border-radius: 12px;
    color: rgba(255, 255, 255, 0.75);
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    margin-bottom: 6px;
    font-weight: 500;

    .el-icon {
      font-size: 22px;
      transition: transform 0.25s ease;
    }

    span {
      font-size: 15px;
      font-weight: 500;
    }

    &:hover {
      background: rgba(255, 255, 255, 0.08);
      color: #fff;
      transform: translateX(4px);

      .el-icon {
        transform: scale(1.1);
      }
    }

    &:active {
      background: rgba(102, 126, 234, 0.25);
      color: #a5b4fc;
      transform: scale(0.98);
    }
  }
}

.mobile-menu-footer {
  padding: 16px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(0, 0, 0, 0.2);

  .user-info {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
    margin-bottom: 12px;
    cursor: pointer;
    transition: all 0.2s ease;

    &:hover {
      background: rgba(255, 255, 255, 0.1);
    }

    &:active {
      transform: scale(0.98);
    }

    .user-details {
      flex: 1;

      .user-name {
        font-size: 14px;
        font-weight: 500;
        color: #fff;
      }

      .user-role {
        font-size: 11px;
        color: rgba(255, 255, 255, 0.5);
        margin-top: 2px;
      }
    }
  }

  .el-button {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: rgba(255, 255, 255, 0.6);
    padding: 12px;
    border-radius: 10px;
    transition: all 0.2s ease;

    &:hover {
      background: rgba(255, 255, 255, 0.05);
      color: rgba(255, 255, 255, 0.8);
    }

    .el-icon {
      font-size: 16px;
    }
  }
}

// ========== 关键帧动画 ==========
@keyframes backgroundShift {
  0% {
    opacity: 1;
  }
  100% {
    opacity: 0.7;
  }
}

@keyframes ringRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.1);
    opacity: 0.9;
  }
}

@keyframes shimmer {
  0% {
    background-position: -200% center;
  }
  100% {
    background-position: 200% center;
  }
}

@keyframes ripple {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  100% {
    transform: scale(4);
    opacity: 0;
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-3px);
  }
}

@keyframes glow {
  0%, 100% {
    box-shadow: 0 0 5px rgba(64, 158, 255, 0.3);
  }
  50% {
    box-shadow: 0 0 20px rgba(64, 158, 255, 0.6), 0 0 30px rgba(64, 158, 255, 0.4);
  }
}

@keyframes pulse-check {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.1);
    opacity: 0.8;
  }
}

@keyframes iconGlow {
  0%, 100% {
    opacity: 0.5;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
    transform: scale(1.1);
  }
}

@keyframes sectionPulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(0.8);
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-15px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes clockPulse {
  0%, 100% {
    transform: scale(1);
  }
  25% {
    transform: scale(1.1) rotate(5deg);
  }
  50% {
    transform: scale(1);
  }
  75% {
    transform: scale(1.1) rotate(-5deg);
  }
}

@keyframes badgePulse {
  0%, 100% {
    transform: scale(1);
    box-shadow: 0 4px 12px rgba(244, 63, 94, 0.4);
  }
  50% {
    transform: scale(1.1);
    box-shadow: 0 6px 16px rgba(244, 63, 94, 0.6);
  }
}

@keyframes headerIconGlow {
  0%, 100% {
    opacity: 0.6;
    transform: scale(1);
  }
  25% {
    opacity: 0.8;
    transform: scale(1.05);
  }
  50% {
    opacity: 0.6;
    transform: scale(1);
  }
  75% {
    opacity: 0.8;
    transform: scale(1.05);
  }
}
</style>
