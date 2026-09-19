struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct DiffPoint<T, U> {
    x: T,
    y: U,
}

struct AnyPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> AnyPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: AnyPoint<X2, Y2>) -> AnyPoint<X1, Y2> {
        AnyPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub use generics_traits_lifetimes::aggregator;
use aggregator::{SocialPost, Summary};
use generics_traits_lifetimes::aggregator::NewsArticle;

fn main() {
    // GENERICS
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    let _both_int = DiffPoint { x: 5, y: 10 };
    let _both_float = DiffPoint { x: 1.0, y: 4.0 };
    let _int_and_float = DiffPoint { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = AnyPoint { x: 5, y: 10.4 };
    let p2 = AnyPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // TRAITS
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("The penguins are migrating north!"),
        location: String::from("Antarctica, Earth"),
        author: String::from("Koko"),
        content: String::from(
            "They went looking for spices."
        ),
    };
    println!("New article available! {}", article.summarize());

    // TODO: LIFETIMES
}
