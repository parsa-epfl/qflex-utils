#!/usr/bin/env bash
# 
#         __ _   _                                                  _             
#    ___ / _(_) (_)_ __ ___   __ _  __ _  ___    ___ _ __ ___  __ _| |_ ___  _ __ 
#   / _ \ |_| | | | '_ ` _ \ / _` |/ _` |/ _ \  / __| '__/ _ \/ _` | __/ _ \| '__|
#  |  __/  _| | | | | | | | | (_| | (_| |  __/ | (__| | |  __/ (_| | || (_) | |   
#   \___|_| |_| |_|_| |_| |_|\__,_|\__, |\___|  \___|_|  \___|\__,_|\__\___/|_|   
#                                  |___/                                          
#
#   Author:
#       Bryan Perdrizat <bryan.perdrizat@epfl.ch>
#   Description:
#       Prepare an EFI disk and the a variable storage 
#       disk to boot aarch64 image in QEMU


truncate -s 64m ./varstore.img
truncate -s 64m ./efi.img
dd if=/usr/share/qemu-efi-aarch64/QEMU_EFI.fd of=./efi.img conv=notrunc