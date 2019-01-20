use super::Word;
use regex::Regex;
use std::sync::mpsc::channel;
use std::thread;

pub fn get_standard_dictionary_words(thread_num: usize) -> Vec<Word> {
  let (sender, receiver) = channel();
  for i in 0..thread_num {
    lazy_static! {
        static ref html_remover: Regex = Regex::new(r"<[^>]*>").unwrap();
        static ref sign_remover: Regex = Regex::new(r"〔.+〕|【.+】").unwrap();
        static ref pos_matcher: Regex = Regex::new(r"「([가-힣 ]+)」").unwrap();
        static ref category_matcher: Regex = Regex::new(r"『([가-힣 ]+)』").unwrap();
        static ref urls: Vec<String> = [
            ('ㄱ', "%E3%84%B1", 65487),
            ('ㄴ', "%E3%84%B4", 17666),
            ('ㄷ', "%E3%84%B7", 31224),
            ('ㄹ', "%E3%84%B9", 9582),
            ('ㅁ', "%E3%85%81", 26771),
            ('ㅂ', "%E3%85%82", 39518),
            ('ㅅ', "%E3%85%85", 55910),
            ('ㅇ', "%E3%85%87", 69582),
            ('ㅈ', "%E3%85%88", 50895),
            ('ㅊ', "%E3%85%8A", 19529),
            ('ㅋ', "%E3%85%8B", 5419),
            ('ㅌ', "%E3%85%8C", 9759),
            ('ㅍ', "%E3%85%8D", 12667),
            ('ㅎ', "%E3%85%8E", 28407),
        ].into_iter().map(|(_, encoded_consonant, amount)| {
            let mut url = vec![];
            for i in 1..(amount / 500 + 2){
                url.push(format!("http://stdweb2.korean.go.kr/search/List_dic.jsp?setJaso={}&PageRow=500&SearchPart=Index&go={}", encoded_consonant, i));
            }
            url
        }).fold(vec![], |mut acc, url| {
            acc.extend(url);
            acc
        });
    }
    let url_size = urls.len();
    let url_unit = url_size / thread_num;
    let urls_allocated = if i == thread_num - 1 {
      &urls[(i * url_unit)..]
    } else {
      &urls[(i * url_unit)..((i + 1) * url_unit)]
    };
    let my_sender = sender.clone();
    thread::spawn(move || {
      for url in urls_allocated {
        let html = reqwest::get(url).ok().unwrap().text().unwrap();
        let elements: Vec<&str> = html
          .split("<span id=\"print_area\">\n\t\t<p class=\"exp\">")
          .nth(1)
          .unwrap()
          .split("</p>\n\n        </span>\n\t\t\n<!-- paging.jsp -->")
          .nth(0)
          .unwrap()
          .trim()
          .split("</p>\n<p class=\"exp\">")
          .collect();
        let mut items = vec![];
        for element in elements {
          let word =
            html_remover.replace_all(&element.split("</font></strong>").nth(0).unwrap(), "");
          let desc = html_remover.replace_all(
            &element
              .split("<img src=\'/image/0715_plus.gif\' /></a>&nbsp;")
              .nth(1)
              .unwrap(),
            "",
          );
          let meaning = sign_remover.replace_all(&desc, "");
          items.push(Word {
            entry: word.to_string(),
            meaning: meaning.trim().replace("\n", " ").to_string(),
            pos: pos_matcher
              .captures_iter(&meaning)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
            category: category_matcher
              .captures_iter(&meaning)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
          });
        }
        my_sender.send(items).unwrap();
      }
    });
  }

  let mut items = vec![];
  for _ in 0..892 {
    items.extend(receiver.recv().unwrap());
  }

  items
}
