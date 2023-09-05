struct Buffer<T> {
    member: Vec<T>,
}
impl<T> Buffer<T>
where
    T: std::ops::Add<Output = T> + Default + Clone,
{
    fn new() -> Self {
        Buffer { member: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.member.push(item);
    }
    fn sum(&self) -> T {
        self.member.iter().cloned().fold(T::default(), |acc, x| acc + x)
    }
}
fn compareString(x: &str, y: &str) -> bool {
    let mut a = x.chars();
    let mut b = y.chars();
    loop {
        match (a.next(), b.next()) {
            (Some(xc), Some(yc)) => {
                if xc < yc {
                    return false;
                } else if xc > yc {
                    return true;
                }
            }
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (None, None) => return false,
        }
    }
}

fn main() {
 // exercise1   
    let mut buf = Buffer::new();
    buf.push(1);
    buf.push(2);
    buf.push(3);
    let sum: u32 = buf.sum();
    println!("Sum = {}", sum);
 // exercise2
    let x = "ab";
    let y = "aaa";
    let ret = compareString(x, y);
    if ret {
        println!("在字典序上，{} 大于 {}", x, y);
    } else {
        println!("在字典序上，{} 小于等于 {}", x, y);
    }
 // exercise3
    let ori: Vec<char> = vec!['a', 'b', 'c', 'd', 'e']; 
    let prt: Vec<char> = ori
        .iter()
        .map(|&c| {
            if c == 'e' {
                'f'
            } else {
                (c as u8 + 1) as char
            }
        })
        .collect();
    println!("{:?}", prt);
    
}
