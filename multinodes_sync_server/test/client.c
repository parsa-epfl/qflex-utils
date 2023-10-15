#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <assert.h>

// Define the UNIX socket, the dummy client will try to connect to
#define SOCKET_PATH "/var/run/ss"
#ifdef DNDEBUG
    // Unset printf if DEBUG is not defined
    #define printf(x...)
#endif

// Fixed byte size packet for incoming message
const size_t MAX_PACKET_SIZE = 256;

/**
 * Command type and their decimal representation.
 * Should always be coherent with the server equivalent stucture
 */
enum Command
{
    Null      = 0, // Do nothing
    Stop      = 1, // Stop multinode-execution
    Start     = 2, // Start multinode-execution
    Snap      = 3, // Snapshot the current state [with filename arg]
    NoFence   = 4, // Remove the thread syncronisation. Shouldn't wait for server approval to continue
    Fence     = 5, // Set budget [with budget arg]
    Terminate = 6, // Kill Instance
};

/**
 * Buffer to process incoming and outgoing communication with the server
 *
 * The Payload is usefull for some communication
 * SNAP:
 * 0           31              63                                 255
 * +---------+------------+----------------------------+
 * |    ID     |   STR LEN   |         PAYLOAD STR             |
 * +---------+------------+----------------------------+
 *
 *  The filename is contained in the payload,
 *  which is not EXPLICITLY null terminated
 *
 * FENCE:
 *
 * 0           31              95
 * +---------+------------+
 * |    ID     |   BUDGET    |
 * +---------+------------+
 *
 * The budget is an unsigned 64 bits number
 *
 */
struct MessageBuffer
        {
    u_int32_t id;

    union
    {
        struct
        {
            u_int64_t payload_str_len;
            u_int8_t payload_str[244];
        };
        u_int32_t payload_u32;

    };

// Packing the structure help avoid padding,
// which make some bytes disappear some times
}__attribute__((packed));

/**
 * A standardized structure to communicate with the internal API
 */
struct Result
{
    enum Command cmd;
    union
    {
        u_int8_t filename[244];
        u_int32_t budget;
    };
};

/**
 *
 * @param buffer A buffer with data from the socket
 * @param res A standardized structure to communicate with the internal API
 */
void parse_message_to_command(struct MessageBuffer* buffer, struct Result* res)
{
    // Cast id to Command enum
    res->cmd = (enum Command)buffer->id;

    // Return directly for most message type that does not have a payload
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

    // Parse Payload
    switch (res->cmd)
    {
        case Snap:
            printf("String of size: %zd\n", (size_t)buffer->payload_str_len);
            strcpy((char*)&res->filename, (char*)&buffer->payload_str);
            break;

        case Fence:
            res->budget = buffer->payload_u32;
            break;

        default:
            perror("No implementation for parsing the payload");
            exit(EXIT_FAILURE);
    }
}

struct Result accept_new_message(int client_socket)
{

        /**
         * Create a Message buffer from the incoming pacquet
         */

        // Reset buffer
        struct MessageBuffer *buffer = malloc(sizeof(struct MessageBuffer));
        memset(buffer, 0, sizeof(struct MessageBuffer));

        /**
         * Actually receive the data from the server
         */
        ssize_t bytes_received = recv(client_socket, buffer, MAX_PACKET_SIZE, 0);
        if (bytes_received == -1)
        {
            perror("Could not receive from socket");
            exit(EXIT_FAILURE);
        }

        /**
         * We should have received 64 bytes exactly
         */
        printf("Received: %zd bytes\n", bytes_received);
        assert(bytes_received == MAX_PACKET_SIZE);

        // ─────────────────────────────────────────────────────────────
        struct Result res;
        memset(&res, 0, sizeof(res));
        parse_message_to_command(buffer, &res);


        printf("\tMessage:\n");
        printf("\t\tid: %u\n", res.cmd);
        printf("\t\tPayload: %s\n", res.filename);
        printf("\t\tPayload: %u\n\n", res.budget);


        free(buffer);
        return res;

}

void send_done_message(int client_socket)
{
       const static char message[] = "DONE";

       if (send(client_socket, message, strlen(message), 0) == -1)
       {
           perror("Send failed");
           exit(EXIT_FAILURE);
       }

       printf("Send DONE message");
}

int main()
{
    /**
     * Aquire socket, configure communication type,
     * here: SOCK_STREAM = TCP
     * otherwise: SOCK_DGRAM = UDP
     */
    int client_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    if (client_socket == -1)
    {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    /**
     * Prepare TCP paquet structure, socket type
     */
    struct sockaddr_un server_address;
    memset(&server_address, 0, sizeof(struct sockaddr_un));
    server_address.sun_family = AF_UNIX;
    strcpy(server_address.sun_path, SOCKET_PATH);

    /**
     * Connect to server through UNIx socket
     */
    if (connect(client_socket, (struct sockaddr *)&server_address, sizeof(struct sockaddr_un)) == -1)
    {
        perror("Connection failed");
        exit(EXIT_FAILURE);
    }

    printf("Connected to socket %s \n", server_address.sun_path);



    // ─────────────────────────────────────────────────────────────


    struct Result res;
    while(1)
    {
        /**
         * Looping into te receiving
         */

        while (1)
        {
            res = accept_new_message(client_socket);

            if (res.cmd == Start) {
                printf("Got message %2x => Breaking from loop\n", res.cmd);
                break;
            }
        }

        printf("Out of inner loop\n");
        // Run
        // if (res.cmd == Terminate) break;
        send_done_message(client_socket);
    }

    //? Don't forget to free the buffer and close the socket
    close(client_socket);

    return 0;
}