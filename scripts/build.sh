#!/bin/bash

set -eu

docker buildx build --load -t textvid-api -f docker/textvid-api.Dockerfile .
