use std::process::Command;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use chrono::Local;

pub async fn network_check() {
    let mut response: String = String::from("");

    response.push_str(&ping_and_analyze("google.com",4).await);
    // response.push_str(&ping_and_analyze("hotmail.com", 60).await);
    // response.push_str(&traceroute("apps4rent.com").await);
    // response.push_str(&traceroute("billing.apps4rent.com").await);
    // response.push_str(&traceroute("cp.hostallapps.com").await);

    let now = Local::now();
    let file_name = format!("network_check_{}.txt", now.format("%Y-%m-%d_%H-%M-%S"));
    let mut file = File::create(&file_name).expect("Failed to create file");
    file.write_all(response.as_bytes()).expect("Failed to write to file");
}

async fn ping_and_analyze(host: &str, duration: u64) -> String {
   
    let output = Command::new("ping")
        .arg(host)
        .arg("-n").arg(duration.to_string())
        .output()
        .expect("Failed to execute ping");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<&str> = output_str.lines().collect();
    
    let start = if lines.len() > 4 {
        lines.len() - 4
    } else {
        0
    };
    let last_lines: String = lines[start..].join("\n");
    let formatted_result: String = format!("Ping Result for {}:\n{}\n", host, last_lines);
    formatted_result
}

async fn traceroute(host: &str) -> String {

    let output = Command::new("tracert")
        .arg(host)
        .output()
        .expect("Failed to execute tracert");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let latency_re = Regex::new(r"(\d+) ms").unwrap();
    let mut hop_count = 0;

    for line in output_str.lines() {
        if let Some(latency_caps) = latency_re.captures(line) {
            let latency = &latency_caps[1];
            println!("Hop {}: Latency = {} ms", hop_count + 1, latency);
        }
        hop_count += 1;
    }

    let formatted_result: String = format!("Route Trace Rresult for {}:\n{}\nTotal number of hops{}\n", host, output_str,hop_count);
    formatted_result
}
