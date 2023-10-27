use std::os::unix::raw::nlink_t;
use std::panic::resume_unwind;
use rand::{random, Rng, thread_rng};
use crate::assignment::ass05::ex02::Category::{ACTION, COMEDY, HORROR};

#[derive(Debug)]
struct Book {
    title: String,
    cat: Category,
}

#[derive(Debug, Default)]
enum Category {
    #[default]
    HORROR,
    COMEDY,
    ACTION,
}

#[derive(Debug, Default)]
pub struct Library {
    bookcase: [Vec<Book>; 10]
}

impl Default for Book {
    fn default() -> Self {
        let mut cat = Category::ACTION;
        let rng : u32 = random::<u32>()%3;
        match rng {
            0 => {cat = ACTION}
            1 => {cat = HORROR}
            _ => {cat = COMEDY}
        }

        let mut gen = rand::thread_rng();

        let mut title = String::new();
        for i in 0..10 {
            let random_char = gen.gen_range('a' as u8..='z' as u8);
            title.push(random_char as char);
        }

        return Book{title: title, cat: cat}
    }
}

impl Book {
    fn default_with_cat(cat: Category) -> Self {
        let mut rng = rand::thread_rng();

        let mut title = String::new();
        for i in 0..10 {
            let random_char = rng.gen_range('a' as u8..='z' as u8);
            title.push(random_char as char);
        }

        return Book{title: title, cat: cat}
    }
}

pub trait Populatable {
    fn populate(&mut self);
}

impl Populatable for Library {
    fn populate(&mut self) {
        for i in 0..10 {
            for n in 0..3 {
                self.bookcase[i].push(Book::default_with_cat(Category::default()));
            }
        }
    }
}