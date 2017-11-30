class Leap {
  constructor(year) {
    this.year = year;
  }

  isLeap() {
    return new Date(Date.UTC(this.year, 1, 29)).getDate() === 29;
  }
}
export default Leap;
