#!/usr/bin/env zsh

# configuration
# TODO support flash offset address
: ${FLASH_OFFSET:=0x00000000}
: ${BITSTREAM:=../../cynthion/python/build/soc.bit}
: ${UART:=/dev/ttyACM0}

echo
echo "Using bitstream: BITSTREAM=$BITSTREAM"
echo "Using flash address: FLASH_OFFSET=$FLASH_OFFSET"
echo "Using uart: UART=$UART"

if [ ! -f $BITSTREAM ]
then
    echo
    echo "Failed to locate the Cynthion SoC bitstream file."
    echo
    echo "The SoC bitstream file can be generated with:"
    echo
    echo "    cd ../../cynthion/python/"
    echo "    make soc"
    echo
    exit 1
fi

# convert ELF executable to bin image
echo "Creating firmware image: $1.bin"
NAME=$(basename $1)
if [[ $1 = *"examples/$NAME" ]]
then
    cargo objcopy --release --example $NAME -- -Obinary $1.bin
else
    cargo objcopy --release --bin $NAME -- -Obinary $1.bin
fi

# flash firmware to cynthion
echo "Flashing firmware image: $1.bin"
cynthion flash --offset $FLASH_OFFSET $1.bin

# configure cynthion with soc bitstream
echo "Configuring fpga: $BITSTREAM"
cynthion configure $BITSTREAM 2>/dev/null

# start a terminal for debug output
pyserial-miniterm $UART 115200
