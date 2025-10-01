'use client';

import { useState, useRef } from 'react';
import Image from 'next/image';
import Button from './Button';
import Modal from './Modal';

interface ImageUploaderProps {
  onImagesSelected: (files: File[]) => void;
  onUploadError: (error: string) => void;
  isUploading?: boolean;
  className?: string;
}

export function ImageUploader({ 
  onImagesSelected, 
  onUploadError, 
  isUploading = false,
  className = '' 
}: ImageUploaderProps) {
  const [selectedFiles, setSelectedFiles] = useState<File[]>([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [previewUrls, setPreviewUrls] = useState<string[]>([]);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const validateFile = (file: File): boolean => {
    const allowedTypes = ['image/png', 'image/jpeg', 'image/jpg'];
    const maxSize = 10 * 1024 * 1024; // 10MB

    if (!allowedTypes.includes(file.type)) {
      onUploadError(`File "${file.name}" is not a valid image type. Only PNG and JPEG files are allowed.`);
      return false;
    }

    if (file.size > maxSize) {
      onUploadError(`File "${file.name}" is too large. Maximum size is 10MB.`);
      return false;
    }

    return true;
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files || []);
    const validFiles: File[] = [];
    const urls: string[] = [];

    // Validate each file and create preview URLs
    for (const file of files) {
      if (validateFile(file)) {
        validFiles.push(file);
        urls.push(URL.createObjectURL(file));
      }
    }

    if (validFiles.length > 0) {
      setSelectedFiles(validFiles);
      setPreviewUrls(urls);
      setIsModalOpen(true);
    }

    // Reset the input
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleClick = () => {
    fileInputRef.current?.click();
  };

  const handleConfirmSelection = () => {
    onImagesSelected(selectedFiles);
    setIsModalOpen(false);
  };

  const handleCancelSelection = () => {
    // Clean up preview URLs
    previewUrls.forEach(url => URL.revokeObjectURL(url));
    setSelectedFiles([]);
    setPreviewUrls([]);
    setIsModalOpen(false);
  };

  const removeFile = (index: number) => {
    const newFiles = selectedFiles.filter((_, i) => i !== index);
    const newUrls = previewUrls.filter((_, i) => i !== index);
    
    // Clean up the removed URL
    URL.revokeObjectURL(previewUrls[index]);
    
    setSelectedFiles(newFiles);
    setPreviewUrls(newUrls);
  };

  return (
    <div className={className}>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/png,image/jpeg,image/jpg"
        multiple
        onChange={handleFileChange}
        className="hidden"
      />
      
      <Button
        onClick={handleClick}
        disabled={isUploading}
        fullWidth={false}
        className="w-full sm:w-auto"
      >
        <svg className="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
        </svg>
        {isUploading ? 'Uploading...' : 'Select Images'}
      </Button>

      <Modal
        isOpen={isModalOpen}
        onClose={handleCancelSelection}
        title="Select Images to Upload"
        className="sm:max-w-4xl"
      >
        <div className="space-y-4">
          {/* File Preview Grid */}
          {previewUrls.length > 0 && (
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4 max-h-96 overflow-y-auto">
              {previewUrls.map((url, index) => (
                <div key={index} className="relative group">
                  <div className="aspect-square bg-gray-100 rounded-lg overflow-hidden">
                    <Image
                      src={url}
                      alt={`Preview ${index + 1}`}
                      width={200}
                      height={200}
                      className="w-full h-full object-cover"
                    />
                  </div>
                  <div className="absolute top-2 right-2">
                    <button
                      onClick={() => removeFile(index)}
                      className="bg-red-500 text-white rounded-full p-1 opacity-0 group-hover:opacity-100 transition-opacity"
                      aria-label="Remove image"
                    >
                      <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                      </svg>
                    </button>
                  </div>
                  <div className="mt-1 text-xs text-gray-600 truncate">
                    {selectedFiles[index].name}
                  </div>
                  <div className="text-xs text-gray-500">
                    {(selectedFiles[index].size / 1024 / 1024).toFixed(1)}MB
                  </div>
                </div>
              ))}
            </div>
          )}

          {/* File Selection Info */}
          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <span className="text-sm font-medium text-gray-700">
              {selectedFiles.length} file{selectedFiles.length !== 1 ? 's' : ''} selected
            </span>
            <div className="text-xs text-gray-500">
              Only PNG and JPEG files up to 10MB each
            </div>
          </div>

          {/* Action Buttons */}
          <div className="flex justify-end space-x-3">
            <Button
              onClick={handleCancelSelection}
              variant="secondary"
              disabled={isUploading}
            >
              Cancel
            </Button>
            <Button
              onClick={handleConfirmSelection}
              disabled={selectedFiles.length === 0 || isUploading}
            >
              Select {selectedFiles.length} Image{selectedFiles.length !== 1 ? 's' : ''}
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
