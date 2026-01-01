<template>
  <div class="base-table-wrapper">
    <el-table
      ref="tableRef"
      v-loading="loading"
      :data="data"
      :height="height"
      :max-height="maxHeight"
      :stripe="stripe"
      :border="border"
      :show-header="showHeader"
      :highlight-current-row="highlightCurrentRow"
      :row-class-name="rowClassName"
      :cell-class-name="cellClassName"
      :empty-text="emptyText"
      :default-sort="defaultSort"
      @sort-change="handleSortChange"
      @selection-change="handleSelectionChange"
      @row-click="handleRowClick"
      class="base-table"
    >
      <!-- 选择列 -->
      <el-table-column v-if="selectable" type="selection" width="55" :fixed="selectFixed" />

      <!-- 序号列 -->
      <el-table-column v-if="showIndex" type="index" width="60" :label="indexLabel" :fixed="indexFixed" />

      <!-- 动态列 -->
      <template v-for="column in columns" :key="column.prop">
        <el-table-column
          :prop="column.prop"
          :label="column.label"
          :width="column.width"
          :min-width="column.minWidth"
          :fixed="column.fixed"
          :sortable="column.sortable"
          :align="column.align || 'left'"
          :class-name="column.className"
        >
          <template #default="scope">
            <slot
              :name="column.prop"
              :row="scope.row"
              :column="column"
              :index="scope.$index"
            >
              <span>{{ scope.row[column.prop] }}</span>
            </slot>
          </template>

          <!-- 表头插槽 -->
          <template v-if="column.headerSlot" #header="scope">
            <slot :name="`${column.prop}-header`" v-bind="scope"></slot>
          </template>
        </el-table-column>
      </template>

      <!-- 操作列 -->
      <el-table-column
        v-if="$slots.actions"
        :label="actionsLabel"
        :width="actionsWidth"
        :fixed="actionsFixed"
        :align="actionsAlign"
      >
        <template #default="scope">
          <slot name="actions" :row="scope.row" :index="scope.$index"></slot>
        </template>
      </el-table-column>

      <!-- 空状态插槽 -->
      <template #empty>
        <slot name="empty">
          <div class="base-table__empty">
            <svg class="empty-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect x="8" y="8" width="48" height="48" rx="8" fill="#E8F5E9"/>
              <path d="M20 32H44M32 20V44" stroke="#4CAF50" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <p class="empty-text">{{ emptyText }}</p>
          </div>
        </slot>
      </template>
    </el-table>

    <!-- 分页 -->
    <div v-if="pagination && total > 0" class="base-table__pagination">
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :page-sizes="pageSizes"
        :total="total"
        :layout="paginationLayout"
        :background="true"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { ElTable } from 'element-plus';

export interface TableColumn {
  prop: string;
  label: string;
  width?: number | string;
  minWidth?: number | string;
  fixed?: boolean | 'left' | 'right';
  sortable?: boolean | 'custom';
  align?: 'left' | 'center' | 'right';
  className?: string;
  headerSlot?: boolean;
}

interface Props {
  data: any[];
  columns?: TableColumn[];
  height?: string | number;
  maxHeight?: string | number;
  stripe?: boolean;
  border?: boolean;
  showHeader?: boolean;
  highlightCurrentRow?: boolean;
  selectable?: boolean;
  selectFixed?: boolean | 'left' | 'right';
  showIndex?: boolean;
  indexLabel?: string;
  indexFixed?: boolean | 'left' | 'right';
  loading?: boolean;
  emptyText?: string;
  defaultSort?: { prop: string; order: string };
  rowClassName?: string | ((row: any, index: number) => string);
  cellClassName?: string | ((row: any, column: any, rowIndex: number, columnIndex: number) => string);
  pagination?: boolean;
  total?: number;
  pageSizes?: number[];
  paginationLayout?: string;
  actionsLabel?: string;
  actionsWidth?: number;
  actionsFixed?: boolean | 'left' | 'right';
  actionsAlign?: 'left' | 'center' | 'right';
}

const props = withDefaults(defineProps<Props>(), {
  columns: () => [],
  stripe: true,
  border: false,
  showHeader: true,
  highlightCurrentRow: false,
  selectable: false,
  selectFixed: false,
  showIndex: false,
  indexLabel: '序号',
  indexFixed: false,
  loading: false,
  emptyText: '暂无数据',
  pagination: false,
  total: 0,
  pageSizes: () => [10, 20, 50, 100],
  paginationLayout: 'total, sizes, prev, pager, next, jumper',
  actionsLabel: '操作',
  actionsWidth: 150,
  actionsFixed: false,
  actionsAlign: 'center',
});

// Define emits using runtime declaration
const emit = defineEmits([
  'sort-change',
  'selection-change',
  'row-click',
  'update:currentPage',
  'update:pageSize',
  'page-change',
  'size-change',
]);

const tableRef = ref<InstanceType<typeof ElTable>>();
const currentPage = ref(1);
const pageSize = ref(props.pageSizes[0]);

function handleSortChange(sort: { column: any; prop: string; order: string }) {
  emit('sort-change', sort);
}

function handleSelectionChange(selection: any[]) {
  emit('selection-change', selection);
}

function handleRowClick(row: any, column: any, event: Event) {
  emit('row-click', row, column, event);
}

function handleCurrentChange(page: number) {
  currentPage.value = page;
  emit('update:currentPage', page);
  emit('page-change', page);
}

function handleSizeChange(size: number) {
  pageSize.value = size;
  emit('update:pageSize', size);
  emit('size-change', size);
}

// 暴露方法
function clearSelection() {
  tableRef.value?.clearSelection();
}

function toggleRowSelection(row: any, selected?: boolean) {
  tableRef.value?.toggleRowSelection(row, selected);
}

function toggleAllSelection() {
  tableRef.value?.toggleAllSelection();
}

function setCurrentRow(row: any) {
  tableRef.value?.setCurrentRow(row);
}

function sort(prop: string, order: string) {
  tableRef.value?.sort(prop, order);
}

defineExpose({
  clearSelection,
  toggleRowSelection,
  toggleAllSelection,
  setCurrentRow,
  sort,
});
</script>

<style scoped lang="scss">
.base-table-wrapper {
  background: #fff;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));
}

.base-table {
  :deep(.el-table__header-wrapper) {
    th {
      background: #fafbfc;
      font-weight: 600;
      color: #303133;
      font-size: 13px;
      border-bottom: 1px solid #ebeef5;
    }
  }

  :deep(.el-table__body-wrapper) {
    td {
      border-bottom: 1px solid #f5f7fa;
    }

    .el-table__row {
      transition: all 0.2s ease;

      &:hover {
        background: #f5f7fa !important;
      }
    }
  }

  :deep(.el-table__empty-block) {
    background: transparent;
  }

  &__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 40px 20px;
    gap: 16px;

    .empty-icon {
      width: 100px;
      height: 100px;
      animation: float 3s ease-in-out infinite;
    }

    .empty-text {
      font-size: 14px;
      color: #909399;
      margin: 0;
    }
  }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-8px); }
}

.base-table__pagination {
  display: flex;
  justify-content: flex-end;
  padding: 16px 20px;
  border-top: 1px solid #ebeef5;
  background: #fafbfc;

  :deep(.el-pagination) {
    .btn-prev,
    .btn-next,
    .el-pager li {
      border-radius: 6px;
      transition: all 0.2s ease;

      &:hover {
        transform: translateY(-1px);
      }

      &.is-active {
        background: #409eff;
        color: #fff;
      }
    }

    .el-select__wrapper {
      border-radius: 6px;
    }
  }
}
</style>