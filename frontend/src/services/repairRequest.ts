import apiClient from './apiClient';
import { RepairRequestResponse, RepairRequestListResponse } from './types';

export const submitRepairRequest = async (customerId: string, description: string) => {
  const response = await apiClient.post<RepairRequestResponse>('/repair_request', {
    customer_id: customerId,
    description,
  });
  return response.data;
};

export const getRepairRequests = async () => {
  const response = await apiClient.get<RepairRequestListResponse[]>('/repair_request');
  return response.data;
};

export const getAvailableRequests = async () => {
  const response = await apiClient.get<RepairRequestListResponse[]>('/available_requests');
  return response.data;
};

