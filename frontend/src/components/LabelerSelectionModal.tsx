'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useApiClient } from '@/utils/api';
import Modal from './Modal';
import Button from './Button';
import LoadingSpinner from './LoadingSpinner';

interface Labeler {
  id: number;
  username: string;
}

interface LabelerListResponse {
  success: boolean;
  message: string;
  data: {
    labelers: Labeler[];
    total: number;
  };
}

interface LabelerSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSelect: (labelerId: number) => void;
  isLoading: boolean;
  title: string;
  excludeLabelerIds?: number[];
}

export default function LabelerSelectionModal({
  isOpen,
  onClose,
  onSelect,
  isLoading,
  title,
  excludeLabelerIds = []
}: LabelerSelectionModalProps) {
  const [labelers, setLabelers] = useState<Labeler[]>([]);
  const [isLoadingLabelers, setIsLoadingLabelers] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const apiClient = useApiClient();
  const apiClientRef = useRef(apiClient);

  // Update ref when apiClient changes
  useEffect(() => {
    apiClientRef.current = apiClient;
  }, [apiClient]);

  const loadLabelers = useCallback(async () => {
    try {
      setIsLoadingLabelers(true);
      setError(null);
      const response = await apiClientRef.current.get<LabelerListResponse>('/admin/labeler');
      
      if (response.success && response.data) {
        // Filter out labelers that are already in the group
        const availableLabelers = response.data.data.labelers.filter(
          labeler => !excludeLabelerIds.includes(labeler.id)
        );
        setLabelers(availableLabelers);
      } else {
        setError(response.error || 'Failed to load labelers');
      }
    } catch (error) {
      console.error('Error loading labelers:', error);
      setError('Failed to load labelers');
    } finally {
      setIsLoadingLabelers(false);
    }
  }, [excludeLabelerIds]);

  useEffect(() => {
    if (isOpen) {
      loadLabelers();
    }
  }, [isOpen, loadLabelers]);

  const filteredLabelers = labelers.filter(labeler =>
    labeler.username.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleSelect = (labelerId: number) => {
    onSelect(labelerId);
    setSearchTerm('');
  };

  const handleClose = () => {
    setSearchTerm('');
    setError(null);
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title={title}>
      <div className="space-y-4">
        {/* Search Input */}
        <div>
          <input
            type="text"
            placeholder="Search labelers..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            disabled={isLoadingLabelers}
          />
        </div>

        {/* Error Message */}
        {error && (
          <div className="p-3 bg-red-50 border border-red-200 rounded-md">
            <p className="text-sm text-red-600">{error}</p>
          </div>
        )}

        {/* Loading State */}
        {isLoadingLabelers ? (
          <div className="flex justify-center py-8">
            <LoadingSpinner />
          </div>
        ) : (
          <>
            {/* Labelers List */}
            {filteredLabelers.length === 0 ? (
              <div className="text-center py-8">
                <p className="text-gray-500">
                  {searchTerm ? 'No labelers found matching your search.' : 'No available labelers to add.'}
                </p>
              </div>
            ) : (
              <div className="max-h-64 overflow-y-auto space-y-2">
                {filteredLabelers.map((labeler) => (
                  <button
                    key={labeler.id}
                    onClick={() => handleSelect(labeler.id)}
                    disabled={isLoading}
                    className="w-full text-left p-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    <span className="text-sm font-medium text-gray-900">
                      {labeler.username}
                    </span>
                  </button>
                ))}
              </div>
            )}
          </>
        )}

        {/* Actions */}
        <div className="flex justify-end space-x-3 pt-4 border-t border-gray-200">
          <Button
            onClick={handleClose}
            variant="secondary"
            disabled={isLoading}
          >
            Cancel
          </Button>
        </div>
      </div>
    </Modal>
  );
}
