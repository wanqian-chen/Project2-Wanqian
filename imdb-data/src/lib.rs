use select::document::Document;
use select::predicate::{Attr, Name};
use serde_json::json;

// get the title of the movie or tv show
pub fn get_title(dom: &Document) -> String {
    match dom
        .find(Attr("data-testid", "hero-title-block__title"))
        .next()
    {
        Some(element) => element.text().trim().to_string(),
        None => "No title found".to_string(), // or handle the error in some other way
    }
}

// get the rate of the movie or tv show
pub fn get_rate(dom: &Document) -> String {
    match dom
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
    }
}

// get the top 2 cast of the movie or tv show
pub fn get_cast(dom: &Document) -> Vec<serde_json::Value> {
    let mut cast = Vec::new();
    for element in dom.find(Attr("data-testid", "title-cast-item")).take(2) {
        let name = match element
            .find(Attr("data-testid", "title-cast-item__actor"))
            .next()
        {
            Some(a_element) => a_element.text().trim().to_string(),
            None => "No name found".to_string(),
        };
        let role = match element
            .find(Attr("data-testid", "cast-item-characters-link"))
            .next()
        {
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
    cast
}

// get origin of the movie or tv show
pub fn get_origin(dom: &Document) -> String {
    match dom
        .find(Attr("data-testid", "title-details-origin"))
        .next()
    {
        Some(element) => match element.find(Name("a")).next() {
            Some(a_element) => a_element.text().trim().to_string(),
            None => "No origin found".to_string(),
        },
        None => "No origin found".to_string(),
    }
}

// get primary language
pub fn get_language(dom: &Document) -> String {
    match dom
        .find(Attr("data-testid", "title-details-languages"))
        .next()
    {
        Some(element) => match element.find(Name("a")).next() {
            Some(a_element) => a_element.text().trim().to_string(),
            None => "No language found".to_string(),
        },
        None => "No language found".to_string(),
    }
}

pub fn parse_info(dom: &Document) -> serde_json::Value {
    // get the title
    let title = get_title(dom);

    // get the rate
    let rate = get_rate(dom);

    // get the top 2 cast
    let cast = get_cast(dom);

    // get the origin
    let origin = get_origin(dom);

    // get the primary language
    let language = get_language(dom);

    // return object
    let result = json!({
        "title": title,
        "rate": rate,
        "cast": cast,
        "origin": origin,
        "language": language
    });
    result
}
