LIBRARY_NAME=libws_server
X86_64_TARGET=x86_64-unknown-linux-gnu
I686_TARGET=i686-unknown-linux-gnu
ARM_TARGET=armv7-unknown-linux-gnueabihf
X86_64_CC = gcc
I686_CC = gcc
ARM_CC = arm-linux-gnueabihf-gcc

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

arm: clean
	cargo build --release --target=$(ARM_TARGET)
	mkdir -p lib
	rm -rf lib/$(LIBRARY_NAME)_armv7.*
	cp target/$(ARM_TARGET)/release/$(LIBRARY_NAME).a ./lib/$(LIBRARY_NAME)_armv7.a
	cp target/$(ARM_TARGET)/release/$(LIBRARY_NAME).so ./lib//$(LIBRARY_NAME)_armv7.so

update:
	cargo update

CFLAGS = -Isrc
LDFLAGS = -ldl -pthread -lssl -lcrypto -lm -O2
X86_64_LDFLAGS = ./lib/$(LIBRARY_NAME)_x86_64.a $(LDFLAGS)
I686_LDFLAGS = ./lib/$(LIBRARY_NAME).a $(LDFLAGS)
ARM_LDFLAGS = ./lib/$(LIBRARY_NAME)_armv7.a $(LDFLAGS)

server_x86_64: x86_64
	$(X86_64_CC) $(CFLAGS) -O2 -c ./examples/server.c -o server.o
	$(X86_64_CC) -o server server.o $(X86_64_LDFLAGS)

server_i686: i686
	$(I686_CC) $(CFLAGS) -O2 -c ./examples/server.c -o server.o
	$(I686_CC) -o server server.o $(I686_LDFLAGS)
	
server_arm: arm  
	$(ARM_CC) $(CFLAGS) -O2 -c ./examples/server.c -o server.o
	$(ARM_CC) -o server server.o $(ARM_LDFLAGS)

run:
	./server
	
scp:
	scp server debian@192.168.7.2:/home/debian/server 
	
debug:
	RUST_BACKTRACE=1 ./advanserver 

.PHONY: clean
	 
clean: 
	rm -rf server */*.c.orig */*.h.orig *.o	
