'use client';

import Link from 'next/link';
import { useAuth } from '@/contexts/AuthContext';

export default function Navbar() {
  const { user, logout, isLoading } = useAuth();

  if (isLoading) {
    return (
      <nav className="bg-white shadow-lg border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex-shrink-0">
              <Link 
                href="/" 
                className="flex items-center space-x-2 text-gray-800 hover:text-blue-600 transition-colors duration-200"
              >
                <svg 
                  className="h-8 w-8" 
                  fill="none" 
                  stroke="currentColor" 
                  viewBox="0 0 24 24"
                >
                  <path 
                    strokeLinecap="round" 
                    strokeLinejoin="round" 
                    strokeWidth={2} 
                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" 
                  />
                  <path 
                    strokeLinecap="round" 
                    strokeLinejoin="round" 
                    strokeWidth={2} 
                    d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" 
                  />
                </svg>
                <span className="font-bold text-xl">Image Labeling</span>
              </Link>
            </div>
            <div className="text-gray-500">Loading...</div>
          </div>
        </div>
      </nav>
    );
  }

  return (
    <nav className="bg-white shadow-lg border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Home Button - Left Side */}
          <div className="flex-shrink-0">
            <Link 
              href="/" 
              className="flex items-center space-x-2 text-gray-800 hover:text-blue-600 transition-colors duration-200"
            >
              <svg 
                className="h-8 w-8" 
                fill="none" 
                stroke="currentColor" 
                viewBox="0 0 24 24"
              >
                <path 
                  strokeLinecap="round" 
                  strokeLinejoin="round" 
                  strokeWidth={2} 
                  d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" 
                />
                <path 
                  strokeLinecap="round" 
                  strokeLinejoin="round" 
                  strokeWidth={2} 
                  d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" 
                />
              </svg>
              <span className="font-bold text-xl">Image Labeling</span>
            </Link>
          </div>

          {/* Navigation Items - Right Side */}
          <div className="flex items-center space-x-4">
            {!user ? (
              // Not logged in - Show Login button
              <Link
                href="/login"
                className="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-gray-50"
              >
                Login
              </Link>
            ) : user.role === 'admin' ? (
              // Admin navigation
              <>
                <Link
                  href="/admin/labelers"
                  className="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-gray-50"
                >
                  Manage Labelers
                </Link>
                <Link
                  href="/admin/groups"
                  className="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-gray-50"
                >
                  Manage Groups
                </Link>
                <span className="text-gray-500 text-sm">Welcome, {user.username}</span>
                <button
                  onClick={logout}
                  className="text-gray-700 hover:text-red-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-red-50"
                >
                  Logout
                </button>
              </>
            ) : (
              // Labeler navigation
              <>
                <Link
                  href="/labeler/groups"
                  className="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-gray-50"
                >
                  My Groups
                </Link>
                <span className="text-gray-500 text-sm">Welcome, {user.username}</span>
                <button
                  onClick={logout}
                  className="text-gray-700 hover:text-red-600 px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 hover:bg-red-50"
                >
                  Logout
                </button>
              </>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
}
