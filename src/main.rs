extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Event, Tag};
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let markdown_string = "\
[I'm an inline-style link](https://www.google.com) \
[I'm a relative reference to a repository file](../blob/master/LICENSE)\
[You can use numbers for reference-style link definitions][1]

[1]: http://slashdot.org
";

    let parser = Parser::new(&markdown_string);

    let parser = parser.map(|event| match event {
        Event::HardBreak => Event::SoftBreak,
        Event::Start(Tag::Link(one, two)) => event,
        _ => event
    });

}