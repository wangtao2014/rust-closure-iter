use std::cmp::Eq;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

pub fn closure_learning() {
    /*
    |param1, param2...| -> return type {
        语句1
        语句2
        返回表达式
    } */

    let expensive_closure = |name| {
        println!("{}", name);
    };

    expensive_closure(String::from("hello world"));

    // 闭包的定义最终只会为参数和返回值推断出唯一具体的类型,并且后面不能再更改
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello world"));
    // let n = example_closure(23);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

struct Cacher<I, O, T>
where
    T: Fn(I) -> O, 
    I: Eq + Hash + Copy,
    O: Copy + Clone,
{
    calculation: T,
    value: HashMap<I, O>,
}

impl<I, O, T> Cacher<I, O, T>
where
    T: Fn(I) -> O, 
    I: Eq + Hash + Copy,
    O: Copy + Clone,
{
    fn new(calculation: T) -> Cacher<I, O, T> { 
        Cacher { 
            calculation, 
            value: HashMap::new(),
        } 
    }

    fn value(&mut self, arg: I) -> O {
        match self.value.get(&arg) {
            Some(val) => *val,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_closure_values() {
        let mut c = Cacher::new(|a: &str| -> usize {
            a.parse::<usize>().unwrap()
        });

        let v1 = c.value("1");
        let v2 = c.value("2");

        assert_eq!(1, v1);
        assert_eq!(2, v2);
    }
}