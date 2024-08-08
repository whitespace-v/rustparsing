// * seclists *
// with ck.carmodoo.com
// 24631894
// with checkpaper.iwsp.co.kr
// 25956913
// with encar
// 24955004
// with checkpaper.jmenetworks.co.kr
// 25941145
// with djauto
// 25496599
// with m-park.co.kr
// 25837384
// with ext.m-cube.co
// 26071714
// with autocafe
// 25879309
// with carinfo
// 25925267
// with ai2.kaai.or.kr
// 25539294
// with 221.143.49.206
// 21422734
// redirection to kbchachacha
// 23220785
// popup seclist
// 24633080
// popup (정기점검검사표가 입력되지 않았습니다.)
// 23469260
// popup images
// 24663799

// doesn't work
// https://erp.carmon.co.kr/office/rest/extservice/OUT4511?CHECK_NO=6780411042
// http://ai.kaai.or.kr/view/carview.do?car_no=180%uB2045114
// http://moldeoncar.com/usedCar/cklist.asp?usedCarID=1301612
// http://ext.kaat.kr/office/rest/extservice/OUT4511?CHECK_NO=6730400579

// очень подробная информация для корейских авто:
// /www.car365.go.kr/ (нужно членство)

// -> be careful expecting "한줄광고 매물" - однострочное объявление
// boss info: https://www.kbchachacha.com/public/layer/shop/info.kbc

// страховые случае при членстве в кб -> вторая кнопка в detail2

// delimit imagefolder by
//
// |kbchachacha
//   |_maker code
//      |__class_code
//         | car_seq
//             |   - sdf09sdf2-sdfgsd8723-fdgdfg86723.jpeg
//             |   - sdf09sdf2-sdfgsd8723-fdgdfg86723.jpeg
// ____________|   - sdf09sdf2-sdfgsd8723-fdgdfg86723.jpeg

// следующие шаги:
// если checkinfo, также попробовать кинуть запрос на ссылку dataurl
// далее причесать логику запросов в pagination::parser
// затем причесываем сбор данных с сек листов
// причесываем клиент билдеры, что бы все запросы были оптимизированные и через прокси
// очень грамотно думаем над тредингом и промежуточным сливом инфы куда-то что бы не было мемори лика...
// посмотрим далее че делать епта
