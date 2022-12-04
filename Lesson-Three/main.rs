fn main() {
  let mut a = [5, 3, 7, 9, 1, 4, 2, 0, 6, 8];
  for i in 0..a.len() {
      for j in 0..a.len() - 1 - i {
          // 交换
          if a[j] > a[j + 1] {
              a[j] = a[j] ^ a[j + 1];
              a[j + 1] = a[j] ^ a[j + 1];
              a[j] = a[j] ^ a[j + 1];
          }
      }
  }
  print!("{:?}", a);
}