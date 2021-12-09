import { Adder } from ".";

const adder = new Adder();
const result = adder.performAdd(1, 2);
if (result == 3) {
  console.log("Great success!");
} else {
  console.error(`Failed - result was ${result}`);
  process.exit(1);
}
