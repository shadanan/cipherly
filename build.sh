#!/bin/bash
set -e

# Build the backend
pushd backend && poetry build && popd

# Build the frontend
pushd frontend && npm install && npm run build && popd

# Build the docker image
docker build -t gcr.io/cipherly/cipherly .
