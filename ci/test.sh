#!/bin/bash
set -e
hubs cynthion reset
sleep 1s
source ci_env/bin/activate
cynthion build selftest
deactivate
