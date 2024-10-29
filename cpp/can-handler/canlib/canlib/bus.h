#pragma once

#include "base.h"
#include "msg.h"

namespace can {

class Bus {
   public:
    Bus(Base& can_base) : can_base_(can_base) {
        can_base_.register_bus(this);
    }

    template <Message T>
    T GetLatest() const;

    template <Message T>
    void Send(T msg) {
        can_base_.send(msg.encode());
    }

   private:
    virtual void AddMessage(const RawMessage& msg) = 0;
    Base& can_base_;

    friend class Base;
};

}  // namespace can