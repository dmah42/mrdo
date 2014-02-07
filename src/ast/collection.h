#ifndef _DO_AST_COLLECTION_H_
#define _DO_AST_COLLECTION_H_

#include "ast/expression.h"

#include <iostream>
#include <vector>

namespace ast {
class Collection : public Expression {
 public:
  explicit Collection(std::vector<const Expression*>& values)
      : values_(values) {
#ifdef DEBUG
    std::cerr << "Collection: " << values.size() << "\n";
#endif
  }
  virtual llvm::Value* Codegen() const;
  size_t size() const { return values_.size(); }

 private:
  std::vector<const Expression*> values_;
};
}  // end namespace ast

#endif
