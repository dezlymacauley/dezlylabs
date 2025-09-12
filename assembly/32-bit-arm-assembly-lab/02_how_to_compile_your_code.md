`arm-linux-gnueabihf-gcc` is special version of gcc.

Unlike the regular `gcc` which compiles a file to regular.

```sh
arm-linux-gnueabihf-gcc test.c -o test -static 
```

Then to run the compiled binary
```sh
qemu-arm ./test
```

You can also use the commadnd `file` to check out the details of a file.
