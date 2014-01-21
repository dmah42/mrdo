#include <cstdio>

// native methods to expose to do
// TODO: register them to remove the need for the 'native' keyword.
extern "C" {

double printd(double d) {
  char ret = putchar((char)d);
  putchar('\n');
  return ret;
}

}
