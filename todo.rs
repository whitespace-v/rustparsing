// -> be careful expecting "한줄광고 매물" - однострочное объявление
// -> remember carlist url

//
// * seclists *
//
// with ck.carmodoo.com
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24631894"
//     .to_owned();
// with checkpaper.iwsp.co.kr
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25956913"
// .to_owned();
// with encar
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24955004".to_owned();
// with checkpaper.jmenetworks.co.kr
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25941145".to_owned();
// with djauto
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25496599".to_owned();
// with m-park.co.kr
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25837384".to_owned();
// with ext.m-cube.co
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=26071714".to_owned();
// with autocafe
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25879309".to_owned();
// with carinfo
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25925267".to_owned();
// with ai2.kaai.or.kr
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25539294".to_owned();
// with 221.143.49.206
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=21422734".to_owned();
// redirection to kbchachacha
// let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23220785".to_owned();

//PARSE POP-WHBOX
// if (div.ch-img > img) -> parse only images;
///// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24663799
// if (정기점검검사표가 입력되지 않았습니다.) in span.mid-txt -> nothing to parse
///// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23469260
// if (div.pop-area) -> parse seclist
//// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24633080

// doesn't work
// https://erp.carmon.co.kr/office/rest/extservice/OUT4511?CHECK_NO=6780411042
// http://ai.kaai.or.kr/view/carview.do?car_no=180%uB2045114
// http://moldeoncar.com/usedCar/cklist.asp?usedCarID=1301612
// http://ext.kaat.kr/office/rest/extservice/OUT4511?CHECK_NO=6730400579

// очень подробная информация для корейских авто:
// /www.car365.go.kr/ (нужно членство)



/// 01.08 pass dynamic params to urls and check if empty, and if all correct -> architect the code 