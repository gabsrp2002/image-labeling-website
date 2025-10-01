'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { LoadingSpinner, PageHeader, Card, EmptyState, BackButton } from '@/components';
import { useApiClient } from '@/utils/api';

interface Group {
  id: number;
  name: string;
  description: string | null;
}

interface Image {
  id: number;
  filename: string;
  status: string;
}

export default function LabelerGroupDetailPage() {
  const params = useParams();
  const router = useRouter();
  const groupId = parseInt(params.id as string);
  
  const [group, setGroup] = useState<Group | null>(null);
  const [images, setImages] = useState<Image[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingImages, setIsLoadingImages] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadImages = useCallback(async (groupId: number) => {
    try {
      setIsLoadingImages(true);
      const response = await apiClientRef.current.getGroupImages(groupId);
      if (response.success && response.data) {
        setImages(response.data);
      } else {
        console.error('Error loading images:', response.error);
        setImages([]);
      }
    } catch (error) {
      console.error('Error loading images:', error);
      setImages([]);
    } finally {
      setIsLoadingImages(false);
    }
  }, []);

  const loadGroupDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      // First get all groups to find the specific group
      const groupsResponse = await apiClientRef.current.getLabelerGroups();
      if (groupsResponse.success && groupsResponse.data) {
        const foundGroup = groupsResponse.data.find(g => g.id === groupId);
        if (foundGroup) {
          setGroup(foundGroup);
          // Load images for this group
          await loadImages(groupId);
        } else {
          setError('Group not found or you do not have access to this group');
        }
      } else {
        setError('Failed to load group details');
      }
    } catch (error) {
      console.error('Error loading group details:', error);
      setError('Failed to load group details');
    } finally {
      setIsLoading(false);
    }
  }, [groupId, apiClientRef, loadImages]);

  useEffect(() => {
    if (groupId) {
      loadGroupDetails();
    }
  }, [groupId, loadGroupDetails]);

  // Calculate progress statistics
  const getProgressStats = () => {
    const totalImages = images.length;
    const doneImages = images.filter(img => img.status === 'done').length;
    const pendingImages = images.filter(img => img.status === 'pending').length;
    const progressPercentage = totalImages > 0 ? (doneImages / totalImages) * 100 : 0;
    
    return {
      total: totalImages,
      done: doneImages,
      pending: pendingImages,
      percentage: progressPercentage
    };
  };

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
          <div className="flex items-center mb-6">
            <BackButton onClick={() => router.push('/labeler/groups')} />
          </div>
          <div className="text-center py-12">
            <div className="text-red-600 text-lg font-medium mb-2">Error</div>
            <div className="text-gray-600">{error}</div>
          </div>
        </div>
      </div>
    );
  }

  if (!group) {
    return (
      <div className="min-h-screen bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
          <div className="flex items-center mb-6">
            <BackButton onClick={() => router.push('/labeler/groups')} />
          </div>
          <div className="text-center py-12">
            <div className="text-gray-600">Group not found</div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
        <div className="flex items-center mb-6">
          <BackButton onClick={() => router.push('/labeler/groups')} />
        </div>
        
        <PageHeader 
          title={group.name}
          description={group.description || 'Image labeling tasks for this group'} 
        />

        <div className="mt-8">
          <Card>
            <div className="px-4 py-5 sm:p-6">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-6 gap-4">
                <div className="flex-1 min-w-0">
                  <h2 className="text-lg sm:text-xl font-medium text-gray-900 truncate">{group.name}</h2>
                  <p className="text-sm text-gray-600 mt-1 line-clamp-2">
                    {group.description || 'No description available'}
                  </p>
                </div>
                <div className="flex-shrink-0 text-center sm:text-right">
                  <div className="text-sm text-gray-500">Progress</div>
                  <div className="text-xl sm:text-2xl font-bold text-blue-600">
                    {getProgressStats().done}/{getProgressStats().total}
                  </div>
                  <div className="w-24 sm:w-32 bg-gray-200 rounded-full h-2 mt-2 mx-auto sm:mx-0">
                    <div
                      className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                      style={{ width: `${getProgressStats().percentage}%` }}
                    />
                  </div>
                </div>
              </div>

              {isLoadingImages ? (
                <div className="flex justify-center py-8">
                  <LoadingSpinner />
                </div>
              ) : (
                <div className="space-y-6">
                  {/* Done Images */}
                  {images.filter(img => img.status === 'done').length > 0 && (
                    <div>
                      <h3 className="text-base sm:text-lg font-medium text-green-800 mb-3 flex items-center">
                        <svg className="w-4 h-4 sm:w-5 sm:h-5 mr-2 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                        </svg>
                        <span className="truncate">Completed Images ({getProgressStats().done})</span>
                      </h3>
                      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3 sm:gap-4">
                        {images.filter(img => img.status === 'done').map((image) => (
                          <div key={image.id} className="border border-green-200 bg-green-50 rounded-lg p-3 sm:p-4">
                            <div className="flex items-center justify-between">
                              <div className="flex-1 min-w-0">
                                <p className="text-xs sm:text-sm font-medium text-gray-900 truncate">
                                  {image.filename}
                                </p>
                                <p className="text-xs text-green-600 mt-1">Completed</p>
                              </div>
                              <div className="ml-2 flex-shrink-0">
                                <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                  Done
                                </span>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {/* Pending Images */}
                  {images.filter(img => img.status === 'pending').length > 0 && (
                    <div>
                      <h3 className="text-base sm:text-lg font-medium text-orange-800 mb-3 flex items-center">
                        <svg className="w-4 h-4 sm:w-5 sm:h-5 mr-2 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clipRule="evenodd" />
                        </svg>
                        <span className="truncate">Pending Images ({getProgressStats().pending})</span>
                      </h3>
                      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3 sm:gap-4">
                        {images.filter(img => img.status === 'pending').map((image) => (
                          <div key={image.id} className="border border-orange-200 bg-orange-50 rounded-lg p-3 sm:p-4">
                            <div className="flex items-center justify-between">
                              <div className="flex-1 min-w-0">
                                <p className="text-xs sm:text-sm font-medium text-gray-900 truncate">
                                  {image.filename}
                                </p>
                                <p className="text-xs text-orange-600 mt-1">Needs labeling</p>
                              </div>
                              <div className="ml-2 flex-shrink-0">
                                <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-orange-100 text-orange-800">
                                  Pending
                                </span>
                              </div>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {/* No images message */}
                  {images.length === 0 && !isLoadingImages && (
                    <div className="text-center py-8">
                      <EmptyState
                        icon={
                          <svg className="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                          </svg>
                        }
                        title="No images found"
                        description="This group doesn't have any images yet."
                      />
                    </div>
                  )}
                </div>
              )}
            </div>
          </Card>
        </div>
      </div>
    </div>
  );
}
