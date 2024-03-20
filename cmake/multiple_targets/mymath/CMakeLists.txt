add_library(mymath)

target_sources(mymath
	PRIVATE
	mymath.cc
)

target_include_directories(mymath
	PUBLIC
	${CMAKE_CURRENT_SOURCE_DIR} # allows other targets to #include "mymath.h"
)