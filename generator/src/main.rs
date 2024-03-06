use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut name_database: HashMap<u64, String> = HashMap::new();

    for line in std::fs::read_to_string("../assets.txt").unwrap().lines() {
        let mut line = line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if line.starts_with("0x") || line.starts_with("0X") {
            line = &line[2..];
        }

        let parts = line.split_once(',');

        if let Some((hash, name)) = parts {
            let hash = u64::from_str_radix(hash.trim(), 16).unwrap();

            name_database.insert(hash, name.trim().to_string());
        }
    }

    let mut output = std::fs::File::create("../assets.pndb").unwrap();

    let mut keys: Vec<u64> = Vec::with_capacity(name_database.len());
    let mut decompressed: Vec<u8> = Vec::new();

    for entry in name_database.iter() {
        keys.push(*entry.0);

        decompressed.extend_from_slice(entry.1.as_bytes());
        decompressed.extend_from_slice(&[0]);
    }

    for key in keys.into_iter() {
        decompressed.extend_from_slice(&key.to_le_bytes());
    }

    let compressed = lz4_flex::compress(&decompressed);

    output.write_all(&[0x50, 0x4E, 0x44, 0x42]).unwrap();

    output
        .write_all(&(name_database.len() as u32).to_le_bytes())
        .unwrap();

    output
        .write_all(&(compressed.len() as u32).to_le_bytes())
        .unwrap();
    output
        .write_all(&(decompressed.len() as u32).to_le_bytes())
        .unwrap();

    output.write_all(&compressed).unwrap();
}
