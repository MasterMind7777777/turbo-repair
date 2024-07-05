import apiClient from './apiClient';
import { AddressResponse } from './types';

// Add a new address to a repair shop
export const addAddress = async (
  repairShopId: string,
  street: string,
  city: string,
  state: string,
  zip: string,
  country: string
): Promise<AddressResponse> => {
  const response = await apiClient.post<AddressResponse>('/address', {
    repair_shop_id: repairShopId,
    street,
    city,
    state,
    zip,
    country,
  });
  return response.data;
};

// Update an existing address
export const updateAddress = async (
  addressId: string,
  street: string,
  city: string,
  state: string,
  zip: string,
  country: string
): Promise<AddressResponse> => {
  const response = await apiClient.patch<AddressResponse>(`/address/${addressId}`, {
    street,
    city,
    state,
    zip,
    country,
  });
  return response.data;
};

// Fetch an address by its ID
export const getAddress = async (addressId: string): Promise<AddressResponse> => {
  const response = await apiClient.get<AddressResponse>(`/address/${addressId}`);
  return response.data;
};

export const getAddressByShopId = async (shopId: string) => {
  const response = await apiClient.get<AddressResponse>(`/address/by_shop/${shopId}`);
  return response.data;
};
