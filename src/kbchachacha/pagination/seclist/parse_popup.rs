use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    //PARSE POP-WHBOX
    // if (div.ch-img > img) -> parse only images;
    ///// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24663799
    // if (정기점검검사표가 입력되지 않았습니다.) in span.mid-txt -> nothing to parse
    ///// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23469260
    // if (div.pop-area) -> parse seclist
    //// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24633080
    Ok(())
}
