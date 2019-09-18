#ifndef __WS_SERVER_H__
#define __WS_SERVER_H__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

// server descriptor
struct WSServer;

/// Library Version
/// return : Library Version
const char * server_version();

/// New WS Server
/// @`max_connections`: Max connections of Server
// @`port`: WS Server port
struct WSServer * server_new(uint32_t max_connections, uint16_t port);

/// Set WDT period (在 listen 設定才會生效)
/// @`server`: WS Server descriptor
/// @`ms`: WDT period
void server_set_wdt_period(struct WSServer * server, uint64_t ms);

/// WS Server Listen
/// @`server`: WS Server descriptor
/// return: 0 in case of success, else < 0
int32_t server_listen(struct WSServer * server);

/// WS Server close
/// @`server`: WS Server descriptor
/// return: 0 in case of success, else < 0
int32_t server_close(struct WSServer * server);

/// WS Server broadcaster (會送出訊息到所有連上的 Client)
/// @`server`: WS Server descriptor
/// @`msg`: output message
/// return: 0 in case of success, else < 0
int32_t server_broadcaster(struct WSServer * server, const char *msg);

/// Set on_open callback (當 WS Client 連上時會呼叫此 Callback function)
/// @`server`: WS Server descriptor
/// @`pointer`: data pointer
/// @`cb`: Callback function
void server_set_on_open_cb(struct WSServer * server,
                           void * pointer,
                           void (* cb)(void * pointer, const char * str));

/// Set on_error callback (當 WS Client 斷線時會呼叫此 Callback function)
/// @`server`: WS Server descriptor
/// @`pointer`: data pointer
/// @`cb`: Callback function
void server_set_on_error_cb(struct WSServer * server,
                            void * pointer,
                            void (* cb)(void * pointer, const char * str));

/// Set on_message callback (當 WS Server 接收到訊息時會呼叫此 Callback function)
/// @`server`: WS Server descriptor
/// @`pointer`: data pointer
/// @`cb`: Callback function
void server_set_on_message_cb(struct WSServer * server,
                              void * pointer,
                              void (* cb)(void * pointer, const char * str));

#ifdef __cplusplus
}
#endif

#endif
