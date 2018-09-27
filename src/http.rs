extern crate reqwest;
extern crate scraper;
extern crate serde_json;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36 ";

fn get_doc(client: reqwest::Client, serial: String) -> scraper::Html {
    let key = reqwest::header::USER_AGENT;
    let value = USER_AGENT;

    let mut res = client
        .get(format!("https://www.youtube.com/channel/{}", serial).as_str())
        .header(key, value)
        .send().unwrap();
    let body = res.text().unwrap();

    let document = body.as_ref();
    scraper::Html::parse_document(document)
}

fn get_json(client: reqwest::Client, serial: String) -> serde_json::Value {
    let doc = get_doc(client, serial);

    let selectors = "script";
    let selector = scraper::Selector::parse(selectors).unwrap();

    for script in doc.select(&selector) {
        let text = script.text().collect::<Vec<_>>();

        let pat = "window[\"ytInitialData\"]";
        if text.len() > 0 && text.first().unwrap().trim_left().starts_with(pat) {

            let pat = "\n    window[\"ytInitialData\"] = ";
            let first_str = text[0]
                .trim_left_matches(pat);

            let pat = ";\n";
            let end_delim = first_str.find(pat).unwrap();

            let s = &first_str[0..end_delim];
            return serde_json::from_str(s).unwrap();
        }
    }

    serde_json::from_str(r#"{}"#).unwrap()
}

fn clean_subs(raw_subs: &str) -> u64 {
    let pat = " subscribers";
    let from = ",";
    let to = "";
    let string = raw_subs.trim_right_matches(pat).replace(from, to);

    string.parse::<u64>().unwrap()
}


pub fn get(client: reqwest::Client, serial: String) -> (String, u64) {
    let json = get_json(client, serial);

    let title = json["header"]["c4TabbedHeaderRenderer"]["title"].as_str().unwrap();
    let raw_subs = json["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"].as_str().unwrap();
    let subs = clean_subs(raw_subs);

    return (title.to_string(), subs)
}