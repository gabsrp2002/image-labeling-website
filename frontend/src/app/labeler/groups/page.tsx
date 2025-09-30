'use client';

import { useState, useEffect } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { useRouter } from 'next/navigation';

interface Group {
  id: string;
  name: string;
  description: string;
  taskCount: number;
  completedTasks: number;
}

interface Task {
  id: string;
  title: string;
  description: string;
  status: 'pending' | 'in_progress' | 'completed';
  priority: 'low' | 'medium' | 'high';
  dueDate?: string;
}

export default function LabelerGroupsPage() {
  const { user, token } = useAuth();
  const router = useRouter();
  const [groups, setGroups] = useState<Group[]>([]);
  const [selectedGroup, setSelectedGroup] = useState<Group | null>(null);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  // Redirect if not labeler
  useEffect(() => {
    if (user && user.role !== 'labeler') {
      router.push('/');
    }
  }, [user, router]);

  // Load groups
  useEffect(() => {
    if (user?.role === 'labeler' && token) {
      loadGroups();
    }
  }, [user, token]);

  const loadGroups = async () => {
    try {
      setIsLoading(true);
      // TODO: Replace with actual API call
      // Mock data for now
      setGroups([
        { id: '1', name: 'Group A', description: 'Image classification tasks', taskCount: 15, completedTasks: 8 },
        { id: '2', name: 'Group B', description: 'Object detection tasks', taskCount: 10, completedTasks: 3 },
        { id: '3', name: 'Group C', description: 'Semantic segmentation tasks', taskCount: 5, completedTasks: 0 },
      ]);
    } catch (error) {
      console.error('Error loading groups:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const loadTasks = async (groupId: string) => {
    try {
      // TODO: Replace with actual API call
      // Mock data for now
      const mockTasks: Task[] = [
        {
          id: '1',
          title: 'Classify images of cats and dogs',
          description: 'Label each image as either cat or dog',
          status: 'pending',
          priority: 'high',
          dueDate: '2024-01-15'
        },
        {
          id: '2',
          title: 'Identify objects in street scenes',
          description: 'Mark all visible objects in the provided street images',
          status: 'in_progress',
          priority: 'medium',
          dueDate: '2024-01-20'
        },
        {
          id: '3',
          title: 'Segment medical images',
          description: 'Identify and segment different organs in medical scans',
          status: 'completed',
          priority: 'low'
        }
      ];
      setTasks(mockTasks);
    } catch (error) {
      console.error('Error loading tasks:', error);
    }
  };

  const handleGroupSelect = (group: Group) => {
    setSelectedGroup(group);
    loadTasks(group.id);
  };

  const handleTaskStatusChange = async (taskId: string, newStatus: Task['status']) => {
    try {
      // TODO: Implement actual API call
      setTasks(tasks.map(task => 
        task.id === taskId ? { ...task, status: newStatus } : task
      ));
    } catch (error) {
      console.error('Error updating task status:', error);
    }
  };

  if (!user || user.role !== 'labeler') {
    return <div>Access denied. Labeler privileges required.</div>;
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
          <h1 className="text-3xl font-bold text-gray-900">My Groups</h1>
          <p className="mt-2 text-gray-600">Select a group to view and work on labeling tasks.</p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Groups List */}
          <div className="lg:col-span-1">
            <div className="bg-white shadow rounded-lg">
              <div className="px-4 py-5 sm:p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Available Groups</h2>
                <div className="space-y-3">
                  {groups.map((group) => (
                    <button
                      key={group.id}
                      onClick={() => handleGroupSelect(group)}
                      className={`w-full text-left p-4 rounded-lg border-2 transition-colors ${
                        selectedGroup?.id === group.id
                          ? 'border-blue-500 bg-blue-50'
                          : 'border-gray-200 hover:border-gray-300'
                      }`}
                    >
                      <h3 className="font-medium text-gray-900">{group.name}</h3>
                      <p className="text-sm text-gray-600 mt-1">{group.description}</p>
                      <div className="mt-2 flex items-center justify-between">
                        <span className="text-sm text-gray-500">
                          {group.completedTasks}/{group.taskCount} tasks completed
                        </span>
                        <div className="w-16 bg-gray-200 rounded-full h-2">
                          <div
                            className="bg-blue-600 h-2 rounded-full"
                            style={{ width: `${(group.completedTasks / group.taskCount) * 100}%` }}
                          />
                        </div>
                      </div>
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </div>

          {/* Tasks List */}
          <div className="lg:col-span-2">
            {selectedGroup ? (
              <div className="bg-white shadow rounded-lg">
                <div className="px-4 py-5 sm:p-6">
                  <div className="flex items-center justify-between mb-6">
                    <div>
                      <h2 className="text-lg font-medium text-gray-900">{selectedGroup.name}</h2>
                      <p className="text-sm text-gray-600">{selectedGroup.description}</p>
                    </div>
                    <div className="text-right">
                      <div className="text-sm text-gray-500">Progress</div>
                      <div className="text-2xl font-bold text-blue-600">
                        {selectedGroup.completedTasks}/{selectedGroup.taskCount}
                      </div>
                    </div>
                  </div>

                  <div className="space-y-4">
                    {tasks.map((task) => (
                      <div key={task.id} className="border border-gray-200 rounded-lg p-4">
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <h3 className="font-medium text-gray-900">{task.title}</h3>
                            <p className="text-sm text-gray-600 mt-1">{task.description}</p>
                            <div className="mt-2 flex items-center space-x-4">
                              <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                                task.priority === 'high' ? 'bg-red-100 text-red-800' :
                                task.priority === 'medium' ? 'bg-yellow-100 text-yellow-800' :
                                'bg-green-100 text-green-800'
                              }`}>
                                {task.priority} priority
                              </span>
                              {task.dueDate && (
                                <span className="text-sm text-gray-500">
                                  Due: {new Date(task.dueDate).toLocaleDateString()}
                                </span>
                              )}
                            </div>
                          </div>
                          <div className="ml-4 flex items-center space-x-2">
                            <select
                              value={task.status}
                              onChange={(e) => handleTaskStatusChange(task.id, e.target.value as Task['status'])}
                              className="text-sm border border-gray-300 rounded-md px-2 py-1 focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                            >
                              <option value="pending">Pending</option>
                              <option value="in_progress">In Progress</option>
                              <option value="completed">Completed</option>
                            </select>
                            <button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
                              Start Task
                            </button>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            ) : (
              <div className="bg-white shadow rounded-lg">
                <div className="px-4 py-5 sm:p-6 text-center">
                  <svg className="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                  </svg>
                  <h3 className="mt-2 text-sm font-medium text-gray-900">No group selected</h3>
                  <p className="mt-1 text-sm text-gray-500">Choose a group from the left to view tasks.</p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
