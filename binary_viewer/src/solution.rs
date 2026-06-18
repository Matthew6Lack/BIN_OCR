pub fn bin_todeci(nbr: String, signed: bool) -> i32 {
    let mut bin: Vec<char> = nbr.chars().rev().collect();

    let negatif = bin[bin.len() - 1] == '1';

    let mut n: i32 = 0;

    if signed && negatif {
        for i in 0..bin.len() {
            if bin[i] == '0' {
                bin[i] = '1';
            } else {
                bin[i] = '0';
            }
        }
        let mut i: usize = 0;
        while i < bin.len() && bin[i] == '1' {
            bin[i] = '0';
            i += 1;
        }
        if i <= bin.len() - 1 {
            bin[i] = '1';
        }
    }

    for i in 0..bin.len() {
        if bin[i] == '1' {
            n += (2_u32.pow(i as u32)) as i32;
        }
    }

    if signed && negatif {
        n = -1 * n;
    }
    n
}
