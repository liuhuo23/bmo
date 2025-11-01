pub fn format_time(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
