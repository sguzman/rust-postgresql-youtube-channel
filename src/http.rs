extern crate reqwest;
extern crate scraper;
extern crate serde_json;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36 ";

fn get_doc(client: reqwest::Client) -> scraper::Html {
    let mut res = client
        .get("https://www.youtube.com/channel/UC0rZoXAD5lxgBHMsjrGwWWQ")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send().unwrap();
    let body = res.text().unwrap();

    scraper::Html::parse_document(body.as_ref())
}

fn get_json(client: reqwest::Client) -> serde_json::Value {
    let doc = get_doc(client);
    let selector = scraper::Selector::parse("script").unwrap();

    for script in doc.select(&selector) {
        let text = script.text().collect::<Vec<_>>();
        if text.len() > 0 && text.first().unwrap().trim_left().starts_with("window[\"ytInitialData\"]") {
            let first_str = text[0]
                .trim_left_matches("\n    window[\"ytInitialData\"] = ");
            let end_delim = first_str.find(";\n").unwrap();
            let json_str = &first_str[0..end_delim];

            return serde_json::from_str(json_str).unwrap();
        }
    }

    serde_json::from_str(r#"{}"#).unwrap()
}

fn clean_subs(raw_subs: &str) -> u64 {
    let string = raw_subs.trim_right_matches(" subscribers").replace(",", "");
    string.parse::<u64>().unwrap()
}


pub fn get(client: reqwest::Client) -> (String, u64) {
    let json = get_json(client);

    let title = json["header"]["c4TabbedHeaderRenderer"]["title"].as_str().unwrap();
    let raw_subs = json["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"].as_str().unwrap();
    let subs = clean_subs(raw_subs);

    return (title.to_string(), subs)
}