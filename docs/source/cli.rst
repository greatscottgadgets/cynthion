=======================================
The ``cynthion`` command line interface
=======================================

.. code:: text

    $ apollo
    usage: cynthion [-h] command ...

    Cynthion command line interface

    positional arguments:
      command
        run       run a bitstream on the FPGA
        flash     overwrite the FPGA's configuration flash with the target bitstream
        update    update MCU firmware and FPGA configuration flash to the latest
                  installed versions
        info      print device information
        setup     install Cynthion support files required for operation (Linux only)

    optional arguments:
      -h, --help  show this help message and exit


Command Documentation
---------------------

Display Cynthion Information
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Display Cynthion bitstream information:

.. code-block :: sh

    cynthion info

Display Cynthion Microcontroller information:

.. code-block :: sh

    cynthion info --force-offline

.. note::

    Once you have switched to the Cynthion Microcontroller by pressing the PROGRAM button or
    the ``--force-offline`` option you will need to press the RESET button to return control
    to the FPGA.


Set up Cynthion
^^^^^^^^^^^^^^^^

Check that your host environment is set up for Cynthion:

.. code-block :: sh

    cynthion setup --check

Set up your host environment for Cynthion:

.. code-block :: sh

    cynthion setup

Remove all files installed during set up:

.. code-block :: sh

    cynthion setup --uninstall


Update Cynthion
^^^^^^^^^^^^^^^

Update both the Cynthion Debug Microcontroller firmware and USB Analyzer bitstream to the latest installed factory versions:

.. code-block :: sh

    cynthion update

Update Cynthion Debug Microcontroller firmware to the latest installed factory version:

.. code-block :: sh

    cynthion update --mcu-firmware

Update Cynthion USB Analyzer bitstream to the latest installed factory version:

.. code-block :: sh

    cynthion update --bitstream


Run bitstream
^^^^^^^^^^^^^

Runs the given factory bitstream on the FPGA:

.. code-block :: sh

    cynthion run <analyzer|facedancer|selftest>

Runs the bitstream specified by ``<filename>`` on the FPGA.

.. code-block :: sh

    cynthion run --bitstream <filename>


Flash firmware and bitstreams
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Overwrite the FPGA's default bitstream with the given factory bitstream:

.. code-block :: sh

    cynthion flash <analyzer|facedancer>

Overwrite the FPGA's default bitstream with the one specified by ``<filename>``:

.. code-block :: sh

    cynthion flash --bitstream <filename>

Overwrite the Microcontroller firmware with the one specified by ``<filename>``:

.. code-block :: sh

    cynthion flash --mcu-firmware <filename>

Overwrite the SoC firmware with the one specified by ``<filename>``:

.. code-block :: sh

    cynthion flash --soc-firmware <filename>
