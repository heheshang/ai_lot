import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Order, Position } from '@/types';

export const useTradeStore = defineStore('trade', () => {
  const positions = ref<Position[]>([]);
  const activeOrders = ref<Order[]>([]);
  const orderHistory = ref<Order[]>([]);

  return {
    positions,
    activeOrders,
    orderHistory,
  };
});
