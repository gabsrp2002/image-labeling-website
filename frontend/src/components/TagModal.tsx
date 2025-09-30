'use client';

import { useState } from 'react';
import Modal from './Modal';
import FormInput from './FormInput';
import Button from './Button';

interface TagModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (tagData: { name: string; description: string }) => Promise<void>;
  isLoading?: boolean;
  initialData?: { name: string; description: string };
  title?: string;
}

export function TagModal({
  isOpen,
  onClose,
  onSubmit,
  isLoading = false,
  initialData = { name: '', description: '' },
  title = 'Add Tag'
}: TagModalProps) {
  const [formData, setFormData] = useState({
    name: initialData.name,
    description: initialData.description,
  });
  const [errors, setErrors] = useState<{ [key: string]: string }>({});

  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const validateForm = () => {
    const newErrors: { [key: string]: string } = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Tag name is required';
    } else if (formData.name.trim().length < 2) {
      newErrors.name = 'Tag name must be at least 2 characters';
    } else if (formData.name.trim().length > 50) {
      newErrors.name = 'Tag name must be less than 50 characters';
    }

    if (formData.description && formData.description.length > 200) {
      newErrors.description = 'Description must be less than 200 characters';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validateForm()) {
      return;
    }

    try {
      await onSubmit({
        name: formData.name.trim(),
        description: formData.description.trim() || '',
      });
      
      // Reset form on successful submission
      setFormData({ name: '', description: '' });
      setErrors({});
    } catch (error) {
      console.error('Error submitting tag:', error);
    }
  };

  const handleClose = () => {
    setFormData({ name: '', description: '' });
    setErrors({});
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={handleClose} title={title}>
      <form onSubmit={handleSubmit} className="space-y-4">
        <FormInput
          label="Tag Name"
          type="text"
          value={formData.name}
          onChange={(value) => handleInputChange('name', value)}
          placeholder="Enter tag name"
          error={errors.name}
          required
          disabled={isLoading}
        />
        
        <FormInput
          label="Description (Optional)"
          type="textarea"
          value={formData.description}
          onChange={(value) => handleInputChange('description', value)}
          placeholder="Enter tag description"
          error={errors.description}
          disabled={isLoading}
          rows={3}
        />
        
        <div className="flex justify-end space-x-3 pt-4">
          <Button
            type="button"
            variant="secondary"
            onClick={handleClose}
            disabled={isLoading}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            disabled={isLoading}
          >
            {isLoading ? 'Saving...' : 'Save Tag'}
          </Button>
        </div>
      </form>
    </Modal>
  );
}
