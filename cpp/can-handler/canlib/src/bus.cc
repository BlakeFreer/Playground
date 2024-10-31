#include "canlib/base.h"
#include "canlib/bus.h"

namespace can {

/**
 * Upon construction, give the Base a reference to the Bus so it can
 * send messages up upon receipt.
 */
Bus::Bus(Base& can_base) : can_base_(can_base) {
    can_base_.RegisterBus(this);
}

}  // namespace can