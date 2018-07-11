#!/bin/bash
SSH_PRIVATE_KEY=`cat scripts/ssh_private_key_no_passphrase`
cd docker/hub
DOCKER_BUILD_TAG=$1
echo "Docker build tag: " $DOCKER_BUILD_TAG
docker build --build-arg BUILD_TAG=$DOCKER_BUILD_TAG --build-arg SSH_PRIVATE_KEY="$SSH_PRIVATE_KEY" --no-cache=true --tag parity/parity:$DOCKER_BUILD_TAG .
docker run -it parity/parity:$DOCKER_BUILD_TAG -v

