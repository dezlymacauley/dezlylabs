.SILENT:
.PHONY: run clean
 
run:
	mkdir -p bin/
	zig cc \
		-target arm-linux-musleabihf src/main.c \
		-static -o bin/main_32_bit_arm_statically_linked
	qemu-arm bin/main_32_bit_arm_statically_linked

clean:
	rm -rf bin/
