#include <stdio.h>

#include "constructable.h"

Constructable::Constructable(double a) : n{a} {};
double Constructable::get_number() { return n; }
