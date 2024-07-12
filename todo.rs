// open:
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode= //filter by brand+model
// -> be careful expecting "한줄광고 매물" - однострочное объявление
// get max cars -> get max pages (max cars / 25)
// iterate car list pages and grab "carSeq":
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode=&page=2&sort=-orderDate
// then iterate pages and grab full info:
// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
