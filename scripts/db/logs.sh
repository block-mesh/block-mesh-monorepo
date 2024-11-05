#!/bin/bash
set -x
heroku logs -p heroku-postgres --tail --app block-mesh-manager
