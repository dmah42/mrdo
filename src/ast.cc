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
std::vector<std::map<std::string, llvm::AllocaInst*>> named_values;

llvm::AllocaInst* CreateEntryBlockAlloca(llvm::Function* function,
                                         llvm::Type* type,
                                         const std::string& var) {
  llvm::IRBuilder<> tmp(&function->getEntryBlock(),
                        function->getEntryBlock().begin());
  return tmp.CreateAlloca(type, nullptr, var.c_str());
}
}  // end namespace

llvm::IRBuilder<> builder(llvm::getGlobalContext());

std::pair<llvm::AllocaInst*, llvm::Value*> CreateNamedVariable(
    llvm::Function* f, const std::string& var_name, const Expression* e) {
  // TODO: get type from Expression*
  llvm::Value* v = e->Codegen();
  const Collection* rhs_coll = dynamic_cast<const Collection*>(e);
  const Variable* rhs_v = dynamic_cast<const Variable*>(e);
  const Real* rhs_r = dynamic_cast<const Real*>(e);
  const BinaryOp* rhs_binop = dynamic_cast<const BinaryOp*>(e);
  const Call* rhs_call = dynamic_cast<const Call*>(e);
  const Func* rhs_func = dynamic_cast<const Func*>(e);
  llvm::Type* alloca_type = nullptr;
  if (rhs_coll) {
    alloca_type = TypeMap<builtin::Collection>::get();
  } else if (rhs_v || rhs_func) {
    // TODO: varargs for func
    alloca_type = v->getType();
  } else if (rhs_r || rhs_binop) {
    alloca_type = TypeMap<double>::get();
  } else if (rhs_call) {
    llvm::Function* func = engine::module->getFunction(rhs_call->name());
    if (!func) {
      ErrorCont("Unknown function: ", rhs_call->name());
      return std::make_pair(nullptr, nullptr);
    }
    alloca_type = func->getReturnType();
  } else {
    ErrorCont("Unknown rhs type: ");
    v->dump();
    return std::make_pair(nullptr, nullptr);
  }
  llvm::AllocaInst* alloca = CreateEntryBlockAlloca(f, alloca_type, var_name);
  SetNamedValue(var_name, alloca);
  return std::make_pair(alloca, v);
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

void PushNamedValueScope() { named_values.push_back({}); }
void PopNamedValueScope() { named_values.pop_back(); }

llvm::Value* ToBool(llvm::Value* val) {
  return builder.CreateFCmpUNE(
      val, llvm::ConstantFP::get(llvm::getGlobalContext(), llvm::APFloat(0.0)),
      "booltmp");
}
}  // end namespace ast
