use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: locate-recent <filename>");
        std::process::exit(1);
    }
    let target_file = &args[1];
    let db_path = "/var/lib/plocate/plocate.db";
    let file = File::open(db_path)?;
    let mut reader = BufReader::new(file);

    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic)?;
    if &magic != b"\0mlocate" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid mlocate database"));
    }

    let mut config_size_buf = [0u8; 4];
    reader.read_exact(&mut config_size_buf)?;
    let config_size = u32::from_be_bytes(config_size_buf);
    
    let mut skip = [0u8; 4];
    reader.read_exact(&mut skip)?;

    let mut root_path = Vec::new();
    reader.read_until(0, &mut root_path)?;

    let mut config_block = vec![0u8; config_size as usize];
    reader.read_exact(&mut config_block)?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let twenty_four_hours = 24 * 60 * 60;

    loop {
        let mut time_secs_buf = [0u8; 8];
        if reader.read_exact(&mut time_secs_buf).is_err() { break; }
        let dir_time = u64::from_be_bytes(time_secs_buf);

        let mut time_nanos_buf = [0u8; 4];
        reader.read_exact(&mut time_nanos_buf)?;
        
        let mut padding = [0u8; 4];
        reader.read_exact(&mut padding)?;

        let mut dir_path_bytes = Vec::new();
        reader.read_until(0, &mut dir_path_bytes)?;
        let dir_path = String::from_utf8_lossy(&dir_path_bytes).trim_matches('\0').to_string();

        let is_recent = now >= dir_time && now - dir_time <= twenty_four_hours;
        let mut found_file = false;

        loop {
            let mut entry_type = [0u8; 1];
            reader.read_exact(&mut entry_type)?;
            if entry_type[0] == 2 { break; }

            let mut name_bytes = Vec::new();
            reader.read_until(0, &mut name_bytes)?;
            
            if is_recent {
                let name = String::from_utf8_lossy(&name_bytes).trim_matches('\0').to_string();
                if name == *target_file {
                    found_file = true;
                }
            }
        }

        if is_recent && found_file {
            println!("[Modified: {}s ago] {}", now - dir_time, dir_path);
        }
    }

    Ok(())
}
