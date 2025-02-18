#[cfg(feature = "curl-from-reqwest")]
#[test]
fn simple_get_to_curl() {
    use myrustutils::to_curl;
    use reqwest::blocking::Client;
    use reqwest::Url;

    let client = Client::new();

    let url = Url::parse("http://example.com/").unwrap();
    let request = client.get(url);
    let result = to_curl(&request).unwrap();
    assert_eq!(result, "curl -X GET \"http://example.com/\"");
}
