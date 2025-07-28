import axios from "axios";

interface ProcessInput {
  pid: number;
  memory: number;
  cpu_usage: number;
}

export async function scoreProcesses(processes: ProcessInput[]) {
  try {
    const response = await axios.post("http://127.0.0.1:8000/api/score", processes);
    return response.data;
  } catch (error) {
    console.error("Error scoring processes:", error);
    return { status: "error", detail: "Request failed" };
  }
}
