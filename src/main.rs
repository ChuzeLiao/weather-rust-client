mod model;
use std::collections::HashMap;
use serde_json::value::Value;

async fn get(token:String) -> Result<HashMap<String, Value>, reqwest::Error>{
  let mut url = "http://34.201.45.238:3000/v1/hello?token=".to_string();
  url+=&token;
  println!("{}",url);
  Ok(reqwest::get(url).await?.json::<HashMap<String, Value>>().await?)
}
async fn get_weather(token:String) -> Result<HashMap<String, Value>, reqwest::Error>{
  let mut url = "http://34.201.45.238:3000/v1/weather?token=".to_string();
  url+=&token;

  Ok(reqwest::get(url).await?.json::<HashMap<String, Value>>().await?)
}
async fn post() -> Result<HashMap<String, Value>, reqwest::Error>{
  let client = reqwest::Client::new();
    let mut data = HashMap::new();
    data.insert("username", "test");
    data.insert("password", "test");

    let response = client
        .post("http://34.201.45.238:3000/v1/auth")
        .json(&data)
        .send()
        .await?;

  // 发起post请求并返回
  Ok(client.post("http://34.201.45.238:3000/v1/auth").json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

if let Ok(response) = post().await {
  let token = str::replace(&response["token"].to_string(), "\"", ""); 
  println!("{:#?}",token);
  let resp = get(token.to_string()).await;
    println!("1111{:#?}",resp);
  
  if let Ok(resp) = get_weather(token.to_string()).await {
    
    println!("1111{:#?}",resp);
  }
  
  
}
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //





    

    Ok(())
}
