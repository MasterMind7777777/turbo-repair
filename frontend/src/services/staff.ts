import apiClient from './apiClient';
import { StaffResponse } from './types';

export const addStaff = async (userId: string, repairShopId: string, role: string) => {
  const response = await apiClient.post<StaffResponse>('/staff/add', {
    user_id: userId,
    repair_shop_id: repairShopId,
    role,
  });
  return response.data;
};

