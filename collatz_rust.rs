// Step one number
fn collatz_step(n: u64) -> u64 {
  if n % 2 == 0 { n / 2 }
  else { 3 * n + 1 }
}

// Iterate all values of n in the Collatz function
fn collatz(n: u64) -> Vec<u64> {
  let mut x = n;
  let mut nums = vec![0; 0];

  while x >= 2 {
    nums.push(x);
    x = collatz_step(x);
  }

  nums
}

fn main() {
  let first: u64 = 100; // Number to start with
  let collatz_nums: Vec<u64> = collatz(first);
  println!("{:?}", collatz_nums);
}
