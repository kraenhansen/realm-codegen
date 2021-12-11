// Demonstrates the binding of a constructor with arguments
interface Constructable {
  constructor(double a);
  double get_number();
};

// Demonstrates the binding of a class which takes and returns string data
interface Stringer {
  ByteString uppercase(ByteString str);
};

/*
interface OptionalArgument {
  double echo(double? a);
};
*/
