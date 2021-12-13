#pragma once

#include "first_level.h"
#include "objectifier_values.h"

class Objectifier {
public:
  explicit Objectifier(ObjectifierValues prefixes);
  ObjectifierValues build(ObjectifierValues suffixes);
  FirstLevel echo(FirstLevel arg);

private:
  ObjectifierValues prefixes;
};
