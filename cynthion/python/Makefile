SHELL := /bin/zsh

UART	?= /dev/ttyACM0
CROSS	?= riscv64-unknown-elf

AR=$(CROSS)-ar

# - gateware ------------------------------------------------------------------

analyzer:
	rm -rf build
	python -m cynthion.gateware.analyzer.top

soc:
	rm -rf build
	AR=$(AR) python -m cynthion.gateware.soc.top
	cp build/genrust/memory.x ../../firmware/memory.x


# - helpers -------------------------------------------------------------------

clean:
	-rm -rf build/

console:
	pyserial-miniterm $(UART) 115200

load:
	apollo configure build/top.bit
