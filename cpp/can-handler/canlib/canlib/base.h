/**
 * Near identical to the current CanBase class.
 */

#pragma once

#include <iostream>

#include "bus.h"
#include "format.h"
#include "msg.h"

namespace can {

class Bus;

class Base {
   public:
    virtual void Send(const RawMessage& msg) = 0;
    void Receive(RawMessage msg);

   private:
    void RegisterBus(Bus* bus);

   private:
    Bus* bus_;

    friend class Bus;
};

}  // namespace can