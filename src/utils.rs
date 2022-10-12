use rand::Rng;

pub fn rdm_alphabet(len: usize) -> String {
    let mut alphabet_vec: Vec<char> = vec![];

    for _num in 1..(len + 1) {
        let rand_num = rand::thread_rng().gen_range(65, 91);
        if let Some(rand_num) = std::char::from_u32(rand_num) {
            alphabet_vec.push(rand_num);
        }
    }

    alphabet_vec.iter().collect::<String>()
}