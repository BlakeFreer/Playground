#include <linux/can.h>
#include <linux/can/raw.h>
#include <net/if.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>

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
    struct sockaddr_can addr;
    struct can_frame frame;
    struct ifreq ifr;

    const char* ifname = "vcan0";

    int s = socket(PF_CAN, SOCK_RAW, CAN_RAW);
    if (s == -1) {
        perror("Error while opening socket");
        return -1;
    }

    strcpy(ifr.ifr_name, ifname);
    ioctl(s, SIOCGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    printf("%s at index %d\n", ifname, ifr.ifr_ifindex);

    if (bind(s, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        perror("Error in socket bind");
        return -2;
    }

    int input;
    while (true) {
        std::cout << "Enter a number (0-255) to send: ";
        std::cin >> input;
        send(s, input);
    }

    return 0;
}