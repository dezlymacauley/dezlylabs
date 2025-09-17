
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

### Instruction from the official docs

```
https://www.kali.org/docs/virtualization/install-qemu-guest-vm/
```
_______________________________________________________________________________

Click `File` -> `New Virtual Machine`

_______________________________________________________________________________
## Step 1 of 4

Connection: QEMU / KVM

Choose how you would like to install the operating system

Select this:
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

Uncheck the option `Automatically detect from the installation media / source`

Click on the search bar, and select this:
```sh
Debian testing (debiantesting)
```
_______________________________________________________________________________

### Continue from here










## Step 3 of 5

Memory: 3072 MB (3 GB)

CPU: 2

_______________________________________________________________________________

## Step 4 of 5

Check the option: `Enable storage for this virtual machine`

Check the option: `Create a disk image for the virtual machine.`

Leave it at `20 GB`

Click `Forward`

_______________________________________________________________________________

## Step 5 of 5

Name: kali-linux

Then click `Finish`

_______________________________________________________________________________

Virtual Network is not active. Would you like to start the network now?

Select `Yes`

_______________________________________________________________________________
