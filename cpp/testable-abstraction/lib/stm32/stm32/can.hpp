/// @author Blake Freer
/// @date 2024-11

#pragma once

#ifdef STM32F7
#include "stm32f7xx_hal.h"
#elif defined(STM32F4)
#include "stm32f4xx_hal.h"
#endif

#ifdef HAL_CAN_MODULE_ENABLED

#include <cstdint>

#include "abstraction.hpp"

#ifdef USE_FREERTOS
#include "FreeRTOS.h"
#include "queue.h"
#include "semphr.h"
#endif

namespace macfe::stm32f {

class CanBase : public macfe::abstraction::Can {
public:
    CanBase(CAN_HandleTypeDef* hcan);

    void Send(const abstraction::CanMessage& can_tx_msg) override;
};

}  // namespace macfe::stm32f

#endif