variable "alpine_version" {
  type = string
  default = "3.18.5"
  description = "Version of Alpine Linux to download"

  validation {
    condition = can(regex("\\d+\\.\\d+(\\.\\d+)?", var.alpine_version))
    error_message = "The alpine_version value must be a valid semverse number."
  }
}

variable "headless" {
  type = bool 
  default = true
  description = "Run without QEMU's GUI"
}

locals {
  major_version = split(".", var.alpine_version)[0]
  minor_version = try(split(".", var.alpine_version)[1], 0)
  patch_version = try(split(".", var.alpine_version)[2], 0)
}

source "qemu" "alpine-base" {

  qemu_binary       = "/usr/local/bin/qemu-system-aarch64"
  headless          = var.headless
  
  // OS
  iso_url           = "https://dl-cdn.alpinelinux.org/alpine/v${local.major_version}.${local.minor_version}/releases/aarch64/alpine-virt-${local.major_version}.${local.minor_version}.${local.patch_version}-aarch64.iso"
  iso_checksum      = "file:https://dl-cdn.alpinelinux.org/alpine/v${local.major_version}.${local.minor_version}/releases/aarch64/alpine-virt-${local.major_version}.${local.minor_version}.${local.patch_version}-aarch64.iso.sha256"
  // cdrom_interface   = "virtio"

  // Disk
  vm_name           = "base"
  disk_size         = "20G"
  format            = "qcow2"
  // disk_interface    = "virtio" 
  disk_compression  = true
  
  // Machine
  machine_type      = "virt"
  accelerator       = "tcg"
  cpus              = 2
  memory            = 2048
  net_device        = "virtio-net"
  qemuargs = [
    ["-machine", "virt,gic-version=max,virtualization=on"],
    ["-cpu", "max,pauth-impdef=on"],
    ["-device", "qemu-xhci"],         //? These lines were added for
    ["-device", "usb-kbd"],           //? host (linux) that do not support
    ["-device", "bochs-display"],     //? serial0 from VNC out of the box (aka. my work desktop)
    // ["-rtc", "clock=vm"],                          //! Works
    // ["-icount","shift=0,sleep=on,align=off"],      //! but make the process slower
    ["-monitor", "none"],
    ["-parallel", "none"],
  ]
  
  //BOOT
  boot_wait         = "10s"
  efi_boot          = true
  efi_firmware_vars = "./varstore.img"  //? need to be aligned on 64MB
  efi_firmware_code = "./efi.img"       //? need to be aligned on 64MB
  
  // SETUP
  http_directory    = "./config"
  boot_steps     = [
     // Optional
		["<enter>FS0:<enter>efi\\boot\\boot<tab><enter>", "Running bootloader"],
    ["<wait1m>", "Waiting to boot"],

    // Starting
    ["root<enter><enter>", "Entering session"],

    // Install OS
    ["setup-interfaces<enter><wait1s><enter><wait1s><enter><wait1s><enter><wait1s>",  "Setting up network"],
    ["rc-service networking --quiet start<enter><wait5s>",                            "Starting network service"],
    ["wget http://{{ .HTTPIP }}:{{ .HTTPPort }}/setup_alpine_repo.sh<enter><wait1s>", "Download setup script"],
    ["chmod +x *.sh<enter>",                                                          "'chmod' the script"],
    ["./setup_alpine_repo.sh<enter><wait5s>",                                         "Run the setup script"],

    ["setup-disk -q -m sys /dev/vda<enter><wait30s>y<enter><wait1m30s>",              "Install Alpine Linux onto disk"],
    ["reboot<enter><wait1m30s>",                                                      "Reboot"],

    // Install Package
    ["root<enter><enter>",                                                            "Entering session"],
    ["setup-hostname -n localhost<enter><wait1s>",                                    "Setting Up Hostname"],
    ["setup-interfaces<enter><wait1s><enter><wait1s><enter><wait1s><enter><wait1s>",  "Setting up network"],
    ["rc-service networking --quiet start<enter><wait5s>",                            "Starting network service"],
    ["wget http://{{ .HTTPIP }}:{{ .HTTPPort }}/setup_alpine_repo.sh<enter><wait1s>", "Download setup script"],
    ["wget http://{{ .HTTPIP }}:{{ .HTTPPort }}/setup_alpine_config.sh<enter><wait1s>", "Download package script"],
    ["wget http://{{ .HTTPIP }}:{{ .HTTPPort }}/pull_cloudsuite.sh<enter><wait1s>",   "Download docker script"],
    ["wget http://{{ .HTTPIP }}:{{ .HTTPPort }}/authorized_keys<enter><wait1s>",      "Download authorized_keys file"],
    ["chmod +x *.sh<enter>",                                                          "'chmod' the script"],
    ["./setup_alpine_repo.sh<enter><wait5s>",                                         "Run the setup script"],
    ["./setup_alpine_config.sh<enter><wait2m>",                                       "Run the package script"],

    // SSH
    ["passwd<enter>root<enter>root<enter>",                                           "Set new password to 'root'"],
    ["setup-sshd<enter><enter><wait5s>yes<enter><wait2s><enter><wait10s>",            "Install OpenSSH server"],
    
  ]

  // VALIDATION
  communicator      = "ssh"
  ssh_username      = "root"
  ssh_password      = "root"
  ssh_timeout       = "1m"
  disable_vnc       = false
  shutdown_command  = "poweroff"

}

build {
  sources = ["source.qemu.alpine-base"]
}

packer {
  required_plugins {
    qemu = {
      version = "~> 1"
      source  = "github.com/hashicorp/qemu"
    }
  }
}
