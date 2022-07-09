#!/bin/sh

set -eu

if [[ -z ${AWS_LAMBDA_RUNTIME_API:-} ]]; then
    exec aws-lambda-rie /app/textvid-api $@
else
    exec /app/textvid-api $@
fi
