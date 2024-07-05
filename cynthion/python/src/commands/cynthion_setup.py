#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'setup' command. """

import logging, os, shutil, subprocess, sys

from .util import find_cynthion_asset

UDEV_PATH     = f"/etc/udev/rules.d"
UDEV_CYNTHION = "54-cynthion.rules"


def cynthion_setup(device, args):
    if args.check:
        _check_udev(device, args)
    elif args.uninstall:
        _uninstall_udev(device, args)
    else:
        _install_udev(device, args)


def _check_udev(device, args):
    logging.info("Checking: Linux udev rules")

    sys_path = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    if not os.path.isfile(sys_path):
        logging.error(f"❌\t{UDEV_CYNTHION} not installed")
        logging.info(f"\nPlease run 'sudo cynthion setup' to install.")
        sys.exit(1)

    logging.info(f"✅\t{UDEV_CYNTHION} is present")

    sys_rules     = open(sys_path, "r").readlines()
    factory_rules = open(find_cynthion_asset(UDEV_CYNTHION), "r").readlines()

    if sys_rules != factory_rules:
        logging.error(f"❌\t{UDEV_CYNTHION} differs from factory rules")
        logging.info(f"\nPlease run 'sudo cynthion setup' to re-install.")
        sys.exit(1)

    logging.info(f"✅\t{UDEV_CYNTHION} matches factory rules")
    logging.info(f"\nAll checks completed successfully.")


def _install_udev(device, args):
    logging.info("Installing: Linux udev rules")

    if os.getuid() != 0:
        logging.error(f"❌\tRoot privileges are required for installation")
        logging.info(f"\nPlease run 'sudo cynthion setup' to install.")
        sys.exit(1)

    src = find_cynthion_asset(UDEV_CYNTHION)
    dst = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    # copy udev rules file
    try:
        shutil.copyfile(src, dst)
    except Exception as e:
        logging.error(f"❌\t{e}")
        sys.exit(1)

    logging.info(f"✅\tcp {UDEV_CYNTHION} {dst}")

    # reload udev rules
    _run_shell_command("udevadm control --reload")

    # apply udev rules to any devices that are already plugged in
    _run_shell_command("udevadm trigger")

    logging.info(f"\nInstallation completed successfully.")


def _uninstall_udev(device, args):
    logging.info("Uninstalling: Linux udev rules")

    if os.getuid() != 0:
        logging.error(f"❌\tRoot privileges are required for uninstallation")
        logging.info(f"\nPlease run 'sudo cynthion setup' to install.")
        sys.exit(1)

    rules = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    if os.path.isfile(rules):
        # remove udev rules file
        try:
            os.remove(rules)
        except Exception as e:
            logging.error(f"❌\t{e}")
            sys.exit(1)

        logging.info(f"✅\trm {rules}")

        # reload udev rules
        _run_shell_command("udevadm control --reload")

        # apply udev rules to any devices that are already plugged in
        _run_shell_command("udevadm trigger")
    else:
        logging.info(f"✅\t{rules} not present, skipping.")

    logging.info(f"\nUninstallation completed successfully.")


def _run_shell_command(cmd):
    proc = subprocess.Popen(args=cmd.split())
    proc.wait()

    if proc.returncode != 0:
        logging.error(f"❌\t{cmd} failed with exit code: {proc.returncode}")
        sys.exit(proc.returncode)

    logging.info(f"✅\t{cmd}")
