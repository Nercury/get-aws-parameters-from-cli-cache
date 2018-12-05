fn main() {
    let home_dir = dirs::home_dir().expect("failed to get home dir");
    let aws_cache_dir = home_dir
        .join(".aws")
        .join("cli")
        .join("cache");

    let first_entry = std::fs::read_dir(&aws_cache_dir)
        .expect("aws cli cache dir was not found")
        .next()
        .expect("aws cli cache dir was empty")
        .expect("failed to get aws cache dir entry info");

    let data = std::fs::read_to_string(&first_entry.path())
        .expect("failed to read aws cache entry");

    let credentials = serde_json::from_str::<CacheData>(&data)
        .expect("failed to parse aws cache")
        .credentials;

    println!("AWS_ACCESS_KEY_ID={}", credentials.access_key_id);
    println!("AWS_SECRET_ACCESS_KEY={}", credentials.secret_access_key);
    println!("AWS_SESSION_TOKEN={}", credentials.session_token);
}

#[derive(serde_derive::Deserialize, Debug)]
struct CacheData {
    #[serde(rename = "Credentials")]
    credentials: Credentials,
}

#[derive(serde_derive::Deserialize, Debug)]
struct Credentials {
    #[serde(rename = "SecretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "AccessKeyId")]
    access_key_id: String,
    #[serde(rename = "SessionToken")]
    session_token: String,
}
