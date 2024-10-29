#include "canlib/base.h"
#include "canlib/bus.h"
#include "canlib/msg.h"

namespace can {

void Base::Receive(RawMessage msg) {
    bus_->AddMessage(msg);
}

void Base::RegisterBus(Bus* bus) {
    bus_ = bus;
}

}  // namespace can