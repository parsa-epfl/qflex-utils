# Packer Image for QEMU

This repository contains configuration files and scripts to build Linux images for QEMU using HashiCorp Packer.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Customization](#customization)

## Overview

This project utilizes HashiCorp Packer to automate the creation of Linux images suitable for use with QEMU. The resulting images can be used for development, testing, or as a base for further customization.

## Prerequisites

Before you begin, ensure you have the following dependencies installed:

- [HashiCorp Packer](https://www.packer.io/)
- [QEMU](https://www.qemu.org/)

Make sure that QEMU was build with these flags at least

```bash
./configure --target-list=aarch64-softmmu       \
            --enable-download --enable-slirp    \
            --enable-vhost-net --enable-gtk     \
            --enable-bochs --enable-libusb
```

## Usage

1. Clone this repository:
2. Navigate to the repository directory:
3. Prepare the EFI image
    ```bash
        chmod +x 00_prepare_efi.sh
        ./00_prepare_efi.sh
    ```
4. Build the Alpine Linux image:

    ```bash
    packer build -var="alpine_version=3.18.5" alpine.pkr.hcl
    ```

5. After the build completes, you will find the QEMU-compatible Alpine Linux image in the `output.xxxx` directory.

## Customization

Feel free to customize the Packer template (`alpine.pkr.hcl`) to suit your specific requirements. You can modify the following aspects:

- **Alpine Linux version**: Update the `alpine_version` variable to choose a specific Alpine Linux version.
- **Provisioning**: Adjust the provisioner scripts in the `config` directory to install additional packages or configure the system as needed.
- **QEMU settings**: Modify the QEMU-related settings in the Packer template to match your environment.

