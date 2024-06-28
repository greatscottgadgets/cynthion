====================
Bitstream Generation
====================

Before proceeding, please ensure you have followed the prerequisites in the :doc:`Setting up a Development Environment <introduction>` section.


Cynthion Gateware
-----------------

The Cynthion repository contains gateware for two designs:

- ``analyzer`` -- USB analyzer for using Cynthion with Packetry.
- ``soc``      -- System-on-Chip for using Cynthion with Facedancer.

Bitstreams can be generated from the cynthion python package sub-directory as follows:

Analyzer Gateware
^^^^^^^^^^^^^^^^^

.. code-block:: sh

    # change to the cynthion python package directory
    cd cynthion/python/

    # generate bitstream
    python3 -m cynthion.gateware.analyzer.top

SoC Gateware
^^^^^^^^^^^^

.. code-block:: sh

    # change to the cynthion python package directory
    cd cynthion/python/

    # generate bitstream
    python3 -m cynthion.gateware.soc.top

Additional Options
^^^^^^^^^^^^^^^^^^

Additional options for bitstream generation can be listed by appending ``--help`` to the command:

.. code-block:: text

    $ python3 -m cynthion.gateware.analyzer.top --help

    usage: top.py [-h] [--output filename] [--erase] [--upload] [--flash]
                  [--dry-run] [--keep-files] [--fpga part_number] [--console port]

    Gateware generation/upload script for 'USBAnalyzerApplet' gateware.

    optional arguments:
      -h, --help            show this help message and exit
      --output filename, -o filename
                            Build and output a bitstream to the given file.
      --erase, -E           Clears the relevant FPGA's flash before performing
                            other options.
      --upload, -U          Uploads the relevant design to the target hardware.
                            Default if no options are provided.
      --flash, -F           Flashes the relevant design to the target hardware's
                            configuration flash.
      --dry-run, -D         When provided as the only option; builds the relevant
                            bitstream without uploading or flashing it.
      --keep-files          Keeps the local files in the default `build` folder.
      --fpga part_number    Overrides build configuration to build for a given
                            FPGA. Useful if no FPGA is connected during build.
      --console port        Attempts to open a convenience 115200 8N1 UART console
                            on the specified port immediately after uploading.
