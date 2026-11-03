use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone)]
pub struct ScoreEntry {
    pub score: i32,
    pub time: String,
    pub name: String,
    pub settings: String,
    pub checksum: i32,
}

pub struct HighScores {
    pub entries: Vec<ScoreEntry>,
}

impl HighScores {
    pub fn load_or_default() -> Self {
        if let Ok(file) = File::open("hiscores.dat") {
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
            if lines.len() >= 50 {
                let mut entries = Vec::new();
                for i in 0..10 {
                    let base = i * 5;
                    let score = lines[base].trim().parse::<i32>().unwrap_or(0);
                    let time = lines.get(base + 1).cloned().unwrap_or_else(|| "10:00".into());
                    let name = lines.get(base + 2).cloned().unwrap_or_else(|| "Player".into());
                    let settings = lines.get(base + 3).cloned().unwrap_or_default();
                    let checksum = lines.get(base + 4).and_then(|s| s.trim().parse::<i32>().ok()).unwrap_or(0);

                    entries.push(ScoreEntry {
                        score,
                        time,
                        name,
                        settings,
                        checksum,
                    });
                }
                return Self { entries };
            }
        }

        // Authentic defaults
        let defaults = vec![
            (200000, "11 mns, 0 secs", "Mark Inc"),
            (181000, "10 mns, 20 secs", "Mark Inc"),
            (162000, "9 mns, 40 secs", "Mark Inc"),
            (143000, "9 mns, 0 secs", "Mark Inc"),
            (124000, "8 mns, 10 secs", "Mark Inc"),
            (105000, "7 mns, 30 secs", "Mark Inc"),
            (86000, "6 mns, 50 secs", "Mark Inc"),
            (67000, "5 mns, 40 secs", "Mark Inc"),
            (48000, "4 mns, 20 secs", "Mark Inc"),
            (29000, "3 mns, 0 secs", "Mark Inc"),
        ];

        let entries = defaults
            .into_iter()
            .map(|(score, time, name)| ScoreEntry {
                score,
                time: time.to_string(),
                name: name.to_string(),
                settings: "0,0,0,0,0,0,0,0,0,0".to_string(),
                checksum: 3000,
            })
            .collect();

        Self { entries }
    }

    pub fn is_high_score(&self, score: i32) -> bool {
        if self.entries.len() < 10 {
            true
        } else {
            score > self.entries.last().map(|e| e.score).unwrap_or(0)
        }
    }

    pub fn add(&mut self, score: i32, name: String, time: String) -> usize {
        let entry = ScoreEntry {
            score,
            time,
            name,
            settings: "0,0,0,0,0,0,0,0,0,0".to_string(),
            checksum: 3000,
        };

        let mut insert_pos = self.entries.len();
        for (i, e) in self.entries.iter().enumerate() {
            if score > e.score {
                insert_pos = i;
                break;
            }
        }

        self.entries.insert(insert_pos, entry);
        if self.entries.len() > 10 {
            self.entries.truncate(10);
        }

        let _ = self.save();
        insert_pos
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut file = File::create("hiscores.dat")?;
        // GridWars format repeats 3 times for 3 tables
        for _ in 0..3 {
            for entry in &self.entries {
                writeln!(file, "{}", entry.score)?;
                writeln!(file, "{}", entry.time)?;
                writeln!(file, "{}", entry.name)?;
                writeln!(file, "{}", entry.settings)?;
                writeln!(file, "{}", entry.checksum)?;
            }
        }
        Ok(())
    }
}
