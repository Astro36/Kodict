use super::Word;
use rayon::prelude::*;
use regex::Regex;

const PAGE_ROW: usize = 500;

lazy_static! {
  static ref HTML_REMOVER: Regex = Regex::new(r"<[^>]*>").unwrap();

  // for Standard Korean Dictionary Crawler
  static ref STANDARD_DIC_URLS: Vec<String> = [
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
    ].into_iter().fold(vec![], |mut urls, (_, encoded_consonant, amount)| {
      for n in 1..(amount / PAGE_ROW + 2){
        urls.push(format!("http://stdweb2.korean.go.kr/search/List_dic.jsp?setJaso={}&PageRow={}&SearchPart=Index&go={}", encoded_consonant, PAGE_ROW, n));
      }
      urls
    });
  static ref STANDARD_DIC_SIGN_REMOVER: Regex = Regex::new(r"〔.+〕|【.+】").unwrap();
  static ref STANDARD_DIC_POS_MATCHER: Regex = Regex::new(r"「([가-힣 ]+)」").unwrap();
  static ref STANDARD_DIC_CAT_MATCHER: Regex = Regex::new(r"『([가-힣 ]+)』").unwrap();
}

pub fn get_standard_dictionary_words() -> Vec<Word> {
  STANDARD_DIC_URLS
    .par_iter()
    .map(|url| {
      reqwest::get(url)
        .ok()
        .unwrap()
        .text()
        .unwrap()
        .split("<span id=\"print_area\">\n\t\t<p class=\"exp\">")
        .nth(1)
        .unwrap()
        .split("</p>\n\n        </span>\n\t\t\n<!-- paging.jsp -->")
        .nth(0)
        .unwrap()
        .trim()
        .split("</p>\n<p class=\"exp\">")
        .map(|element| {
          let word =
            HTML_REMOVER.replace_all(&element.split("</font></strong>").nth(0).unwrap(), "");
          let desc = HTML_REMOVER.replace_all(
            &element
              .split("<img src=\'/image/0715_plus.gif\' /></a>&nbsp;")
              .nth(1)
              .unwrap(),
            "",
          );
          let meaning = STANDARD_DIC_SIGN_REMOVER.replace_all(&desc, "");
          Word {
            entry: word.to_string(),
            meaning: meaning.trim().replace("\n", " ").to_string(),
            pos: STANDARD_DIC_POS_MATCHER
              .captures_iter(&meaning)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
            category: STANDARD_DIC_CAT_MATCHER
              .captures_iter(&meaning)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
          }
        })
        .collect::<Vec<_>>()
    })
    .flatten()
    .collect()
}
