LIBRARY_NAME=libws_server
X86_64_TARGET=x86_64-unknown-linux-gnu
ARM64_TARGET=aarch64-unknown-linux-gnu
X86_64_CC = gcc
ARM64_CC = aarch64-linux-gnu-gcc
LIB_DIR=lib
TARGET_DIR=target

x86_64: clean
	cargo build --release --target=$(X86_64_TARGET)
	mkdir -p $(LIB_DIR)
	rm -rf $(LIB_DIR)/$(LIBRARY_NAME)_x86_64.*
	cp $(TARGET_DIR)/$(X86_64_TARGET)/release/$(LIBRARY_NAME).a $(LIB_DIR)/$(LIBRARY_NAME)_x86_64.a
	cp $(TARGET_DIR)/$(X86_64_TARGET)/release/$(LIBRARY_NAME).so $(LIB_DIR)/$(LIBRARY_NAME)_x86_64.so

arm64: clean
	cargo build --release --target=$(ARM64_TARGET)
	mkdir -p $(LIB_DIR)
	rm -rf $(LIB_DIR)/$(LIBRARY_NAME)_arm64.*
	cp $(TARGET_DIR)/$(ARM64_TARGET)/release/$(LIBRARY_NAME).a $(LIB_DIR)/$(LIBRARY_NAME)_arm64.a
	cp $(TARGET_DIR)/$(ARM64_TARGET)/release/$(LIBRARY_NAME).so $(LIB_DIR)/$(LIBRARY_NAME)_arm64.so

CFLAGS = -Isrc
LDFLAGS = -ldl -pthread -lm -O2
X86_64_LDFLAGS = ./lib/$(LIBRARY_NAME)_x86_64.a $(LDFLAGS)
ARM64_LDFLAGS = ./lib/$(LIBRARY_NAME)_arm64.a $(LDFLAGS)

c_server_x86_64: x86_64
	$(X86_64_CC) $(CFLAGS) -O2 -c ./c_examples/c_server.c -o c_server.o
	$(X86_64_CC) -o c_server c_server.o $(X86_64_LDFLAGS)

c_server_arm64: arm64  
	$(ARM64_CC) $(CFLAGS) -O2 -c ./c_examples/c_server.c -o c_server.o
	$(ARM64_CC) -o c_server c_server.o $(ARM64_LDFLAGS)

c_server: c_examples/c_server.c
ifeq ($(shell uname -m), x86_64)
	$(MAKE) c_server_x86_64
else ifeq ($(shell uname -m), aarch64)
	$(MAKE) c_server_arm64
else
	@echo "Unsupported architecture"
endif

run:
	./c_server
	
scp:
	scp c_server debian@192.168.7.2:/home/debian/c_server 
	
debug:
	RUST_BACKTRACE=1 ./c_server 

.PHONY: clean clean_lib clean_all help

clean:
	rm -rf c_server */*.c.orig */*.h.orig *.o

clean_lib:
	rm -rf $(LIB_DIR)/$(LIBRARY_NAME)_*.*
	
clean_all: clean clean_lib

help:
	@echo "Available targets:"
	@echo "  x86_64          - Build for x86_64 architecture"
	@echo "  arm64           - Build for ARM64 architecture"
	@echo "  c_server_x86_64 - Compile C server for x86_64"
	@echo "  c_server_arm64  - Compile C server for ARM64"
	@echo "  c_server        - Compile C server for native architecture"
	@echo "  run             - Run the C server"
	@echo "  scp             - Copy the C server to remote machine"
	@echo "  debug           - Run the C server with debug mode"
	@echo "  clean           - Clean the build files"
	@echo "  clean_lib       - Clean the library files"
	@echo "  clean_all       - Clean all files"
