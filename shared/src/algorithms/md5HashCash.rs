fn md5hash_cash(complexity: u32, message: String) {
    let mut finish = false;
    let mut seed = 0;

    while finish == false {
        let hex_seed = format_dec_to_hex(seed);
        let concat_seed = concat_string(hex_seed.to_string(), message.to_string());
        let digest = md5::compute(concat_seed);
        let hash_code = format_digest_to_hex(digest);
        let mut binary_hash: String = format_to_binary(hash_code);
        finish = check_seed(binary_hash, complexity);
        seed += 1;
    }
}

fn concat_string(seed: String, message: String) -> String {
    format!("{}{}\n", seed, message)
}

fn format_dec_to_hex(seed: i32) -> String {
    format!("{:016X}", seed)
}

fn format_digest_to_hex(digest: Digest) -> String {
    format!("{:032X}", digest)
}

fn format_to_binary(hashcode: String) -> String {
    hashcode.chars().map(to_binary).collect()
}

fn check_seed(binary_hash: String, complexity: u32) -> bool {
    let mut index = 0;
    for character in binary_hash.chars() {
        if character == '1' && index < complexity {
            print!("false\n");
            return false;
        } else if index >= complexity {
            print!("good ");//envoie du resultat au server
            return true;
        }
        index += 1;
    }
    return false;
}

fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    return String::from(b);
}