#include "stringer.h"

#include <algorithm>

std::string Stringer::uppercase(std::string str) {
  std::transform(str.begin(), str.end(), str.begin(), ::toupper);
  return str;
};
