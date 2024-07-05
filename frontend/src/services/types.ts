export interface RegisterResponse {
  id: string;
}

export interface RepairShopResponse {
  id: string;
  name: string;
  created_at: string;
}

export interface AddressResponse {
  id: string;
  repair_shop_id: string;
  street: string;
  city: string;
  state: string;
  zip: string;
  country: string;
  created_at: string;
}

export interface RepairRequestResponse {
  id: string;
}

export interface BidResponse {
  id: string;
  repair_request_id: string;
  repair_shop_id: string;
  bid_amount: number;
  status: string;
  created_at: string;
}

export interface OrderResponse {
  id: string;
  repair_request_id: string;
  repair_shop_id: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface StatusPipelineResponse {
  id: string;
  order_id: string;
  status: string;
  timestamp: string;
}

