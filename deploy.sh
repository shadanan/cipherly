#!/bin/bash
set -e

# Set the project
gcloud config set project cipherly

# Upload to Google Container Registry
gcloud builds submit --tag gcr.io/cipherly/cipherly

# Deploy
if [ "$1" = "--prod" ]; then
  gcloud run deploy cipherly --image gcr.io/cipherly/cipherly --platform managed --region us-west1 --allow-unauthenticated
else
  gcloud run deploy cipherly-staging --image gcr.io/cipherly/cipherly --platform managed --region us-west1 --allow-unauthenticated
fi
