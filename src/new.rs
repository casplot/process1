
struct Sheep {}
struct Cow {}

pub trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// 使用 `Box<dyn Animal>` 特征对象来修正函数返回类型
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    
    if random_number < 0.5 {
        Box::new(Sheep {}) as Box<dyn Animal>
    } else {
        Box::new(Cow {}) as Box<dyn Animal>
    }
}

fn main() {
    let random_number = 2.44; // 随机生成一个数
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
