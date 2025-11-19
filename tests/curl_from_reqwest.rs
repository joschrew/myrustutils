#[cfg(feature = "curl-from-reqwest")]
use {
    myrustutils::{create_url, to_curl, to_curl_multipart},
    reqwest::Url,
    reqwest::blocking::Client,
    reqwest::blocking::multipart::Form,
    reqwest::header::{CACHE_CONTROL, CONTENT_TYPE, HeaderMap, HeaderValue},
};

#[cfg(feature = "curl-from-reqwest")]
#[test]
fn simple_get_to_curl() {
    let url = Url::parse("http://example.com/").unwrap();
    let request = Client::new().get(url);
    let result = to_curl(&request).unwrap();
    assert_eq!(result, "curl -X GET \"http://example.com/\"");
}

#[cfg(feature = "curl-from-reqwest")]
#[test]
fn curl_from_requests_multipart_headers() {
    let url_str = String::from("http://example.com/foo/");
    let url = Url::parse(&url_str).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    let key1 = "username";
    let val1 = "foo";
    let key2 = "param2";
    let val2 = "bar";
    let form = Form::new().text(key1, val1).text(key2, val1);
    let request = Client::new()
        .post(url)
        .multipart(form)
        .headers(headers.clone());
    let result = to_curl_multipart(&request, &[(key1, val1), (key2, val2)], None);
    assert_eq!(
        result,
        format!(
            "curl -X POST \"{}\" -H \"{}\" -H \"{}: {}\" -H \"{}: {}\" -F \"{}={}\" -F \"{}={}\"",
            url_str,
            "Content-type: multipart/form-data",
            "content-type",
            headers.get(CONTENT_TYPE).unwrap().to_str().unwrap(),
            "cache-control",
            headers.get(CACHE_CONTROL).unwrap().to_str().unwrap(),
            key1,
            val1,
            key2,
            val2,
        )
    );
}

#[cfg(feature = "curl-from-reqwest")]
#[rustfmt::skip]
#[test]
fn create_url() {
    assert_eq!(create_url!("http://a", "b").as_str(), "http://a/b");
    assert_eq!(create_url!("http://a/", "/b").as_str(), "http://a/b");
    assert_eq!(create_url!("http://a", "b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a", "b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a", "/b/").as_str(), "http://a/b/");
    assert_eq!(create_url!("http://a//", "///b///").as_str(), "http://a/b/");
    // urls with only hostname always have trailing slash
    assert_eq!(create_url!("http://a").as_str(), "http://a/");
}
