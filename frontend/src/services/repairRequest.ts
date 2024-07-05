import apiClient from './apiClient';
import { RepairRequestResponse } from './types';

export const submitRepairRequest = async (
  customerId: string,
  description: string
) => {
  const response = await apiClient.post<RepairRequestResponse>('/repair_request', {
    customer_id: customerId,
    description,
  });
  return response.data;
};

