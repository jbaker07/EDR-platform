from fastapi import FastAPI
from .routes import telemetry  # only if telemetry.py exists

app = FastAPI()

@app.get("/")
def root():
    return {"message": "EDR Backend API is running"}

# Optional: only if you have telemetry.py already
app.include_router(telemetry.router)

