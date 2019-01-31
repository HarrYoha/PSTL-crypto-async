use wrapper_des::des;

mod wrapper_des;

use transposition::real_ortho;

mod transposition;

/*main contient une fonction pour tester DES et real_ortho
 et des trois fonctions pour tester le Unit Testing
*/
fn main() {

    //variable pour tester DES

    let plain: [u64; 64] = [0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0];

    let key: [u64; 56] = [0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0];

    let cipher: [u64; 64] = [0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
    ];

    let mut data: [u64; 64] = [0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
        , 0, 0, 0, 0, 0, 0, 0, 0
    ];

    des(plain, key, cipher);

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
}



