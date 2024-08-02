#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker, pagination, structs::CarMaker};
use std::error::Error;

use super::structs::Car;

pub fn parse() -> Result<(), Box<dyn Error>> {
    /* Add to Makers: maker_code & class_code */
    // let makers = maker::maker::generate_makers_list()?;
    // /* Add to Makers: total & pages */
    // let makers = crawler::crawler::collect_param_list(makers)?;
    /* Add to Makers: car_seq */
    // let makers = CarMaker {
    //     class_code: "1101".to_owned(),
    //     maker_code: "101".to_owned(),
    //     pages: Some(2),
    //     total: Some(16),
    //     total_url: None,
    // };
    // let makers = vec![makers];
    // let cars = crawler::crawler::collect_seq_list(makers);

    let cars = vec![
        // http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2023300215707
        // https://ck.carmodoo.com/carCheck/carmodooPrint.do?print=0&checkNum=6623017076
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "24631894".to_owned(),
        },
        // https://checkpaper.iwsp.co.kr/Service/CheckPaper?checkNo=0213056966
        // https://checkpaper.iwsp.co.kr/Service/CheckPaper?checkNo=0213056966
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25956913".to_owned(),
        },
        // http://www.encar.com/md/sl/mdsl_regcar.do?method=inspectionViewNew&carid=35790674&wtClick_carview=015
        // http://www.encar.com/md/sl/mdsl_regcar.do?method=inspectionViewNew&carid=35790674&wtClick_carview=015
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "24955004".to_owned(),
        },
        // Sold out
        // http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2024300201447
        // https://checkpaper.jmenetworks.co.kr/Service/CheckPaper?checkNo=5501118334&print=0&iframe=1&key=
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25941145".to_owned(),
        },
        // http://www.djauto.co.kr/car/carViewFrameUsedCarCheck.html?checkFlag=446632
        // http://www.djauto.co.kr/car/carViewFrameUsedCarCheck.html?checkFlag=446632
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25496599".to_owned(),
        },
        // https://www.m-park.co.kr/popup/performance/24052410034
        // https://www.m-park.co.kr/popup/performance/24052410034
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25837384".to_owned(),
        },
        // http://ext.m-cube.co/inspect/web?pino=97-21-176617&company=POINTBANK_AUTOPLEX
        // http://ext.m-cube.co/inspect/web?pino=97-21-176617&company=POINTBANK_AUTOPLEX
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "26071714".to_owned(),
        },
        // sold out
        // http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=202420035489
        // https://www.autocafe.co.kr/asso/Check_Form_2020.asp?ChkSeq=1704389
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25879309".to_owned(),
        },
        // http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=659021819764
        // https://ai.carinfo.co.kr/view/carinfo?check_no=2427801069
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25925267".to_owned(),
        },
        // http://ai2.kaai.or.kr/view/carview.do?car_no=82%EC%86%8C2373
        // http://ai2.kaai.or.kr/view/carview.do?car_no=82%EC%86%8C2373
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25539294".to_owned(),
        },
        // http://221.143.49.206/CarCheck/popupCheck.asp?ckno=2006006298
        // http://221.143.49.206/CarCheck/popupCheck.asp?ckno=2006006298
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "21422734".to_owned(),
        },
        // www.autocafe.co.kr
        // kb -> 원하시는 페이지를 찾을 수 없습니다.
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "23220785".to_owned(),
        },
        // ""
        // popup seclist
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "24633080".to_owned(),
        },
        // ""
        // popup empty 정기점검검사표가 입력되지 않았습니다.
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "23469260".to_owned(),
        },
        // http://checkpaper.iwsp.co.kr/Service/JohabCheckPaper?code=KB&checkNo=0213037965
        // popup images
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "24663799".to_owned(),
        },
    ];
    let data = pagination::parser::parse(cars);
    Ok(())
}
