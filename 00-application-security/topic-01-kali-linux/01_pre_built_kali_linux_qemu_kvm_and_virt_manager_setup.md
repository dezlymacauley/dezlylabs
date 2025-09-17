# Pre-Built Kali Linux VM - QEMU/KVM and Virt Manager Setup
_______________________________________________________________________________

## System Requirement

1. Enable virtualization in the BIOS of your computer

2. I am on Arch Linux so these are the bare minimum packages needed.

```sh
# A cross-platform open-source machine emulator and virtualizer.
sudo pacman -S --needed qemu-full

# A GUI to manager create and manage virtual machines.
sudo pacman -S --needed virt-manager

# libvirt is a daemon (a local API server). When started, it listens for
# requests/instructions from the client (virt-manager), and then passes
# those instructions on in a format that QEMU understands.
#
# This package is actually part of qemu-full but I like to list it here
# for an understanding of how things work.
sudo pacman -S --needed libvirt

# Lightweight DHCP and DNS server that provides networking services for VMs.
# Creates virtual networks (like the "default" 192.168.122.x network) and
# automatically assigns IP addresses to VMs so they can communicate with
# each other and access the internet through NAT. Without this, VMs would
# have no network connectivity unless manually configured.
sudo pacman -S --needed dnsmasq
```
_______________________________________________________________________________

### Download the official pre-built VM for QEMU:

```
https://www.kali.org/get-kali/#kali-virtual-machines
```

It should be a file that looks like this and ends with `.7z`
```
kali-linux-2025.2-qemu-amd64.7z
```
_______________________________________________________________________________

### Create a directory for your virtual machines

```sh
mkdir -p $HOME/virtual-machines
```

Move the kali linux image you downloaded to this directory.


You should see this when you navigated to the `virtual-machines` directory 
and use the `ls` command to view its contents.

```
❯ cd $HOME/virtual-machines
~/virtual-machines
❯ ls
kali-linux-2025.2-qemu-amd64.7z
```

_______________________________________________________________________________

### Extract the file with `7z`:

```sh
cd $HOME/virtual-machines
7z x kali-linux-2025.2-qemu-amd64.7z
```

_______________________________________________________________________________

You will now have two files in the `virtual-machines` directory:

```
kali-linux-2025.2-qemu-amd64.7z
kali-linux-2025.2-qemu-amd64.qcow2
```

Delete the one that ends with `.7z`

_______________________________________________________________________________

This is the only file you should have in the `virtual-machines` directory.
```
kali-linux-2025.2-qemu-amd64.qcow2
```
_______________________________________________________________________________

### Start the libvirtd service and its sockets

```sh
sudo systemctl start libvirtd.service \
libvirtd.socket libvirtd-admin.socket libvirtd-ro.socket
```
_______________________________________________________________________________

### Open the `Virt Manager` graphical user interface

You should see `QEMU/KVM` listed as a connection.

_______________________________________________________________________________

Click `File` -> `New Virtual Machine`

_______________________________________________________________________________
## Step 1 of 4

Connection: QEMU / KVM

Choose how you would like to install the operating system

Make sure to select this:
```sh
Import existing disk image
```

Then click `Forward`

_______________________________________________________________________________

## Step 2 of 4

Provide the existing storage path:


Click `Browse` -> `Browse Local`

Select `kali-linux-2025.2-qemu-amd64.qcow2` from `$HOME/virtual-machines/`

_______________________________________________________________________________

Choose the operating system you are installing:

Click on the search bar, type `debiantesting` and select this option:
```sh
Debian testing (debiantesting)
```

The reason for this is because Kali Linux is actually based 
of the testing branch of a Linux distribution called `Debian`.

Click `Forward`

_______________________________________________________________________________

## Step 3 of 4

Memory: 3072 MB (3 GB)

CPU: 2

_______________________________________________________________________________

## Step 4 of 4

Name: kali-linux

Then click `Finish`

_______________________________________________________________________________

Virtual Network is not active. Would you like to start the network now?

Select `Yes`

_______________________________________________________________________________

Once it opens up you can close `Virt Manager` workspace and just leave the
window with the VM running.

_______________________________________________________________________________
### Login to you Virtual Machine

The default username for a kali VM is `kali`, 
and the default password is `kali` as well.

_______________________________________________________________________________
### Check that you can access `google.com`

Now you are ready to go.

_______________________________________________________________________________
