#include <linux/can.h>
#include <linux/can/raw.h>
#include <net/if.h>
#include <sys/ioctl.h>
#include <sys/socket.h>

#include <cstdio>
#include <cstring>
#include <iostream>
#include <string>

#include "vcan.h"

namespace vcan {

int create_can_socket(std::string iface) {
    int s;
    struct sockaddr_can addr;
    struct ifreq ifr;

    s = socket(PF_CAN, SOCK_RAW, CAN_RAW);
    if (s == -1) {
        perror("Error while opening socket.");
        return -1;
    }

    strcpy(ifr.ifr_name, iface.c_str());
    ioctl(s, SIOGIFINDEX, &ifr);

    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;

    if (bind(s, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        perror("Error in socket bind.");
        return -1;
    }

    std::cout << "Opened " << iface << " at index " << addr.can_ifindex
              << std::endl;

    return s;
}

}  // namespace vcan
