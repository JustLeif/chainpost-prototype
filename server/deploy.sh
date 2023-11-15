#!/bin/bash

# Docker build and push
docker build -t us-central1-docker.pkg.dev/chainpost/chainpost/chainpost:latest .
docker push us-central1-docker.pkg.dev/chainpost/chainpost/chainpost:latest .

# Deploy to Cloud Run
gcloud run deploy chainpost \
  --image us-central1-docker.pkg.dev/chainpost/chainpost/chainpost:latest \
  --region us-central1 \
  --platform managed \
  --allow-unauthenticated

echo "Deployment complete."