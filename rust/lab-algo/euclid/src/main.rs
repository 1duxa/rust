pub fn gcd_two(a: &mut i64, b: &mut i64) -> i64 {
    while *b != 0 {
        let c = *a % *b;
        *a = *b;
        *b = c;
    }
    a.abs()
}

pub fn alg_euclid(a: &mut i64, b: &mut i64, x: &mut i64, y: &mut i64, d: &mut i64) {
    let (a_initial, b_initial) = (*a, *b);
    let mut q;
    let mut r;
    let mut x1 = 0;
    let mut x2 = 1;
    let mut y1 = 1;
    let mut y2 = 0;

    if *b == 0 {
        *d = *a;
        *x = 1;
        *y = 0;
        return;
    }

    while *b > 0 {
        q = *a / *b;
        r = *a - q * *b;

        *a = *b;
        *b = r;

        let new_x = x2 - q * x1;
        let new_y = y2 - q * y1;

        x2 = x1;
        x1 = new_x;
        y2 = y1;
        y1 = new_y;
    }

    *d = *a;
    *x = x2;
    *y = y2;

    println!(
        "Verification: {}*({}) + {}*({}) = {}",
        a_initial, x, b_initial, y, d
    );
}

pub fn gcd_of_three(a: &mut i64, b: &mut i64, c: &mut i64) -> i64 {
    let mut a_copy = *a;
    let mut b_copy = *b;
    let gcd_ab = gcd_two(&mut a_copy, &mut b_copy);
    let mut gcd_ab_copy = gcd_ab;
    gcd_two(&mut gcd_ab_copy, c)
}

fn main() {
    let mut a: i64 = 594;
    let mut b: i64 = 792;
    let mut c: i64 = 962;
    let mut a1: i64 = 1926;
    let mut b1: i64 = 2322;

    // task 1
    let gcd_three = gcd_of_three(&mut a, &mut b, &mut c);
    println!("GCD of {}, {}, and {} is: {}", 594, 792, 962, gcd_three);

    let gcd_two1 = gcd_two(&mut a1, &mut b1);
    println!("GCD of {}, {} is: {}", 1926, 2322, gcd_two1);

    let gcd_two12 = gcd_two(&mut gcd_two1.clone(), &mut gcd_three.clone());

    println!(
        "GCD of 1: {} and 2: {} is: {}",
        gcd_two1.clone(),
        gcd_three.clone(),
        gcd_two12
    );
    // task 2
    let mut a: i64 = 154;
    let mut b: i64 = 195;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut d: i64 = 0;

    alg_euclid(&mut a, &mut b, &mut x, &mut y, &mut d);

    println!("Results of the extended Euclidean algorithm:");
    println!("d (GCD): {}", d);
    println!("x: {}", x);
    println!("y: {}", y);

    // task 3

    let mut a = 66;
    let mut b = 88;
    let gcd_task3 = gcd_two(&mut a, &mut b);

    if gcd_task3 > 1 {
        println!("Not simple");
    } else if gcd_task3 == 1 {
        println!("Mutualy simple");
    } else {
        println!("Unknown");
    }
}
