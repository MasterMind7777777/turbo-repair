import apiClient from './apiClient';
import { RegisterResponse } from './types';

export const registerUser = async (email: string, password: string) => {
  const response = await apiClient.post<RegisterResponse>('/auth/register', { email, password });
  return response.data;
};

export const loginUser = async (email: string, password: string) => {
  const response = await apiClient.post<string>('/auth/login', { email, password });
  console.log('Login response:', response.data);
  return response.data;
};
