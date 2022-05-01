use std::collections::{HashMap};
use std::path::{Path};
use std::io::prelude::*;
use std::fs::{File};

pub struct INIFile
{
    pub filepath: String,
    pub data: HashMap<String, String>
}
impl INIFile
{
    pub fn new(filepath: String) -> INIFile
    {
        INIFile {
            filepath: filepath,
            data: HashMap::new()
        }
    }

    pub fn read(&mut self) -> std::io::Result<()>
    {
        if Path::new(&self.filepath).exists() == false {
            self.save()?;
        }

        let mut data_new: HashMap<String, String> = HashMap::new();

        let file = File::open(&self.filepath).unwrap();
        let file_lines = std::io::BufReader::new(file).lines();
        for line in file_lines {
            let line_content = line.unwrap();
            match line_content.split_once('=') {
                Some((key, value)) => {
                    data_new.insert(String::from(key), String::from(value));
                    println!("config->read: {:?}={:?}", key, value);
                }
                None => {
                    println!("expected a key-value pair but found '{}'", line_content);
                }
            }
        }

        self.data.clear();
        self.data = data_new;

        Ok(())
    }

    pub fn save(&self) -> std::io::Result<()>
    {
        File::create(&self.filepath)?;

        let mut file_content = vec![];

        for (key, value) in (&self.data).iter() {
            file_content.push(format!("{}={}", key, value));
            println!("config->save: {:?}={:?}", key, value);
        }

        let full_file_content = file_content.join("\n");
        std::fs::write(&self.filepath, full_file_content)?;

        Ok(())
    }
}