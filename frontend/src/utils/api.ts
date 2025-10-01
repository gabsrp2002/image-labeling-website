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

  async uploadImage(
    filename: string,
    filetype: string,
    base64Data: string,
    groupId: number
  ): Promise<ApiResponse<{
    id: number;
    filename: string;
    filetype: string;
    uploaded_at: string;
  }>> {
    return this.post('/admin/image', {
      filename,
      filetype,
      base64_data: base64Data,
      group_id: groupId,
    });
  }

  // Tag management methods
  async createTag(
    name: string,
    description: string | null,
    groupId: number
  ): Promise<ApiResponse<{
    id: number;
    name: string;
    description: string | null;
  }>> {
    return this.post('/admin/tag', {
      name,
      description,
      group_id: groupId,
    });
  }

  async getTag(tagId: number): Promise<ApiResponse<{
    id: number;
    name: string;
    description: string | null;
  }>> {
    return this.get(`/admin/tag/${tagId}`);
  }

  async updateTag(
    tagId: number,
    name?: string,
    description?: string | null
  ): Promise<ApiResponse<{
    id: number;
    name: string;
    description: string | null;
  }>> {
    return this.put(`/admin/tag/${tagId}`, {
      name,
      description,
    });
  }

  async deleteTag(tagId: number): Promise<ApiResponse<null>> {
    return this.delete(`/admin/tag/${tagId}`);
  }

  async getTagsByGroup(groupId: number): Promise<ApiResponse<Array<{
    id: number;
    name: string;
    description: string | null;
  }>>> {
    return this.get(`/admin/tag/group/${groupId}`);
  }

  // Labeler methods
  async getLabelerGroups(): Promise<ApiResponse<Array<{
    id: number;
    name: string;
    description: string | null;
  }>>> {
    return this.get('/labeler/groups');
  }

  async getGroupImages(groupId: number): Promise<ApiResponse<Array<{
    id: number;
    filename: string;
    status: string;
  }>>> {
    return this.get(`/labeler/groups/${groupId}/images`);
  }

  async getImageDetails(groupId: number, imageId: number): Promise<ApiResponse<{
    image: {
      id: number;
      filename: string;
      status: string;
      base64_data: string;
      filetype: string;
    };
    group_tags: Array<{
      id: number;
      name: string;
      description: string | null;
    }>;
    current_tags: Array<{
      id: number;
      name: string;
      description: string | null;
    }>;
  }>> {
    return this.get(`/labeler/groups/${groupId}/images/${imageId}`);
  }

  async updateImageTags(groupId: number, imageId: number, tagIds: number[]): Promise<ApiResponse<null>> {
    return this.put(`/labeler/groups/${groupId}/images/${imageId}/tags`, {
      tag_ids: tagIds
    });
  }

  async suggestTags(imageId: number, ignoredTagIds: number[]): Promise<ApiResponse<{
    success: boolean;
    message: string;
    data: {
      suggested_tags: string[];
    };
  }>> {
    return this.post(`/labeler/images/${imageId}/suggest_tags`, {
      ignored_tag_ids: ignoredTagIds
    });
  }

  // Group labeler management
  async addLabelerToGroup(groupId: number, labelerId: number): Promise<ApiResponse<{
    success: boolean;
    message: string;
    data: null;
  }>> {
    return this.post(`/admin/groups/${groupId}/labelers`, {
      labeler_id: labelerId
    });
  }

  async removeLabelerFromGroup(groupId: number, labelerId: number): Promise<ApiResponse<{
    success: boolean;
    message: string;
    data: null;
  }>> {
    return this.delete(`/admin/groups/${groupId}/labelers/${labelerId}`);
  }

  // Helper method to convert file to base64
  static async fileToBase64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        const result = reader.result as string;
        // Remove the data:image/...;base64, prefix
        const base64 = result.split(',')[1];
        resolve(base64);
      };
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });
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
