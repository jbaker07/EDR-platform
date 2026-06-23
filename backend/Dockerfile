FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

# Copy the backend source into the image (was missing — only requirements were
# copied, so the app code never made it in).
COPY . .

# The app entrypoint is backend/main.py -> module `main`, not `app.main`.
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]

