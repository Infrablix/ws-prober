#!/bin/bash

MODE=$1
TAGS=$2

if [ -z "$MODE" ] || [ -z "$TAGS" ]; then
  echo "Usage: $0 <publish|build> <tag1,tag2>"
  exit 1
fi

export REGISTRY="europe-west3-docker.pkg.dev"
export REPOSITORY="infrablix/images"
export IMAGE_TAGS="$(echo -ne "$TAGS" | sed "s/[^a-zA-Z0-9\n,]/-/g")"
export GIT_COMMIT="$(git rev-parse HEAD)"
export GIT_DATE="$(git show -s --format='%ct')"
export GIT_VERSION="untagged"

echo "Setting GIT_VERSION=$GIT_VERSION"

# Create, start (bootstrap) and use a *named* docker builder
# This allows us to cross-build multi-platform,
# and naming allows us to use the DLC (docker-layer-cache)
docker buildx create --driver=docker-container --name=buildx-build --bootstrap --use

DOCKER_OUTPUT_DESTINATION=""
if [ "$MODE" == "publish" ]; then
  export PLATFORMS="linux/amd64,linux/arm64"

  gcloud auth configure-docker $REGISTRY
  echo "Building for platforms $PLATFORMS and then publishing to registry"
  DOCKER_OUTPUT_DESTINATION="--push"
else
  export PLATFORMS="linux/arm64"

  echo "Running single-platform $PLATFORMS build and loading into docker"
  DOCKER_OUTPUT_DESTINATION="--load"
fi

# Let them cook!
docker buildx bake \
  --progress plain \
  --builder=buildx-build \
  -f docker-bake.hcl \
  $DOCKER_OUTPUT_DESTINATION \
  ws-prober