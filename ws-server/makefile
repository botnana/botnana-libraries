LIBRARY_NAME=libws_server
X86_64_TARGET=x86_64-unknown-linux-gnu
I686_TARGET=i686-unknown-linux-gnu
ARM64_TARGET=aarch64-unknown-linux-gnu
X86_64_CC = gcc
I686_CC = gcc
ARM64_CC = aarch64-linux-gnu-gcc

fmt:
	cargo fmt
	astyle --style=allman -r *.c *.h

x86_64: clean
	cargo build --release --target=$(X86_64_TARGET)
	mkdir -p lib
	rm -rf lib/$(LIBRARY_NAME)_x86_64.*
	cp target/$(X86_64_TARGET)/release/$(LIBRARY_NAME).a ./lib/$(LIBRARY_NAME)_x86_64.a
	cp target/$(X86_64_TARGET)/release/$(LIBRARY_NAME).so ./lib/$(LIBRARY_NAME)_x86_64.so

i686: clean
	cargo build --release --target=$(I686_TARGET)
	mkdir -p lib
	rm -rf lib/$(LIBRARY_NAME).*
	cp target/$(I686_TARGET)/release/$(LIBRARY_NAME).a ./lib/$(LIBRARY_NAME).a
	cp target/$(I686_TARGET)/release/$(LIBRARY_NAME).so ./lib/$(LIBRARY_NAME).so

arm64: clean
	cargo build --release --target=$(ARM64_TARGET)
	mkdir -p lib
	rm -rf lib/$(LIBRARY_NAME)_arm64.*
	cp target/$(ARM64_TARGET)/release/$(LIBRARY_NAME).a ./lib/$(LIBRARY_NAME)_arm64.a
	cp target/$(ARM64_TARGET)/release/$(LIBRARY_NAME).so ./lib//$(LIBRARY_NAME)_arm64.so

update:
	cargo update

CFLAGS = -Isrc
LDFLAGS = -ldl -pthread -lssl -lcrypto -lm -O2
X86_64_LDFLAGS = ./lib/$(LIBRARY_NAME)_x86_64.a $(LDFLAGS)
I686_LDFLAGS = ./lib/$(LIBRARY_NAME).a $(LDFLAGS)
ARM64_LDFLAGS = ./lib/$(LIBRARY_NAME)_arm64.a $(LDFLAGS)

c_server_x86_64: x86_64
	$(X86_64_CC) $(CFLAGS) -O2 -c ./c_examples/c_server.c -o c_server.o
	$(X86_64_CC) -o c_server c_server.o $(X86_64_LDFLAGS)

c_server_i686: i686
	$(I686_CC) $(CFLAGS) -O2 -c ./c_examples/c_server.c -o c_server.o
	$(I686_CC) -o c_server c_server.o $(I686_LDFLAGS)
	
c_server_arm64: arm64  
	$(ARM64_CC) $(CFLAGS) -O2 -c ./c_examples/c_server.c -o c_server.o
	$(ARM64_CC) -o c_server c_server.o $(ARM64_LDFLAGS)

run:
	./c_server
	
scp:
	scp c_server debian@192.168.7.2:/home/debian/c_server 
	
debug:
	RUST_BACKTRACE=1 ./c_server 

.PHONY: clean
	 
clean: 
	rm -rf c_server */*.c.orig */*.h.orig *.o	
