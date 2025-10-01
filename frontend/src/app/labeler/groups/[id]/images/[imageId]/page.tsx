'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { LoadingSpinner, PageHeader, Card, BackButton, Button } from '@/components';
import { useApiClient } from '@/utils/api';

interface Tag {
  id: number;
  name: string;
  description: string | null;
}

interface ImageDetails {
  image: {
    id: number;
    filename: string;
    status: string;
    base64_data: string;
    filetype: string;
  };
  group_tags: Tag[];
  current_tags: Tag[];
}

export default function ImageLabelingPage() {
  const params = useParams();
  const router = useRouter();
  const groupId = parseInt(params.id as string);
  const imageId = parseInt(params.imageId as string);
  
  const [imageDetails, setImageDetails] = useState<ImageDetails | null>(null);
  const [selectedTags, setSelectedTags] = useState<number[]>([]);
  const [suggestedTags, setSuggestedTags] = useState<string[]>([]);
  const [hasTriedSuggesting, setHasTriedSuggesting] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuggesting, setIsSuggesting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadImageDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const response = await apiClientRef.current.getImageDetails(groupId, imageId);
      if (response.success && response.data) {
        setImageDetails(response.data);
        setSelectedTags(response.data.current_tags.map(tag => tag.id));
      } else {
        setError(response.error || 'Failed to load image details');
      }
    } catch (error) {
      console.error('Error loading image details:', error);
      setError('Failed to load image details');
    } finally {
      setIsLoading(false);
    }
  }, [groupId, imageId, apiClientRef]);

  const handleSuggestTags = async () => {
    try {
      setIsSuggesting(true);
      setHasTriedSuggesting(true);
      const response = await apiClientRef.current.suggestTags(imageId, selectedTags);
      
      if (response.success && response.data && response.data.data && response.data.data.suggested_tags) {
        setSuggestedTags(response.data.data.suggested_tags);
      } else {
        const errorMessage = response.error || 'Unknown error occurred';
        console.error('Error suggesting tags:', errorMessage);
        setSuggestedTags([]);
      }
    } catch (error) {
      console.error('Exception in suggest tags:', error);
      setSuggestedTags([]);
    } finally {
      setIsSuggesting(false);
    }
  };

  const handleTagToggle = (tagId: number) => {
    setSelectedTags(prev => 
      prev.includes(tagId) 
        ? prev.filter(id => id !== tagId)
        : [...prev, tagId]
    );
  };

  const handleSubmit = async (exitAfterSubmit: boolean = false) => {
    try {
      setIsSubmitting(true);
      const response = await apiClientRef.current.updateImageTags(groupId, imageId, selectedTags);
      if (response.success) {
        if (exitAfterSubmit) {
          router.push('/labeler/groups');
        } else {
          // Find next pending image in the group
          const imagesResponse = await apiClientRef.current.getGroupImages(groupId);
          if (imagesResponse.success && imagesResponse.data) {
            const pendingImages = imagesResponse.data.filter(img => img.status === 'pending' && img.id !== imageId);
            if (pendingImages.length > 0) {
              router.push(`/labeler/groups/${groupId}/images/${pendingImages[0].id}`);
            } else {
              router.push(`/labeler/groups/${groupId}`);
            }
          } else {
            router.push(`/labeler/groups/${groupId}`);
          }
        }
      } else {
        setError(response.error || 'Failed to update tags');
      }
    } catch (error) {
      console.error('Error updating tags:', error);
      setError('Failed to update tags');
    } finally {
      setIsSubmitting(false);
    }
  };

  useEffect(() => {
    if (groupId && imageId) {
      loadImageDetails();
    }
  }, [groupId, imageId, loadImageDetails]);

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
          <div className="flex items-center mb-6">
            <BackButton onClick={() => router.push(`/labeler/groups/${groupId}`)} />
          </div>
          <div className="text-center py-12">
            <div className="text-red-600 text-lg font-medium mb-2">Error</div>
            <div className="text-gray-600">{error}</div>
          </div>
        </div>
      </div>
    );
  }

  if (!imageDetails) {
    return (
      <div className="min-h-screen bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
          <div className="flex items-center mb-6">
            <BackButton onClick={() => router.push(`/labeler/groups/${groupId}`)} />
          </div>
          <div className="text-center py-12">
            <div className="text-gray-600">Image not found</div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6 lg:py-8">
        <div className="flex items-center mb-6">
          <BackButton onClick={() => router.push(`/labeler/groups/${groupId}`)} />
        </div>
        
        <PageHeader 
          title={`Labeling: ${imageDetails.image.filename}`}
          description="Select tags that best describe this image" 
        />

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Image Display */}
          <div>
            <Card maxHeight="500px">
              <div className="px-4 py-5 sm:p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Image</h2>
                <div className="aspect-square bg-gray-100 rounded-lg overflow-hidden">
                  <img
                    src={`data:${imageDetails.image.filetype};base64,${imageDetails.image.base64_data}`}
                    alt={imageDetails.image.filename}
                    className="w-full h-full object-contain"
                  />
                </div>
                <div className="mt-2 text-center">
                  <p className="text-sm text-gray-600">{imageDetails.image.filename}</p>
                </div>
              </div>
            </Card>
          </div>

          {/* Tag Selection */}
          <div>
            <Card maxHeight="500px">
              <div className="px-4 py-5 sm:p-6">
                <div className="flex items-center justify-between mb-4">
                  <h2 className="text-lg font-medium text-gray-900">Select Tags</h2>
                  <Button
                    onClick={handleSuggestTags}
                    disabled={isSuggesting}
                    className="flex items-center space-x-2"
                  >
                    {isSuggesting ? (
                      <LoadingSpinner />
                    ) : (
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
                      </svg>
                    )}
                    <span>Suggest more tags</span>
                  </Button>
                </div>

                {/* Suggested Tags */}
                {suggestedTags && suggestedTags.length > 0 ? (
                  <div className="mb-6">
                    <h3 className="text-sm font-medium text-gray-700 mb-2">AI Suggestions</h3>
                    <div className="flex flex-wrap gap-2">
                      {suggestedTags.map((suggestion, index) => (
                        <span
                          key={index}
                          className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800"
                        >
                          {suggestion}
                        </span>
                      ))}
                    </div>
                  </div>
                ) : hasTriedSuggesting && suggestedTags && suggestedTags.length === 0 ? (
                  <div className="mb-6">
                    <div className="text-center py-4 text-gray-500 text-sm">
                      No new tag suggestions available. All available tags may already be selected.
                    </div>
                  </div>
                ) : null}

                {/* Available Tags */}
                <div className="space-y-3">
                  <h3 className="text-sm font-medium text-gray-700">Available Tags</h3>
                  <div className="grid grid-cols-1 gap-2 max-h-96 overflow-y-auto">
                    {imageDetails.group_tags.map((tag) => (
                      <label
                        key={tag.id}
                        className="flex items-start space-x-3 p-3 rounded-lg border border-gray-200 hover:bg-gray-50 cursor-pointer"
                      >
                        <input
                          type="checkbox"
                          checked={selectedTags.includes(tag.id)}
                          onChange={() => handleTagToggle(tag.id)}
                          className="mt-1 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        />
                        <div className="flex-1 min-w-0">
                          <div className="text-sm font-medium text-gray-900">{tag.name}</div>
                          {tag.description && (
                            <div className="text-xs text-gray-500 mt-1">{tag.description}</div>
                          )}
                        </div>
                      </label>
                    ))}
                  </div>
                </div>

                {/* Selected Tags Summary */}
                {selectedTags.length > 0 && (
                  <Card maxHeight="200px" className="mt-6">
                    <div className="p-4 bg-blue-50 rounded-lg">
                    <h4 className="text-sm font-medium text-blue-900 mb-2">
                      Selected Tags ({selectedTags.length})
                    </h4>
                    <div className="flex flex-wrap gap-2">
                      {selectedTags.map(tagId => {
                        const tag = imageDetails.group_tags.find(t => t.id === tagId);
                        return tag ? (
                          <span
                            key={tagId}
                            className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                          >
                            {tag.name}
                          </span>
                        ) : null;
                      })}
                    </div>
                    </div>
                  </Card>
                )}

                {/* Action Buttons */}
                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                  <Button
                    onClick={() => handleSubmit(false)}
                    disabled={isSubmitting}
                    className="flex-1"
                  >
                    {isSubmitting ? <LoadingSpinner /> : 'Submit'}
                  </Button>
                  <Button
                    onClick={() => handleSubmit(true)}
                    disabled={isSubmitting}
                    variant="secondary"
                    className="flex-1"
                  >
                    {isSubmitting ? <LoadingSpinner /> : 'Submit and Exit'}
                  </Button>
                </div>
              </div>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
