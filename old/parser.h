#ifndef PARSER_H_
#define PARSER_H_

#include <memory>

#include "ast.h"

namespace parser {
ast::Function* Function();
ast::Function* TopLevel();
ast::Prototype* Native();
}  // end parser

#endif
