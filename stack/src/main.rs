
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
    top: usize,
}

impl <T> Stack<T> {
    fn new(size: usize) -> Self{
        return Stack{
            data: Vec::with_capacity(size),
            top: 0,
        }
    }

    fn push(&mut self,item: T) -> Result<(),String> {
        if self.top >= self.data.capacity(){
            return Err(String::from("there is no space"))
        }
        self.data.push(item);
        self.top +=1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -=1;
        self.data.pop()
    }

    fn top(&self) -> usize {
        return self.top;
    }
}

fn main() {
    let mut s = Stack::new(4);
    if let Err(error) = s.push(1){
        println!("{}",error);
    }

    for _ in 0..3 {
        if let Some(a) = s.pop(){
            println!("{}",a);
        }else {
            println!("Empty");
        }
    }
}
