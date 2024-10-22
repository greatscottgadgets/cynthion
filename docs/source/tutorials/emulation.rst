Emulation of a USB Device
=========================

This tutorial walks through the whole process of emulating a USB device with Cynthion and `Facedancer <https://facedancer.readthedocs.io/>`__. We'll emulate `HackRF One <https://greatscottgadgets.com/hackrf/one/>`__, a software-defined radio platform. The goal of our emulation is to fool the ``hackrf_info`` command into reporting that a HackRF One is connected.


Prerequisites
-------------

 * Install the Cynthion tools by following :doc:`/getting_started`.
 * Install HackRF Tools by following `Installing HackRF Software <https://hackrf.readthedocs.io/en/latest/installing_hackrf_software.html>`__.
 * Install the Facedancer library and run the Facedancer bitstream and firmware as described in :doc:`/getting_started_facedancer`.

    .. note::

        If you would like to configure your Cynthion for Facedancer operation permanently instead of temporarily, use ``cynthion flash facedancer`` instead of ``cynthion run facedancer``.


Try to Detect a HackRF One
--------------------------

Use the ``hackrf_info`` command to detect any connected HackRF devices:

.. code-block :: sh

    hackrf_info

The command output should indicate that no HackRF devices are found:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    No HackRF boards found.


Connect
-------

We need to connect our Cynthion before we can use it to emulate a HackRF One. If you followed the prerequisites above, you should already have connected the Cynthion's **CONTROL** port to your computer.

Now also connect the **TARGET C** port to your computer. Facedancer software uses **CONTROL** to control the Cynthion and **TARGET C** to connect to the target host, the computer which we'll try to fool into thinking that there is a HackRF One connected. The control host and target host can be two separate computers, but in this tutorial we will use the same computer as both the control host and the target host.

.. image:: ../../images/cynthion-connections-facedancer-single-host.svg
  :alt: Connection diagram for using Cynthion with Facedancer on a single host computer.


Emulate the Vendor ID and Product ID
------------------------------------

Use your favorite text editor to create a new Python program called ``hackrf_emulation.py`` with the following contents:

.. code-block :: python

    from facedancer import *
    from facedancer import main

    @use_inner_classes_automatically
    class HackRF(USBDevice):
        product_string       : str = "HackRF One (Emulated)"
        manufacturer_string  : str = "Facedancer"
        serial_number_string : str = "1234"
        vendor_id            : int = 0x1d50
        product_id           : int = 0x6089

        class DefaultConfiguration(USBConfiguration):
            class DefaultInterface(USBInterface):
                pass

    main(HackRF)

Every USB device identifies itself to its host computer using a 16-bit Vendor ID and 16-bit Product ID. This program uses the Facedancer library to implement a device with the Vendor ID and Product ID associated with HackRF One. It also configures some strings which make our emulated HackRF distinguishable from an actual HackRF One (with tools such as ``lsusb``) for convenience.

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, open another terminal and execute ``hackrf_info``. It should display output similar to this:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    hackrf_board_id_read() failed: Pipe error (-1000)

We've just convinced ``hackrf_info`` that it has found a HackRF device! However, ``hackrf_info`` failed to read the HackRF's board ID which distinguishes between the various hardware platforms supported by HackRF software. The pipe error indicates that the device did not provide the expected response to the host's request for the board ID.

Terminate ``hackrf_emulation.py`` by typing ``ctrl-c``. Because we used the ``--suggest`` option, it should provide output like this:

.. code-block :: python

    Automatic Suggestions
    These suggestions are based on simple observed behavior;
    not all of these suggestions may be useful / desirable.


    Request handler code:

        @vendor_request_handler(number=14, direction=USBDirection.IN)
        @to_device
        def handle_control_request_14(self, request):
            # Most recent request was for 1B of data.
            # Replace me with your handler.
            request.stall()


Try the Suggested Code
----------------------

Add the suggested code to the ``HackRF`` class in ``hackrf_emulation.py``. The program should now look like:

.. code-block :: python

    from facedancer import *
    from facedancer import main

    @use_inner_classes_automatically
    class HackRF(USBDevice):
        product_string       : str = "HackRF One (Emulated)"
        manufacturer_string  : str = "Facedancer"
        serial_number_string : str = "1234"
        vendor_id            : int = 0x1d50
        product_id           : int = 0x6089

        class DefaultConfiguration(USBConfiguration):
            class DefaultInterface(USBInterface):
                pass

        @vendor_request_handler(number=14, direction=USBDirection.IN)
        @to_device
        def handle_control_request_14(self, request):
            # Most recent request was for 1B of data.
            # Replace me with your handler.
            request.stall()

    main(HackRF)

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    hackrf_board_id_read() failed: Pipe error (-1000)

It turns out that our emulation still results in a pipe error. This is because we are stalling vendor request number 14 which is meant to return a 1 byte board ID. Terminate ``hackrf_emulation.py`` and replace the ``request_stall()`` line with:

.. code-block :: python

   request.reply([1])

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    Board ID Number: 1 (Jawbreaker)
    hackrf_version_string_read() failed: Pipe error (-1000)

We've now convinced ``hackrf_info`` that our Cynthion is a HackRF Jawbreaker which was the beta platform that preceded HackRF One. Let's try a higher board ID number. Replace ``request.reply([1])`` with:

.. code-block :: python

   request.reply([2])

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    Board ID Number: 2 (HackRF One)
    hackrf_version_string_read() failed: Pipe error (-1000)

We did it! Our new board ID represents HackRF One! In this example we guessed low numbers for the board ID byte, but we could have discovered that ``2`` represents HackRF One by observing the behavior of an actual HackRF One or by reading the `libhackrf source code <https://github.com/greatscottgadgets/hackrf/blob/17f394331d16e11d835092bed14a5b7feb4f47e0/host/libhackrf/src/hackrf.h#L660>`__ or `HackRF firmware source code <https://github.com/greatscottgadgets/hackrf/blob/17f394331d16e11d835092bed14a5b7feb4f47e0/host/libhackrf/src/hackrf.h#L660>`__.


Handle the Version String Request
---------------------------------

Unfortunately, ``hackrf_info`` still indicates an error, this time with reading a version string. The ``--suggest`` option on your Facedancer program should give you an idea of how to handle that request:

.. code-block :: python

    @vendor_request_handler(number=15, direction=USBDirection.IN)
    @to_device
    def handle_control_request_15(self, request):
        # Most recent request was for 255B of data.
        # Replace me with your handler.
        request.stall()

Notice that this time the host has requested ``255`` bytes instead of just one byte. USB devices often return a smaller number of bytes than the length requested by the host. In this case we can guess that the host is requesting a maximum length string and that we can probably return something shorter. Let's try adding this to the ``HackRF`` class in ``hackrf_emulation.py``:

.. code-block :: python

    @vendor_request_handler(number=15, direction=USBDirection.IN)
    @to_device
    def handle_control_request_15(self, request):
        # Most recent request was for 255B of data.
        request.reply(b"tutorial version")

The complete program should now look like:

.. code-block :: python

    from facedancer import *
    from facedancer import main

    @use_inner_classes_automatically
    class HackRF(USBDevice):
        product_string       : str = "HackRF One (Emulated)"
        manufacturer_string  : str = "Facedancer"
        serial_number_string : str = "1234"
        vendor_id            : int = 0x1d50
        product_id           : int = 0x6089

        class DefaultConfiguration(USBConfiguration):
            class DefaultInterface(USBInterface):
                pass

        @vendor_request_handler(number=14, direction=USBDirection.IN)
        @to_device
        def handle_control_request_14(self, request):
            # Most recent request was for 1B of data.
            # Replace me with your handler.
            request.reply([2])

        @vendor_request_handler(number=15, direction=USBDirection.IN)
        @to_device
        def handle_control_request_15(self, request):
            # Most recent request was for 255B of data.
            request.reply(b"tutorial version")

    main(HackRF)

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    Board ID Number: 2 (HackRF One)
    Firmware Version: tutorial version (API:0.00)
    hackrf_board_partid_serialno_read() failed: Pipe error (-1000)


Handle the Part ID Request
--------------------------

Now we can see another unhandled request made by ``hackrf_info``. The ``--suggest`` output tells us that we can handle it with something like:

.. code-block :: python

    @vendor_request_handler(number=18, direction=USBDirection.IN)
    @to_device
    def handle_control_request_18(self, request):
        # Most recent request was for 24B of data.
        # Replace me with your handler.
        request.stall()

The host is asking for 24 bytes this time, suggesting that it is looking for exactly 24 bytes. Let's try replying with 24 bytes of dummy data:

.. code-block :: python

    @vendor_request_handler(number=18, direction=USBDirection.IN)
    @to_device
    def handle_control_request_18(self, request):
        # Most recent request was for 24B of data.
        request.reply(b"A" * 24)

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    Board ID Number: 2 (HackRF One)
    Firmware Version: tutorial version (API:0.00)
    Part ID Number: 0x41414141 0x41414141
    hackrf_close() failed: Pipe error (-1000)

It looks like the part ID was interpreted as a valid number, and now ``hackrf_info`` is trying to close the device! We're almost done!


Handle the Close Request
------------------------

Based on the ``--suggest`` output, add the following to ``hackrf_emulation.py``:

.. code-block :: python

    @vendor_request_handler(number=1, direction=USBDirection.OUT)
    @to_device
    def handle_control_request_1(self, request):
        request.ack()

Notice that this time the direction of the vendor request is ``OUT`` instead of ``IN``. This means that the host is sending data to the device, not asking the device to send data to the host. We acknowledge the request instead of replying with data.

Execute the program:

.. code-block :: sh

    python hackrf_emulation.py --suggest

While the program is running, execute ``hackrf_info`` in another terminal:

.. code-block :: sh

    hackrf_info version: 2023.01.1
    libhackrf version: 2023.01.1 (0.8)
    Found HackRF
    Index: 0
    Serial number: 1234
    Board ID Number: 2 (HackRF One)
    Firmware Version: tutorial version (API:0.00)
    Part ID Number: 0x41414141 0x41414141

Success! ``hackrf_info`` now exits without error!


Put It All Together
-------------------

With a few edits based on what we've learned, our complete program might look like this:

.. code-block :: python

    from facedancer import *
    from facedancer import main

    @use_inner_classes_automatically
    class HackRF(USBDevice):
        product_string       : str = "HackRF One (Emulated)"
        manufacturer_string  : str = "Facedancer"
        serial_number_string : str = "1234"
        vendor_id            : int = 0x1d50
        product_id           : int = 0x6089

        class DefaultConfiguration(USBConfiguration):
            class DefaultInterface(USBInterface):
                pass

        @vendor_request_handler(number=14, direction=USBDirection.IN)
        @to_device
        def handle_board_id_request(self, request):
            # return 1-byte board ID
            request.reply([2])

        @vendor_request_handler(number=15, direction=USBDirection.IN)
        @to_device
        def handle_version_string_request(self, request):
            # return up to 255 bytes
            request.reply(b"tutorial version")

        @vendor_request_handler(number=18, direction=USBDirection.IN)
        @to_device
        def handle_part_id_request(self, request):
            # return 24 byte part ID
            request.reply(b"A" * 24)

        @vendor_request_handler(number=1, direction=USBDirection.OUT)
        @to_device
        def handle_close_request(self, request):
            request.reply([])

    main(HackRF)
