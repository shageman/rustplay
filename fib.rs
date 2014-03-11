fn fib(n: int) -> int {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    fib(n-1) + fib(n-2)
  }
}

fn main() {
  println(fib(10).to_str());
}
