#ifndef _DO_TYPE_H_
#define _DO_TYPE_H_

namespace llvm { class Type; }

template <typename T> class TypeMap {
 public:
  static llvm::Type* get();
};

#endif
