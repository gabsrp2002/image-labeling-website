'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useApiClient, ApiClient } from '@/utils/api';
import { LoadingSpinner, PageHeader, BackButton, Card, Button, EmptyState, SuccessMessage, ErrorMessage } from '@/components';

interface ImageData {
  id: number;
  filename: string;
  filetype: string;
  base64_data: string;
  uploaded_at: string;
}

interface TagStatistic {
  tag_id: number;
  tag_name: string;
  percentage: number;
  count: number;
  total_labelers: number;
}

interface FinalTagData {
  id: number;
  tag_id: number;
  tag_name: string;
  is_admin_override: boolean;
  created_at: string;
}

interface ImageDetailsResponse {
  success: boolean;
  message: string;
  data: {
    image: ImageData;
    tag_statistics: TagStatistic[];
    final_tags: FinalTagData[];
    has_admin_override: boolean;
  };
}

interface UpdateFinalTagsRequest {
  tag_ids: number[];
}

export default function ImageDetailPage() {
  const params = useParams();
  const router = useRouter();
  const groupId = parseInt(params.id as string);
  const imageId = parseInt(params.imageId as string);
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);
  
  const [image, setImage] = useState<ImageData | null>(null);
  const [tagStatistics, setTagStatistics] = useState<TagStatistic[]>([]);
  const [finalTags, setFinalTags] = useState<FinalTagData[]>([]);
  const [hasAdminOverride, setHasAdminOverride] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isUpdating, setIsUpdating] = useState(false);
  const [updateError, setUpdateError] = useState<string | null>(null);
  const [updateSuccess, setUpdateSuccess] = useState<string | null>(null);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadImageDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const response = await apiClientRef.current.get<ImageDetailsResponse>(`/admin/groups/${groupId}/image/${imageId}`);
      
      if (response.success && response.data) {
        setImage(response.data.data.image);
        setTagStatistics(response.data.data.tag_statistics);
        setFinalTags(response.data.data.final_tags);
        setHasAdminOverride(response.data.data.has_admin_override);
        
        // Auto-generate final tags if none exist and no admin override
        if (response.data.data.final_tags.length === 0 && !response.data.data.has_admin_override) {
          try {
            const autoGenResponse = await apiClientRef.current.post(`/admin/image/${imageId}/final-tags/auto-generate`);
            if (autoGenResponse.success && autoGenResponse.data) {
              setFinalTags((autoGenResponse.data as any).data || []);
              setHasAdminOverride(false);
            }
          } catch (error) {
            console.error('Error auto-generating final tags:', error);
          }
        }
      } else {
        setError(response.error || 'Failed to load image details');
      }
    } catch (error) {
      console.error('Error loading image details:', error);
      setError('Failed to load image details');
    } finally {
      setIsLoading(false);
    }
  }, [groupId, imageId]);

  useEffect(() => {
    if (groupId && imageId) {
      loadImageDetails();
    }
  }, [groupId, imageId, loadImageDetails]);


  const handleUpdateFinalTags = useCallback(async (tagIds: number[]) => {
    setIsUpdating(true);
    setUpdateError(null);
    setUpdateSuccess(null);

    try {
      const request: UpdateFinalTagsRequest = { tag_ids: tagIds };
      const response = await apiClientRef.current.put(`/admin/image/${imageId}/final-tags`, request);
      
      if (response.success && response.data) {
        setFinalTags((response.data as any).data || []);
        setHasAdminOverride(true);
        setUpdateSuccess('Final tags updated successfully!');
      } else {
        setUpdateError(response.error || 'Failed to update final tags');
      }
    } catch (error) {
      console.error('Error updating final tags:', error);
      setUpdateError('Failed to update final tags. Please try again.');
    } finally {
      setIsUpdating(false);
    }
  }, [imageId]);

  const handleToggleTag = useCallback((tagId: number) => {
    const isCurrentlySelected = finalTags.some(tag => tag.tag_id === tagId);
    let newTagIds: number[];
    
    if (isCurrentlySelected) {
      newTagIds = finalTags.filter(tag => tag.tag_id !== tagId).map(tag => tag.tag_id);
    } else {
      newTagIds = [...finalTags.map(tag => tag.tag_id), tagId];
    }
    
    handleUpdateFinalTags(newTagIds);
  }, [finalTags, handleUpdateFinalTags]);

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error || !image) {
    return <LoadingSpinner message={`Error: ${error || 'Image not found'}`} className="text-red-600" />;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-6 sm:mb-8">
          <BackButton onClick={() => router.back()} className="mb-4">
            Back to Group
          </BackButton>
          <PageHeader 
            title={image.filename}
            description={`Uploaded: ${new Date(image.uploaded_at).toLocaleString()}`}
          />
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Image Display */}
          <Card>
            <div className="p-4 sm:p-6">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Image</h3>
              <div className="flex justify-center">
                <img
                  src={`data:image/${image.filetype};base64,${image.base64_data}`}
                  alt={image.filename}
                  className="max-w-full h-auto max-h-96 rounded-lg shadow-lg"
                />
              </div>
            </div>
          </Card>

          {/* Tag Statistics Dashboard */}
          <Card>
            <div className="p-4 sm:p-6">
              <div className="mb-4">
                <h3 className="text-lg font-medium text-gray-900">Tag Statistics</h3>
              </div>

              {tagStatistics.length === 0 ? (
                <EmptyState
                  title="No tag statistics"
                  description="No labelers have tagged this image yet."
                />
              ) : (
                <div className="space-y-3">
                  {tagStatistics.map((stat) => (
                    <div key={stat.tag_id} className="bg-gray-50 rounded-lg p-3">
                      <div className="flex justify-between items-center mb-2">
                        <span className="text-sm font-medium text-gray-900">{stat.tag_name}</span>
                        <span className="text-sm text-gray-500">
                          {stat.count}/{stat.total_labelers} labelers
                        </span>
                      </div>
                      <div className="w-full bg-gray-200 rounded-full h-2">
                        <div
                          className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                          style={{ width: `${stat.percentage}%` }}
                        ></div>
                      </div>
                      <div className="text-xs text-gray-500 mt-1">
                        {stat.percentage.toFixed(1)}%
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </Card>
        </div>

        {/* Final Tags Section */}
        <Card className="mt-6">
          <div className="p-4 sm:p-6">
            <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-4 space-y-3 sm:space-y-0">
              <h3 className="text-lg font-medium text-gray-900">
                Final Tags {hasAdminOverride && <span className="text-sm text-orange-600">(Admin Override)</span>}
              </h3>
              <div className="text-sm text-gray-500">
                Click tags to toggle them as final tags
              </div>
            </div>

            {/* Messages */}
            {updateError && (
              <ErrorMessage message={updateError} onClose={() => setUpdateError(null)} />
            )}

            {updateSuccess && (
              <SuccessMessage message={updateSuccess} onClose={() => setUpdateSuccess(null)} />
            )}

            {tagStatistics.length === 0 ? (
              <EmptyState
                title="No tags available"
                description="No labelers have tagged this image yet."
              />
            ) : (
              <div className="flex flex-wrap gap-2">
                {tagStatistics.map((stat) => {
                  const isSelected = finalTags.some(tag => tag.tag_id === stat.tag_id);
                  return (
                    <button
                      key={stat.tag_id}
                      onClick={() => handleToggleTag(stat.tag_id)}
                      disabled={isUpdating}
                      className={`px-3 py-2 rounded-full text-sm font-medium transition-colors ${
                        isSelected
                          ? 'bg-green-100 text-green-800 border-2 border-green-300'
                          : 'bg-gray-100 text-gray-800 border-2 border-gray-200 hover:bg-gray-200'
                      } ${isUpdating ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
                    >
                      {stat.tag_name}
                      {isSelected && (
                        <span className="ml-1 text-green-600">✓</span>
                      )}
                    </button>
                  );
                })}
              </div>
            )}

            {finalTags.length > 0 && (
              <div className="mt-4">
                <h4 className="text-sm font-medium text-gray-700 mb-2">Current Final Tags:</h4>
                <div className="flex flex-wrap gap-2">
                  {finalTags.map((tag) => (
                    <span
                      key={tag.id}
                      className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800"
                    >
                      {tag.tag_name}
                      {tag.is_admin_override && (
                        <span className="ml-1 text-orange-600" title="Admin Override">⚡</span>
                      )}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        </Card>
      </div>
    </div>
  );
}
