import { useAuth } from '@/contexts/AuthContext';

const API_BASE_URL = 'http://localhost:8080/api/v1';

export interface ApiResponse<T = unknown> {
  data?: T;
  error?: string;
  success: boolean;
  status?: number;
  statusText?: string;
}

export class ApiClient {
  private baseUrl: string;
  private getToken: () => string | null;

  constructor(baseUrl: string, getToken: () => string | null) {
    this.baseUrl = baseUrl;
    this.getToken = getToken;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<ApiResponse<T>> {
    try {
      const token = this.getToken();
      const url = `${this.baseUrl}${endpoint}`;

      const response = await fetch(url, {
        ...options,
        headers: {
          'Content-Type': 'application/json',
          ...(token && { Authorization: `Bearer ${token}` }),
          ...options.headers,
        },
      });

      if (!response.ok) {
        const errorText = await response.text();
        return {
          success: false,
          error: errorText,
          status: response.status,
          statusText: response.statusText,
        };
      }

      const data = await response.json();
      return {
        success: true,
        data,
        status: response.status,
        statusText: response.statusText,
      };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error occurred',
      };
    }
  }

  async get<T>(endpoint: string): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'GET' });
  }

  async post<T>(endpoint: string, data?: unknown): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async put<T>(endpoint: string, data?: unknown): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, {
      method: 'PUT',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async delete<T>(endpoint: string): Promise<ApiResponse<T>> {
    return this.request<T>(endpoint, { method: 'DELETE' });
  }
}

// Hook to get API client with current auth token
export const useApiClient = () => {
  const { token } = useAuth();
  return new ApiClient(API_BASE_URL, () => token);
};

// Export a default instance for use outside of React components
export const apiClient = new ApiClient(API_BASE_URL, () => {
  if (typeof window !== 'undefined') {
    return localStorage.getItem('auth_token');
  }
  return null;
});
