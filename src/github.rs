use std::fs::File;
use std::fmt;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::error::Error as StdError;
use tokio::time;
use chrono::{self, Duration};
use std::io::{self, Read, Write, BufReader, BufRead};

const CLIENT_ID: &str = "69211095bf074c356f0a";
const SCOPE: &str = "public_repo delete_repo admin:public_key user:email";

#[derive(Deserialize, Debug)]
struct VerificationCodeResponse {
  device_code: String,
  user_code: String,
  verification_uri: String,
  expires_in: u32,
  interval: u32,
}

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
  access_token: Option<String>,
  token_type: Option<String>,
  scope: Option<String>,
  error: Option<String>,
}

#[derive(Debug, Clone)]
struct CodeExpiredError;

impl fmt::Display for CodeExpiredError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  write!(f, "CodeExpiredError is here!")
  }
}

impl StdError for CodeExpiredError {}

#[derive(Debug, Clone)]
struct CreateRepoError;

impl fmt::Display for CreateRepoError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  write!(f, "CreateRepoError is here!")
  }
}

impl StdError for CreateRepoError {}

type Result<T> = std::result::Result<T, Box<dyn StdError>>;

async fn get_verification_code() -> Result<VerificationCodeResponse> {
  let input_parameters = json!({
    "client_id": CLIENT_ID,
    "scope": SCOPE,
  });

  let response = reqwest::Client::new()
    .post("https://github.com/login/device/code")
    .header("Accept", "application/json")
    .json(&input_parameters)
    .send()
    .await?;

  let verification = response
    .json::<VerificationCodeResponse>()
    .await?;

  Ok(verification)
}

pub async fn get_access_token() -> Result<String> {
  let result = get_verification_code().await;
  if let Ok(verification) = result {
    println!("");
    println!("Go to {}", verification.verification_uri);
    println!("");
    println!("Enter the code {}", verification.user_code);
    println!("");
  
    let input_parameters = json!({
      "client_id": CLIENT_ID,
      "device_code": verification.device_code,
      "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
    });

    let expiration_time = chrono::offset::Local::now() + Duration::seconds(verification.expires_in as i64);
    println!("This code will expire at {}", expiration_time.format("%Y-%m-%d %H:%M:%S").to_string());
    
    // repeat every `verification.interval` until `verification.expires_in`
    loop {
      if expiration_time < chrono::offset::Local::now() {
        let e = io::Error::new(io::ErrorKind::Other, "Code has expired!");
        return Err(CodeExpiredError.into());
      }

      println!("Check for access token");
      let response = reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&input_parameters)
        .send()
        .await?;

      let token = response
        .json::<AccessTokenResponse>()
        .await?;
      
      
      if let Some(err) = token.error {
        match err.as_str() {
          "authorization_pending" => println!("Wait {} seconds for another attempt...", verification.interval),
          _ => println!("Failed obtaining the access token: {}", err),
        }
      } else if let Some(access_token) = token.access_token {
        println!("access_token {}", access_token);
        println!("token_type {}", token.token_type.unwrap());
        println!("scope {}", token.scope.unwrap());
        return Ok(access_token);
        // break;
      }
        
      time::sleep(time::Duration::from_secs(verification.interval as u64)).await;
    }
  }

  Err(result.expect_err("verification code was nor recieved"))

}

pub async fn check_token(token: &String) -> Result<bool> {
  println!("{}", token);
  let response = reqwest::Client::new()
    .get(format!("https://api.github.com/user/repos"))
    .header("Accept", "application/vnd.github+json")
    .header("Authorization", format!("token {token}"))
    .send()
    .await?;

  println!("{:?}", response);
  Ok(response.status() == StatusCode::OK)
}

pub async fn repo_exists(repo: &String, owner: &String, token: &String) -> Result<bool> {
  let response = reqwest::Client::new()
    .get(format!("https://api.github.com/repos/{owner}/{repo}"))
    .header("Accept", "application/vnd.github+json")
    .header("Authorization", format!("token {token}"))
    .send()
    .await?;

  Ok(response.status() == StatusCode::OK)
}

pub async fn create_repo(repo: &String, owner: &String, token: &String) -> Result<()> {
  let input_parameters = json!({
    "name": repo,
    "description": "Special repository for displaying nice activity in the profile",
    "private": false,
    "has_issues": false,
    "has_projects": false,
    "has_wiki": false,
    "has_downloads": false,
    "is_template": false,
  });

  let response = reqwest::Client::new()
    .post("https://api.github.com/user/repos")
    .header("Accept", "application/vnd.github+json")
    .header("Authorization", format!("token {token}"))
    .json(&input_parameters)
    .send()
    .await?;

  if response.status() == StatusCode::CREATED {
    return Ok(());
  }
  println!("{}", response.status().as_u16());
  Err(CreateRepoError.into())
}

pub async fn delete_repo(repo: &String, owner: &String, token: &String) -> Result<()> {
  let response = reqwest::Client::new()
    .delete(format!("https://api.github.com/repos/{owner}/{repo}"))
    .header("Accept", "application/vnd.github+json")
    .header("Authorization", format!("token {token}"))
    .send()
    .await?;

  if response.status() == StatusCode::NO_CONTENT {
    return Ok(());
  }
  Err(CreateRepoError.into())
}

#[tokio::main]
pub async fn prepare_github() -> Result<()> {
  let bak_path = "token.bak";
  let mut token: String = String::new();
  match File::open(bak_path) {
    Ok(file) => {
      let mut buffered = BufReader::new(file);
      buffered.read_to_string(&mut token)?;
    },
    Err(_) => {
      println!("Backup not found")
    },
  }
  if token.is_empty() || !check_token(&token).await? {
    token = get_access_token().await?;
    let mut file = File::create(bak_path)?;
    file.write_all(token.as_bytes())?;
  } else {
    println!("Reusing stored token");
  }

  let repo = "activity-repo".to_string();
  let owner = "boooobs".to_string(); // TODO
  if repo_exists(&repo, &owner, &token).await? {
    println!("Repo already exists, deleting...");
    delete_repo(&repo, &owner, &token).await?;
  } else {
    println!("Need to create repo '{repo}'");
    create_repo(&repo, &owner, &token).await?;
  }
  Ok(())
}