#[macro_use]
extern crate criterion;
extern crate rand;
use criterion::Criterion;
mod rng;
use wrapper_des::des;

mod wrapper_des;

use transposition::real_ortho;
mod transposition;
//use criterion::Criterion;
/*main contient une fonction pour tester DES et real_ortho
 et des trois fonctions pour tester le Unit Testing
*/

fn main() {
    //variable pour tester DES

    let plain: [u64; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let key: [u64; 56] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let cipher: [u64; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let mut data: [u64; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    des(plain, key, cipher);
    let i = rng::get_rand_float_0_and_1();
    println!("random float between 0 and 1 -> {} ",i);

    let i = rng::get_rand_int_0_and_10();
    println!("random int between 0 and 10 -> {} ",i);


    real_ortho(&mut data);
    println!("hello from main.rs");
}

/************TESTS************/

pub fn hello() -> String {
    return String::from("hello world");
}

pub fn square(val: i32) -> i32 {
    return val.pow(2);
}

pub fn tab() -> [i32; 2] {
    return [0, 0];
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "hello world");
    }

    #[test]
    fn test_tab() {
        assert_eq!(tab(), [0, 0]);
    }

    fn criterion_benchmark(c: &mut Criterion){
        c.bench_function("add2",|b|b.iter(||add_two(2)));
    }
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
}
