'use client';

import { ReactNode } from 'react';
import ProtectedRoute from '@/components/ProtectedRoute';

interface LabelerLayoutProps {
  children: ReactNode;
}

export default function LabelerLayout({ children }: LabelerLayoutProps) {
  return (
    <ProtectedRoute requiredRole="labeler">
      {children}
    </ProtectedRoute>
  );
}
