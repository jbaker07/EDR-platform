from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import os
import signal

app = FastAPI()

# CORS middleware to allow frontend requests
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.post("/api/kill/{pid}")
async def kill_process(pid: int):
    try:
        os.kill(pid, signal.SIGKILL)
        return {"message": f"Process {pid} killed successfully"}
    except Exception as e:
        return {"error": str(e)}

@app.post("/api/quarantine/{pid}")
async def quarantine_process(pid: int):
    try:
        # Just a placeholder – add real logic to move/quarantine binary or block access
        return {"message": f"Process {pid} would be quarantined"}
    except Exception as e:
        return {"error": str(e)}

@app.post("/api/extract-logs/{pid}")
async def extract_logs(pid: int):
    try:
        # Placeholder – simulate log extraction
        return {"message": f"Logs extracted for process {pid}"}
    except Exception as e:
        return {"error": str(e)}
