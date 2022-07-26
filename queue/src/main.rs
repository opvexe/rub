#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    // 初始化队列
    fn new() -> Self {
        return Queue { data: Vec::new() };
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn delete(&mut self) -> Option<T> {
        let l = self.data.len();

        if l > 0 {
            let v = self.data.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        return self.data.len();
    }
}

fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);

    println!("{}",q.size());

    q.delete();
    println!("{}",q.size());
}
