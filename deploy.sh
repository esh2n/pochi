#!bin/bash
source ./.env

SHORT_SHA=$(git rev-parse --short HEAD) # need unique tag
gcloud builds submit --substitutions _SECRET_TOKEN=$SECRET_TOKEN,SHORT_SHA=$SHORT_SHA