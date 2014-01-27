cmake_minimum_required (VERSION 2.8)
project(do)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR} ${CMAKE_MODULE_PATH})

find_package(LLVM REQUIRED)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/bin)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/lib)

set(CMAKE_CXX_FLAGS "-Wall -Werror -pedantic-errors --std=c++0x -fvisibility-inlines-hidden ${LLVM_CFLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-fno-strict-aliasing -O3 -DNDEBUG")

include_directories(${LLVM_INCLUDE_DIR})

FILE(GLOB LIB_SOURCE_FILES "src/*.cc")
set(JIT_SOURCE_FILE "src/main.cc")
LIST(REMOVE_ITEM LIB_SOURCE_FILES ${JIT_SOURCE_FILE})

add_library(do STATIC ${LIB_SOURCE_FILES})


link_directories(${LLVM_LIBRARY_DIR})
add_executable(dojit ${JIT_SOURCE_FILE})
set_target_properties(dojit PROPERTIES LINK_FLAGS ${LLVM_LFLAGS})
target_link_libraries(dojit do ${LLVM_MODULE_LIBS} dl pthread)
