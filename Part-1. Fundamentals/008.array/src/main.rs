fn main() {


    // let array1 = ["one", "two", "three"]; // This one is type
    // let array2 = ["One", "Two", "Five", "Six"]; // But this one is type [&str: 4]


    // let arr = [1, 3, 3, 5, 0, 22];
    //
    // let n = 3;
    // let mut k =2;
    //
    // println!("{}", arr[1] + arr[2]);



    // 509. Fibonacci Number (Leetcode)

    let mut array = [0; 31];

    array[0] = 0;
    array[1] = 1;
    let mut k=2;
    let n = 20;

    for i in 0..n-1
    {
        array[k] += array[i] + array[i+1];

        k+=1;
    }

   println!("result: {}", array[k-1]);
}
