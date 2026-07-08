use std::io::{self, Write};

pub struct ProgressBar {
    total_size: u64,
    current_size: u64,
    bar_width: usize,
}

impl ProgressBar {
    pub fn new(total_size: u64) -> Self {
        Self {
            total_size,
            current_size: 0,
            bar_width: 40,
        }
    }
    
    pub fn update(&mut self, amount: u64) {
        self.current_size = (self.current_size + amount).min(self.total_size);
        let percentage = (self.current_size as f64 / self.total_size as f64) * 100.0;
        
        let filled_len = ((self.current_size as f64 / self.total_size as f64) * self.bar_width as f64) as usize;
        let empty_len = self.bar_width - filled_len;
        
        let filled = "█".repeat(filled_len);
        let empty = "░".repeat(empty_len);
        
        print!("\r[{}{}] {:.2}% ({}/{})", filled, empty, percentage, self.current_size, self.total_size);
        
        io::stdout().flush().unwrap();
    }
    
    pub fn finish(&self) {
        println!("\nProcess complete.")
    }
}

pub fn simulate_progressbar(filesize: u64) {
    let total_filesize = filesize;
    let mut pb = ProgressBar::new(total_filesize);
    
    let mut downloaded = 0;
    while downloaded < total_filesize {
        let speed_chunk = match downloaded {
            0..=20   => 1,  
            21..=60  => 12, 
            61..=80  => 2,  
            _        => 5, 
        };

        downloaded += speed_chunk;
        pb.update(speed_chunk);

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    pb.finish();
}

pub fn spawn_ketupat(n: i8) {
    let range = n * 2 + 1;
    let mut i = 0;
    while i < range {
        let mut j = 0;
        while j < range {
            if i + j == n {
                print!("/");
            } else if j - i == n {
                print!("\\");
            } else if i - j == n {
                print!("\\");
            } else if i + j == n * 3 {
                print!("/");
            } else if n < i + j && i + j < n * 3 && (i - j).abs() < n {
                if (i + j) % 2 == 0 {
                    print!("—");
                } else {
                    print!("|")
                }
            }
            else {
                print!(" ");
            }
            j += 1;
        }
        println!();
        i += 1;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
