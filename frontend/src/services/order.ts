import apiClient from './apiClient';
import { OrderResponse } from './types';

export const createOrder = async (
  repairRequestId: string,
  repairShopId: string,
  status: string
) => {
  const response = await apiClient.post<OrderResponse>('/order', {
    repair_request_id: repairRequestId,
    repair_shop_id: repairShopId,
    status,
  });
  return response.data;
};

export const updateOrderStatus = async (orderId: string, status: string) => {
  const response = await apiClient.patch<OrderResponse>(`/order/${orderId}`, { status });
  return response.data;
};

export const getOrders = async () => {
  const response = await apiClient.get<OrderResponse[]>('/order');
  return response.data;
};
