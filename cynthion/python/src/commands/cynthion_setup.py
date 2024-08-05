#!/usr/bin/env python
#
# This file is part of Cynthion.
#
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

""" Cynthion 'setup' command. """

import logging, os, shutil, subprocess, sys

from .util import find_cynthion_asset

UDEV_PATH     = "/etc/udev/rules.d"
UDEV_CYNTHION = "54-cynthion.rules"


def cynthion_setup(args):
    if args.check:
        _check_udev(args)
    elif args.uninstall:
        _uninstall_udev(args)
    else:
        _install_udev(args)


def _check_udev(args):
    logging.info("Checking: Linux udev rules")

    sys_path = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    if not os.path.isfile(sys_path):
        logging.error(f"❌\t{UDEV_CYNTHION} not installed")
        logging.info("\nPlease run 'cynthion setup' to install.")
        sys.exit(1)

    logging.info(f"✅\t{UDEV_CYNTHION} is present")

    sys_rules     = open(sys_path, "r").readlines()
    factory_rules = open(find_cynthion_asset(UDEV_CYNTHION), "r").readlines()

    if sys_rules != factory_rules:
        logging.error(f"❌\t{UDEV_CYNTHION} differs from factory rules")
        logging.info("\nPlease run 'cynthion setup' to re-install.")
        sys.exit(1)

    logging.info(f"✅\t{UDEV_CYNTHION} matches factory rules")
    logging.info("\nAll checks completed successfully.")


def _install_udev(args):
    logging.info("Installing: Linux udev rules")

    src = find_cynthion_asset(UDEV_CYNTHION)
    dst = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    # copy udev rules file
    _run_shell_command(f"cp {src} {dst}", root=True)

    # reload udev rules
    _run_shell_command("udevadm control --reload", root=True)

    # apply udev rules to any devices that are already plugged in
    _run_shell_command("udevadm trigger", root=True)

    logging.info("\nInstallation completed successfully.")


def _uninstall_udev(args):
    logging.info("Uninstalling: Linux udev rules")

    rules = os.path.join(UDEV_PATH, UDEV_CYNTHION)

    if os.path.isfile(rules):
        # remove udev rules file
        _run_shell_command(f"rm {rules}", root=True)

        # reload udev rules
        _run_shell_command("udevadm control --reload", root=True)

        # apply udev rules to any devices that are already plugged in
        _run_shell_command("udevadm trigger", root=True)
    else:
        logging.info(f"✅\t{rules} not present, skipping.")

    logging.info("\nUninstallation completed successfully.")


def _run_shell_command(cmd, root=False):

    if root and os.getuid() != 0:
        SUDO_PATH = shutil.which('sudo')
        if SUDO_PATH is None:
            raise OSError('Cannot find sudo executable.')
        cmd = f"sudo {cmd}"
        
    proc = subprocess.Popen(args=cmd.split())
    proc.wait()

    if proc.returncode != 0:
        logging.error(f"❌\t{cmd} failed with exit code: {proc.returncode}")
        sys.exit(proc.returncode)

    logging.info(f"✅\t{cmd}")
