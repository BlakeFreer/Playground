# Enum-Struct

What datatypes can be used to represent enums?

Ex. Can we extend

```c++
enum class Switch : bool {
    OPEN = false,
    CLOSED = true
};
```

to more complicated types beyond `bool`/`int` like a struct of `bool`?

The purpose is to represent the BatteryMonitor FSM state and the contactor command with a single enumeration instead of using separate data types for both (see current implementation [here](https://github.com/macformula/racecar/pull/380/files#diff-1166d63edacf576dfd70567d4e2f3693ad64ddc6209a89c4c53e56647db19ef6R46)).

## Answer 1

Not trivially

```c++
struct Cmd {
    bool precharge;
    bool negative;
    bool positive;
};

enum class States : Cmd {
};
```

produces the error

```bash
$ g++ main.cpp 
main.cpp:12:21: error: underlying type 'Cmd' of 'States' must be an integral type
   12 | enum class States : Cmd {
      |                     ^~~
```

## Answer 2

Switch case makes is somewhat possible

```c++
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
```
