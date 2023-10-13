#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <assert.h>

#define SOCKET_PATH "/var/run/ss"
#define LEN(array) sizeof(array)/sizeof(array[0])

enum Command
{
    Null      = 0,
    Stop      = 1,
    Start     = 2,
    Snap      = 3,
    NoFence   = 4,
    Fence     = 5,
    Terminate = 6,
};

struct Message
{
    u_int32_t id;
    union {
        u_int8_t payload_str[60];
        u_int32_t payload_u32;
    };
};

struct Result {
    enum Command cmd;
    union {
        u_int8_t filename[60];
        u_int32_t budget;
    };
};

void parse_message_to_command(struct Message* buffer, struct Result* res)
{
    res->cmd = (enum Command)buffer->id;

    switch (res->cmd)
    {
        case Null:
        case Stop:
        case Start:
        case NoFence:
        case Terminate:
            return;

        default:
            break;
    }

    u_int32_t value = 0;

    switch (res->cmd)
    {
        case Snap:
            printf("String of size: %zd\n", (size_t)buffer->payload_str[0]);
            // buffer->payload_str[LEN(buffer->payload_str) - 1] = '\0';
            strcpy((char*)&res->filename, (char*)&buffer->payload_str[1]);
            break;

        case Fence:
            res->budget = buffer->payload_u32;
            break;

        default:
            break;
    }
}

int main()
{
    int client_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    if (client_socket == -1)
    {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    struct sockaddr_un server_address;
    memset(&server_address, 0, sizeof(struct sockaddr_un));
    server_address.sun_family = AF_UNIX;
    strcpy(server_address.sun_path, SOCKET_PATH);

    if (connect(client_socket, (struct sockaddr *)&server_address, sizeof(struct sockaddr_un)) == -1)
    {
        perror("Connection failed");
        exit(EXIT_FAILURE);
    }

    printf("Connected to socket \n");

    /**
     * Create a Message buffer from
     */
    struct Message *buffer = malloc(sizeof(struct Message));
    memset(buffer, 0, sizeof(struct Message));

    // ─────────────────────────────────────────────────────────────

    /**
     * Looping into te receiving
     */
    while (1)
    {

        /**
         * Actually receive the data from the server
         */
        ssize_t bytes_received = recv(client_socket, buffer, sizeof(buffer), 0);
        if (bytes_received == -1)
        {
            perror("Receive failed");
            exit(EXIT_FAILURE);
        }

        /**
         * We should have received 64 bytes exactly
         */
        printf("Received: %zd bytes\n", bytes_received);
        // assert(bytes_received == 64);

        // ─────────────────────────────────────────────────────────────
        struct Result res;
        memset(&res, 0, sizeof(res));
        parse_message_to_command(buffer, &res);
        printf("Result:\n");
        printf("\tid: %u\n", res.cmd);
        // printf("\tPayload: %s\n", res.filename);
        // printf("\tPayload: %u\n", res.budget);
    }
    free(buffer);
    close(client_socket);

    return 0;
}