#!/bin/sh

set -eu

if [[ -z ${AWS_LAMBDA_RUNTIME_API:-} ]]; then
    exec /aws-lambda-rie /textvid_api $@
else
    exec /textvid_api $@
fi
