#ifndef _DO_AST_COLLECTION_H_
#define _DO_AST_COLLECTION_H_

#include "ast/expression.h"

#include <iostream>
#include <vector>

namespace ast {
class Collection : public Expression {
 public:
  explicit Collection(bool is_sequence, std::vector<const Expression*>& values)
      : is_sequence_(is_sequence), values_(values) {
#ifdef DEBUG
    std::cerr << (is_sequence ? "Sequence: " : "Collection: ")
              << values.size() << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;

  bool is_sequence() const { return is_sequence_; }
  size_t size() const { return values_.size(); }

 private:
  bool is_sequence_;
  std::vector<const Expression*> values_;
};
}  // end namespace ast

#endif
