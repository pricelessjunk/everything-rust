use std::fmt::{Debug, Display};

pub fn traits_example() {
    let art = NewsArticle {
        author: String::from("Kaustuv"),
        content: String::from("Some article"),
    };

    println!("{}", art.summarize());
    println!("{}", art.default_summarize());
    notify(&art);
}

struct NewsArticle {
    author: String,
    content: String,
}

// Trait definition
trait Summary {
    fn summarize(&self) -> String;

    fn default_summarize(&self) -> String {
        return format!("{}", self.summarize());
    }
}

// Traits for structs
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.author, self.content);
    }
}

// Traits as parameter
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn notify_genral<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Traits combined
fn combined_func(item: &(impl Summary + Display)) {}

fn combined_func_gen<T: Summary + Display>(item: &T) {}

// Traits with where clause
fn where_clause<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    2
}

// Traits as return types
fn return_types(flag: bool) -> impl Summary {
    if flag {
        return NewsArticle {
            author: String::from("1"),
            content: String::from("one"),
        };
    } else {
        return NewsArticle {
            author: String::from("1"),
            content: String::from("one"),
        };
    }
}

// Blanket implementation
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

// Generics with generic lifetimes
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement {}", ann);
    if x.len() > y.len() { x } else { y }
}
