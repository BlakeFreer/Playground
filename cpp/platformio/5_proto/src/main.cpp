#include <iostream>

#include "demo.pb.h"
#include "pb_decode.h"
#include "pb_encode.h"

int main(void) {
    Command command{.ecu = ECU_FRONTCONTROLLER, .data = 1U << 6};

    uint8_t buffer[Command_size];
    pb_ostream_t ostream = pb_ostream_from_buffer(buffer, sizeof(buffer));

    if (!pb_encode(&ostream, &Command_msg, &command)) {
        std::cerr << "Failed to encode command: " << PB_GET_ERROR(&ostream)
                  << std::endl;
        return 1;
    }
    std::cout << "Encoded command successfully." << std::endl;

    for (size_t i = 0; i < ostream.bytes_written; i++) {
        std::cout << "Byte " << i << ": " << static_cast<int>(buffer[i])
                  << std::endl;
    }

    std::cout << "Bytes written: " << ostream.bytes_written << std::endl;

    // For demonstration, let's decode it back
    Command decoded_command;
    pb_istream_t istream =
        pb_istream_from_buffer(buffer, ostream.bytes_written);

    if (pb_decode(&istream, Command_fields, &decoded_command)) {
        std::cout << "Decoded command successfully." << std::endl;
        std::cout << "ECU: " << decoded_command.ecu
                  << ", Data: " << decoded_command.data << std::endl;
    } else {
        std::cerr << "Failed to decode command: " << PB_GET_ERROR(&istream)
                  << std::endl;
        return 1;
    }

    return 0;
}