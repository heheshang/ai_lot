import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Strategy, StrategyInstance } from '@/types';

export const useStrategyStore = defineStore('strategy', () => {
  const strategies = ref<Strategy[]>([]);
  const currentStrategy = ref<Strategy | null>(null);
  const runningInstances = ref<StrategyInstance[]>([]);

  return {
    strategies,
    currentStrategy,
    runningInstances,
  };
});
