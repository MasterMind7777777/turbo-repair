import apiClient from './apiClient';
import { UserDetailsResponce } from './types';


export const getCurrentUser = async (): Promise<UserDetailsResponce> => {
  const response = await apiClient.get<UserDetailsResponce>('/user/self');
  return response.data;
};

export const updateUserProfile = async (userId: string, userDetails: Partial<UserDetailsResponce>) => {
  const response = await apiClient.put(`/user/${userId}`, userDetails);
  return response.data;
};

