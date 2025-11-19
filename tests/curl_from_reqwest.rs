#[cfg(feature = "curl-from-reqwest")]
#[test]
fn simple_get_to_curl() {
    use myrustutils::to_curl;
    use reqwest::Url;
    use reqwest::blocking::Client;

    let client = Client::new();

    let url = Url::parse("http://example.com/").unwrap();
    let request = client.get(url);
    let result = to_curl(&request).unwrap();
    assert_eq!(result, "curl -X GET \"http://example.com/\"");
}

#[cfg(feature = "curl-from-reqwest")]
#[rustfmt::skip]
#[test]
fn create_url() {
    use myrustutils::create_url;

    assert_eq!(create_url!("http://a", "b").as_str(), "http://a/b");
    assert_eq!(create_url!("http://a/", "/b").as_str(), "http://a/b");
    assert_eq!(create_url!("http://a", "b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a", "b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a", "/b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a//", "///b///").as_str(), "http://a/b/");
    // urls with only hostname always have trailing slash
    assert_eq!(create_url!("http://a").as_str(), "http://a/");
}
