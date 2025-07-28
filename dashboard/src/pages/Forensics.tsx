import React from 'react';
import SessionTimelinePanel from '@/components/SessionTimelinePanel';
import IncidentTimelinePanel from '@/components/IncidentTimelinePanel';
// import TelemetryTimeline from '@/components/TelemetryTimeline'; (commented out for now)

export default function Forensics() {
  return (
    <>
      <SessionTimelinePanel />
      <IncidentTimelinePanel />
      {/* <TelemetryTimeline /> */}
    </>
  );
}
