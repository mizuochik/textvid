#!/bin/sh

set -eu

if [[ -z ${AWS_LAMBDA_RUNTIME_API:-} ]]; then
    exec aws-lambda-rie /app/textvid_api $@
else
    exec /app/textvid_api $@
fi
