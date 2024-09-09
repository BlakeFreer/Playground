#include <linux/can.h>
#include <unistd.h>

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>

#include "vcan.h"

void send(int socket, uint8_t number) {
    static int message_count = 0;
    message_count++;

    struct can_frame frame;
    frame.can_id = 0x123;
    frame.can_dlc = 2;
    frame.data[0] = message_count;
    frame.data[1] = number;

    int nbytes = write(socket, &frame, sizeof(struct can_frame));
    std::printf("Sent %d bytes\n", nbytes);
}

int main(void) {
    std::string iface = "vcan0";

    int sock = vcan::create_can_socket(iface);
    if (sock < 0) {
        std::cout << "Failed to create a socket." << std::endl;
        return 1;
    }

    int input;
    while (true) {
        std::cout << "Enter a number (0-255) to send: ";
        std::cin >> input;
        send(sock, input);
    }

    return 0;
}