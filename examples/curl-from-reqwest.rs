use myrustutils::{print_curl, print_curl_multipart};
use reqwest::Url;
use reqwest::blocking::Client;
use reqwest::blocking::multipart;
use reqwest::header::ACCEPT;

fn main() {
    let client = Client::new();
    simple_get_with_auth(&client);
    get_with_download(&client);
    post_multipart(&client);
    post_json_body(&client);
}

fn simple_get_with_auth(client: &Client) {
    let url = Url::parse("http://example.com/api/")
        .unwrap()
        .join("admin/import-status")
        .unwrap();
    let request = client
        .get(url)
        .header(ACCEPT, "application/json")
        .query(&[("username", "admin"), ("page", "0"), ("limit", "2")])
        .basic_auth("admin", Some("secret"));
    print_curl(&request).unwrap();
}

fn get_with_download(client: &Client) {
    let url = Url::parse("http://example.com/api/")
        .unwrap()
        .join("workspace/test1")
        .unwrap();
    let request = client.get(url).header(ACCEPT, "application/vnd.ocrd+zip");
    // TODO: the output flag ("--output foo.zip") is not part of the request. Enable a user to
    // optionally add to the generated curl-command. Create a new function print_curl3 or
    // print_curl_custom which takes a string which is automatically appended to the command
    print_curl(&request).unwrap();
}

fn post_multipart(client: &Client) {
    let workspace_path = "foo.zip";
    let password = "secret";
    let url = Url::parse("http://example.com/")
        .unwrap()
        .join("workspace/test1")
        .unwrap();
    assert!(
        std::path::Path::new("foo.zip").exists(),
        "File '{}' needed to run the example. Create it with `touch foo.zip`",
        workspace_path
    );
    let form = multipart::Form::new()
        .file("workspace", workspace_path)
        .unwrap();
    let request = client
        .post(url)
        .multipart(form)
        .basic_auth("ocrd", Some(password));
    print_curl_multipart(&request, &[("workspace", "@foo.zip")], Some(password));
}

fn post_json_body(client: &Client) {
    let url = Url::parse("http://example.com/")
        .unwrap()
        .join("workflow")
        .unwrap();
    let body = "{\"workspace-id\": \"test1\", \"input-file-grp\": \"OCR-D-IMG\"}";

    let request = client
        .post(url)
        .body(body)
        .basic_auth("ocrd", Some("secret"));
    print_curl(&request).unwrap();
}
