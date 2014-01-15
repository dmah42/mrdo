#ifndef _PARSER_H_
#define _PARSER_H_

#include <memory>

#include "ast.h"

namespace parser {

void SetBinaryOpPrecedence(char c, int p);

ast::Function* Function();
ast::Function* TopLevel();
ast::Prototype* Extern();

}  // end parser

#endif
