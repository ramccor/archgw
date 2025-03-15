#!/bin/bash
set -eu

# for demo in currency_exchange hr_agent
for demo in currency_exchange weather_forecast
do
  echo "******************************************"
  echo "Running tests for $demo ..."
  echo "****************************************"
  cd ../../samples_python/$demo
  echo "starting archgw"
  archgw up arch_config.yaml
  echo "starting docker containers"
  docker compose up -d 2>&1 > /dev/null
  echo "starting hurl tests"
  pwd
  ls
  hurl --test hurl_tests
  echo "stopping docker containers and archgw"
  archgw down
  docker compose down -v
  cd ../../shared/test_runner
done
