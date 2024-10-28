fn main() {
    let arr: Box<[u8; 9]> = Box::new([0,0,0,0,0,0,0,0,0]);
    solve(*arr, 0);
}

fn find_possible_values(arr: [u8; 9], i: usize) -> Vec<u8> {
    let mut v = vec![1,2,3,4,5,6,7,8,9];
    for j in 0..i {
        v.retain(|&x | arr[j] != x);
    }
    v
}

fn solve(mut a: [u8; 9], i: usize) {
    if i == 9 {
        println!("{}{}{}{}{}{}{}{}{}", a[0],a[1],a[2],a[3],a[4],a[5],a[6],a[7],a[8]);
        return ();
    } else {
        let values = find_possible_values(a, i);
        for v in values {
            a[i] = v;
            solve(a, i+1);
        }
    }
}