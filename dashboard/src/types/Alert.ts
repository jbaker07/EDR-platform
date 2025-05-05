// src/types/Alert.ts
export interface ProcessInfo {
    endpoint_id: string;          // add this
    pid: number;
    name: string;
    cmd: string;
    cpu_usage: number;            // no longer optional
    memory: number;               // no longer optional
    risk_score: number;
    risk_level: 'low' | 'medium' | 'high';
    history: { timestamp: number; risk_score: number }[];
  }
  
  export interface Alert {
    hostname: string;
    timestamp: number;
    processes: ProcessInfo[];
  }
  