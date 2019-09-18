use std::{
    ffi::{CStr, CString},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    os::raw::{c_char, c_void},
    str,
    sync::{Arc, Mutex},
    thread,
};
use ws::{util::Token, CloseCode, Handler, Handshake, Message, Result};
const EXPIRE: Token = Token(1);
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Callback Handler
struct CallbackHandler {
    /// 用來回傳指標給使用者
    pointer: *mut c_void,
    /// callback 函式指標
    callback: Box<Fn(*mut c_void, *const c_char) + Send>,
}

unsafe impl Send for CallbackHandler {}

/// WS Server
#[repr(C)]
pub struct WSServer {
    /// WS Broadcaster
    broadcaster: Option<ws::Sender>,
    /// Max connections
    max_connections: usize,
    /// Port
    port: u16,
    /// Watch Dog Timer Period
    wdt_period_ms: u64,
    /// Client 連上時的 Callback Function
    on_open_cb: Arc<Mutex<Option<CallbackHandler>>>,
    /// Client 斷線或是錯誤時的 Callback Function
    on_error_cb: Arc<Mutex<Option<CallbackHandler>>>,
    /// 接收到 Client 傳過來的訊息時的 Callback Function
    on_message_cb: Arc<Mutex<Option<CallbackHandler>>>,
}

impl WSServer {
    /// New
    pub fn new(max_connections: u32, port: u16) -> WSServer {
        WSServer {
            broadcaster: None,
            max_connections: max_connections as usize,
            port: port,
            wdt_period_ms: 30_000,
            on_open_cb: Arc::new(Mutex::new(None)),
            on_error_cb: Arc::new(Mutex::new(None)),
            on_message_cb: Arc::new(Mutex::new(None)),
        }
    }

    /// Listen
    pub fn listen(&mut self) -> Result<()> {
        let max_connections = self.max_connections;
        let port = self.port;
        let wdt_period_ms = self.wdt_period_ms;
        let on_open_cb = self.on_open_cb.clone();
        let on_error_cb = self.on_error_cb.clone();
        let on_message_cb = self.on_message_cb.clone();

        // Build WebSocket Server
        let websocket = ws::Builder::new()
            .with_settings(ws::Settings {
                max_connections: max_connections,
                ..ws::Settings::default()
            })
            .build(move |out: ws::Sender| {
                
                WSServerHandler {
                    sender: out.clone(),
                    is_timeout: true,
                    wdt_period_ms: wdt_period_ms,
                    on_open_cb: on_open_cb.clone(),
                    on_error_cb: on_error_cb.clone(),
                    on_message_cb: on_message_cb.clone(),
                }
            })?;

        self.broadcaster = Some(websocket.broadcaster());

        // WebSocket Server listen
        thread::Builder::new()
            .name("WS Server Listen".to_string())
            .spawn(move || {
                let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
                if let Err(e) = websocket.listen(address) {
                    panic!("WebSocket Server Listen Failed = {}", e);
                }
            })?;
        Ok(())
    }

    /// Close
    pub fn close(&mut self) -> Result<()> {
        if let Some(ref sender) = self.broadcaster {
            sender.close(CloseCode::Normal)?;
        }
        Ok(())
    }

    /// Broadcaster
    pub fn broadcaster(&mut self, msg: &str) -> Result<()> {
        if let Some(ref sender) = self.broadcaster {
            sender.send(Message::Text(msg.to_string()))
        } else {
            Ok(())
        }
    }

    /// Set WDT period
    pub fn set_wdt_period(&mut self, ms: u64) {
        self.wdt_period_ms = ms;
    }

    /// Set on_open callback
    pub fn set_on_open_cb<F>(&mut self, pointer: *mut c_void, handler: F)
    where
        F: Fn(*mut c_void, *const c_char) + Send + 'static,
    {
        *self.on_open_cb.lock().unwrap() = Some(CallbackHandler {
            pointer: pointer,
            callback: Box::new(handler),
        });
    }

    /// Set on_error callback
    pub fn set_on_error_cb<F>(&mut self, pointer: *mut c_void, handler: F)
    where
        F: Fn(*mut c_void, *const c_char) + Send + 'static,
    {
        *self.on_error_cb.lock().unwrap() = Some(CallbackHandler {
            pointer: pointer,
            callback: Box::new(handler),
        });
    }

    /// Set on_message callback
    pub fn set_on_message_cb<F>(&mut self, pointer: *mut c_void, handler: F)
    where
        F: Fn(*mut c_void, *const c_char) + Send + 'static,
    {
        *self.on_message_cb.lock().unwrap() = Some(CallbackHandler {
            pointer: pointer,
            callback: Box::new(handler),
        });
    }
}

#[no_mangle]
/// Library Version
pub extern "C" fn server_version() -> *const c_char {
    let version = CString::new(VERSION).expect("library version");
    version.into_raw()
}

/// New WS Server
/// @`max_connections`: Max connections of Server
/// @`port`: WS Server port
/// return: WSServer  
#[no_mangle]
pub extern "C" fn server_new(max_connections: u32, port: u16) -> Box<WSServer> {
    Box::new(WSServer::new(max_connections, port))
}

/// Set WDT period (在 listen 設定才會生效)
/// @`server`: WS Server descriptor
/// @`ms`: WDT period
#[no_mangle]
pub extern "C" fn server_set_wdt_period(server: Box<WSServer>, ms: u64) {
    let s = Box::into_raw(server);
    unsafe {
        (*s).set_wdt_period(ms);
    }
}

/// WS Server Listen
/// @`server`: WS Server descriptor
/// return: 0 in case of success, else < 0
#[no_mangle]
pub extern "C" fn server_listen(server: Box<WSServer>) -> i32 {
    let s = Box::into_raw(server);
    unsafe {
        if (*s).listen().is_ok() {
            return 0;
        } else {
            return -1;
        }
    }
}

/// WS Server close
/// @`server`: WS Server descriptor
/// return: 0 in case of success, else < 0
#[no_mangle]
pub extern "C" fn server_close(server: Box<WSServer>) -> i32 {
    let s = Box::into_raw(server);
    unsafe {
        if (*s).close().is_ok() {
            return 0;
        } else {
            return -1;
        }
    }
}

/// WS Server broadcaster (會送出訊息到所有連上的 Client)
/// @`server`: WS Server descriptor
/// @`msg`: output message
/// return: 0 in case of success, else < 0
#[no_mangle]
pub extern "C" fn server_broadcaster(server: Box<WSServer>, msg: *const c_char) -> i32 {
    let message = unsafe {
        assert!(!msg.is_null());
        str::from_utf8(CStr::from_ptr(msg).to_bytes()).unwrap()
    };
    let s = Box::into_raw(server);
    unsafe {
        if (*s).broadcaster(message).is_ok() {
            return 0;
        } else {
            return -1;
        }
    };
}

/// Set on_open callback
/// @`server`: WS Server descriptor
/// @`cb`: Callback function
#[no_mangle]
pub extern "C" fn server_set_on_open_cb(
    server: Box<WSServer>,
    pointer: *mut c_void,
    cb: fn(*mut c_void, *const c_char),
) {
    let s = Box::into_raw(server);
    unsafe { (*s).set_on_open_cb(pointer, cb) };
}

/// Set on_error callback
/// @`server`: WS Server descriptor
/// @`cb`: Callback function
#[no_mangle]
pub extern "C" fn server_set_on_error_cb(
    server: Box<WSServer>,
    pointer: *mut c_void,
    cb: fn(*mut c_void, *const c_char),
) {
    let s = Box::into_raw(server);
    unsafe { (*s).set_on_error_cb(pointer, cb) };
}

/// Set on_message callback
/// @`server`: WS Server descriptor
/// @`cb`: Callback function
#[no_mangle]
pub extern "C" fn server_set_on_message_cb(
    server: Box<WSServer>,
    pointer: *mut c_void,
    cb: fn(*mut c_void, *const c_char),
) {
    let s = Box::into_raw(server);
    unsafe { (*s).set_on_message_cb(pointer, cb) };
}

/// WS Server Handler
pub struct WSServerHandler {
    /// WS Sender
    sender: ws::Sender,
    /// 用來檢查 Client 是否都沒有送訊息過來
    is_timeout: bool,
    /// Watchdog period
    wdt_period_ms: u64,
    /// on_open callback
    on_open_cb: Arc<Mutex<Option<CallbackHandler>>>,
    /// on_error callback
    on_error_cb: Arc<Mutex<Option<CallbackHandler>>>,
    /// on_message callback
    on_message_cb: Arc<Mutex<Option<CallbackHandler>>>,
}

impl WSServerHandler {
    /// Send Message
    pub fn send_message(&mut self, msg: &str) -> Result<()> {
        self.sender.send(Message::Text(msg.to_string()))
    }

    /// Execute on_error callback
    fn execute_on_error_cb(&mut self, msg: &str) {
        if let Some(ref cb) = *self.on_error_cb.lock().expect("execute_on_error_cb") {
            let mut temp_msg = String::from(msg).into_bytes();
            temp_msg.push(0);
            let msg = CStr::from_bytes_with_nul(temp_msg.as_slice())
                .expect("toCstr")
                .as_ptr();
            (cb.callback)(cb.pointer, msg);
        }
    }

    /// Execute on_open callback
    fn execute_on_open_cb(&self) {
        if let Some(ref cb) = *self.on_open_cb.lock().expect("execute_on_open_cb") {
            let mut temp_msg = String::from("WS Connected".to_owned()).into_bytes();
            temp_msg.push(0);
            let msg = CStr::from_bytes_with_nul(temp_msg.as_slice())
                .expect("toCstr")
                .as_ptr();

            (cb.callback)(cb.pointer, msg);
        }
    }
}

impl Handler for WSServerHandler {
    /// On open
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.execute_on_open_cb();
        // 每 EXPIRE_MS 秒檢查是否沒有任何連線活動
        self.sender.timeout(self.wdt_period_ms, EXPIRE)
    }

    /// On message
    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        self.is_timeout = false;
        if let ws::Message::Text(text) = msg {
            if let Some(ref cb) = *self.on_message_cb.lock().unwrap() {
                if text.len() > 0 {
                    let mut temp_msg = text.into_bytes();
                    // 如果不是換行結束的,補上換行符號,如果沒有在 C 的輸出有問題
                    if temp_msg[temp_msg.len() - 1] != 10 {
                        temp_msg.push(10);
                    }
                    temp_msg.push(0);
                    let msg = CStr::from_bytes_with_nul(temp_msg.as_slice())
                        .expect("toCstr")
                        .as_ptr();
                    (cb.callback)(cb.pointer, msg);
                }
            }
            Ok(())
        } else {
            self.sender.send(Message::Text(
                r#"{"ws_error":"Invalid WS Message Type"}"#.to_string(),
            ))
        }
    }

    /// On_timeout
    fn on_timeout(&mut self, _: Token) -> Result<()> {
        if self.is_timeout {
            self.sender.close_with_reason(CloseCode::Away, "Timeout")
        } else {
            self.is_timeout = true;
            // 每 EXPIRE_MS 秒檢查是否沒有任何連線活動
            self.sender.timeout(self.wdt_period_ms, EXPIRE)
        }
    }

    /// On Close
    fn on_close(&mut self, _: CloseCode, reason: &str) {
        self.execute_on_error_cb(reason);
    }
}
