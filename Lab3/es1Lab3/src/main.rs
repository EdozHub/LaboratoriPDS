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
        if conv_c != '-' || conv_c != prev_c {
            final_string.push(conv_c);
        }
        prev_c = conv_c;
    }
    return final_string;
}

/*trait MySlug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}*/

trait Slug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

/*impl MySlug for &str {
    fn is_slug(&self) -> bool {
        let new_string: String = slugify(*self);
        if new_string == *self {
            return true;
        }
        return false;
    }
    fn to_slug(&self) -> String {
        return slugify(self);
    }
}

impl MySlug for String {
    fn is_slug(&self) -> bool {
        if slugify(&self) == self.as_str() {
            return true;
        }
        return false;
    }
    fn to_slug(&self) -> String {
        return slugify(&self);
    }
}*/

impl <T> Slug for T
where T: AsRef<str>{
    fn is_slug(&self) -> bool {
        if self.as_ref() == slugify(self.as_ref()){
            return true;
        }
        return false;
    }
    fn to_slug(&self) -> String {
        return slugify(self.as_ref());
    }
}

fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{}\ns4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}
