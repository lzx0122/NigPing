#!/bin/bash

# Usage: ./publish.sh <docker-hub-username>
# Example: ./publish.sh myuser

if [ -z "$1" ]; then
    echo "Usage: ./publish.sh <docker-hub-username>"
    exit 1
fi

USERNAME=$1
IMAGE_NAME="nigping-vps-agent"
TAG="latest"

FULL_IMAGE="$USERNAME/$IMAGE_NAME:$TAG"

echo "Building Docker Image: $FULL_IMAGE..."
docker build -t $FULL_IMAGE .

echo "Pushing to Docker Hub..."
docker push $FULL_IMAGE

echo "Done!"
echo "Now you can check your Docker Hub repository."
echo "On your VPS, set DOCKER_IMAGE_NAME=$FULL_IMAGE in your .env file."
