cmake_minimum_required (VERSION 2.8)
project(do)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR} ${CMAKE_MODULE_PATH})

find_package(LLVM REQUIRED)

set(CMAKE_CXX_FLAGS "-Wall -Werror -pedantic-errors --std=c++0x -Isrc/ -fvisibility-inlines-hidden ${LLVM_CFLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-fno-strict-aliasing -O3 -DNDEBUG")

include_directories(${LLVM_INCLUDE_DIR})

file(GLOB_RECURSE SOURCE_FILES "src/*.cc")

add_executable(do ${SOURCE_FILES})
set_target_properties(do PROPERTIES LINK_FLAGS ${LLVM_LFLAGS})
target_link_libraries(do ${LLVM_MODULE_LIBS} dl pthread)

file(GLOB TEST_FILES "test/*.do")
foreach(TESTFILE ${TEST_FILES})
	add_custom_command(TARGET do POST_BUILD
			   COMMAND do ARGS ${TESTFILE} > /dev/null
			   MAIN_DEPENDENCY ${TESTFILE}
			   DEPENDS do
			   COMMENT "Running test: ${TESTFILE}"
			   VERBATIM)
endforeach()

install(TARGETS do
	RUNTIME DESTINATION bin)
