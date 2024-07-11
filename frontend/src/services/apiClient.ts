import axios from 'axios';

const API_URL = 'http://127.0.0.1:8080/api';

const apiClient = axios.create({
  baseURL: API_URL,
});

let authToken: string | null = null;

export const setAuthToken = (token: string | null) => {
  console.log('Setting auth token:', token);
  authToken = token;
};

apiClient.interceptors.request.use(
  (config) => {
    if (authToken) {
      config.headers['Authorization'] = `Bearer ${authToken}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

export default apiClient;

