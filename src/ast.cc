#include "ast.h"

#include <map>
#include <string>
#include <vector>

namespace ast {
namespace {
std::vector<std::map<std::string, llvm::AllocaInst*>> named_values;
}  // end namespace

llvm::IRBuilder<> builder(llvm::getGlobalContext());

llvm::AllocaInst* GetNamedValue(const std::string& name) {
  // NOTE: this iterates forwards suggesting that shadowing breaks things.
  for (const auto& m : named_values) {
    if (m.count(name)) {
      return m.at(name);
    }
  }
  return nullptr;
}

void SetNamedValue(const std::string& name, llvm::AllocaInst* alloca) {
  named_values.back().insert(std::make_pair(name, alloca));
}

void PushNamedValueScope() { named_values.push_back({}); }
void PopNamedValueScope() { named_values.pop_back(); }

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val, llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}
}  // end namespace ast
