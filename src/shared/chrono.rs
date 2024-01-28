use chrono::{ Local, DateTime };

pub fn getCurrentTimeStr() -> String{
    let localDateTime: DateTime<Local> = Local::now();
    
    let formatISO = format!("{}", localDateTime.format("%Y-%m-%d %H:%M:%S"));
    return formatISO;
}