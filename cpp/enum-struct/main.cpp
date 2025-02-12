enum class BmState {
    INIT,
    STARTUP,
    PRECHARGE,
    PRECHARGE_DONE,
    RUNNING,
};

namespace Contactor {

// Feeback and Command use opposite logic
enum class Feedback : bool {
    IS_OPEN = true,
    IS_CLOSED = false
};
enum class Command : bool {
    OPEN = false,
    CLOSE = true
};

struct CommandSet {
    Command negative;
    Command precharge;
    Command positive;
};

CommandSet GetCmd(BmState state) {
    using enum Command;
    using enum BmState;
    switch (state) {
        default:
        case INIT:
            return {.negative = OPEN, .precharge = OPEN, .positive = OPEN};
        case STARTUP:
            return {.negative = CLOSE, .precharge = OPEN, .positive = OPEN};
        case PRECHARGE:
            return {.negative = CLOSE, .precharge = CLOSE, .positive = OPEN};
        case PRECHARGE_DONE:
            return {.negative = CLOSE, .precharge = CLOSE, .positive = CLOSE};
        case RUNNING:
            return {.negative = CLOSE, .precharge = OPEN, .positive = CLOSE};
    }
}
}  // namespace Contactor
