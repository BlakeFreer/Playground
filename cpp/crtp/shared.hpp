#pragma once

namespace shared {

template <typename T>
class DigitalOut {
public:
    void Set(bool value) {
        static_cast<T*>(this)->InnerSet(value);
    }
};

}  // namespace shared
