'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useRouter } from 'next/navigation';
import { LoadingSpinner, PageHeader, Card, EmptyState } from '@/components';
import { useApiClient } from '@/utils/api';

interface Group {
  id: number;
  name: string;
  description: string | null;
}

export default function LabelerGroupsPage() {
  const [groups, setGroups] = useState<Group[]>([]);
  const [isLoading, setIsLoading] = useState(true);
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
      const response = await apiClientRef.current.getLabelerGroups();
      if (response.success && response.data) {
        setGroups(response.data);
      } else {
        console.error('Error loading groups:', response.error);
      }
    } catch (error) {
      console.error('Error loading groups:', error);
    } finally {
      setIsLoading(false);
    }
  }, []); // Empty dependency array since we use ref

  // Load groups
  useEffect(() => {
    loadGroups();
  }, [loadGroups]);

  const handleGroupSelect = (group: Group) => {
    router.push(`/labeler/groups/${group.id}`);
  };


  if (isLoading) {
    return <LoadingSpinner />;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
        <PageHeader 
          title="My Groups" 
          description="Select a group to view and work on labeling tasks." 
        />

        <div className="mt-8">
          <Card>
            <div className="px-4 py-5 sm:p-6">
              <h2 className="text-lg font-medium text-gray-900 mb-6">Available Groups</h2>
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {groups.length === 0 ? (
                  <div className="col-span-full text-center py-12">
                    <EmptyState
                      icon={
                        <svg className="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                      }
                      title="No groups assigned"
                      description="You haven't been assigned to any groups yet."
                    />
                  </div>
                ) : (
                  groups.map((group) => (
                    <button
                      key={group.id}
                      onClick={() => handleGroupSelect(group)}
                      className="w-full text-left p-4 sm:p-6 rounded-lg border-2 border-gray-200 hover:border-blue-300 hover:bg-blue-50 transition-colors group"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1 min-w-0">
                          <h3 className="font-medium text-gray-900 text-sm sm:text-base group-hover:text-blue-900">
                            {group.name}
                          </h3>
                          <p className="text-xs sm:text-sm text-gray-600 mt-2 line-clamp-3">
                            {group.description || 'No description available'}
                          </p>
                        </div>
                        <div className="ml-4 flex-shrink-0">
                          <svg className="w-5 h-5 text-gray-400 group-hover:text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                          </svg>
                        </div>
                      </div>
                    </button>
                  ))
                )}
              </div>
            </div>
          </Card>
        </div>
      </div>
    </div>
  );
}
