use decode::vec_to_tag;

mod types;
mod decode;

fn main() {

    let data = vec![
        3, // Int
        0, 0, 0, 4 // 4 
    ];

    let result = vec_to_tag(data).unwrap();

    println!("{:?}", result);
}

