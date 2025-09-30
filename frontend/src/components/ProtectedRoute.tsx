'use client';

import { useAuth } from '@/contexts/AuthContext';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

interface ProtectedRouteProps {
  children: React.ReactNode;
  requiredRole?: 'admin' | 'labeler';
  fallbackPath?: string;
}

export default function ProtectedRoute({ 
  children, 
  requiredRole, 
  fallbackPath = '/' 
}: ProtectedRouteProps) {
  const { user, isLoading } = useAuth();
  const router = useRouter();
  const [showError, setShowError] = useState(false);

  useEffect(() => {
    // Don't redirect while loading
    if (isLoading) return;

    // Check if user is authenticated
    if (!user) {
      setShowError(true);
      // Redirect after showing error message
      setTimeout(() => {
        router.push(fallbackPath);
      }, 5000);
      return;
    }

    // Check if user has required role
    if (requiredRole && user.role !== requiredRole) {
      setShowError(true);
      // Redirect after showing error message
      setTimeout(() => {
        router.push(fallbackPath);
      }, 5000);
      return;
    }
  }, [user, isLoading, requiredRole, router, fallbackPath]);

  // Show loading while checking authentication
  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

  // Show error message if unauthorized
  if (showError) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="max-w-md w-full bg-white shadow-lg rounded-lg p-6 text-center">
          <div className="flex justify-center mb-4">
            <svg className="h-16 w-16 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
          </div>
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Access Denied</h2>
          <p className="text-gray-600 mb-4">
            {!user 
              ? 'You must be logged in to access this page.'
              : requiredRole 
                ? `You need ${requiredRole} privileges to access this page.`
                : 'You do not have permission to access this page.'
            }
          </p>
          <p className="text-sm text-gray-500">
            Redirecting to home page in a moment...
          </p>
        </div>
      </div>
    );
  }

  // User is authenticated and has correct role
  return <>{children}</>;
}
