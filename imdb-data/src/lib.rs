use select::document::Document;
use select::predicate::{Attr, Name};
use serde_json::json;

pub fn parse_info(dom: &Document) -> serde_json::Value {
    // get the title
    let title = match dom
        .find(Attr("data-testid", "hero-title-block__title"))
        .next()
    {
        Some(element) => element.text().trim().to_string(),
        None => "No title found".to_string(), // or handle the error in some other way
    };

    // get the rate
    let rate = match dom
        .find(Attr(
            "data-testid",
            "hero-rating-bar__aggregate-rating__score",
        ))
        .next()
    {
        Some(element) => match element.find(Name("span")).next() {
            Some(span_element) => span_element.text().trim().to_string(),
            None => "No rate found".to_string(),
        },
        None => "No rate found".to_string(),
    };

    // get the top 5 cast
    let mut cast = Vec::new();
    for element in dom.find(Attr("data-testid", "title-cast-item")).take(2) {
        let name = match element.find(Attr("data-testid", "title-cast-item__actor")).next() {
            Some(a_element) => a_element.text().trim().to_string(),
            None => "No name found".to_string(),
        };
        let role = match element.find(Attr("data-testid", "cast-item-characters-link")).next() {
            Some(a_element) => match a_element.find(Name("span")).next() {
                Some(span_element) => span_element.text().trim().to_string(),
                None => "No role found".to_string(),
            },
            None => "No role found".to_string(),
        };
        cast.push(json!({
            "name": name,
            "role": role
        }));
    }

    // return object
    let result = json!({
        "title": title,
        "rate": rate,
        "cast": cast
    });
    result
}
