//
// This is only a SKELETON file for the 'Hello World' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

class HelloWorld {
  hello() {
    return this.constructor.name.split(/(?=W)/).join(", ").padEnd(13, "!");
  }
}

export default HelloWorld;
