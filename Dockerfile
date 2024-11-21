# Use the official image as a parent image
FROM ubuntu:22.04
USER root

# Copy usb hub script from Jenkins' container
COPY --from=gsg-jenkins /startup/hubs.py /startup/hubs.py
COPY --from=gsg-jenkins /startup/.hubs /startup/.hubs
RUN ln -s /startup/hubs.py /usr/local/bin/hubs

# Override interactive installations and install dependencies
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    bison \
    build-essential \
    clang \
    cmake \
    curl \
    dfu-util \
    flex \
    gawk \
    gcc-arm-none-eabi \
    git \
    jq \
    libboost-all-dev \
    libeigen3-dev \
    libreadline-dev \
    libusb-1.0-0-dev \
    openocd \
    pkg-config \
    python3.11 \
    python3.11-venv \
    python3-pip \
    tcl \
    tcl-dev \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# install oss-cad-suite 2024-11-01
RUN curl -L $(curl -s "https://api.github.com/repos/YosysHQ/oss-cad-suite-build/releases/183038843" \
    | jq --raw-output '.assets[].browser_download_url' | grep "linux-x64") --output oss-cad-suite-linux-x64-20241101.tgz \
    && tar zxvf oss-cad-suite-linux-x64-20241101.tgz

# Install USB hub PPPS dependencies
RUN pip3 install python-dotenv git+https://github.com/CapableRobot/CapableRobot_USBHub_Driver --upgrade
RUN curl -L https://github.com/mvp/uhubctl/archive/refs/tags/v2.5.0.tar.gz > uhubctl-2.5.0.tar.gz \
    && mkdir uhubctl-2.5.0 \
    && tar -xvzf uhubctl-2.5.0.tar.gz -C uhubctl-2.5.0 --strip-components 1 \
    && rm uhubctl-2.5.0.tar.gz \
    && cd uhubctl-2.5.0 \
    && make \
    && make install

USER jenkins

# add oss-cad-suite to PATH for pip/source package installations
ENV PATH="/root/.local/bin:/oss-cad-suite/bin:$PATH"

# add the Cynthion board rev
ENV LUNA_PLATFORM="cynthion.gateware.platform:CynthionPlatformRev0D4"

# Inform Docker that the container is listening on port 8080 at runtime
EXPOSE 8080
