import apiClient from './apiClient';
import { BidResponse } from './types';

export const submitBid = async (
  repairRequestId: string,
  repairShopId: string,
  status: string,
  bidAmount: number
) => {
  const response = await apiClient.post<BidResponse>('/bid', {
    repair_request_id: repairRequestId,
    repair_shop_id: repairShopId,
    status,
    bid_amount: bidAmount,
  });
  return response.data;
};

