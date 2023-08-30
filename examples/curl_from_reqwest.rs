use myrustutils::{print_curl, print_curl_multipart};
use reqwest::blocking::multipart;
use reqwest::blocking::Client;
use reqwest::header::ACCEPT;
use reqwest::Url;

fn main() {
    // GET command with authentication
    let password = "not-the-real-password";
    let client = Client::new();
    let url = Url::parse("http://example.com/api/")
        .unwrap()
        .join("admin/import-status")
        .unwrap();
    let request = client
        .get(url)
        .header(ACCEPT, "application/json")
        .query(&[("username", "admin"), ("page", "0"), ("limit", "2")])
        .basic_auth("admin", Some(password));
    print_curl(&request).unwrap();

    // POST multipart
    let workspace_path = "foo.zip";
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

    // POST json body
    let url = Url::parse("http://example.com/")
        .unwrap()
        .join("workflow")
        .unwrap();
    let body = "{\"workspace-id\": \"test1\", \"input-file-grp\": \"OCR-D-IMG\"}";

    let request = client
        .post(url)
        .body(body)
        .basic_auth("ocrd", Some(password));
    print_curl(&request).unwrap();
}
