'use client';

import { useState, useCallback, useEffect } from 'react';
import Link from 'next/link';
import { PageHeader, Card, Button } from '@/components';
import { useApiClient, ApiResponse } from '@/utils/api';

// Types for API responses
interface LabelerListResponse {
  labelers: Array<{
    id: number;
    username: string;
    group_ids: number[];
  }>;
  total: number;
}

interface GroupListResponse {
  groups: Array<{
    id: number;
    name: string;
    description: string | null;
  }>;
  total: number;
}

export default function AdminDashboardPage() {
  const apiClient = useApiClient();
  const [isExporting, setIsExporting] = useState(false);
  const [exportError, setExportError] = useState<string | null>(null);
  
  // State for dashboard data
  const [labelerCount, setLabelerCount] = useState<number | null>(null);
  const [groupCount, setGroupCount] = useState<number | null>(null);
  const [isLoadingStats, setIsLoadingStats] = useState(true);
  const [statsError, setStatsError] = useState<string | null>(null);

  // Load dashboard statistics
  const loadDashboardStats = useCallback(async () => {
    try {
      setIsLoadingStats(true);
      setStatsError(null);
      
      // Load labelers and groups in parallel
      const [labelersResponse, groupsResponse] = await Promise.all([
        apiClient.get<ApiResponse<LabelerListResponse>>('/admin/labeler'),
        apiClient.get<ApiResponse<GroupListResponse>>('/admin/groups')
      ]);
      
      let hasError = false;
      
      if (labelersResponse.success && labelersResponse.data?.success && labelersResponse.data.data) {
        setLabelerCount(labelersResponse.data.data.total);
      } else {
        console.error('Failed to load labelers:', labelersResponse.error);
        setLabelerCount(0); // Set to 0 on error instead of leaving as null
        hasError = true;
      }

      if (groupsResponse.success && groupsResponse.data?.success && groupsResponse.data.data) {
        setGroupCount(groupsResponse.data.data.total);
      } else {
        console.error('Failed to load groups:', groupsResponse.error);
        setGroupCount(0); // Set to 0 on error instead of leaving as null
        hasError = true;
      }
      
      if (hasError) {
        setStatsError('Failed to load some dashboard statistics');
      }
    } catch (error) {
      console.error('Error loading dashboard stats:', error);
      setStatsError('Failed to load dashboard statistics');
      setLabelerCount(0); // Set to 0 on error instead of leaving as null
      setGroupCount(0); // Set to 0 on error instead of leaving as null
    } finally {
      setIsLoadingStats(false);
    }
  }, [apiClient]);

  // Load stats on component mount
  useEffect(() => {
    loadDashboardStats();
  }, [loadDashboardStats]);

  const handleBulkExport = useCallback(async () => {
    setIsExporting(true);
    setExportError(null);

    try {
      const response = await apiClient.get('/admin/export/bulk');
      
      if (response.success && response.data) {
        // Create and download the JSON file
        const dataStr = JSON.stringify(response.data, null, 2);
        const dataBlob = new Blob([dataStr], { type: 'application/json' });
        const url = URL.createObjectURL(dataBlob);
        const link = document.createElement('a');
        link.href = url;
        link.download = `image-labeling-export-${new Date().toISOString().split('T')[0]}.json`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);
      } else {
        setExportError(response.error || 'Failed to export data');
      }
    } catch (error) {
      console.error('Export error:', error);
      setExportError('Failed to export data. Please try again.');
    } finally {
      setIsExporting(false);
    }
  }, [apiClient]);

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center mb-6">
          <PageHeader 
            title="Admin Dashboard" 
            description="Manage your image labeling platform." 
          />
          <div className="mt-4 sm:mt-0">
            <Button
              onClick={handleBulkExport}
              disabled={isExporting}
              className="w-full sm:w-auto"
            >
              {isExporting ? (
                <>
                  <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  Exporting...
                </>
              ) : (
                <>
                  <svg className="h-5 w-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  Bulk Export
                </>
              )}
            </Button>
          </div>
        </div>

        {exportError && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-md">
            <div className="flex">
              <div className="flex-shrink-0">
                <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                </svg>
              </div>
              <div className="ml-3">
                <h3 className="text-sm font-medium text-red-800">Export Error</h3>
                <div className="mt-2 text-sm text-red-700">
                  <p>{exportError}</p>
                </div>
              </div>
            </div>
          </div>
        )}

        <div className="flex justify-center">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl">
          {/* Manage Labelers Card */}
          <Link href="/admin/labelers">
            <Card hover className="overflow-hidden">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <svg className="h-8 w-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                  </svg>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Manage Labelers</dt>
                    <dd className="text-lg font-medium text-gray-900">User Management</dd>
                  </dl>
                </div>
              </div>
            </div>
            <div className="bg-gray-50 px-5 py-3">
              <div className="text-sm">
                <span className="text-gray-600">Create accounts, assign groups, and manage labeler access</span>
              </div>
            </div>
            </Card>
          </Link>

          {/* Manage Groups Card */}
          <Link href="/admin/groups">
            <Card hover className="overflow-hidden">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <svg className="h-8 w-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                  </svg>
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Manage Groups</dt>
                    <dd className="text-lg font-medium text-gray-900">Group Management</dd>
                  </dl>
                </div>
              </div>
            </div>
            <div className="bg-gray-50 px-5 py-3">
              <div className="text-sm">
                <span className="text-gray-600">Create and organize labeling groups for different tasks</span>
              </div>
            </div>
            </Card>
          </Link>

          </div>
        </div>

        {/* Quick Stats */}
        <div className="mt-8">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Quick Stats</h2>
          
          {statsError && (
            <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
              <div className="flex">
                <div className="flex-shrink-0">
                  <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                  </svg>
                </div>
                <div className="ml-3">
                  <h3 className="text-sm font-medium text-red-800">Stats Error</h3>
                  <div className="mt-2 text-sm text-red-700">
                    <p>{statsError}</p>
                  </div>
                </div>
              </div>
            </div>
          )}
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <svg className="h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">Total Labelers</dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {labelerCount === null ? (
                          <div className="animate-pulse bg-gray-200 h-6 w-8 rounded"></div>
                        ) : (
                          labelerCount
                        )}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-white overflow-hidden shadow rounded-lg">
              <div className="p-5">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <svg className="h-6 w-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                  </div>
                  <div className="ml-5 w-0 flex-1">
                    <dl>
                      <dt className="text-sm font-medium text-gray-500 truncate">Total Groups</dt>
                      <dd className="text-lg font-medium text-gray-900">
                        {groupCount === null ? (
                          <div className="animate-pulse bg-gray-200 h-6 w-8 rounded"></div>
                        ) : (
                          groupCount
                        )}
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
