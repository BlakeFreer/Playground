# vcan

Create 2 programs which interface over SocketCAN (vcan).

1. __Send__ a CAN message.
2. __Receive__ a the message and print.

## Setup

_From <https://en.wikipedia.org/wiki/SocketCAN>_

```bash
modprobe can
modprobe can_raw
modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
ip link show vcan0
```