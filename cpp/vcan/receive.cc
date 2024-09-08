#include <linux/can.h>
#include <linux/can/raw.h>
#include <net/if.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#include <cstdlib>
#include <iostream>

int main() {
    const char* ifname = "vcan0";

    int s = socket(PF_CAN, SOCK_RAW, CAN_RAW);
    if (s == -1) {
        std::cout << "Error opening socket." << std::endl;
        return 1;
    }

    struct ifreq ifr;
    strcpy(ifr.ifr_name, "vcan0");
    ioctl(s, SIOCGIFINDEX, &ifr);

    struct sockaddr_can addr;
    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    printf("%s at index %d\n", ifname, ifr.ifr_ifindex);

    if (bind(s, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        perror("Error in socket bind");
        return -2;
    }

    struct can_frame frame;

    while (true) {
        int nbytes = read(s, &frame, sizeof(struct can_frame));

        printf("0x%03X [%d] ", frame.can_id, frame.can_dlc);
        for (int i = 0; i < frame.can_dlc; i++) {
            printf("%02X ", frame.data[i]);
        }
        printf("\r\n");
    }
    return 0;
}