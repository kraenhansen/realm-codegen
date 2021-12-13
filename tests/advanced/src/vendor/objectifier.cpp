#include <stdexcept>
#include <stdio.h>

#include "objectifier.h"
#include "vendor/first_level.h"

Objectifier::Objectifier(ObjectifierValues prefixes) : prefixes{prefixes} {};

ObjectifierValues Objectifier::build(ObjectifierValues suffixes) {
  return ObjectifierValues{
      prefixes.a_greeting + suffixes.a_greeting,
      prefixes.a_number + suffixes.a_number,
  };
};

FirstLevel Objectifier::echo(FirstLevel arg) { return arg; };
