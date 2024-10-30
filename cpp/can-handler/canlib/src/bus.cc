#include "canlib/base.h"
#include "canlib/bus.h"

namespace can {

Bus::Bus(Base& can_base) : can_base_(can_base) {
    can_base_.RegisterBus(this);
}

}  // namespace can