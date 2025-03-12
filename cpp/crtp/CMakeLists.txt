cmake_minimum_required(VERSION 3.27)

option(MCAL "MCAL Name" ON)

project(crtp_demo LANGUAGES CXX)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)

add_executable(main main.cpp)

if(MCAL STREQUAL "stm")
    add_subdirectory(mcal-stm)
    target_link_libraries(main mcalstm)
    add_subdirectory(platform-stm)
elseif(MCAL STREQUAL "cli")
    add_subdirectory(mcal-cli)
    target_link_libraries(main mcalcli)
    add_subdirectory(platform-cli)
else()
    message(FATAL_ERROR "Invalid MCAL, must be 'stm' or 'cli'")
endif()