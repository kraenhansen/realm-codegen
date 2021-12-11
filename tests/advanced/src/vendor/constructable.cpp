#include <stdio.h>

#include "constructable.h"

Constructable::Constructable(double a) : n{a} {
  printf("Constructed Foo(%f)\n", a);
};
double Constructable::get_number() { return n; }
