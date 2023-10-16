cmake_minimum_required(VERSION 3.20)
project(DummyUnixSocketClient)

# Set the C++ standard (you can change it to your preferred standard)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


set(CMAKE_C_COMPILER /usr/bin/clang)
set(CMAKE_CXX_COMPILER /usr/bin/clang)

set(CMAKE_C_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra")
set(RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Add your source files here
set(SRC_FILES client.c)

# Set optimization flags and debugging symbols for the Debug build type
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -O0 -g")
# Create a Debug target
add_executable(client_debug ${SRC_FILES})



# Set optimization flags for the Release build type
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3 -march=native")
# Create a Release target
add_executable(client_release ${SRC_FILES})

