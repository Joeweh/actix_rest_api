use rand::Rng;

const UID_LENGTH: i32 = 20;
pub fn generate_uid() -> String {
    let mut id: Vec<char> = vec![];
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();

    let mut rng = rand::thread_rng();

    for _i in 0..UID_LENGTH {
        let index = rng.gen_range(0..chars.len());

        id.push(chars[index]);
    }

    return String::from_iter(id);
}