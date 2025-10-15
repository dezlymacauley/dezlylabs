`arm-linux-gnueabihf-gcc` is special version of gcc.

Unlike the regular `gcc` which compiles a file to regular.

```sh
arm-linux-gnueabihf-gcc test.c -o test -static
```

_______________________________________________________________________________

To check that your C code was successfully compiled to run on 32 bit ARM,
run the `file` command on the binary executable.

```sh
file test 
```

test: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), 
statically linked, 
BuildID[sha1]=b4cc83a1552a43da4bb39c6ef6df97c5b5823040,
for GNU/Linux 3.2.0, not stripped

_______________________________________________________________________________

Then to run the compiled binary
```sh
qemu-arm ./test
```

You can also use the commadnd `file` to check out the details of a file.
