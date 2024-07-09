import apiClient from './apiClient';
import { RepairShopResponse } from './types';

export const createRepairShop = async (name: string): Promise<RepairShopResponse> => {
  const response = await apiClient.post<RepairShopResponse>('/repair_shop', { name });
  return response.data;
};


export const getRepairShopById = async (id: string): Promise<RepairShopResponse> => {
  const response = await apiClient.get<RepairShopResponse>(`/repair_shop/${id}`);
  return response.data;
};

export const getRepairShops = async (): Promise<RepairShopResponse[]> => {
  const response = await apiClient.get<RepairShopResponse[]>('/repair_shop');
  return response.data;
};

export const updateRepairShop = async (id: string, name: string): Promise<RepairShopResponse> => {
  const response = await apiClient.put<RepairShopResponse>(`/repair_shop/${id}`, { name });
  return response.data;
};

export const partiallyUpdateRepairShop = async (id: string, name: Partial<string>): Promise<RepairShopResponse> => {
  const response = await apiClient.patch<RepairShopResponse>(`/repair_shop/${id}`, { name });
  return response.data;
};

export const deleteRepairShop = async (id: string): Promise<void> => {
  await apiClient.delete(`/repair_shop/${id}`);
};
