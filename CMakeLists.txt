cmake_minimum_required (VERSION 2.8)
project(do)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR} ${CMAKE_MODULE_PATH})

find_package(LLVM REQUIRED)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/bin)

set(CMAKE_CXX_FLAGS "-Wall -Werror -pedantic-errors --std=c++0x -fvisibility-inlines-hidden ${LLVM_CFLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-fno-strict-aliasing -O3 -DNDEBUG")

include_directories(${LLVM_INCLUDE_DIR})

FILE(GLOB SOURCE_FILES "src/*.cc")

add_executable(do ${SOURCE_FILES})
set_target_properties(do PROPERTIES LINK_FLAGS ${LLVM_LFLAGS})
target_link_libraries(do ${LLVM_MODULE_LIBS} dl pthread)

add_custom_command(TARGET do POST_BUILD
		   COMMAND bin/do ARGS < test/basic.do
		   MAIN_DEPENDENCY test/basic.do
		   DEPENDS bin/do
		   COMMENT Running basic test
		   VERBATIM)
