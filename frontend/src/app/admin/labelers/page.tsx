'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useApiClient } from '@/utils/api';
import { LoadingSpinner, ErrorMessage, SuccessMessage, PageHeader, Button, Card, Table, FormInput } from '@/components';

interface Labeler {
  id: number;
  username: string;
  group_ids: number[];
}

interface Group {
  id: number;
  name: string;
  description?: string;
}

interface UpdateLabelerRequest {
  username?: string;
  password?: string;
  group_ids?: number[];
}

interface LabelerResponse {
  id: number;
  username: string;
  group_ids: number[];
}

interface LabelerListResponse {
  labelers: LabelerResponse[];
  total: number;
}

interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
}

export default function AdminLabelersPage() {
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);
  const [labelers, setLabelers] = useState<Labeler[]>([]);
  const [groups, setGroups] = useState<Group[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);
  const [showEditForm, setShowEditForm] = useState(false);
  const [editingLabeler, setEditingLabeler] = useState<Labeler | null>(null);
  const [newLabeler, setNewLabeler] = useState({ username: '', password: '', group_ids: [] as number[] });
  const [editLabeler, setEditLabeler] = useState({ username: '', password: '', group_ids: [] as number[] });
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadData = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Load labelers and groups in parallel
      const [labelersResponse, groupsResponse] = await Promise.all([
        apiClientRef.current.get<ApiResponse<LabelerListResponse>>('/admin/labeler'),
        apiClientRef.current.get<ApiResponse<{ groups: Group[]; total: number }>>('/admin/groups')
      ]);
      
      if (labelersResponse.success) {
        if (labelersResponse.data?.success && labelersResponse.data.data) {
          setLabelers(labelersResponse.data.data.labelers);
        } else {
          setError(labelersResponse.data?.message || 'Failed to load labelers');
        }
      } else {
        setError(labelersResponse.error || 'Failed to load labelers');
      }

      if (groupsResponse.success) {
        if (groupsResponse.data?.success && groupsResponse.data.data) {
          setGroups(groupsResponse.data.data.groups);
        } else {
          console.error('Failed to load groups:', groupsResponse.data?.message || groupsResponse.error);
        }
      } else {
        console.error('Failed to load groups:', groupsResponse.error);
      }
    } catch (error) {
      console.error('Error loading data:', error);
      setError('Failed to load data');
    } finally {
      setIsLoading(false);
    }
  }, []); // Empty dependency array since we use ref

  // Load labelers on component mount
  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleAddLabeler = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setError(null);
      setSuccess(null);
      
      const response = await apiClient.post<ApiResponse<LabelerResponse>>('/admin/labeler', {
        username: newLabeler.username,
        password: newLabeler.password,
        group_ids: newLabeler.group_ids.length > 0 ? newLabeler.group_ids : undefined,
      });
      
      if (response.success) {
        // Check if the response data indicates success
        if (response.data?.success) {
          setSuccess('Labeler created successfully');
          setShowAddForm(false);
          setNewLabeler({ username: '', password: '', group_ids: [] });
          loadData();
        } else {
          // Handle case where request succeeded but business logic failed
          setError(response.data?.message || 'Failed to create labeler');
        }
      } else {
        // Handle case where request failed - try to parse error as JSON
        try {
          const errorData = JSON.parse(response.error || '{}');
          setError(errorData.message || response.error || 'Failed to create labeler');
        } catch {
          setError(response.error || 'Failed to create labeler');
        }
      }
    } catch (error) {
      console.error('Error adding labeler:', error);
      setError('Failed to create labeler');
    }
  };

  const handleDeleteLabeler = async (id: number) => {
    if (confirm('Are you sure you want to delete this labeler?')) {
      try {
        setError(null);
        setSuccess(null);
        
        const response = await apiClient.delete<ApiResponse<null>>(`/admin/labeler/${id}`);
        
        if (response.success) {
          if (response.data?.success) {
            setSuccess('Labeler deleted successfully');
            loadData();
          } else {
            setError(response.data?.message || 'Failed to delete labeler');
          }
        } else {
          // Handle case where request failed - try to parse error as JSON
          try {
            const errorData = JSON.parse(response.error || '{}');
            setError(errorData.message || response.error || 'Failed to delete labeler');
          } catch {
            setError(response.error || 'Failed to delete labeler');
          }
        }
      } catch (error) {
        console.error('Error deleting labeler:', error);
        setError('Failed to delete labeler');
      }
    }
  };

  const handleEditLabeler = (labeler: Labeler) => {
    setEditingLabeler(labeler);
    setEditLabeler({ username: labeler.username, password: '', group_ids: labeler.group_ids });
    setShowEditForm(true);
  };

  const handleUpdateLabeler = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingLabeler) return;
    
    try {
      setError(null);
      setSuccess(null);
      
      const updateData: UpdateLabelerRequest = {
        username: editLabeler.username !== editingLabeler.username ? editLabeler.username : undefined,
        password: editLabeler.password ? editLabeler.password : undefined,
        group_ids: editLabeler.group_ids.length > 0 ? editLabeler.group_ids : undefined,
      };
      
      const response = await apiClient.put<ApiResponse<LabelerResponse>>(`/admin/labeler/${editingLabeler.id}`, updateData);
      
      if (response.success) {
        if (response.data?.success) {
          setSuccess('Labeler updated successfully');
          setShowEditForm(false);
          setEditingLabeler(null);
          setEditLabeler({ username: '', password: '', group_ids: [] });
          loadData();
        } else {
          setError(response.data?.message || 'Failed to update labeler');
        }
      } else {
        // Handle case where request failed - try to parse error as JSON
        try {
          const errorData = JSON.parse(response.error || '{}');
          setError(errorData.message || response.error || 'Failed to update labeler');
        } catch {
          setError(response.error || 'Failed to update labeler');
        }
      }
    } catch (error) {
      console.error('Error updating labeler:', error);
      setError('Failed to update labeler');
    }
  };


  if (isLoading) {
    return <LoadingSpinner />;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <PageHeader 
          title="Manage Labelers" 
          description="Create and manage labeler accounts and their group assignments." 
        />

        <Card>
          <div className="px-4 py-5 sm:p-6">
            {/* Error and Success Messages */}
            {error && (
              <ErrorMessage message={error} onClose={() => setError(null)} className="mb-4" />
            )}
            {success && (
              <SuccessMessage message={success} onClose={() => setSuccess(null)} className="mb-4" />
            )}

            <div className="flex justify-between items-center mb-6">
              <h2 className="text-lg font-medium text-gray-900">Labelers</h2>
              <Button onClick={() => setShowAddForm(true)}>
                <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                Add Labeler
              </Button>
            </div>

            {showAddForm && (
              <div className="mb-6 p-4 bg-gray-50 rounded-lg">
                <h3 className="text-lg font-medium text-gray-900 mb-4">Add New Labeler</h3>
                <form onSubmit={handleAddLabeler} className="space-y-4">
                  <FormInput
                    label="Username"
                    value={newLabeler.username}
                    onChange={(value) => setNewLabeler({ ...newLabeler, username: value })}
                    required
                  />
                  <FormInput
                    label="Password"
                    type="password"
                    value={newLabeler.password}
                    onChange={(value) => setNewLabeler({ ...newLabeler, password: value })}
                    required
                  />
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Groups</label>
                    <div className="mt-2 space-y-2">
                      {groups.map((group) => (
                        <label key={group.id} className="flex items-center">
                          <input
                            type="checkbox"
                            checked={newLabeler.group_ids.includes(group.id)}
                            onChange={(e) => {
                              if (e.target.checked) {
                                setNewLabeler({ ...newLabeler, group_ids: [...newLabeler.group_ids, group.id] });
                              } else {
                                setNewLabeler({ ...newLabeler, group_ids: newLabeler.group_ids.filter(id => id !== group.id) });
                              }
                            }}
                            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                          />
                          <span className="ml-2 text-sm text-gray-700">{group.name}</span>
                        </label>
                      ))}
                    </div>
                  </div>
                  <div className="flex space-x-3">
                    <Button type="submit">
                      Add Labeler
                    </Button>
                    <Button
                      type="button"
                      onClick={() => setShowAddForm(false)}
                      variant="secondary"
                    >
                      Cancel
                    </Button>
                  </div>
                </form>
              </div>
            )}

            {showEditForm && editingLabeler && (
              <div className="mb-6 p-4 bg-gray-50 rounded-lg">
                <h3 className="text-lg font-medium text-gray-900 mb-4">Edit Labeler</h3>
                <form onSubmit={handleUpdateLabeler} className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Username</label>
                    <input
                      type="text"
                      value={editLabeler.username}
                      onChange={(e) => setEditLabeler({ ...editLabeler, username: e.target.value })}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      required
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700">New Password (leave blank to keep current)</label>
                    <input
                      type="password"
                      value={editLabeler.password}
                      onChange={(e) => setEditLabeler({ ...editLabeler, password: e.target.value })}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      placeholder="Enter new password or leave blank"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Groups</label>
                    <div className="mt-2 space-y-2">
                      {groups.map((group) => (
                        <label key={group.id} className="flex items-center">
                          <input
                            type="checkbox"
                            checked={editLabeler.group_ids.includes(group.id)}
                            onChange={(e) => {
                              if (e.target.checked) {
                                setEditLabeler({ ...editLabeler, group_ids: [...editLabeler.group_ids, group.id] });
                              } else {
                                setEditLabeler({ ...editLabeler, group_ids: editLabeler.group_ids.filter(id => id !== group.id) });
                              }
                            }}
                            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                          />
                          <span className="ml-2 text-sm text-gray-700">{group.name}</span>
                        </label>
                      ))}
                    </div>
                  </div>
                  <div className="flex space-x-3">
                    <button
                      type="submit"
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                    >
                      Update Labeler
                    </button>
                    <button
                      type="button"
                      onClick={() => {
                        setShowEditForm(false);
                        setEditingLabeler(null);
                        setEditLabeler({ username: '', password: '', group_ids: [] });
                      }}
                      className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                    >
                      Cancel
                    </button>
                  </div>
                </form>
              </div>
            )}

            <Table
              data={labelers}
              columns={[
                { key: 'username', label: 'Username', className: 'font-medium text-gray-900' },
                { 
                  key: 'group_ids', 
                  label: 'Groups',
                  render: (groupIds) => (
                    <div className="flex flex-wrap gap-1">
                      {(groupIds as number[]).map((groupId: number) => {
                        const group = groups.find(g => g.id === groupId);
                        return group ? (
                          <span
                            key={groupId}
                            className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                          >
                            {group.name}
                          </span>
                        ) : null;
                      })}
                      {(groupIds as number[]).length === 0 && (
                        <span className="text-gray-400 text-xs">No groups assigned</span>
                      )}
                    </div>
                  ),
                  className: 'text-gray-500'
                },
                { 
                  key: 'id', 
                  label: 'Actions',
                  render: (_, labeler) => (
                    <div className="flex space-x-2">
                      <button
                        onClick={() => handleEditLabeler(labeler)}
                        className="text-blue-600 hover:text-blue-900"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => handleDeleteLabeler(labeler.id)}
                        className="text-red-600 hover:text-red-900"
                      >
                        Delete
                      </button>
                    </div>
                  ),
                  className: 'font-medium'
                }
              ]}
            />
          </div>
        </Card>
      </div>
    </div>
  );
}
