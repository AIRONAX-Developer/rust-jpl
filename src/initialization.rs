use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;

mod structures;

use structures::{Date, Ephemerides, SpaceObject};

fn read_init_data(tmp: &mut Ephemerides) -> io::Result<()> {
    let mut indat = File::open("Initial_data.dat")?;

    let mut buffer = String::new();
    indat.read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    while let Some(line) = lines.next() {
        if line == "BODIES:" {
            while let Some(body_line) = lines.next() {
                if body_line == "DATE:" {
                    break;
                }
                let mut parts = body_line.split_whitespace();
                if let Some(name) = parts.next() {
                    let mut so = SpaceObject::default();
                    so.object_name = String::from(name);
                    if let Some(state_str) = parts.next() {
                        so.object_state = bool::from_str(state_str).unwrap_or_default();
                    }
                    tmp.bodies.push(so);
                }
            }
        }

        if line == "DATE:" {
            if let Some(start_year_str) = lines.next() {
                if start_year_str.starts_with("Start_year") {
                    tmp.start_year = start_year_str.split_whitespace().last().and_then(|s| i32::from_str(s).ok()).unwrap_or_default();
                }
            }

            if let Some(end_year_str) = lines.next() {
                if end_year_str.starts_with("End_year") {
                    tmp.end_year = end_year_str.split_whitespace().last().and_then(|s| i32::from_str(s).ok()).unwrap_or_default();
                }
            }
        }
    }

    Ok(())
}

fn read_header_ascp(tmp: &mut Ephemerides) -> io::Result<()> {
    let mut header = File::open("files\\header.405")?;

    let mut buffer = String::new();
    header.read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let mut number_emrat = 0;

    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        if let Some(key) = parts.next() {
            match key {
                "NCOEFF=" => {
                    tmp.ncoeff = parts.next().and_then(|s| i32::from_str(s).ok()).unwrap_or_default();
                }
                "GROUP" => {
                    if let Some(group_number_str) = parts.next() {
                        match group_number_str {
                            "1030" => {
                                if let Some(start_str) = parts.next() {
                                    tmp.julian_start = f64::from_str(start_str).unwrap_or_default();
                                }
                                if let Some(end_str) = parts.next() {
                                    tmp.julian_end = f64::from_str(end_str).unwrap_or_default();
                                }
                                if let Some(interval_str) = parts.next() {
                                    tmp.interval = i32::from_str(interval_str).unwrap_or_default();
                                }
                            }
                            "1040" => {
                                if let Some(size_str) = parts.next() {
                                    let size = i32::from_str(size_str).unwrap_or_default();
                                    for _ in 0..size {
                                        if let Some(tmp) = parts.next() {
                                            if tmp == "EMRAT" {
                                                number_emrat = i;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            "1041" => {
                                for _ in 0..number_emrat {
                                    parts.next();
                                }
                                if let Some(emrat_str) = parts.next() {
                                    let mut emrat = emrat_str.to_string();
                                    let len = emrat.len();
                                    emrat.truncate(len - 4);
                                    emrat.push('E');
                                    tmp.emrat = f64::from_str(&emrat).unwrap_or_default();
                                }
                            }
                            "1050" => {
                                for _ in 0..3 {
                                    let mut header_data = HashMap::new();
                                    for body in &mut tmp.bodies {
                                        if let Some(buf_str) = parts.next() {
                                            let buf = i32::from_str(buf_str).unwrap_or_default();
                                            body.object_header_data.push(buf);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn calculate_number_of_elements(eph: &mut Ephemerides) {
    for i in 0..eph.bodies.len() - 1 {
        eph.bodies[i].object_length = eph.bodies[i + 1].object_header_data[0] - eph.bodies[i].object_header_data[0];
    }
    eph.bodies[eph.bodies.len() - 1].object_length = eph.ncoeff - eph.bodies[eph.bodies.len() - 1].object_header_data[0];
}

fn initialization(eph: &mut Ephemerides) -> io::Result<()> {
    read_init_data(eph)?;
    read_header_ascp(eph)?;
    calculate_number_of_elements(eph);

    Ok(())
}

fn main() -> io::Result<()> {
    let mut ephemerides_data = Ephemerides::default();

    initialization(&mut ephemerides_data)?;

    // Continue with the rest of your Rust code
    Ok(())
}
