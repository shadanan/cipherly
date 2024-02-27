FROM python:3.12-slim

ENV PYTHONUNBUFFERED True
ENV PORT 8000

WORKDIR /app

COPY backend/dist/cipherly-*.whl /app/
RUN pip install --no-cache-dir cipherly-*.whl

COPY frontend/build /app/frontend

CMD uvicorn cipherly:app --host 0.0.0.0 --port ${PORT}
