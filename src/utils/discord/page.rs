use poise::serenity_prelude as serenity;
use serenity::builder::CreateEmbed;


pub trait Paging {
    fn from_vec(pages: Vec<CreateEmbed>) -> Self;
    fn next(&mut self) -> Option<CreateEmbed>;
    fn last(&mut self) -> Option<CreateEmbed>;
    fn previous(&mut self) -> Option<CreateEmbed>;
    fn first(&mut self) -> Option<CreateEmbed>;
    fn get_page(&self, index: usize) -> Option<CreateEmbed>;
    fn push(&mut self, embed: CreateEmbed);
    fn get_current(&self) -> usize;
    fn available_page(&mut self) -> (bool, bool);
}


#[derive(Debug)]
pub struct Page {
    pages: Vec<CreateEmbed>,
    current: usize,
}


impl Paging for Page {
    fn from_vec(pages: Vec<CreateEmbed>) -> Self {
        Self {
            pages,
            current: 1,
        }
    }

    fn next(&mut self) -> Option<CreateEmbed> {
        if self.pages.len() >= self.current {
            self.current += 1;
            return self.get_page(self.current - 1);
        }
        None
    }

    fn last(&mut self) -> Option<CreateEmbed> {
        if self.pages.len() >= self.current + 1 {
            self.current = self.pages.len() - 1;
            return self.get_page(self.current);
        }
        None
    }

    fn previous(&mut self) -> Option<CreateEmbed> {
        if self.current - 1 >= 1 {
            self.current -= 1;
            return self.get_page(self.current - 1);
        }
        None
    }

    fn first(&mut self) -> Option<CreateEmbed> {
        if self.current != 0 {
            self.current = 1;
            return self.get_page(0);
        }
        None
    }

    fn get_page(&self, index: usize) -> Option<CreateEmbed> {
        self.pages.get(index).cloned()
    }

    fn push(&mut self, embed: CreateEmbed) {
        self.pages.push(embed);
    }

    fn get_current(&self) -> usize { self.current - 1 }

    fn available_page(&mut self) -> (bool, bool) {
        let (mut left, mut right) = (true, false);
        if self.current as i8 != 0 { left = false };
        if self.pages.len() == self.current { right = true };

        (left, right)
    }
}
