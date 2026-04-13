use axum::{
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
    extract::{Path},
};
use tower_http::cors::{Any, CorsLayer};
use serde_json::json;
mod data;

#[derive(Debug)]
enum ApiError {
  NotFound, //404
  IBANValidationError(String), 
  InvalidInput(String), //400
  InternalError,  //500
}

// #[derive(Debug)]
// enum IBANValidationError {
//   InvalidFormat,
//   InvalidLength,
//   InvalidCountryCode,
// }

impl IntoResponse for ApiError{
  fn into_response(self)->axum::response::Response{
    let (status,error_message) = match self{
      ApiError::NotFound => (StatusCode::NOT_FOUND,"Data not found".to_string()),
      ApiError::IBANValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
      ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
      ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
     };

    let body = Json(json!({
      "error": error_message  
    }));

    (status,body).into_response()
  }
}


async fn handler(Path(iban): Path<String>) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
  println!("Received IBAN: {}", iban);
  // ? why  
  if iban.is_empty(){
    return Err(Json(json!({
      "error": "IBAN cannot be empty"
    })));
  }
  let is_valid = validate_iban(&iban);
  

  // if is_valid != "true" {
  //   Err(Json(json!({
  //     "iban": iban,
  //     "valid": is_valid,
  //   })));
  //   // return Err(ApiError::IBANValidationError(is_valid));
  // }
  if is_valid == "true"{
    Ok(Json(json!({
        "iban": iban,
        "valid": is_valid,
    })))
  }
  else {
    Err(Json(json!({
      "iban": iban,
      "valid": false,
      "error": is_valid,
    })))
  }

}

fn validate_iban(mut iban: &str) -> String { 
    if iban.is_empty() {
        return "IBAN cannot be empty".to_string();
    } 
    // Check length
    // println!("Validating IBAN: {}", iban.len());
    let normalized = iban.replace(" ", "");
    // println!("Validating IBAN: {}", normalized.len());

    if normalized.len() < 15 {
        return "IBAN is too short".to_string();
    } 

    if normalized.len() > 34 {
        return "IBAN is too long".to_string();
    }

    // country code
    let country_code = &normalized[0..2];
    // Check if country code is 2 uppercase letters
    if country_code.chars().any(|c| !c.is_ascii_uppercase()) {
        return "IBAN must start with two uppercase letters".to_string();
    }
    // Check if country code is valid and matches the length
    let valid_country = data::IBAN_LENGTHS.iter().find(|&&(code, _)| code == country_code);
    if let Some((_, expected_length)) = valid_country {
        if normalized.len() != *expected_length {
            return format!("IBAN length for country {} should be {}", country_code, expected_length);
        }
    } else {
        return "Invalid country code".to_string();
    }

    mod_iban(&normalized)
}

fn mod_iban(iban:&str)->String{
  let rearranged_iban = format!("{}{}",&iban[4..],&iban[0..4]);
  let mut numeric_iban = String::new();
  for c in rearranged_iban.chars(){
    if c.is_digit(10){
      numeric_iban.push(c);
      continue;
    } 
    if c.is_ascii_uppercase(){
      numeric_iban.push_str(&(c as u32 - 'A' as u32 + 10).to_string());
    } else {
      // Lowercase letters in IBAN
      return "IBAN can only contain uppercase letters and digits".to_string();
    }
  }

  // Remove leading zeros
  numeric_iban.trim_start_matches('0');
  println!("Numeric IBAN: {}", numeric_iban);
  // u128 is not enough for IBAN
  // let num_iban = numeric_iban.parse::<u128>();
  // rolling mod 97
  let mut remainder = 0u32;
  for c in numeric_iban.chars(){
    let digit = c.to_digit(10).unwrap();
    remainder = (remainder * 10 + digit) % 97;
  }
  println!("Remainder: {}", remainder);
  if remainder != 1 {
    return "IBAN failed checksum validation".to_string();
  }else{
    return "true".to_string();
  }
}

async fn empty_handler() -> Result<Json<serde_json::Value>, ApiError> {
    Err(ApiError::InvalidInput("Please input an IBAN".to_string()))
}

fn create_app() -> Router {
    Router::new().route("/iban_validation/{iban}", get(handler))
        .route("/iban_validation/", get(empty_handler))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);
    let app = create_app().layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener,app).await.expect("Failed to start server");
}