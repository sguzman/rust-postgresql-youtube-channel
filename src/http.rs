const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100 Safari/537.36 ";

fn get_doc(client: reqwest::Client, serial: String) -> Option<scraper::Html> {
    let key = reqwest::header::USER_AGENT;
    let value: &str = USER_AGENT;

    let res_result = client
        .get(format!("https://www.youtube.com/channel/{}", serial).as_str())
        .header(key, value)
        .send();

    match res_result {
        Ok(mut res) => {
            let body = res.text().unwrap();

            let document = body.as_ref();
            Some(scraper::Html::parse_document(document))
        },
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

fn get_json(client: reqwest::Client, serial: String) -> Option<serde_json::Value> {
    let doc_option: Option<scraper::Html> = get_doc(client, serial);
    if doc_option == None {
        return None
    }

    let doc = doc_option.unwrap();

    let selectors: &str = "script";
    let selector = scraper::Selector::parse(selectors).unwrap();

    for script in doc.select(&selector) {
        let text = script.text().collect::<Vec<_>>();

        let pat: &str = "window[\"ytInitialData\"]";
        if text.len() > 0 && text.first().unwrap().trim_left().starts_with(pat) {

            let pat: &str = "\n    window[\"ytInitialData\"] = ";
            let first_str = text[0]
                .trim_left_matches(pat);

            let pat: &str = ";\n";
            let end_delim = first_str.find(pat).unwrap();

            let s = &first_str[0..end_delim];
            return Some(serde_json::from_str(s).unwrap());
        }
    }

    None
}

fn clean_subs(raw_subs: &str) -> u64 {
    let pat: &str = " subscribers";
    let from: &str = ",";
    let to: &str = "";
    let string: String = raw_subs.trim_right_matches(pat).replace(from, to);

    string.parse::<u64>().unwrap()
}

pub fn get(serial: String) -> Option<(String, u64)> {
    let client = reqwest::Client::new();
    let json_option: Option<serde_json::Value> = get_json(client, serial);
    if json_option == None {
        return None
    }

    let json = json_option.unwrap();

    let title_option = json["header"]["c4TabbedHeaderRenderer"]["title"].as_str();
    let raw_subs_option = json["header"]["c4TabbedHeaderRenderer"]["subscriberCountText"]["simpleText"].as_str();

    match (title_option, raw_subs_option) {
        (Some(title), Some(raw_subs)) => Some((title.to_string(), clean_subs(raw_subs))),
        _ => None
    }
}