
cmake_minimum_required(VERSION 3.0)
project(DummyUnixSocketClient)

# Specify the C++ standard (CMake 3.1 and above)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set the source files for the client
set(SRC_FILES client.c)

# Create an executable for the client
add_executable(client ${SRC_FILES})

# Debugging flags
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_VERBOSE_MAKEFILE False)

# Set the path to the GDB executable
set(CMAKE_C_COMPILER gcc)
set(CMAKE_CXX_COMPILER g++)

# Specify the compiler flags for debugging
set(CMAKE_C_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_DEBUG "-g")

# Include the necessary libraries (sockets)
#target_link_libraries(client -lm)

# Output directory for the executable
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/bin)

# Create a custom target for debugging
add_custom_target(debug
        COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target client
        COMMAND ${CMAKE_C_COMPILER} -g -o ${EXECUTABLE_OUTPUT_PATH}/client ${SRC_FILES}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        COMMENT "Building and debugging client..."
)
