
_______________________________________________________________________________

```sh
mkdir -p $HOME/virtual-machines/
```

Download the pre-built VM for QEMU:
```
kali-linux-2025.2-qemu-amd64.7z
```

Extract it:
```sh
7z x kali-linux-2025.2-qemu-amd64.7z
```

You should now have a `.qcow2` file

You can delete the .7z file

_______________________________________________________________________________

Start the virtual-machines service

```sh
sudo systemctl start libvirtd.service libvirtd.socket libvirtd-admin.socket libvirtd-ro.socket
```

_______________________________________________________________________________

Click `File` -> `New Virtual Machine`

_______________________________________________________________________________
## Step 1 of 5

Connection: QEMU / KVM

Choose how you would like to install the operating system

Select this:
```
Local install media (ISO image or CDROM)
```

_______________________________________________________________________________

## Step 2 of 5

Choose ISO or CDROM install media:

Click `Browse` -> `Browse Local`

Select `kali-linux-2025.2-qemu-amd64.qcow2` from `$HOME/virtual-machines/`

_______________________________________________________________________________

Choose the operating system you are installing:

Uncheck the option `Automatically detect from the installation media / source`

Click on the search bar, and select this:
```sh
Debian testing (debiantesting)
```
_______________________________________________________________________________

## Step 3 of 5

Memory: 3072 MB (3 GB)

CPU: 2

_______________________________________________________________________________

## Step 4 of 5

Check the option: `Enable storage for this virtual machine`

Uncheck the optioon `Create a disk image for the virtual machine.`

And check the option `Select or create custom storage`

Then click `Manage`

Select the `virtual-machines` directory.

Select the `kali-linux-2025.2-qemu-amd64.qcow2 80.09 GiB qcow2`

Click `Choose Volume` then click `Forward`

_______________________________________________________________________________

## Step 5 of 5

Name: Kali Linux

Then click `Finish`

_______________________________________________________________________________

Virtual Network is not active. Would you like to start the network now?

Select `Yes`

_______________________________________________________________________________
