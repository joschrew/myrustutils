use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use reqwest::blocking::RequestBuilder;

/// Prinst the curl command according to the request
///
/// Some information are extracted with regex from the debug output of the `RequestBuilder`, some
/// information have to be provided. Headers are not extracted currently, only multipart/form-data
/// is set.
pub fn print_curl_multipart(
    request: &RequestBuilder,
    parts: &[(&str, &str)],
    credentials: Option<&str>,
) {
    println!("{}", to_curl_multipart(request, parts, credentials));
}

pub fn to_curl_multipart(
    request: &RequestBuilder,
    parts: &[(&str, &str)],
    credentials: Option<&str>,
) -> String {
    let mut res: Vec<String> = Vec::new();
    res.push("curl".to_owned());
    let str_debug = format!("{:?}", request);
    let myfind = "Ok(Request ";
    let start = str_debug.find(myfind).unwrap();
    let str_debug = &str_debug[start + myfind.len()..];

    let re_method = Regex::new(r"method[:] (\w+)").unwrap();
    let method = &re_method.captures(str_debug).expect("regex findet nix")[1];
    res.push("-X".to_owned());
    res.push(method.to_owned());

    let re_scheme = Regex::new(r#"scheme[:] "(\w+)""#).unwrap();
    let scheme = &re_scheme.captures(str_debug).expect("regex findet nix")[1];

    let re_host = Regex::new(r#"host[:] Some[(]\w+[(]["]?([^"()]+)"#).unwrap();
    let host = &re_host.captures(str_debug).expect("regex findet nix")[1];

    let re_path = Regex::new(r#"path[:] ["]?([^"()]+)"#).unwrap();
    let path = &re_path.captures(str_debug).expect("regex findet nix")[1];

    let re_query = Regex::new(r#"query: Some[(]["]?([^"()]+)"#).unwrap();
    let query_match = &re_query.captures(str_debug);
    let query = match query_match {
        Some(hit) => &hit[1],
        None => "",
    };

    let url = format!(
        "{scheme}://{host}{path}{}{query}",
        if query.is_empty() { "?" } else { "" }
    );
    res.push(format!("\"{}\"", url));

    if let Some(credentials) = credentials {
        res.push("--user".to_owned());
        res.push(format!("\"{}\"", credentials.to_owned()));
    }

    // TODO: extract all headers from debug output instead of setting only this one
    res.push("-H".to_owned());
    res.push("\"Content-type: multipart/form-data\"".to_owned());

    for (key, value) in parts {
        res.push("-F".to_owned());
        res.push(format!("\"{key}={value}\""));
    }
    res.join(" ")
}

/// Tries to print a curl command according to RequestBuilder`
///
/// This only works if the request can be cloned, which for example does not work with multipart
/// requests.
pub fn print_curl(request: &RequestBuilder) -> Result<()> {
    println!("{}", to_curl(request)?);
    Ok(())
}

pub fn to_curl(request: &RequestBuilder) -> Result<String> {
    let clone = match request.try_clone() {
        Some(val) => val,
        None => {
            return Err(anyhow!("Request cannot be cloned"));
        }
    };
    let request = clone.build()?;
    let mut res: Vec<String> = Vec::new();
    res.push("curl".to_owned());
    res.push("-X".to_owned());
    res.push(request.method().as_str().to_owned());
    res.push(format!("\"{}\"", request.url().as_str().to_owned()));

    for (key, value) in request.headers() {
        if key == "authorization" {
            let auth_str_base64 = value.to_str().unwrap();
            let credentials_base64 = &auth_str_base64[6..];
            let decoded_bin = general_purpose::STANDARD
                .decode(credentials_base64)
                .unwrap();
            let decoded = std::str::from_utf8(&decoded_bin).unwrap();
            res.push("--user".to_owned());
            res.push(format!("\"{}\"", decoded));
        } else {
            res.push("-H".to_owned());
            res.push(format!("\"{}: {}\"", key, value.to_str()?));
        }
    }

    if let Some(body) = request.body() {
        if let Some(bytes) = body.as_bytes() {
            res.push("-d".to_owned());
            res.push(format!("'{}'", std::str::from_utf8(bytes).unwrap()));
        }
    }
    Ok(res.join(" "))
}
