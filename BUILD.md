# Building Cynthion


## Install Environmental Dependencies

### Python

I use `pyenv` to manage my Python environments but you can use anything you like:

    # debian
    curl https://pyenv.run | bash

    # macos brew
    brew install pyenv


If you're using `pyenv` you'll also need these to be able to build Python versions:

    # debian
    apt install build-essential libssl-dev zlib1g-dev libbz2-dev \
        libreadline-dev libsqlite3-dev libncursesw5-dev xz-utils \
        tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev

    # macos brew
    brew install openssl readline sqlite3 xz zlib tcl-tk

Finally, I create my environment with:

    # install python
    pyenv install 3.11

    # create a new virtual environment
    pyenv virtualenv 3.11 gsg-cynthion

    # enable virtual environment for project repos
    cd cynthion.git
    pyenv local gsg-cynthion

    # upgrade pip to latest
    python -m pip install --upgrade pip


### Yosys

Grab and install the latest toolchain from:

    https://github.com/YosysHQ/oss-cad-suite-build/releases/latest

Remember to mollify Gatekeeper if you're on macOS:

    oss-cad-suite/activate

Enable environment with:

    source <path-to>/oss-cad-suite/environment


### Rust

I use `rustup` to manage my Rust environment but you can use anything you like:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install RISC-V embedded target support:

    rustup target add riscv32imac-unknown-none-elf
    rustup component add llvm-tools-preview
    cargo install cargo-binutils

Install a RISC-V GNU toolchain so we can build litex-bios or run the C examples:

    # debian
    apt install gcc-riscv64-unknown-elf

    # macos brew - https://github.com/riscv-software-src/homebrew-riscv
    brew tap riscv-software-src/riscv
    brew install riscv-gnu-toolchain

I keep my Rust environment up to date with:

    # update rust compiler and targets
    rustup update

    # update installed binaries
    cargo install cargo-update
    cargo install-update -a


## Build Cynthion Components

Things can get somewhat hairy over time so I usually clean out my pyenv environment before starting:

    pyenv activate gsg-cynthion
    pip uninstall -y -r <(pip freeze)


### The `cynthion` python package

    cd cynthion.git/cynthion/python/
    pip3 install --upgrade ".[gateware,gateware-soc]"


### Packetry Gateware

    cd cynthion.git/cynthion/python/
    LUNA_PLATFORM="cynthion.gateware.platform:CynthionPlatformRev1D3" make analyzer


### Moondancer SoC

    cd cynthion.git/cynthion/python/
    LUNA_PLATFORM="cynthion.gateware.platform:CynthionPlatformRev1D3" make soc


### Moondancer Firmware

    cd cynthion.git/firmware/moondancer/
    cargo build --release

Running the firmware will automatically flash the SoC image to the FPGA:

    cargo run --release
