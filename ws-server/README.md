# Botnana websocket server library

## Build the library

Install rust compiler, see https://rustup.rs/ .
After the installation, login again so that the rustup/cargo command can be found.

Install the arm64 toolchain:

```
rustup target add aarch64-unknown-linux-gnu
```

Ather the toolchain is installed, execute the following commands to build the library.

```
make arm64
```

---
以下是舊資料，更新中。
---

### 目錄結構

```
ws-server
    |----> examples
    |             |-----> server.c     server example
    |----> lib                         編譯出來的靜態連結檔
    |             |-----> armv7-unknown-linux-gnueabihf/libws_server.a
    |             |-----> i686-unknown-linux-gnu/libws_server.a
    |----> src
    |             |-----> lib.rs
    |             |-----> ws_server.h   引用時所需要的 C 語言 Header file
    |             |-----> ws_server.rs  函式庫原始碼 (Rust)
    |----> Cargo.lock                   函式庫編譯時引用的套件資訊
    |----> Cargo.toml                   函式庫描述檔
    |----> makefile
    |----> readme.md
```

### 程式開發指令

* 函式說明請參考 src/ws_server.h
* 版號： Cargo.toml 中 [package] version 

**以下命令在虛擬機內的終端機內執行**

* 更新引用函式庫 `make update`
* 編譯出 Linux 引用的函式庫連結檔 

```
For x86 
$ make x86

For arm
$ make arm
```
* 編譯範例 server

```
For x86 
$ make server_x86

For arm64
$ make server_arm64

```

* 執行的命令
```
$ ./server
```
