from fastapi import FastAPI
from app.routes import telemetry

app = FastAPI()

app.include_router(telemetry.router)

