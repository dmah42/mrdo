cmake_minimum_required (VERSION 2.8)
project(mrdo)

set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR} ${CMAKE_MODULE_PATH})

find_package(LLVM REQUIRED)

set(CMAKE_CXX_FLAGS "-Wall -Werror -pedantic-errors --std=c++0x -Isrc/ -fvisibility-inlines-hidden ${LLVM_CFLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0 -DDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE "-fno-strict-aliasing -O3 -DNDEBUG")

include_directories(${LLVM_INCLUDE_DIR})

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
		 "src/parser/while.cc")

add_executable(mrdo ${SOURCE_FILES})
set_target_properties(mrdo PROPERTIES LINK_FLAGS ${LLVM_LFLAGS})
target_link_libraries(mrdo ${LLVM_MODULE_LIBS} dl pthread)

set(TEST_FILES "test/basic.do"
	       "test/coll.do"
	       "test/func.do"
	       "test/if.do"
	       "test/while.do")

       #add_custom_command(TARGET mrdo POST_BUILD
       #		   COMMAND ./run_tests.sh > /dev/null
       #		   MAIN_DEPENDENCY ${TESTFILE}
       #		   DEPENDS mrdo run_tests.sh
       #		   COMMENT "Running tests"
       #		   VERBATIM)

install(TARGETS mrdo
	RUNTIME DESTINATION bin)
