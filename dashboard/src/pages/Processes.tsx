import React, { useState } from 'react';
import ProcessTable from '@/components/ProcessTable';
import SOCAssistantPanel from '@/components/SOCAssistantPanel';

export default function Processes() {
  const [selectedAlertId, setSelectedAlertId] = useState<string | null>(null);

  return (
    <div>
      <ProcessTable onAlertSelect={setSelectedAlertId} />
      {selectedAlertId && <SOCAssistantPanel alertId={selectedAlertId} />}
    </div>
  );
}
