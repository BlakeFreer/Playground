add_library(ctrl STATIC)

add_subdirectory(lls)

add_subdirectory(third-party/eigen)
target_link_libraries(ctrl PUBLIC Eigen3::Eigen)