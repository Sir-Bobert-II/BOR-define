use bor_define::*;

#[tokio::main]
async fn main()
{   
    println!("{}", run("Cheese").await);
}
