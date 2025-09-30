'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { apiClient } from '@/utils/api';

export type UserRole = 'admin' | 'labeler';

export interface User {
  id: string;
  username: string;
  role: UserRole;
}

export interface LoginResult {
  success: boolean;
  errorType?: 'credentials' | 'server' | 'network';
}

export interface AuthContextType {
  user: User | null;
  token: string | null;
  login: (username: string, password: string, role: UserRole) => Promise<LoginResult>;
  logout: () => void;
  isLoading: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Load authentication state from localStorage on mount
  useEffect(() => {
    const storedToken = localStorage.getItem('auth_token');
    const storedUser = localStorage.getItem('auth_user');
    
    if (storedToken && storedUser) {
      try {
        const userData = JSON.parse(storedUser);
        setToken(storedToken);
        setUser(userData);
      } catch (error) {
        console.error('Error parsing stored user data:', error);
        localStorage.removeItem('auth_token');
        localStorage.removeItem('auth_user');
      }
    }
    setIsLoading(false);
  }, []);

  const login = async (username: string, password: string, role: UserRole): Promise<LoginResult> => {
    try {
      setIsLoading(true);
      
      // Call the backend API using the API client
      const response = await apiClient.post('/login', {
        username,
        password,
        role,
      });

      if (!response.success) {
        // Determine error type based on status code
        let errorType: 'credentials' | 'server' | 'network' = 'network';
        
        if (response.status === 403) {
          errorType = 'credentials';
        } else if (response.status === 500) {
          errorType = 'server';
        }
        
        return {
          success: false,
          errorType,
        };
      }

      const data = response.data as { user_id: string; token: string };
      
      // Store token and user data
      const userData: User = {
        id: data.user_id,
        username,
        role,
      };
      
      setToken(data.token);
      setUser(userData);
      
      // Persist to localStorage
      localStorage.setItem('auth_token', data.token);
      localStorage.setItem('auth_user', JSON.stringify(userData));
      
      return { success: true };
    } catch (error) {
      console.error('Login error:', error);
      return {
        success: false,
        errorType: 'network',
      };
    } finally {
      setIsLoading(false);
    }
  };

  const logout = () => {
    setUser(null);
    setToken(null);
    localStorage.removeItem('auth_token');
    localStorage.removeItem('auth_user');
  };

  const value: AuthContextType = {
    user,
    token,
    login,
    logout,
    isLoading,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};
