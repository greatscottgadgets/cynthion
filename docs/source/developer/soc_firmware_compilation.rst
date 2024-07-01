========================
SoC Firmware Compilation
========================

Before proceeding, please ensure you have followed the prerequisites in the :doc:`Setting up a Development Environment <introduction>` section.

Building and Running
--------------------

Firmware for the Cynthion SoC can be found in the ``firmware/moondancer/`` sub-directory.

You can rebuild the firmware using ``cargo`` as follows:

.. code-block:: text

    # change to the Cynthion firmware directory
    cd firmware/moondancer/

    # rebuild the firmware
    cargo build --release

To upload the firmware binary to Cynthion and flash the SoC bitstream you can run:

.. code-block:: text

    # change to the Cynthion firmware directory
    cd firmware/moondancer/

    # upload firmware and run it
    cargo run --release

.. note::

    By default the firmware's flash script will look for the SoC UART
    on ``/dev/ttyACM0``, if this is not the case on your machine you
    will need to specify the correct path using the ``UART`` environment
    variable, for example:

    .. code-block:: sh

        UART=/dev/cu.usbmodem22401 cargo run --release

    By default the SoC bitstream is obtained from the latest build in
    ``cynthion/python/build/top.bit`` but you can override
    it with:

    .. code-block:: sh

        BITSTREAM=path/to/bitstream.bit cargo run --release


Running Firmware Unit Tests
---------------------------

Once the firmware is running on the SoC you can execute some unit tests to exercise the firmware.

In order to do this you will need to connect both the **CONTROL** and
**AUX** ports of the Cynthion to the host and then run:

.. code-block:: sh

    # change to the Cynthion firmware directory
    cd firmware/moondancer/

    # run firmware unit tests
    python -m unittest


Firmware Examples
-----------------

There are a number of firmware examples in the ``firmware/moondancer/examples/`` sub-directory.

.. code-block:: sh

    # change to the Cynthion firmware directory
    cd firmware/moondancer/

    # run example
    cargo run --release --example <example name>
