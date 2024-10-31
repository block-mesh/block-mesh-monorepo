#!/usr/bin/env bash
set -x
export APP="block-mesh-manager"

function query() {
  local command=$1
  mkdir -p db_logs
  heroku "pg:${command}" --app ${APP} > db_logs/"${command}.txt"
}

commands_array=("bloat" "blocking" "cache-hit" "calls" "diagnose" "extensions" "index-size" "index-usage" "locks" "long-running-queries" "maintenance" "outliers" "seq-scans" "table-indexes-size" "table-size" "unused-indexes" "vacuum-stats")

for element in "${commands_array[@]}"
do
    query "$element"
done


