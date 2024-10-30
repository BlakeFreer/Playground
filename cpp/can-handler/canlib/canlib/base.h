/**
 * Near identical to the current CanBase class.
 */

#pragma once

#include <iostream>

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
    virtual uint32_t GetTimestamp() = 0;

    Bus* bus_;

    friend class Bus;
};

}  // namespace can