fn conv(c: char) -> char {
    const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    if let Some(index) = SUBS_I.find(c) {
        return SUBS_O.chars().nth(index).unwrap();
    }
    if c.is_ascii_alphanumeric(){
        return c;
    }
    return '-';
}

fn slugify(s: &str) -> String {
    let chars = s.to_string().to_lowercase().chars().collect::<Vec<char>>();
    let mut final_string = String :: new();
    let mut conv_c;
    let mut prev_c = '_';

    for c in chars {
        conv_c = conv(c);
        if conv_c != prev_c {
            final_string.push(conv_c);
        }
        prev_c = conv_c;
    }
    return final_string;
}

fn main() {
    let mut s = String::new();
    s = slugify("Questo che slug   sarà???");
    println!("{}", s);
}
