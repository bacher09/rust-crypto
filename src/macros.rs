

// this macro can be used for loop unrolling
// http://en.wikipedia.org/wiki/Loop_unrolling
// In code block you can use const I to access
// loop number.
// Example usage:
// const_loop!([1, 2, 4, 5, {println!("{}", I)});
// output:
// 1
// 2
// 4
// 5
macro_rules! const_loop {
    ([$($i:expr),*], $code:block) => {
        $({
            const I: usize = $i;
            $code;
        };)*
    }
}
