#!/bin/bash
set -e
usbhub --disable-i2c --hub D9D1 power state --port 3 --reset
sleep 1s
source ci_env/bin/activate
python ci/interactive-test.py
deactivate
