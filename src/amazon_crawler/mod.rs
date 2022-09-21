use select::document::Document;
use select::predicate::{Attr, Class, Name};

#[derive(Debug)]
pub struct Book {
  pub title: String,
  pub amazon_url: String
}

pub fn get_daily_sale_books() -> Vec<Book> {
  let response = reqwest::blocking::get(
    "https://www.amazon.co.jp/hko/deals/"
  )
    .unwrap();

  let document = Document::from_read(response).unwrap();

  let daily_sale_widget = document.find(Attr("cel_widget_id", "ebooks-deals-storefront_KindleDailyDealsStrategy")).next().unwrap();

  let mut books: Vec<Book> = Vec::new();

  for book_list in daily_sale_widget.find(Class("a-spacing-top-micro")) {
    let title = book_list.find(Name("a"))
      .next()
      .unwrap()
      .text()
      .trim()
      .to_string();

    let mut amazon_url = "https://www.amazon.co.jp/".to_string() + book_list.find(Name("a")).next().unwrap().attr("href").unwrap();
    let query_position = amazon_url.chars().position(|c| c == '?').unwrap();
    amazon_url = amazon_url[..query_position].to_string();

    books.push(Book {
      title,
      amazon_url
    });
  }

  books
}