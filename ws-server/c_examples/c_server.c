#include <stdio.h>
#include <stdlib.h>
#include "ws_server.h"

struct Server
{
    struct WSServer *ws;
    int opened;
};

void on_open_cb(void *pointer, const char *src)
{
    struct Server *server = (struct Server *)pointer;
    server->opened = 1;
    printf("WS Open: %s\n", src);
}

void on_error_cb(void *pointer, const char *src)
{
    struct Server *server = (struct Server *)pointer;
    server->opened = 0;
    printf("WS client error: %s\n", src);
}

void on_message_cb(void *pointer, const char *src)
{
    struct Server *server = (struct Server *)pointer;
    printf("on_message: %s\n", src);

    // 回傳相同的訊息給 client
    server_broadcaster((*server).ws, src);
}

int main()
{

    struct Server server;
    server.opened = 0;

    // New WS server
    server.ws = server_new(10, 3013);
    // Set on_open callback
    server_set_on_open_cb(server.ws, (void *)&server, on_open_cb);
    // Set on_error callback
    server_set_on_error_cb(server.ws, (void *)&server, on_error_cb);
    // Set on_message callback
    server_set_on_message_cb(server.ws, (void *)&server, on_message_cb);

    // Server listen
    printf("Server Listen: %d\n", server_listen(server.ws));

    printf("Run server for 6 seconds\n");
    for (int i = 0; i < 6; i++)
    {
        printf("%d\n", i);
        sleep(1);
    }
    return 0;
}
