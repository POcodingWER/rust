use std::borrow::Cow;

//Cow = clone on Write
#[derive(Debug)]
struct User<'a>{
    name: Cow<'a,str>
} 

impl User<'_>{
    fn is_borrowed(&self) {
        match &self.name {
            Cow::Borrowed(name) => println!("It's borrowed {}",name),
            Cow::Owned(name) => println!("Ti's owned {}",name)
        }
    }
}
fn main() {
   let mut user_1 = User{
    name: "User 1".into()
   };

   let user_2 = User{
    name: "User 2".to_string().into()
   };
   user_1.is_borrowed();
   user_1.name.to_mut().push('!');
   user_1.is_borrowed();

   user_2.is_borrowed();



}