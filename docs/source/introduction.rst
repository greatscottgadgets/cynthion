============
Introduction
============

What is Cynthion?
-----------------

Cynthion is an all-in-one tool for building, testing, monitoring, and experimenting with USB devices. Built around a unique FPGA-based architecture, Cynthion’s digital hardware can be fully customized to suit the application at hand. As a result, it can act as a no-compromise High-Speed USB protocol analyzer, a USB-hacking multi-tool, or a USB development platform.

Out-of-the-box, Cynthion acts as a USB protocol analyzer capable of capturing and analyzing traffic between a host and any Low-, Full-, or High-Speed ("USB 2.0") USB device. It works seamlessly with our `Packetry <https://github.com/greatscottgadgets/packetry>`__ open-source analysis software.

Combined with our `LUNA <https://luna.readthedocs.io>`__ gateware and `Facedancer <https://facedancer.readthedocs.io>`__ libraries, Cynthion becomes a versatile USB-hacking and development tool. Facedancer makes it quick and easy to create or tamper with real USB devices—not just emulations—even if you don’t have experience with digital hardware design, HDL, or FPGA architecture!


Device Overview
---------------


Top View
~~~~~~~~

.. image:: ../images/cynthion-top.svg
  :width: 300
  :alt: Cynthion Top View

- **A-E** - Five status LEDs managed by the debug microcontroller.

    - **A** - Power Indicator.
    - **B** - FPGA is online.
    - **C** - FPGA has requested control of the  **CONTROL** port.
    - **D** - FPGA has control of the **CONTROL** port.
    - **E** - Reserved for future use.

- **0-5** - Six user LEDs managed by the FPGA.


Left View
~~~~~~~~~

.. image:: ../images/cynthion-left.svg
  :width: 400
  :alt: Cynthion Left View

- **PROGRAM** - Press this button to return control of the **CONTROL** port to the debug microcontroller and hold the FPGA in an unconfigured state.

    - *Recovery mode*: Press this button during power-on to invoke the `Saturn-V <https://github.com/greatscottgadgets/saturn-v>`__ bootloader on the **CONTROL** port.

- **CONTROL**  - Primary USB connector used by the host computer to control Cynthion.
- **USER**     - A user-assignable button that can be used in your own designs.
- **AUX**      - An auxiliary USB connection that can be used in your own designs.


Right View
~~~~~~~~~~

.. image:: ../images/cynthion-right.svg
  :width: 400
  :alt: Cynthion Right View

- **TARGET C** - USB Type-C connector for Packetry traffic capture and Facedancer device emulation.
- **TARGET A** - USB Type-A connector shared with the **TARGET C** connector.
- **RESET**    - Press this button to reset Cynthion's debug microcontroller and reconfigure the FPGA from flash.


Front View
~~~~~~~~~~

.. image:: ../images/cynthion-front.svg
  :width: 400
  :alt: Cynthion Front View

- **A & B** - Two Digilent Pmod™ Compatible I/O connectors for a total of 16 high-speed FPGA user IOs.

    - **B** can also be configured to act as a serial port and JTAG connector for debugging SoC designs:

        - **1**  - SERIAL RX
        - **2**  - SERIAL TX
        - **7**  - JTAG TMS
        - **8**  - JTAG TDI
        - **9**  - JTAG TDO
        - **10** - JTAG TCK



Bottom View
~~~~~~~~~~~

.. image:: ../images/cynthion-bottom.svg
  :width: 300
  :alt: Cynthion Bottom View
