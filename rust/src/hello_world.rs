use std::fmt;

fn print_refernce(param: &String) {
    println!("{}", param);
}

fn print(param: String) -> String {
    println!("{}", param);
    return param;
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

trait PrintHello {
    fn print(&self);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}


impl Drop for Point {
    fn drop(&mut self) {
        println!("drop {:?}", self);
    }
}

impl PrintHello for Point {
    fn print(&self) {
        println!("Hello World");
    }
}

impl Point {
    fn increate_x(&mut self) {
        *self = Point {
            x: 0,
            y: 0
        };
        self.x += 1;
    }
}

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         return write!(f, "{}", self.x);
//     }
// }

fn main() {
    // let s = String::from("aaa");
    // let mut a = s.clone();
    // print_refernce(&a);
    // a = print(a);
    // println!("{}", a);

    // let v = vec![1, 2, 3];

    // v[99];

    // let x = String::from("aaaa");
    // let y;
    // {
    //     y = String::from("bbb").as_str();
    // }
    // let r = longest(x.as_str(), y);
    // println!("{}", r);

    // let arr: Vec<i32> = vec![1, 2, 3];
    // let equal_vec = |a: &Vec<i32>| {
    //     return *a == arr;
    // };
    // equal_vec(&vec![1, 2, 3]);
    // let point = Point {
    //     x: 3,
    //     y: 4
    // };

    // let ys: [i32; 100] = [0; 100];
    // println!("{}", ys[0]);
    // println!("{}", point);
    // println!("{:?}", point);
    // let num: i32 = 3;
    // println!("{}", std::mem::size_of_val(&num));

    // let aa = {
    //     5
    // };

    // let ab = {
    //     5;
    // };

    // println!("{:?}", aa);

    // println!("{:?}", ab);

    // let mut count = &mut 5;
    // let mut fn_c = || {
    //     *count += 1;
    //     println!("{}", count);
    //     return count;
    // };
    // let count_2 = fn_c();
    // println!("{}", count_2);

    let a = &mut Point {
        x: 5,
        y: 0
    };
    a.print();
    // a.drop();
    // a.x += 3;
    // println!("{}", a.x);
    // let c = &mut Point {
    //     x: 1,
    //     y: 2
    // };
    // let mut b = &mut Point {
    //     x: 5,
    //     y: 0
    // };

    // let mut d = Point {
    //     x: 0,
    //     y: 0
    // };

    // d = Point {
    //     x: 1,
    //     y: 1
    // };

    // d.x += 2;

    // *b = Point {
    //     x: 6,
    //     y: 1
    // };
    // println!("{:?}", b);
    // b = c;
    // println!("{:?}", b);

    // let x = &mut 0; // let mut _hide = 0; let x = &mut _hide;

    // *x = 6;

    // let mut x = 5;
    // {
    //     let y = &mut x;
    //     *y = 6;
    // }

    // println!("{}", x);

    // let mut t = Point {
    //     x: 8,
    //     y: 9
    // };
    // t.increate_x();
    //(*b).x += 1;
}