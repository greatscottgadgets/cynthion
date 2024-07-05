#!/bin/bash
set -e
python3.11 -m venv ci_env
source ci_env/bin/activate
pip install --upgrade cynthion/python/.
deactivate
