#include "canlib/base.h"
#include "canlib/bus.h"
#include "canlib/msg.h"

namespace can {

/**
 * Upon construction, given the Base a reference to the Bus to it can
 * send messages up upon receipt.
 */
Bus::Bus(Base& can_base) : can_base_(can_base) {
    can_base_.RegisterBus(this);
}

/**
 * Relay the message to the base.
 */
void Bus::Send(RawMessage msg) {
    can_base_.Send(msg);
}

}  // namespace can