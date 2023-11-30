trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!(
            "The author of post {} is {} With {}",
            self.title, self.author, self.content
        )
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo: Weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(post);
    summary(weibo);
}

fn summary<T>(instance: T)
where
    T: Summary,
{
    println!("{}", instance.summarize());
}

// fn summary(instance: &impl Summary) {
//     println!("{}", instance.summarize());
// }

// fn summary(instance: &dyn Summary) {
//     println!("{}", instance.summarize());
// }

// fn summary<T: Summary>(instance: &T) {
//     println!("{}", instance.summarize());
// }
