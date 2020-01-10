#![no_std]

#[macro_use]
extern crate num_derive;

mod cpu;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
