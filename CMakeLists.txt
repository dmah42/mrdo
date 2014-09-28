cmake_minimum_required (VERSION 2.8)
project(mrdo)

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR})

find_package(LLVM REQUIRED)
find_package(gflags REQUIRED)

set(CMAKE_CXX_FLAGS "-Wall -Werror -pedantic-errors --std=c++11 -fvisibility-inlines-hidden ${LLVM_CFLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-fno-strict-aliasing -O3 -DNDEBUG")

# Compiler specific C++11 tests.
if ("${CMAKE_CXX_COMPILER_ID}" MATCHES "GNU")
  execute_process(COMMAND ${CMAKE_CXX_COMPILER} -dumpversion OUTPUT_VARIABLE GCC_VERSION)
  if (NOT (GCC_VERSION VERSION_GREATER 4.7 OR GCC_VERSION VERSION_EQUAL 4.7))
    message(FATAL_ERROR "${PROJECT_NAME} requires g++ 4.7 or greater or clang.")
  endif()
elseif ("${CMAKE_CXX_COMPILER_ID}" MATCHES "Clang")
  #set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++")
else()
  message(FATAL_ERROR "${PROJECT_NAME} requires g++ 4.7 or greater or clang.")
endif()

include_directories(${LLVM_INCLUDE_DIR} ${GFLAGS_INCLUDE_DIR} "src/")

set(SOURCE_FILES "src/ast.cc"
		 "src/ast/binary_op.cc"
		 "src/ast/call.cc"
		 "src/ast/collection.cc"
		 "src/ast/func.cc"
		 "src/ast/if.cc"
		 "src/ast/program.cc"
		 "src/ast/prototype.cc"
		 "src/ast/real.cc"
		 "src/ast/return.cc"
		 "src/ast/unary_op.cc"
		 "src/ast/variable.cc"
		 "src/ast/while.cc"
		 "src/builtin.cc"
		 "src/engine.cc"
		 "src/error.cc"
		 "src/lexer.cc"
		 "src/llvm_type.cc"
		 "src/main.cc"
		 "src/parser/binary.cc"
		 "src/parser/collection.cc"
		 "src/parser/do.cc"
		 "src/parser/expression.cc"
		 "src/parser/func.cc"
		 "src/parser/ident.cc"
		 "src/parser/if.cc"
		 "src/parser/nested.cc"
		 "src/parser/program.cc"
		 "src/parser/real.cc"
		 "src/parser/return.cc"
		 "src/parser/rvalue.cc"
		 "src/parser/statement.cc"
		 "src/parser/unary.cc"
		 "src/parser/while.cc"
     "src/stopwatch.cc"
		 "src/threadpool.cc")

add_executable(mrdo ${SOURCE_FILES})
set_target_properties(mrdo PROPERTIES LINK_FLAGS ${LLVM_LFLAGS})
target_link_libraries(mrdo ${LLVM_MODULE_LIBS} ${GFLAGS_LIBRARY} dl pthread)

set(TEST_FILES "test/basic.do"
	       "test/coll.do"
	       "test/filter.do"
	       "test/func.do"
	       "test/ident.do"
	       "test/if.do"
	       "test/io.do"
	       "test/real.do"
	       "test/while.do")

       #add_custom_command(TARGET mrdo POST_BUILD
       #		   COMMAND ./run_tests.sh > /dev/null
       #		   MAIN_DEPENDENCY ${TESTFILE}
       #		   DEPENDS mrdo run_tests.sh
       #		   COMMENT "Running tests"
       #		   VERBATIM)

install(TARGETS mrdo
	RUNTIME DESTINATION bin)
