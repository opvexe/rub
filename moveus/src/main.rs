
// 验证带move与不带move的区别
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

fn main() {
    if ture {
        let mut  p = Point{
            x: 25,
            y: 30,
        };
        println!("One: {:p}",&p);
        (||{
            println!("Two: {:p}",&p);
        })()
    }else {
        let mut  p = Point{
            x: 25,
            y: 30,
        };
        println!("One: {:p}",&p);
        (move||{
            println!("Three: {:p}",&p);
        })()
    }
}

