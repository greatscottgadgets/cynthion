SHELL := /bin/zsh

UART	?= /dev/ttyACM0
CROSS	?= riscv64-unknown-elf

AR=$(CROSS)-ar

# - gateware ------------------------------------------------------------------

top:
	rm -rf build
	AR=$(AR) python top.py
	cp build/genrust/memory.x ../../firmware/memory.x

load:
	apollo configure build/top.bit

load-ulx3s:
	openFPGALoader --board ulx3s build/top.bit


# - helpers -------------------------------------------------------------------

console:
	pyserial-miniterm $(UART) 115200

clean:
	-rm -rf build/
