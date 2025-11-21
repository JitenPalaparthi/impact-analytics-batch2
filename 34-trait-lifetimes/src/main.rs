fn main() {
    let s = MyString("Hello World".to_string());
    let slice = s.as_slice();
    println!("{}", slice);

    let message="hello World";
    let config = Config{data:message};
    let ext =config.extract();
    println!("{}",ext);
}

trait As_Slice {
    fn as_slice(&self) -> &str;
}
struct MyString(String); // not holding any reference in the strucre
impl As_Slice for MyString {
    fn as_slice(&self) -> &str {
        &self.0
    }
}

trait Extract<'a> {
    type Output: 'a; // Associated type, what if the associagted type is a ref
    fn extract(&'a self) -> Self::Output; // if the return type of a definition is a ref ,then give lifetime to the trait
}

struct Config<'a> {
    data: &'a str, // Yes there is a reference, hence give a lifetime
}

impl<'a> Extract<'a> for Config<'a> {
    type Output = &'a str;
    fn extract(&'a self) -> Self::Output {
        self.data
    }
}
