// Demonstrates the binding of a constructor with arguments
interface Constructable {
  constructor(double a);
  double get_number();
};

// Demonstrates the binding of a class which takes and returns string data
interface Stringer {
  ByteString uppercase(ByteString str);
};

dictionary ObjectifierValues {
  ByteString a_greeting;
  double a_number;
};

dictionary Unused {};

/*
dictionary FirstLevel {
  SecondLevel second;
};

dictionary SecondLevel {
  ByteString message;
};
*/

interface Objectifier {
  constructor(ObjectifierValues first_values);
  ObjectifierValues build(ObjectifierValues second_values);
  // FirstLevel echo(FirstLevel arg);
};

/*
interface OptionalArgument {
  double echo(double? a);
};
*/
