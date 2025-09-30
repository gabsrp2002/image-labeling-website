'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useApiClient, ApiClient } from '@/utils/api';
import { LoadingSpinner, PageHeader, BackButton, Card, Tabs, Button, EmptyState, ImageUploader } from '@/components';

interface Group {
  id: number;
  name: string;
  description: string | null;
}

interface Labeler {
  id: number;
  username: string;
}

interface Tag {
  id: number;
  name: string;
  description?: string;
}

interface Image {
  id: number;
  filename: string;
  filetype: string;
  uploaded_at: string;
}

interface GroupDetailResponse {
  success: boolean;
  message: string;
  data: {
    group: Group;
    labelers: Labeler[];
    tags: Tag[];
    images: Image[];
  };
}

export default function GroupDetailPage() {
  const params = useParams();
  const router = useRouter();
  const groupId = parseInt(params.id as string);
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);
  
  const [group, setGroup] = useState<Group | null>(null);
  const [labelers, setLabelers] = useState<Labeler[]>([]);
  const [tags, setTags] = useState<Tag[]>([]);
  const [images, setImages] = useState<Image[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'labelers' | 'tags' | 'images'>('labelers');
  const [isUploading, setIsUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [uploadSuccess, setUploadSuccess] = useState<string | null>(null);
  const [selectedFiles, setSelectedFiles] = useState<File[]>([]);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadGroupDetails = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      
      const response = await apiClientRef.current.get<GroupDetailResponse>(`/admin/groups/${groupId}`);
      
      if (response.success && response.data) {
        setGroup(response.data.data.group);
        setLabelers(response.data.data.labelers);
        setTags(response.data.data.tags);
        setImages(response.data.data.images);
      } else {
        setError(response.error || 'Failed to load group details');
      }
    } catch (error) {
      console.error('Error loading group details:', error);
      setError('Failed to load group details');
    } finally {
      setIsLoading(false);
    }
  }, [groupId]);

  useEffect(() => {
    if (groupId) {
      loadGroupDetails();
    }
  }, [groupId, loadGroupDetails]);

  const handleImagesSelected = useCallback((files: File[]) => {
    setSelectedFiles(files);
    setUploadError(null);
    setUploadSuccess(null);
  }, []);

  const handleUploadComplete = useCallback(() => {
    setSelectedFiles([]);
    setUploadSuccess('Images uploaded successfully!');
    setUploadError(null);
    // Reload group details to show new images
    loadGroupDetails();
  }, [loadGroupDetails]);

  const handleUploadError = useCallback((error: string) => {
    setUploadError(error);
    setUploadSuccess(null);
  }, []);

  const uploadImages = useCallback(async () => {
    if (selectedFiles.length === 0) return;

    setIsUploading(true);
    setUploadError(null);
    setUploadSuccess(null);

    try {
      const uploadPromises = selectedFiles.map(async (file) => {
        const base64Data = await ApiClient.fileToBase64(file);
        const filetype = file.type.split('/')[1]; // Extract extension from MIME type
        
        return apiClientRef.current.uploadImage(
          file.name,
          filetype,
          base64Data,
          groupId
        );
      });

      const results = await Promise.all(uploadPromises);
      
      // Check if all uploads were successful
      const failedUploads = results.filter(result => !result.success);
      
      if (failedUploads.length > 0) {
        const errorMessages = failedUploads.map(result => result.error).join(', ');
        setUploadError(`Some uploads failed: ${errorMessages}`);
      } else {
        handleUploadComplete();
      }
    } catch (error) {
      console.error('Upload error:', error);
      setUploadError('Failed to upload images. Please try again.');
    } finally {
      setIsUploading(false);
    }
  }, [selectedFiles, groupId, handleUploadComplete]);

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error || !group) {
    return <LoadingSpinner message={`Error: ${error || 'Group not found'}`} className="text-red-600" />;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-6 sm:mb-8">
          <BackButton onClick={() => router.back()} className="mb-4">
            Back to Groups
          </BackButton>
          <PageHeader 
            title={group.name}
            description={group.description || 'No description'}
          />
        </div>

        {/* Tabs */}
        <Card>
          <Tabs
            tabs={[
              { id: 'labelers', label: 'Labelers', count: labelers.length },
              { id: 'tags', label: 'Tags', count: tags.length },
              { id: 'images', label: 'Images', count: images.length }
            ]}
            activeTab={activeTab}
            onTabChange={(tabId) => setActiveTab(tabId as 'labelers' | 'tags' | 'images')}
          />

          <div className="p-4 sm:p-6">
            {/* Labelers Tab */}
            {activeTab === 'labelers' && (
              <div>
                <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-4 space-y-3 sm:space-y-0">
                  <h3 className="text-lg font-medium text-gray-900">Group Labelers</h3>
                  <Button fullWidth={false} className="w-full sm:w-auto">
                    <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                    </svg>
                    Add Labeler
                  </Button>
                </div>
                {labelers.length === 0 ? (
                  <EmptyState
                    title="No labelers assigned"
                    description="No labelers assigned to this group."
                  />
                ) : (
                  <div className="space-y-2">
                    {labelers.map((labeler) => (
                      <div key={labeler.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                        <span className="text-sm font-medium text-gray-900">{labeler.username}</span>
                        <button className="text-red-600 hover:text-red-900 text-sm">
                          Remove
                        </button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}

            {/* Tags Tab */}
            {activeTab === 'tags' && (
              <div>
                <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-4 space-y-3 sm:space-y-0">
                  <h3 className="text-lg font-medium text-gray-900">Group Tags</h3>
                  <button className="inline-flex items-center justify-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 w-full sm:w-auto">
                    <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                    </svg>
                    Add Tag
                  </button>
                </div>
                {tags.length === 0 ? (
                  <p className="text-gray-500">No tags defined for this group.</p>
                ) : (
                  <div className="flex flex-wrap gap-2">
                    {tags.map((tag) => (
                      <div
                        key={tag.id}
                        className="group relative inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800 hover:bg-blue-200 transition-colors"
                        title={tag.description || tag.name}
                      >
                        <span className="truncate max-w-32">{tag.name}</span>
                        <button
                          onClick={() => {
                            // TODO: Implement remove tag functionality
                            console.log('Remove tag:', tag.id);
                          }}
                          className="ml-2 text-blue-600 hover:text-blue-800"
                        >
                          <svg className="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                          </svg>
                        </button>
                        {/* Tooltip */}
                        {tag.description && (
                          <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-gray-900 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity duration-200 whitespace-nowrap z-10">
                            {tag.description}
                            <div className="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-2 border-r-2 border-t-2 border-transparent border-t-gray-900"></div>
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}

            {/* Images Tab */}
            {activeTab === 'images' && (
              <div>
                <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-4 space-y-3 sm:space-y-0">
                  <h3 className="text-lg font-medium text-gray-900">Group Images</h3>
                  <ImageUploader
                    onImagesSelected={handleImagesSelected}
                    onUploadComplete={handleUploadComplete}
                    onUploadError={handleUploadError}
                    isUploading={isUploading}
                  />
                </div>

                {/* Upload Messages */}
                {uploadError && (
                  <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md">
                    <div className="flex items-center justify-between">
                      <p className="text-sm text-red-600">{uploadError}</p>
                      <button
                        onClick={() => setUploadError(null)}
                        className="ml-3 text-red-400 hover:text-red-600 transition-colors"
                        aria-label="Dismiss error message"
                      >
                        <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                      </button>
                    </div>
                  </div>
                )}

                {uploadSuccess && (
                  <div className="mb-4 p-3 bg-green-50 border border-green-200 rounded-md">
                    <div className="flex items-center justify-between">
                      <p className="text-sm text-green-600">{uploadSuccess}</p>
                      <button
                        onClick={() => setUploadSuccess(null)}
                        className="ml-3 text-green-400 hover:text-green-600 transition-colors"
                        aria-label="Dismiss success message"
                      >
                        <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                      </button>
                    </div>
                  </div>
                )}

                {/* Upload Button */}
                {selectedFiles.length > 0 && (
                  <div className="mb-4 flex justify-end">
                    <Button
                      onClick={uploadImages}
                      disabled={isUploading}
                      fullWidth={false}
                      className="w-full sm:w-auto"
                    >
                      {isUploading ? 'Uploading...' : `Upload ${selectedFiles.length} Image${selectedFiles.length !== 1 ? 's' : ''}`}
                    </Button>
                  </div>
                )}
                {images.length === 0 ? (
                  <p className="text-gray-500">No images assigned to this group.</p>
                ) : (
                  <div className="space-y-3">
                    {images.map((image) => (
                      <div key={image.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center space-x-3">
                            <div className="flex-shrink-0">
                              <div className="w-10 h-10 bg-gray-200 rounded-lg flex items-center justify-center">
                                <span className="text-xs font-medium text-gray-500 uppercase">
                                  {image.filetype}
                                </span>
                              </div>
                            </div>
                            <div className="flex-1 min-w-0">
                              <p className="text-sm font-medium text-gray-900 truncate">
                                {image.filename}
                              </p>
                              <p className="text-xs text-gray-500">
                                Uploaded: {image.uploaded_at}
                              </p>
                            </div>
                          </div>
                        </div>
                        <button className="text-red-600 hover:text-red-900 text-sm font-medium">
                          Remove
                        </button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        </Card>
      </div>
    </div>
  );
}
