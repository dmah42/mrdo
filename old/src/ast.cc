#include "ast.h"

#include <map>
#include <string>
#include <vector>

#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/Function.h>
#include <llvm/IR/Module.h>

#include "ast/binary_op.h"
#include "ast/call.h"
#include "ast/collection.h"
#include "ast/expression.h"
#include "ast/func.h"
#include "ast/real.h"
#include "ast/variable.h"
#include "builtin.h"
#include "llvm_type.h"

namespace ast {
namespace {
typedef std::map<std::string, llvm::AllocaInst*> NamedValues;
std::vector<NamedValues> named_values;

llvm::AllocaInst* CreateEntryBlockAlloca(llvm::Function* function,
                                         llvm::Type* type,
                                         const std::string& var) {
  llvm::IRBuilder<> tmp(&function->getEntryBlock(),
                        function->getEntryBlock().begin());
  return tmp.CreateAlloca(type, nullptr, var.c_str());
}
}  // end namespace

llvm::IRBuilder<> builder(llvm::getGlobalContext());

llvm::AllocaInst* CreateNamedVariable(llvm::Function* f,
                                      const std::string& var_name,
                                      const Expression* e) {
  llvm::Type* alloca_type = e->Type();

  if (!alloca_type) {
    ErrorCont("Unknown rhs type in assignment to '", var_name, "'");
    return nullptr;
  }
  llvm::AllocaInst* alloca = CreateEntryBlockAlloca(f, alloca_type, var_name);
  SetNamedValue(var_name, alloca);
  return alloca;
}

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

void PushNamedValueScope() { named_values.push_back(NamedValues()); }
void PopNamedValueScope() { named_values.pop_back(); }

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val,
      llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}
}  // end namespace ast
