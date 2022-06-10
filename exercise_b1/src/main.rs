fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    for i in 0..org_arr.len() {
        if sub_arr[0] == org_arr[i] {
            let mut count = 0;
            for j in 0..sub_arr.len() {
                if sub_arr[j] == org_arr[i + j] {
                    count += 1;
                }
            }
            if count == sub_arr.len() {
                println!("mang sub_arr la mang con cua org_arr");
                break;
            }
        }
        if i == org_arr.len() - 1 {
            println!("mang sub_arr la không phải là mang con cua org_arr");
        }
    }
}
