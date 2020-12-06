fn main() {
    impl Solution {
        pub fn kth_factor(n: i32, k: i32) -> i32 {

            let mut factor = 0;
            let mut i: i32;

            for i in 1..n+1
            {
                if n % i ==0
                {
                    factor +=1;
                }

                if factor == k
                {
                    pringln!(i);
                }
            }

            println!(-1);
        }
    }
}
