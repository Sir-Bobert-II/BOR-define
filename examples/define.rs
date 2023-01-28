use leb_define::*;

#[tokio::main]
async fn main()
{   
    println!("{}", define_word("Cheese").await);
}