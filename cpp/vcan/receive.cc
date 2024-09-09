#include <linux/can.h>
#include <unistd.h>

#include <cstdlib>
#include <iostream>
#include <string>

#include "vcan.h"

int main() {
    std::string iface = "vcan0";

    int sock = vcan::create_can_socket(iface);

    struct can_frame frame;

    while (true) {
        int nbytes = read(sock, &frame, sizeof(struct can_frame));

        printf("0x%03X [%d] ", frame.can_id, frame.can_dlc);
        for (int i = 0; i < frame.can_dlc; i++) {
            printf("%02X ", frame.data[i]);
        }
        printf("\r\n");
    }
    return 0;
}