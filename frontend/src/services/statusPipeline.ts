import apiClient from './apiClient';
import { StatusPipelineResponse } from './types';

export const addStatusPipelineEntry = async (orderId: string, status: string) => {
  const response = await apiClient.post<StatusPipelineResponse>('/status_pipeline', { order_id: orderId, status });
  return response.data;
};

