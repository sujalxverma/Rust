

fn main() {
    let num: i32 = 100;
    let target: i32 = 36;
    let found : bool= find_it(num, target);
    if found {
        println!("Number found");
    }
    else{
        println!("Number not found");
    }
}
// boinary search.

fn find_it(num: i32, target: i32) -> bool {
    let mut s: i32 = 0;
    let mut e: i32 = num;
    // let mut m: i32 = s + (e - s) / 2;
    while s <= e {
          let m = s + (e - s) / 2;
        if target == m {
            return true;
        } else {
            if target > m {
                s = m + 1;
            } else {
                e = m - 1;
            }
        }
       
    }

    return false;
}
