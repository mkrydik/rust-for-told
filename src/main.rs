use rustc_serialize::json::Json;

/// Google CGI API for Japanese Input
const API_URL: &str = "http://www.google.com/transliterate?langpair=ja-Hira|ja&text=";

/// Main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 第1引数を取得する
    let input_text = std::env::args().nth(1).expect("Please input text");

    // API コールする
    let url = format!("{}{}", API_URL, input_text);
    let response = reqwest::get(url).await?;
    let response_text = response.text().await?;
    // JSON パースする
    let json = Json::from_str(response_text.as_str())?;

    // 変換の第一候補を取得していく
    let array = json.as_array().ok_or("Top level is not an array")?;
    let mut converted_text = String::new();
    for item in array {
        let converts = item[1].as_array().ok_or("Invalid JSON structure")?;
        let converted_item = converts[0].as_string().ok_or("Invalid JSON structure")?;
        converted_text.push_str(converted_item);
    }

    println!("{}", converted_text);
    Ok(())
}
