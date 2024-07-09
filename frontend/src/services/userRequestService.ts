import apiClient from './apiClient';
import { UserRequestResponse, UserRequestListResponse } from './types';

export const createUserRequest = async (repairShopId: string, content: string): Promise<UserRequestResponse> => {
  const response = await apiClient.post<UserRequestResponse>('/user_request', {
    repair_shop_id: repairShopId,
    content,
  });
  return response.data;
};

export const getUserRequests = async () => {
  const response = await apiClient.get<UserRequestListResponse[]>('/user_request');
  return response.data;
};

export const getUserRequestById = async (id: string) => {
  const response = await apiClient.get<UserRequestResponse>(`/user_request/${id}`);
  return response.data;
};

export const updateUserRequest = async (id: string, content: string) => {
  const response = await apiClient.put<UserRequestResponse>(`/user_request/${id}`, {
    content,
  });
  return response.data;
};

export const patchUserRequest = async (id: string, content: string) => {
  const response = await apiClient.patch<UserRequestResponse>(`/user_request/${id}`, {
    content,
  });
  return response.data;
};

export const deleteUserRequest = async (id: string) => {
  const response = await apiClient.delete(`/user_request/${id}`);
  return response.data;
};

