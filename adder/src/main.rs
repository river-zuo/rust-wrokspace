use add_one;

fn main() {
    println!("Hello, world!");

    let a = add_one::add_one(1);
    println!("add a num -> {}", a);

    let data = "ccc".to_string();

    let combine = "aaa".to_string() + "cccc" + "eeee";
    println!("combine -> [{}]", combine);
    println!("len -> {}", "a哈哈1".to_string().len());
    println!("len -> {}", &"a哈哈1".to_string()[0..4]);
    
}

fn modyfy_minus_one(num: &mut i32) {
    *num = *num - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        // unimplemented!();
        assert_eq!(3, add_one::add_one(2));
    }

    #[test]
    fn test_minus() {
        let mut i = 3;
        modyfy_minus_one(&mut i);
        assert_eq!(2, i);
    }

}

