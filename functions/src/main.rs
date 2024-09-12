fn main() {
  println!("Hello, world!");

  another_function(200);
  print_labeled_measurement(5, 'h');
  function_with_return()
}

fn another_function(x: i32) {
  println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn function_with_return() {
  let x_1 = five();
  let x_2 = plus_one(5);

  println!("The value of x_1 is: {x_1}");
  println!("The valye of x_2 is: {x_2}")
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
