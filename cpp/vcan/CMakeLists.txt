cmake_minimum_required(VERSION 3.15)
project(vcan)

# Specify the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

add_subdirectory(vcan)

# Add executable targets
add_executable(send send.cc)
add_executable(receive receive.cc)

target_link_libraries(send PRIVATE vcan)
target_link_libraries(receive PRIVATE vcan)