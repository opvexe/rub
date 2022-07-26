
#[derive(Debug,Clone)]
struct StackNode<T> {
    data: T,
    next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>
}

impl <T> StackNode<T> {
    fn new(data: T) -> StackNode<T> {
        return StackNode{
            data: data,
            next: None,
        }
    }
}

impl <T> Stack<T> {
    fn new() -> Stack<T> {
        return Stack{
            top: None,
        }
    }

    fn push(&mut self,data: T) {
        let mut node = StackNode::new(data);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            None => None,
            Some(mut x) =>{
                self.top = x.next.take();
                Some(x.data)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
