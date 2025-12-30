// 全局通用组件库导出

import BaseCard from './BaseCard.vue';
import BaseTable from './BaseTable.vue';
import LoadingSkeleton from './LoadingSkeleton.vue';
import EmptyState from './EmptyState.vue';
import ErrorState from './ErrorState.vue';

export { BaseCard, BaseTable, LoadingSkeleton, EmptyState, ErrorState };

export { default as BaseCard } from './BaseCard.vue';
export { default as BaseTable } from './BaseTable.vue';
export { default as LoadingSkeleton } from './LoadingSkeleton.vue';
export { default as EmptyState } from './EmptyState.vue';
export { default as ErrorState } from './ErrorState.vue';

// 导出 BaseTable 的类型
export type { TableColumn } from './BaseTable.vue';
