========================
The ``cynthion`` Utility
========================

The ``cynthion`` distribution provides the ``cynthion`` command-line utility,
that can be used to perform various simple functions useful in development;
including simple JTAG operations, SVF playback, manipulating the board’s flash,
and debug comms.

.. code:: text

    $ cynthion
    usage: cynthion [-h] command ...

    Cynthion FPGA Configuration / Debug tool

    positional arguments:
      command
        info                Print device info.
        jtag-scan           Prints information about devices on the onboard JTAG chain.
        flash-info          Prints information about the FPGA's attached configuration flash.
        flash-erase         Erases the contents of the FPGA's flash memory.
        flash-program (flash)
                            Programs the target bitstream onto the FPGA's configuration flash.
        flash-fast          Programs a bitstream onto the FPGA's configuration flash using a SPI bridge
        flash-read          Reads the contents of the attached FPGA's configuration flash.
        svf                 Plays a given SVF file over JTAG.
        configure           Uploads a bitstream to the device's FPGA over JTAG.
        reconfigure         Requests the attached ECP5 reconfigure itself from its SPI flash.
        force-offline       Forces the board's FPGA offline.
        spi                 Sends the given list of bytes over debug-SPI, and returns the response.
        spi-inv             Sends the given list of bytes over SPI with inverted CS.
        spi-reg             Reads or writes to a provided register over the debug-SPI.
        jtag-spi            Sends the given list of bytes over SPI-over-JTAG, and returns the response.
        jtag-reg            Reads or writes to a provided register of JTAG-tunneled debug SPI.
        leds                Sets the specified pattern for the Debug LEDs.
        selftest            Run a hardware self-test on a connected Cynthion.

    optional arguments:
      -h, --help            show this help message and exit
