use mac_address::get_mac_address;
use chrono::{Utc, DateTime};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct AppArg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    V1
}

fn main() {
    let cli = AppArg::parse();
    let uuid = match cli.action {
        Action::V1 => {
            generate_v1()
        }
    };
    println!("{}", uuid)
}

fn generate_v1() -> String {
    let diff_from_reference_vec = timestamp_to_vec(diff_from_reference());
    let clock_sequence = clock_sequence();
    let mac_address = mac_address_hex();

    let mut elements = diff_from_reference_vec;
    elements.push(clock_sequence);
    elements.push(mac_address);

    elements.join("-")
}

fn diff_from_reference() -> String {
    let current_timestamp = Utc::now().timestamp_nanos() / 100;

    // timestamp_nanosを使いたいが計算過程でi64の桁数を超えてpanicになる。
    // 1970/01/01 00:00:00 +0000 の差分ではナノ秒レベルの差分はないのでこれでOk
    let reference_timestamp = DateTime::parse_from_str("1582/10/15 00:00:00 +0000", "%Y/%m/%d %H:%M:%S %z")
                                            .unwrap()
                                            .timestamp() * 10_000_000;
    let diff = current_timestamp - reference_timestamp;
    format!("{:02x}", diff)
}

fn timestamp_to_vec(timestamp_hex: String) -> Vec<String> {
    let mut time_hi_and_version = "1".to_string();
    let mut time_mid = "".to_string();
    let mut time_low = "".to_string();
    for (i, c) in timestamp_hex.chars().enumerate() {
        if i < 3 {
            time_hi_and_version.push(c);
        } else if i >= 3 && i < 7 {
            time_mid.push(c);
        } else {
            time_low.push(c);
        }
    }
    vec![time_low, time_mid, time_hi_and_version]
}

fn clock_sequence() -> String {
    "0000".into()
}

fn mac_address_hex() -> String {
    let address = get_mac_address().unwrap().unwrap();
    address
        .bytes()
        .into_iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}
