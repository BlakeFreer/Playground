#include "../bindings.hpp"

#include "../shared.hpp"
#include "mcal.hpp"

namespace bindings {

shared::DigitalOut<mcal::DigitalOut> light{};

}  // namespace bindings