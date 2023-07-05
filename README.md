# batt_stats
A Linux notebook battery drain logger made in Rust. 

## [WIP] Installation
Clone the repository
```
git clone https://github.com/marcofpadeiro/batt_stats
```
And build the project using cargo
```
cd batt_stats
cargo build --release
```
After the build the executable will be at `./target/release/batt_stats`

## Usage
The script just needs to be running and it will automatically detect if the notebook is charging or discharging and log into `~/.cache/batt_stats`

To automatically run this script on startup you can create a simple systemd service
```
/etc/systemd/system/batt_stats.service
```
```
[Unit]
Description=A battery percentage logger

[Service]
ExecStart=/path/to/script <number_of_seconds_to_wait_between_log>
Restart=always

[Install]
WantedBy=default.target
```
And enable it 
```
systemctl enable batt_stats.service
```
