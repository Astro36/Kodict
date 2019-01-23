use super::Word;
use rayon::prelude::*;
use regex::Regex;

const OPEN_DIC_PAGE_ROW: usize = 30; // Max: 30
const STANDARD_DIC_PAGE_ROW: usize = 500;

lazy_static! {
  static ref HTML_REMOVER: Regex = Regex::new(r"<[^>]*>").unwrap();
  static ref HTML_COMMENT_REMOVER: Regex = Regex::new(r"<!--(.*?)-->").unwrap();

  // for Open Korean Dictionary Crawler
  static ref OPEN_DIC_URLS: Vec<String> = [
      ('ㄱ', "%E3%84%B1", 153242),
      ('ㄴ', "%E3%84%B4", 41589),
      ('ㄷ', "%E3%84%B7", 71535),
      ('ㄹ', "%E3%84%B9", 19449),
      ('ㅁ', "%E3%85%81", 65455),
      ('ㅂ', "%E3%85%82", 98187),
      ('ㅅ', "%E3%85%85", 131654),
      ('ㅇ', "%E3%85%87", 175682),
      ('ㅈ', "%E3%85%88", 124060),
      ('ㅊ', "%E3%85%8A", 45090),
      ('ㅋ', "%E3%85%8B", 18295),
      ('ㅌ', "%E3%85%8C", 26783),
      ('ㅍ', "%E3%85%8D", 39259),
      ('ㅎ', "%E3%85%8E", 69770),
    ].into_iter().fold(vec![], |mut urls, (_, encoded_consonant, amount)| {
      for n in 1..(amount / OPEN_DIC_PAGE_ROW + 2){
        urls.push(format!("https://opendict.korean.go.kr/search/deepSearchResult?firstView=false&searchType=2&letter1_s={}&rowsperPage={}&currentPage={}", encoded_consonant, OPEN_DIC_PAGE_ROW, n));
      }
      urls
    });
  static ref OPEN_DIC_POS_MATCHER: Regex = Regex::new(r"「([가-힣 ]+)」").unwrap();
  static ref OPEN_DIC_CAT_MATCHER: Regex = Regex::new(r"『([가-힣 ]+)』").unwrap();

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
      for n in 1..(amount / STANDARD_DIC_PAGE_ROW + 2){
        urls.push(format!("http://stdweb2.korean.go.kr/search/List_dic.jsp?setJaso={}&PageRow={}&SearchPart=Index&go={}", encoded_consonant, STANDARD_DIC_PAGE_ROW, n));
      }
      urls
    });
  static ref STANDARD_DIC_SIGN_REMOVER: Regex = Regex::new(r"〔.+〕|【.+】").unwrap();
  static ref STANDARD_DIC_POS_MATCHER: Regex = Regex::new(r"「([가-힣 ]+)」").unwrap();
  static ref STANDARD_DIC_CAT_MATCHER: Regex = Regex::new(r"『([가-힣 ]+)』").unwrap();
}

pub fn get_open_dictionary_words() -> Vec<Word> {
  println!("note: Crawling Open Korean Dictionary spends too much time. You can download the dictionary from its website.");
  OPEN_DIC_URLS
    .par_iter()
    .map(|url| {
      reqwest::get(url)
        .ok()
        .unwrap()
        .text()
        .unwrap()
        .split("<dl class=\"search_result_list deep_search\">")
        .nth(1)
        .unwrap()
        .split("</dl>")
        .nth(0)
        .unwrap()
        .trim()
        .split("<dd style=\"margin: 0 0 0 0;\">")
        .skip(1)
        .map(|element| {
          let element = element
            .split("<span class=\"search_word2\">")
            .nth(1)
            .unwrap();
          Word {
            entry: element.split("</span>").next().unwrap().to_string(),
            meaning: element
              .split("<span class=\"word_dis ml_10\">")
              .nth(1)
              .unwrap()
              .split("</span><span class=\"none \">")
              .nth(0)
              .unwrap()
              .to_string(),
            pos: OPEN_DIC_POS_MATCHER
              .captures_iter(&element)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
            category: OPEN_DIC_CAT_MATCHER
              .captures_iter(&element)
              .map(|c| c.get(1).unwrap().as_str().to_string())
              .collect(),
          }
        })
        .collect::<Vec<_>>()
    })
    .flatten()
    .collect()
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
