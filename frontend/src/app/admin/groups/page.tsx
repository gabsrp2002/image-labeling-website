'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useApiClient } from '@/utils/api';
import { useRouter } from 'next/navigation';
import { LoadingSpinner, PageHeader, Button, Card, Table, FormInput } from '@/components';

interface Group {
  id: number;
  name: string;
  description: string | null;
  labelerCount?: number;
}

interface GroupListResponse {
  success: boolean;
  message: string;
  data: {
    groups: Group[];
    total: number;
  };
}

interface CreateGroupRequest {
  name: string;
  description?: string;
}

export default function AdminGroupsPage() {
  const [groups, setGroups] = useState<Group[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newGroup, setNewGroup] = useState({ name: '', description: '' });
  const [error, setError] = useState<string | null>(null);
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);
  const router = useRouter();

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadGroups = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      const response = await apiClientRef.current.get<GroupListResponse>('/admin/groups');
      
      if (response.success && response.data) {
        setGroups(response.data.data.groups);
      } else {
        setError(response.error || 'Failed to load groups');
      }
    } catch (error) {
      console.error('Error loading groups:', error);
      setError('Failed to load groups');
    } finally {
      setIsLoading(false);
    }
  }, []); // Empty dependency array since we use ref

  // Load groups
  useEffect(() => {
    loadGroups();
  }, [loadGroups]);

  const handleAddGroup = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setError(null);
      const requestData: CreateGroupRequest = {
        name: newGroup.name,
        description: newGroup.description || undefined,
      };
      
      const response = await apiClientRef.current.post('/admin/groups', requestData);
      
      if (response.success) {
        setShowAddForm(false);
        setNewGroup({ name: '', description: '' });
        // Refresh data
        loadGroups();
      } else {
        setError(response.error || 'Failed to create group');
      }
    } catch (error) {
      console.error('Error adding group:', error);
      setError('Failed to create group');
    }
  };

  const handleDeleteGroup = async (id: number) => {
    if (confirm('Are you sure you want to delete this group? This action cannot be undone.')) {
      try {
        setError(null);
        const response = await apiClientRef.current.delete(`/admin/groups/${id}`);
        
        if (response.success) {
          // Refresh the groups list
          loadGroups();
        } else {
          setError(response.error || 'Failed to delete group');
        }
      } catch (error) {
        console.error('Error deleting group:', error);
        setError('Failed to delete group');
      }
    }
  };

  const handleGroupClick = (groupId: number) => {
    router.push(`/admin/groups/${groupId}`);
  };


  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <LoadingSpinner message={`Error: ${error}`} className="text-red-600" />;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <PageHeader 
          title="Manage Groups" 
          description="Create and manage labeling groups for organizing tasks and labelers." 
        />

        <Card>
          <div className="px-4 py-5 sm:p-6">
            <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-6 space-y-3 sm:space-y-0">
              <h2 className="text-lg font-medium text-gray-900">Groups</h2>
              <Button
                onClick={() => setShowAddForm(true)}
                fullWidth={false}
                className="w-full sm:w-auto"
              >
                <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                Add Group
              </Button>
            </div>

            {showAddForm && (
              <div className="mb-6 p-4 bg-gray-50 rounded-lg">
                <h3 className="text-lg font-medium text-gray-900 mb-4">Add New Group</h3>
                <form onSubmit={handleAddGroup} className="space-y-4">
                  <FormInput
                    label="Group Name"
                    value={newGroup.name}
                    onChange={(value) => setNewGroup({ ...newGroup, name: value })}
                    required
                  />
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Description</label>
                    <textarea
                      value={newGroup.description}
                      onChange={(e) => setNewGroup({ ...newGroup, description: e.target.value })}
                      rows={3}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      required
                    />
                  </div>
                  <div className="flex flex-col sm:flex-row space-y-3 sm:space-y-0 sm:space-x-3">
                    <Button
                      type="submit"
                      fullWidth={false}
                      className="w-full sm:w-auto"
                    >
                      Add Group
                    </Button>
                    <Button
                      type="button"
                      onClick={() => setShowAddForm(false)}
                      variant="secondary"
                      fullWidth={false}
                      className="w-full sm:w-auto"
                    >
                      Cancel
                    </Button>
                  </div>
                </form>
              </div>
            )}

            {/* Mobile Card View */}
            <div className="block sm:hidden space-y-4">
              {groups.map((group) => (
                <div
                  key={group.id}
                  className="bg-white border border-gray-200 rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer"
                  onClick={() => handleGroupClick(group.id)}
                >
                  <div className="flex justify-between items-start mb-2">
                    <h3 className="text-lg font-medium text-gray-900">{group.name}</h3>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDeleteGroup(group.id);
                      }}
                      className="text-red-600 hover:text-red-900 text-sm font-medium"
                    >
                      Delete
                    </button>
                  </div>
                  <p className="text-sm text-gray-500 mb-2">
                    {group.description || 'No description'}
                  </p>
                  <p className="text-sm text-gray-600">
                    {group.labelerCount || 0} labeler{(group.labelerCount || 0) !== 1 ? 's' : ''}
                  </p>
                </div>
              ))}
            </div>

            {/* Desktop Table View */}
            <div className="hidden sm:block">
              <Table
                data={groups}
                columns={[
                  { key: 'name', label: 'Name', className: 'font-medium text-gray-900' },
                  { 
                    key: 'description', 
                    label: 'Description',
                    render: (value) => String(value || 'No description'),
                    className: 'text-gray-500'
                  },
                  { 
                    key: 'labelerCount', 
                    label: 'Labelers',
                    render: (value) => `${value || 0} labeler${(value || 0) !== 1 ? 's' : ''}`,
                    className: 'text-gray-500'
                  },
                  { 
                    key: 'id', 
                    label: 'Actions',
                    render: (_, group) => (
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDeleteGroup(group.id);
                        }}
                        className="text-red-600 hover:text-red-900"
                      >
                        Delete
                      </button>
                    ),
                    className: 'font-medium'
                  }
                ]}
                onRowClick={(group) => handleGroupClick(group.id)}
              />
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
