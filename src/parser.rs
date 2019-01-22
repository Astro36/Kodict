use super::{Word, Words};
use calamine::Reader;
use std::path::Path;

pub fn parse_open_dictionary_xls<P: AsRef<Path>>(path: P) -> Words {
  match calamine::open_workbook_auto(path)
    .unwrap()
    .worksheet_range("Sheet0")
  {
    Some(Ok(range)) => range
      .rows()
      .map(|data| Word {
        entry: data[0].get_string().unwrap().to_string(),
        meaning: data[13].get_string().unwrap().to_string(),
        pos: vec![data[11].get_string().unwrap().to_string()],
        category: vec![data[18]
          .get_string()
          .unwrap()
          .replace("『", "")
          .replace("』", "")
          .to_string()],
      })
      .collect(),
    Some(Err(_)) | None => vec![],
  }
}
