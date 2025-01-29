#
# This file is part of Cynthion.
#

"""
The values in this module are generated at runtime from the
corresponding TOML files located in the `cynthion.git/host/cynthion/shared/`
directory.
"""

import glob
import tomli

from collections          import namedtuple
from importlib.resources  import files
from os                   import path
from pathlib              import Path


SHARED_TOML_PATH = Path(files("cynthion").joinpath("shared"))


def generate_module_values():
    for toml in SHARED_TOML_PATH.glob("*.toml"):
        with toml.open("rb") as f:
            globals()[toml.stem] = _dict_to_namedtuple(tomli.load(f))


def _dict_to_namedtuple(data, typename="_"):
    return namedtuple(typename, data.keys())(
        *(_dict_to_namedtuple(v, typename + '_' + k) if isinstance(v, dict) else v for k, v in data.items())
    )

generate_module_values()
