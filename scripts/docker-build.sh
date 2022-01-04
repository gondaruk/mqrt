#!/usr/bin/env bash
# Taken from https://github.com/vectordotdev/vector/blob/master/scripts/build-docker.sh

set -euox pipefail

VERSION="$(scripts/version.sh)"
PLATFORM="${PLATFORM:-}"
DOCKER_PUSH="${DOCKER_PUSH:-"true"}"
DOCKER_USE_CACHE="${DOCKER_USE_CACHE:-"false"}"
DOCKER_REGISTRY="${DOCKER_REGISTRY:-"gondaruk/mqrt"}"

build() {
  local BASE="$1"
  local VERSION="$2"

  local TAG="$DOCKER_REGISTRY:$VERSION-$BASE"
  local DOCKERFILE="distribution/docker/$BASE/Dockerfile"

  if [ -n "$PLATFORM" ]; then
    ARGS=()
    if [[ "$DOCKER_PUSH" == "true" ]]; then
      ARGS+=(--push)
    fi
    if [[ "$DOCKER_USE_CACHE" == "false" ]]; then
      ARGS+=(--no-cache)
    fi

    docker buildx build \
      --platform="$PLATFORM" \
      --tag "$TAG" \
      target/artifacts \
      -f "$DOCKERFILE" \
      "${ARGS[@]}"
  else
    ARGS=()
    if [[ "$DOCKER_USE_CACHE" == "false" ]]; then
      ARGS+=(--no-cache)
    fi

    docker build \
      --tag "$TAG" \
      target/artifacts \
      -f "$DOCKERFILE" \
      "${ARGS[@]}"


    if [[ "$DOCKER_PUSH" == "true" ]]; then
      docker push "$TAG"
    fi
  fi
}

#
# Build
#
main() {
  echo "Building $DOCKER_REGISTRY:* Docker images"

  VERSION_EXACT="$VERSION"
  # shellcheck disable=SC2001
  VERSION_MINOR_X=$(echo "$VERSION" | sed 's/\.[0-9]*$/.X/g')
  # shellcheck disable=SC2001
  VERSION_MAJOR_X=$(echo "$VERSION" | sed 's/\.[0-9]*\.[0-9]*$/.X/g')

  for VERSION_TAG in "$VERSION_EXACT" "$VERSION_MINOR_X" "$VERSION_MAJOR_X" latest; do
    build alpine "$VERSION_TAG"
    build debian "$VERSION_TAG"
    build distroless-static "$VERSION_TAG"
    build distroless-libc "$VERSION_TAG"
  done
}

main
