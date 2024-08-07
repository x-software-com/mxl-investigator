#!/usr/bin/env python3
"""Script to install tools and build VCPKG third party libraries"""

import argparse
import os
import subprocess
import pathlib

from scripts.shared.vcpkg import setup_vcpkg
from scripts.shared.setup import setup_tools, setup_write_mxl_env


def setup():
    """Parse command line and call setup functions"""
    parser = argparse.ArgumentParser(description='Setup mxl environment')
    parser.add_argument('--ci', dest='ci', action=argparse.BooleanOptionalAction, default=False, help='Setup for CI pipeline')
    parser.add_argument('--vcpkg', dest='vcpkg', action=argparse.BooleanOptionalAction, default=True, help='Setup vcpkg')
    options = parser.parse_args()

    setup_tools(options.ci)
    if options.vcpkg:
        setup_vcpkg('mxl-investigator', '2024.05.24')
    setup_write_mxl_env()


if __name__ == "__main__":
    setup()
