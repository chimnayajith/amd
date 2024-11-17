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
use serenity::all::{ChannelId, Context, CreateMessage};
use crate::{
    ids::{THE_LAB_CHANNEL_ID},
    utils::{graphql::fetch_members, graphql::fetch_attendance, time::get_five_forty_pm_timestamp},
};

use chrono::{NaiveTime, ParseError, Utc, DateTime, Local, Timelike, Datelike, TimeZone};

pub async fn check_lab_attendance(ctx: Context) {
    let members = fetch_members().await.expect("Root must be up.");
    let attendance = fetch_attendance().await.expect("Root must be up");

    let time = chrono::Local::now().with_timezone(&chrono_tz::Asia::Kolkata);
    let threshold_time = get_five_forty_pm_timestamp(time);

    let mut absent_list = Vec::new();
    let mut late_list = Vec::new();

    for (id, timein) in attendance {
        if timein == "00:00:00" {
            if let Some((_, name)) = members.iter().find(|(member_id, _)| member_id == &id) {
                absent_list.push(name.clone());
            }
        } else if let Ok(time) = parse_time(&timein) {
            if time > threshold_time {
                if let Some((_, name)) = members.iter().find(|(member_id, _)| member_id == &id) {
                    late_list.push(name.clone());
                }
            }
        }
    }
    if absent_list.len() == members.len(){
        let today_date = Utc::now().format("%B %d, %Y").to_string();
        let mut message_content = format!("## Presence Report - {}\n\n", today_date);
        message_content.push_str("Uh-oh, seems like the lab is closed today! 🏖️ Everyone is absent!");
        let lab_attendance_channel = ChannelId::new(THE_LAB_CHANNEL_ID);
        lab_attendance_channel
            .send_message(ctx.http.clone(), CreateMessage::new().content(message_content))
            .await;
    } else {
        send_attendance_report(ctx, absent_list, late_list, THE_LAB_CHANNEL_ID).await;
    }
}


// function to convert time in string to Local time.
fn parse_time(time_str: &str) -> Result<DateTime<Local>, ParseError> {
    let time_only = time_str.split('.').next().unwrap();
    let naive_time = NaiveTime::parse_from_str(time_only, "%H:%M:%S")?;
    let now = Local::now();
    let datetime = Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(naive_time.hour(), naive_time.minute(), naive_time.second());
    Ok(datetime)
}

// function to format and send the message to #the-lab
async fn send_attendance_report(
    ctx: Context,
    absent_list: Vec<String>,
    late_list: Vec<String>,
    channel_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let lab_attendance_channel = ChannelId::new(channel_id);

    let today_date = Utc::now().format("%B %d, %Y").to_string();

    let mut message_content = format!("## Presence Report - {}\n\n", today_date);

    if absent_list.is_empty() {
        message_content.push_str("**Absent**\nNo one is absent today! 🎉\n\n");
    } else {
        let formatted_absent_list = absent_list
            .iter()
            .enumerate()
            .map(|(i, member)| format!("{}. {}", i + 1, member))
            .collect::<Vec<String>>()
            .join("\n");
        message_content.push_str(format!("**Absent**\n{}\n\n", formatted_absent_list).as_str());
    }

    if late_list.is_empty() {
        message_content.push_str("**Late**\nNo one is late today! 🙌\n\n");
    } else {
        let formatted_late_list = late_list
            .iter()
            .enumerate()
            .map(|(i, member)| format!("{}. {}", i + 1, member))
            .collect::<Vec<String>>()
            .join("\n");
        message_content.push_str(format!("**Late**\n{}\n\n", formatted_late_list).as_str());
    }

    lab_attendance_channel
        .send_message(ctx.http.clone(), CreateMessage::new().content(message_content))
        .await?;

    Ok(())
}
