use crate::{
    extractor::extract::{
        extract_attr, extract_attrs, extract_from_js, extract_value, extract_values,
    },
    kbchachacha::structs::{
        CarData, CarDataDealer, CarDataDetails, CarDataParams, CarDataSeclist, ParamSecListType,
    },
};
use scraper::Html;

pub fn parse(document: &Html) -> CarData {
    let car_price = extract_value(document, "dd > strong");
    let car_name = extract_value(document, "strong.car-buy-name");
    let images = extract_attrs(document, "src", "div.page01 > a > img").unwrap();
    let detail01 = extract_values(document, "table.detail-info-table > tbody > tr > td");
    let detail02 = extract_values(document, "div.detail-info02 > div.mg-t40 > dl > dd");
    let detail03 = extract_value(document, "div.detail-info02 > div.mg-t40 > span");
    let mut mark_sold: bool = true;
    let sold_text = extract_value(document, "div.dim-txt");
    if sold_text.is_empty() {
        mark_sold = false;
    }
    let dealer_name = extract_value(document, "div.dealer-cnt > span.name");
    let dealer_place = extract_value(document, "span.place-add");
    let dealer_tel = extract_value(document, "div.dealer-tel-num");
    let dealer_location = extract_value(document, "div.map-txt");
    let dealer_info = extract_value(document, "div.dealer-scroll");
    let dealer_selling = extract_value(document, "span[id=btnDealerHome3]");
    let dealer_sold = extract_value(document, "span[id=btnDealerHome4]");
    let sec_list_url = extract_attr(document, "data-link-url", "a[id=btnCarCheckView1]");

    let param_diag_car_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var diagCarYn = ",
        ";",
    );

    let param_diag_car_seq = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var diagCarSeq = ",
        ";",
    );
    let param_premium_car_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var premiumCarYn = ",
        ";",
    );
    let param_diag_att_img_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var diagAttImgYn = ",
        ";",
    );
    let param_premium_att_img_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var premiumAttImgYn = ",
        ";",
    );
    let mut param_view_type = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var viewType = ",
        ";",
    );
    let param_sec_list_type: ParamSecListType;
    if param_diag_car_yn.is_empty() {
        param_sec_list_type = ParamSecListType::Nothing
    } else {
        if param_diag_car_yn == "Y" {
            if param_diag_att_img_yn == "Y" {
                param_view_type = "160110".to_owned();
            } else {
                param_view_type = "160120".to_owned();
            }
        }

        if param_premium_car_yn == "Y" {
            if param_premium_att_img_yn == "Y" {
                param_view_type = "160110".to_owned();
            } else {
                param_view_type = "160120".to_owned();
            }
        }

        if param_view_type == "160120" {
            if param_diag_car_yn == "Y" {
                param_sec_list_type = ParamSecListType::CheckInfo;
            } else if param_premium_car_yn == "Y" {
                param_sec_list_type = ParamSecListType::CheckInfo
            } else {
                param_sec_list_type = ParamSecListType::SecListUrl
            }
        } else if param_view_type == "160140" {
            param_sec_list_type = ParamSecListType::Nothing
        } else {
            param_sec_list_type = ParamSecListType::CheckInfo
        }
    }

    CarData {
        mark_sold,
        name: car_name,
        price: car_price,
        maker_code: "".to_owned(),
        class_code: "".to_owned(),
        car_seq: "".to_owned(),
        seclist: CarDataSeclist { url: sec_list_url },
        params: CarDataParams {
            param_diag_car_yn,
            param_diag_car_seq,
            param_premium_car_yn,
            param_sec_list_type,
        },
        dealer: CarDataDealer {
            dealer_name,
            dealer_place,
            dealer_tel,
            dealer_location,
            dealer_info,
            dealer_selling,
            dealer_sold,
        },
        images,
        details: CarDataDetails {
            license_plate: detail01[0].to_owned(),
            release_year: detail01[1].to_owned(),
            mileage: detail01[2].to_owned(),
            fuel_type: detail01[3].to_owned(),
            transmission_type: detail01[4].to_owned(),
            fuel_effeciency: detail01[5].to_owned(),
            body_type: detail01[6].to_owned(),
            engine_displacement: detail01[7].to_owned(),
            color: detail01[8].to_owned(),
            taxes: detail01[9].to_owned(),
            foreclosures: detail01[10].to_owned(),
            credit: detail01[11].to_owned(),
            presentation_number: detail01[12].to_owned(),
            total_loss: detail02[0].to_owned(),
            immersions: detail02[1].to_owned(),
            usage: detail02[2].to_owned(),
            owners: detail02[3].to_owned(),
            insurance_inquiry_date: detail03.to_owned(),
        },
    }
}
