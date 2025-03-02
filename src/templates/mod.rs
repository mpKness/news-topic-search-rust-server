use actix_web::{web, HttpResponse, HttpRequest};
use tera::{Tera, Context};
use serde::Deserialize;
use serde_json::{Value, json}; // Add json here
use reqwest;
use std::process::Command;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn index(tmpl: web::Data<Tera>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("name", "world");
    let rendered = tmpl.render("home.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn about(tmpl: web::Data<Tera>) -> HttpResponse {
    let ctx = Context::new();
    let rendered = tmpl.render("about.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn handle_post_topic(form: web::Form<FormData>, tmpl: web::Data<Tera>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("name", &form.name);

    // Create a directory to store the Python scripts
    let script_dir = "python_scripts";
    fs::create_dir_all(script_dir).unwrap();

    // List of Python files to download
    let files = vec![
        ("https://raw.githubusercontent.com/mpKness/news-topic-search/main/src/main.py", "main.py"),
        ("https://raw.githubusercontent.com/mpKness/news-topic-search/main/src/scraper.py", "scraper.py"),
        // Add more files as needed
    ];

    // Download each file and save it to the directory if it doesn't already exist
    for (url, filename) in files {
        let file_path = format!("{}/{}", script_dir, filename);
        if !Path::new(&file_path).exists() {
            let response = reqwest::get(url).await.unwrap();
            let content = response.text().await.unwrap();
            let mut file = File::create(file_path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }

    // Save the requirements.txt file to the directory if it doesn't already exist
    let requirements_url = "https://raw.githubusercontent.com/mpKness/news-topic-search/main/requirements.txt";
    let requirements_path = format!("{}/requirements.txt", script_dir);
    if !Path::new(&requirements_path).exists() {
        let response = reqwest::get(requirements_url).await.unwrap();
        let content = response.text().await.unwrap();
        let mut file = File::create(&requirements_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    // Install the Python dependencies
    Command::new("pip3")
        .arg("install")
        .arg("-r")
        .arg(&requirements_path)
        .output()
        .expect("Failed to install dependencies");

    // Run the main Python script with the --topic argument
    let output = Command::new("python3")
        .arg(format!("{}/main.py", script_dir))
        .arg("--topic")
        .arg(&form.name) // Pass the name as --topic
        .output()
        .expect("Failed to execute script");

    // Parse the JSON output from the script
    let script_output = String::from_utf8_lossy(&output.stdout);
    let json_output: Vec<Value> = serde_json::from_str(&script_output).unwrap_or_else(|_| vec![json!({"error": "Invalid JSON output"})]);
    println!("Script output: {:?}", json_output);

    // Insert the JSON output into the context
    ctx.insert("list_of_links", &json_output);

    let rendered = tmpl.render("home.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}
