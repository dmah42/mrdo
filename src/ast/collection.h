#ifndef _DO_AST_COLLECTION_H_
#define _DO_AST_COLLECTION_H_

#include "ast/expression.h"
#include "debug_log.h"

#include <iostream>
#include <vector>

namespace ast {
class Collection : public Expression {
 public:
  Collection(lexer::Position position,
             bool is_sequence,
             std::vector<const Expression*>& values)
      : Expression(position), is_sequence_(is_sequence), values_(values) {
    DebugLog(
        position, (is_sequence ? "Sequence: " : "Collection: "), values.size());
  }
  llvm::Value* Codegen() const override;
  llvm::Type* Type() const override;

  bool is_sequence() const { return is_sequence_; }
  size_t size() const { return values_.size(); }

 private:
  bool is_sequence_;
  std::vector<const Expression*> values_;
};
}  // end namespace ast

#endif
