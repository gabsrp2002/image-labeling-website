'use client';

import { useState, useEffect } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { useRouter } from 'next/navigation';

interface Labeler {
  id: string;
  username: string;
  groups: string[];
}

export default function AdminLabelersPage() {
  const { user, token } = useAuth();
  const router = useRouter();
  const [labelers, setLabelers] = useState<Labeler[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newLabeler, setNewLabeler] = useState({ username: '', password: '', groups: [] });
  const [availableGroups, setAvailableGroups] = useState<string[]>([]);

  // Redirect if not admin
  useEffect(() => {
    if (user && user.role !== 'admin') {
      router.push('/');
    }
  }, [user, router]);

  // Load labelers and groups
  useEffect(() => {
    if (user?.role === 'admin' && token) {
      loadData();
    }
  }, [user, token]);

  const loadData = async () => {
    try {
      setIsLoading(true);
      // TODO: Replace with actual API calls
      // Mock data for now
      setLabelers([
        { id: '1', username: 'labeler1', groups: ['Group A', 'Group B'] },
        { id: '2', username: 'labeler2', groups: ['Group A'] },
        { id: '3', username: 'labeler3', groups: ['Group B', 'Group C'] },
      ]);
      setAvailableGroups(['Group A', 'Group B', 'Group C', 'Group D']);
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleAddLabeler = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      // TODO: Implement actual API call
      console.log('Adding labeler:', newLabeler);
      setShowAddForm(false);
      setNewLabeler({ username: '', password: '', groups: [] });
      // Refresh data
      loadData();
    } catch (error) {
      console.error('Error adding labeler:', error);
    }
  };

  const handleDeleteLabeler = async (id: string) => {
    if (confirm('Are you sure you want to delete this labeler?')) {
      try {
        // TODO: Implement actual API call
        console.log('Deleting labeler:', id);
        loadData();
      } catch (error) {
        console.error('Error deleting labeler:', error);
      }
    }
  };

  if (!user || user.role !== 'admin') {
    return <div>Access denied. Admin privileges required.</div>;
  }

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-gray-600">Loading...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">Manage Labelers</h1>
          <p className="mt-2 text-gray-600">Create and manage labeler accounts and their group assignments.</p>
        </div>

        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-lg font-medium text-gray-900">Labelers</h2>
              <button
                onClick={() => setShowAddForm(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                Add Labeler
              </button>
            </div>

            {showAddForm && (
              <div className="mb-6 p-4 bg-gray-50 rounded-lg">
                <h3 className="text-lg font-medium text-gray-900 mb-4">Add New Labeler</h3>
                <form onSubmit={handleAddLabeler} className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Username</label>
                    <input
                      type="text"
                      value={newLabeler.username}
                      onChange={(e) => setNewLabeler({ ...newLabeler, username: e.target.value })}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      required
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Password</label>
                    <input
                      type="password"
                      value={newLabeler.password}
                      onChange={(e) => setNewLabeler({ ...newLabeler, password: e.target.value })}
                      className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                      required
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700">Groups</label>
                    <div className="mt-2 space-y-2">
                      {availableGroups.map((group) => (
                        <label key={group} className="flex items-center">
                          <input
                            type="checkbox"
                            checked={newLabeler.groups.includes(group)}
                            onChange={(e) => {
                              if (e.target.checked) {
                                setNewLabeler({ ...newLabeler, groups: [...newLabeler.groups, group] });
                              } else {
                                setNewLabeler({ ...newLabeler, groups: newLabeler.groups.filter(g => g !== group) });
                              }
                            }}
                            className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                          />
                          <span className="ml-2 text-sm text-gray-700">{group}</span>
                        </label>
                      ))}
                    </div>
                  </div>
                  <div className="flex space-x-3">
                    <button
                      type="submit"
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                    >
                      Add Labeler
                    </button>
                    <button
                      type="button"
                      onClick={() => setShowAddForm(false)}
                      className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                    >
                      Cancel
                    </button>
                  </div>
                </form>
              </div>
            )}

            <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
              <table className="min-w-full divide-y divide-gray-300">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Username
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Groups
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Actions
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {labelers.map((labeler) => (
                    <tr key={labeler.id}>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                        {labeler.username}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        <div className="flex flex-wrap gap-1">
                          {labeler.groups.map((group) => (
                            <span
                              key={group}
                              className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                            >
                              {group}
                            </span>
                          ))}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                        <button
                          onClick={() => handleDeleteLabeler(labeler.id)}
                          className="text-red-600 hover:text-red-900"
                        >
                          Delete
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
