import React, { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import { setAuthToken } from '../services/apiClient';

interface AuthContextType {
  token: string | null;
  userId: string | null;
  setToken: (token: string | null, userId: string | null) => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [token, setTokenState] = useState<string | null>(() => localStorage.getItem('token'));
  const [userId, setUserId] = useState<string | null>(() => localStorage.getItem('userId'));

  const setToken = (token: string | null, userId: string | null) => {
    if (token && userId) {
      localStorage.setItem('token', token);
      localStorage.setItem('userId', userId);
    } else {
      localStorage.removeItem('token');
      localStorage.removeItem('userId');
    }
    setTokenState(token);
    setUserId(userId);
    setAuthToken(token);
  };

  useEffect(() => {
    const savedToken = localStorage.getItem('token');
    const savedUserId = localStorage.getItem('userId');
    if (savedToken && savedUserId) {
      setAuthToken(savedToken);
      setUserId(savedUserId);
    }
  }, []);

  return (
    <AuthContext.Provider value={{ token, userId, setToken }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

