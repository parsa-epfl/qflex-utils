#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>

#define SOCKET_PATH "/run/ss"

int main() {
    int client_socket = socket(AF_UNIX, SOCK_STREAM, 0);
    if (client_socket == -1) {
        perror("Socket creation failed");
        exit(EXIT_FAILURE);
    }

    struct sockaddr_un server_address;
    memset(&server_address, 0, sizeof(struct sockaddr_un));
    server_address.sun_family = AF_UNIX;
    strcpy(server_address.sun_path, SOCKET_PATH);

    if (connect(client_socket, (struct sockaddr *)&server_address, sizeof(struct sockaddr_un)) == -1) {
        perror("Connection failed");
        exit(EXIT_FAILURE);
    }

    printf("Connected to socket \n");


    while (1) {
        char buffer[64];
        memset(&buffer, 0, sizeof(buffer));


        ssize_t bytes_received = recv(client_socket, buffer, sizeof(buffer), 0);
        if (bytes_received == -1) {
            perror("Receive failed");
            exit(EXIT_FAILURE);
        }
        buffer[bytes_received] = '\0';


        printf("Received: \n");
        for (size_t i = 0 ; i < sizeof(buffer); i++) {
            printf(" %x", buffer[i]);
        }
        printf("\n");
    }

    close(client_socket);

    return 0;
}