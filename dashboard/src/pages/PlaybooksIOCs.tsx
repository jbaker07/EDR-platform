import React, { Suspense } from 'react';
import PlaybookPanel from '@/components/PlaybookPanel';
import IOCViewerPanel from '@/components/IOCViewerPanel';

export default function PlaybooksIOCsPage() {
  return (
    <>
      <PlaybookPanel />
      <Suspense fallback={<div>Loading IOCs...</div>}>
        <IOCViewerPanel />
      </Suspense>
    </>
  );
}
