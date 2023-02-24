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

// get the top 5 cast of the movie or tv show
pub fn get_cast(dom: &Document) -> Vec<serde_json::Value> {
    let mut cast = Vec::new();
    for element in dom.find(Attr("data-testid", "title-cast-item")).take(5) {
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
            "Name": name,
            "Role": role
        }));
    }
    cast
}

// get origin of the movie or tv show
pub fn get_origin(dom: &Document) -> String {
    match dom.find(Attr("data-testid", "title-details-origin")).next() {
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

// get details
pub fn parse_info(dom: &Document) -> serde_json::Value {
    // get the title
    let title = get_title(dom);

    // get the rate
    let rate = get_rate(dom);
    // convert to float
    let rate_float = rate.parse::<f32>().unwrap();

    // get the cast
    let cast = get_cast(dom);

    // get the origin
    let origin = get_origin(dom);

    // get the primary language
    let language = get_language(dom);

    // return object
    let result = json!({
        "title": title,
        "rate": rate_float,
        "cast": cast,
        "origin": origin,
        "language": language
    });
    result
}

// get 5 reviews
pub fn parse_reviews(dom: &Document) -> Vec<serde_json::Value> {
    let mut reviews = Vec::new();
    for element in dom.find(Attr("class", "review-container")).take(5) {
        let title = match element.find(Attr("class", "title")).next() {
            Some(element) => element.text().trim().to_string(),
            None => "No title found".to_string(),
        };

        let author = match element.find(Attr("class", "display-name-link")).next() {
            Some(element) => element.text().trim().to_string(),
            None => "No author found".to_string(),
        };

        let date = match element.find(Attr("class", "review-date")).next() {
            Some(element) => element.text().trim().to_string(),
            None => "No date found".to_string(),
        };

        let content = match element
            .find(Attr("class", "text show-more__control"))
            .next()
        {
            Some(element) => element.text().trim().to_string(),
            None => "No review found".to_string(),
        };

        let rate = match element
            .find(Attr("class", "rating-other-user-rating"))
            .next()
        {
            Some(element) => match element.find(Name("span")).next() {
                Some(span_element) => span_element.text().trim().to_string(),
                None => "No rate found".to_string(),
            },
            None => "No rate found".to_string(),
        };
        // convert to float
        let rate = rate.parse::<f32>().unwrap();

        reviews.push(json!({
            "title": title,
            "content": content,
            "rate": rate,
            "author": author,
            "date": date
        }));
    }
    reviews
}

// get 5 search results
pub fn search_result(dom: &Document) -> Vec<serde_json::Value> {
    let mut results = Vec::new();
    for element in dom
        .find(Attr("class", "ipc-metadata-list-summary-item__c"))
        .take(5)
    {
        let title = match element
            .find(Attr("class", "ipc-metadata-list-summary-item__tc"))
            .next()
        {
            Some(element) => match element.find(Name("a")).next() {
                Some(a_element) => a_element.text().trim().to_string(),
                None => "No title found".to_string(),
            },
            None => "No title found".to_string(),
        };

        let link = match element
            .find(Attr("class", "ipc-metadata-list-summary-item__tc"))
            .next()
        {
            Some(element) => match element.find(Name("a")).next() {
                Some(a_element) => a_element
                    .attr("href")
                    .unwrap_or("No link found")
                    .to_string(),
                None => "No link found".to_string(),
            },
            None => "No link found".to_string(),
        };

        // get the id of the movie or tv show which is between the first and second slash
        let id = link
            .split('/')
            .nth(2)
            .unwrap_or("No link found")
            .to_string();

        let time = match element
            .find(Attr("class", "ipc-metadata-list-summary-item__tc"))
            .next()
        {
            Some(element) => match element.find(Name("label")).next() {
                Some(a_element) => a_element.text().trim().to_string(),
                None => "No time found".to_string(),
            },
            None => "No time found".to_string(),
        };
        // replace "–" with "-"
        let time = time.replace("–", "-");

        results.push(json!({
            "title": title,
            "id": id,
            "time": time
        }));
    }
    results
}
