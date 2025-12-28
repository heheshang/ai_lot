import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Order, Position, OrderSide, OrderType, OrderStatus } from '@/types';
import * as api from '@/api/tauri';

/**
 * Place Order Request Interface
 */
export interface PlaceOrderRequest {
  symbol: string;
  side: OrderSide;
  orderType: OrderType;
  quantity: number;
  price?: number;
  stopPrice?: number;
  clientOrderId?: string;
  timeInForce?: 'GTC' | 'IOC' | 'FOK';
}

/**
 * Trade Store - Manages trading state and operations
 */
export const useTradeStore = defineStore('trade', () => {
  // ========== State ==========
  const orders = ref<Order[]>([]);
  const positions = ref<Position[]>([]);
  const trades = ref<any[]>([]); // Trade history
  const loading = ref(false);
  const error = ref<string | null>(null);

  // ========== Computed Getters ==========

  /**
   * Get active orders (open or partially filled)
   */
  const activeOrders = computed(() =>
    orders.value.filter(
      o => o.status === 'open' || o.status === 'partially_filled'
    )
  );

  /**
   * Get completed orders (filled, canceled, or rejected)
   */
  const completedOrders = computed(() =>
    orders.value.filter(
      o => o.status === 'filled' || o.status === 'canceled' || o.status === 'rejected'
    )
  );

  /**
   * Get orders by symbol
   */
  const getOrdersBySymbol = computed(() => {
    return (symbol: string) => orders.value.filter(o => o.symbol === symbol);
  });

  /**
   * Get positions by symbol
   */
  const getPositionsBySymbol = computed(() => {
    return (symbol: string) => positions.value.filter(p => p.symbol === symbol);
  });

  /**
   * Calculate total unrealized PnL
   */
  const totalUnrealizedPnl = computed(() =>
    positions.value.reduce((sum, p) => sum + p.unrealizedPnl, 0)
  );

  /**
   * Calculate total realized PnL
   */
  const totalRealizedPnl = computed(() =>
    positions.value.reduce((sum, p) => sum + p.realizedPnl, 0)
  );

  /**
   * Get long positions
   */
  const longPositions = computed(() =>
    positions.value.filter(p => p.side === 'long')
  );

  /**
   * Get short positions
   */
  const shortPositions = computed(() =>
    positions.value.filter(p => p.side === 'short')
  );

  // ========== Actions ==========

  /**
   * Fetch orders from backend
   */
  async function fetchOrders(userId: string, symbol?: string, status?: OrderStatus) {
    try {
      loading.value = true;
      error.value = null;

      const params: any = { userId };
      if (symbol) params.symbol = symbol;
      if (status) params.status = status;

      const data = await api.invokeRaw<Order[]>('trade_get_orders', params);
      orders.value = data;

      return data;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch orders';
      console.error('Failed to fetch orders:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Fetch open orders
   */
  async function fetchOpenOrders(userId: string) {
    try {
      loading.value = true;
      error.value = null;

      const data = await api.invokeRaw<Order[]>('trade_get_open_orders', { userId });
      orders.value = data;

      return data;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch open orders';
      console.error('Failed to fetch open orders:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Fetch positions from backend
   */
  async function fetchPositions(userId: string) {
    try {
      loading.value = true;
      error.value = null;

      const data = await api.invokeRaw<Position[]>('trade_get_positions', { userId });
      positions.value = data;

      return data;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch positions';
      console.error('Failed to fetch positions:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Place a new order
   */
  async function placeOrder(userId: string, request: PlaceOrderRequest) {
    try {
      loading.value = true;
      error.value = null;

      // Convert to backend format
      const orderRequest = {
        symbol: request.symbol,
        side: request.side,
        orderType: request.orderType,
        quantity: request.quantity,
        price: request.price,
        stopPrice: request.stopPrice,
        clientOrderId: request.clientOrderId,
        timeInForce: request.timeInForce,
      };

      const order = await api.invokeRaw<Order>('trade_place_order', {
        userId,
        request: orderRequest,
      });

      // Add to orders list
      orders.value.push(order);

      return order;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to place order';
      console.error('Failed to place order:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Cancel an order
   */
  async function cancelOrder(userId: string, orderId: string) {
    try {
      loading.value = true;
      error.value = null;

      await api.invokeRaw<void>('trade_cancel_order', { userId, orderId });

      // Update local order status
      const orderIndex = orders.value.findIndex(o => o.id === orderId);
      if (orderIndex >= 0) {
        orders.value[orderIndex].status = 'canceled' as OrderStatus;
      }

      return true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to cancel order';
      console.error('Failed to cancel order:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Cancel all orders (optionally by symbol)
   */
  async function cancelAllOrders(userId: string, symbol?: string) {
    try {
      loading.value = true;
      error.value = null;

      const params: any = { userId };
      if (symbol) params.symbol = symbol;

      const count = await api.invokeRaw<number>('trade_cancel_all_orders', params);

      // Update local orders status
      if (symbol) {
        orders.value
          .filter(o => o.symbol === symbol && (o.status === 'open' || o.status === 'partially_filled'))
          .forEach(o => o.status = 'canceled' as OrderStatus);
      } else {
        orders.value
          .filter(o => o.status === 'open' || o.status === 'partially_filled')
          .forEach(o => o.status = 'canceled' as OrderStatus);
      }

      return count;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to cancel all orders';
      console.error('Failed to cancel all orders:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Sync order status from exchange
   */
  async function syncOrderStatus(userId: string, orderId: string) {
    try {
      loading.value = true;
      error.value = null;

      const updatedOrder = await api.invokeRaw<Order>('trade_sync_order_status', {
        userId,
        orderId,
      });

      // Update local order
      const index = orders.value.findIndex(o => o.id === orderId);
      if (index >= 0) {
        orders.value[index] = updatedOrder;
      }

      return updatedOrder;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to sync order status';
      console.error('Failed to sync order status:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Close a position
   */
  async function closePosition(userId: string, symbol: string, side: 'long' | 'short', quantity?: number) {
    try {
      loading.value = true;
      error.value = null;

      const pnl = await api.invokeRaw<number>('trade_close_position', {
        userId,
        symbol,
        side,
        quantity,
      });

      // Refresh positions after closing
      await fetchPositions(userId);

      return pnl;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to close position';
      console.error('Failed to close position:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Get account balance
   */
  async function getBalance() {
    try {
      loading.value = true;
      error.value = null;

      const balance = await api.invokeRaw<any[]>('trade_get_balance');
      return balance;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to get balance';
      console.error('Failed to get balance:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  /**
   * Update order locally (from WebSocket event)
   */
  function updateOrder(order: Order) {
    const index = orders.value.findIndex(o => o.id === order.id);
    if (index >= 0) {
      orders.value[index] = order;
    } else {
      orders.value.push(order);
    }
  }

  /**
   * Update position locally (from WebSocket event)
   */
  function updatePosition(position: Position) {
    const index = positions.value.findIndex(
      p => p.symbol === position.symbol && p.side === position.side
    );
    if (index >= 0) {
      positions.value[index] = position;
    } else {
      positions.value.push(position);
    }
  }

  /**
   * Add trade to history
   */
  function addTrade(trade: any) {
    trades.value.unshift(trade);
    // Keep max 1000 trades
    if (trades.value.length > 1000) {
      trades.value = trades.value.slice(0, 1000);
    }
  }

  /**
   * Clear all data
   */
  function clear() {
    orders.value = [];
    positions.value = [];
    trades.value = [];
    error.value = null;
  }

  /**
   * Initialize trade data
   */
  async function initialize(userId: string) {
    try {
      loading.value = true;
      await Promise.all([
        fetchOpenOrders(userId),
        fetchPositions(userId),
      ]);
    } catch (err) {
      console.error('Failed to initialize trade store:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  return {
    // State
    orders,
    positions,
    trades,
    loading,
    error,

    // Computed
    activeOrders,
    completedOrders,
    getOrdersBySymbol,
    getPositionsBySymbol,
    totalUnrealizedPnl,
    totalRealizedPnl,
    longPositions,
    shortPositions,

    // Actions
    fetchOrders,
    fetchOpenOrders,
    fetchPositions,
    placeOrder,
    cancelOrder,
    cancelAllOrders,
    syncOrderStatus,
    closePosition,
    getBalance,
    updateOrder,
    updatePosition,
    addTrade,
    clear,
    initialize,
  };
});
