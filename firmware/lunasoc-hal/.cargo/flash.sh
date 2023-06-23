#!/usr/bin/env zsh

# configuration
${UART:=/dev/ttyACM0}
${BASE_MEM:=0x40000000}
${BITSTREAM:=../../gateware/moondancer_soc/build/top.bit}

echo "Using SoC uart: UART=$UART"
echo "Using SoC base memory address: BASE_MEM=$BASE_MEM"
echo "Using SoC bitstream: BITSTREAM=$BITSTREAM"

# create bin file
NAME=$(basename $1)
cargo objcopy --release --example $NAME -- -Obinary $1.bin

# configure fpga with soc bitstream
echo "Configuring fpga: $BITSTREAM"
apollo configure $BITSTREAM 2>/dev/null

# lxterm command
LXTERM="litex_term --kernel $1.bin --kernel-adr $BASE_MEM --speed 115200 $UART"

# flash firmware to soc
echo "Flashing: $1.bin"
expect -c "spawn $LXTERM; send \nserialboot\n; interact"
