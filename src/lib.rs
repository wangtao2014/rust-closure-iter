pub mod counter;

pub fn test_iter() {
    let vec = vec![1, 2, 3, 4];
    let vec_iter = vec.iter();

    for item in vec_iter {
        println!("{}", item);
    }
}

pub fn test_sum() -> i32 {
    let vec = vec![1, 2, 3, 4];
    let vec_iter = vec.iter();

    let results: i32 = vec_iter.sum();
    results
}

#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(10, test_sum());
    }

    #[test]
    fn test_map() {
        let vec = vec![1, 2, 3, 4];
        let vec_iter = vec.iter();

        let v2: Vec<_> = vec_iter.map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4, 5]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("adidas"),
            },
            Shoe {
                size: 15,
                style: String::from("jordan"),
            },
            Shoe {
                size: 10,
                style: String::from("nike"),
            },
        ];

        assert_eq!(
            shoes_in_size(shoes, 10),
            vec![
                Shoe {
                    size: 10,
                    style: String::from("adidas"),
                },
                Shoe {
                    size: 10,
                    style: String::from("nike"),
                },
            ]
        );
    }
}
