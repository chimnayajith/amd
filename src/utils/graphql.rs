/*
amFOSS Daemon: A discord bot for the amFOSS Discord server.
Copyright (C) 2024 amFOSS

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use serde_json::Value;
use chrono::{ Local };

const REQUEST_URL: &str = "https://root.shuttleapp.rs/";

pub async fn fetch_members() -> Result<Vec<(String, String)>, reqwest::Error> {
    let client = reqwest::Client::new();
    let query = r#"
    query {
        getMember {
            id,
            name
        }
    }"#;

    let response = client
        .post(REQUEST_URL)
        .json(&serde_json::json!({"query": query}))
        .send()
        .await?;

    let json: Value = response.json().await?;

    let members: Vec<(String, String)> = json["data"]["getMember"]
    .as_array()
    .unwrap()
    .iter()
    .map(|member| {
        let id = member["id"].as_i64().map(|num| num.to_string()).unwrap_or_default();
        let name = member["name"].as_str().unwrap_or("").to_string();
        (id, name)
    })
    .collect();

    Ok(members)
}

pub async fn fetch_attendance() -> Result<Vec<(String, String)>, reqwest::Error> {
    let client = reqwest::Client::new();

    let today = Local::now().format("%Y-%m-%d").to_string();
    let query = format!(
        r#"
        query {{
            getAttendance(date: "{}") {{
                id,
                timein
            }}
        }}"#,
        today
    );

    let response = client
        .post(REQUEST_URL)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;

    let json: Value = response.json().await?;

    let attendance: Vec<(String, String)> = json["data"]["getAttendance"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let id = entry["id"].to_string();
            let timein = entry["timein"].as_str().unwrap_or("").to_string();
            (id, timein)
        })
        .collect();

    Ok(attendance)
}
