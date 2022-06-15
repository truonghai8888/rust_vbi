//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let  a = vec![1,2,3,4,5];
    let mut i = 0;
    // let c = 0;
    loop {
        // c1 using clone
        // let (a, c) = test(a.clone());
        let (a, c) = test(&a);
        println!("{:?}",a);
        println!("{}",c);
        i +=1;
        if i >=5 {break;}
    }
}

pub fn test(a: &[u8]) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    for &d in a {
        c = c+d;
        b.insert(0, d);
    }

    (b, c as i32)
}