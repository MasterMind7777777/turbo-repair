import apiClient from './apiClient';
import { StaffTaskResponse, StaffTaskListResponse } from './types';

export const createStaffTask = async (repairShopId: string, content: string, staffIds: string[]) => {
  const response = await apiClient.post<StaffTaskResponse>('/staff_task', {
    repair_shop_id: repairShopId,
    content,
    staff_ids: staffIds,
  });
  return response.data;
};

export const getStaffTasks = async () => {
  const response = await apiClient.get<StaffTaskListResponse[]>('/staff_task');
  return response.data;
};

export const getStaffTaskById = async (id: string) => {
  const response = await apiClient.get<StaffTaskResponse>(`/staff_task/${id}`);
  return response.data;
};

export const updateStaffTask = async (id: string, content: string) => {
  const response = await apiClient.put<StaffTaskResponse>(`/staff_task/${id}`, {
    content,
  });
  return response.data;
};

export const patchStaffTask = async (id: string, content: string) => {
  const response = await apiClient.patch<StaffTaskResponse>(`/staff_task/${id}`, {
    content,
  });
  return response.data;
};

export const deleteStaffTask = async (id: string) => {
  const response = await apiClient.delete(`/staff_task/${id}`);
  return response.data;
};

export const linkStaffToTask = async (taskId: string, staffIds: string[]) => {
  const response = await apiClient.post(`/staff_task/link`, {
    task_id: taskId,
    staff_ids: staffIds,
  });
  return response.data;
};

