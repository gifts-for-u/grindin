use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayRoutine {
    pub day_key: String,       // "senin", "selasa", etc.
    pub day_name: String,      // "Senin", "Selasa", etc.
    pub focus: String,         // "Lower Body & Core", etc.
    pub moves: Vec<String>,    // 5 exercises
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSchedule {
    pub routines: Vec<DayRoutine>,
}

impl Default for WorkoutSchedule {
    fn default() -> Self {
        Self {
            routines: vec![
                DayRoutine {
                    day_key: "senin".to_string(),
                    day_name: "Senin".to_string(),
                    focus: "Lower Body & Core".to_string(),
                    moves: vec![
                        "Squat".to_string(),
                        "Reverse Lunge".to_string(),
                        "Glute Bridge".to_string(),
                        "Plank Shoulder Tap".to_string(),
                        "Bicycle Crunch".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "selasa".to_string(),
                    day_name: "Selasa".to_string(),
                    focus: "Upper Body & Cardio".to_string(),
                    moves: vec![
                        "Push-Up".to_string(),
                        "Mountain Climber".to_string(),
                        "Pike Push-Up / Downward Dog".to_string(),
                        "Skater Steps".to_string(),
                        "Dead Bug".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "rabu".to_string(),
                    day_name: "Rabu".to_string(),
                    focus: "Full-Body Functional".to_string(),
                    moves: vec![
                        "Sumo Squat".to_string(),
                        "Inchworm".to_string(),
                        "Bear Crawl Hold".to_string(),
                        "Lunge to Knee Drive".to_string(),
                        "High Plank".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "kamis".to_string(),
                    day_name: "Kamis".to_string(),
                    focus: "Core Stability & Posture".to_string(),
                    moves: vec![
                        "Bird-Dog".to_string(),
                        "Side Plank (Kanan/Kiri)".to_string(),
                        "Glute Bridge March".to_string(),
                        "Superman Hold".to_string(),
                        "Cat-Cow".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "jumat".to_string(),
                    day_name: "Jumat".to_string(),
                    focus: "HIIT Fat Burn".to_string(),
                    moves: vec![
                        "Squat to Calf Raise".to_string(),
                        "Speed Skaters".to_string(),
                        "Push-Up to T-Rotation".to_string(),
                        "High Marching".to_string(),
                        "Hollow Tuck".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "sabtu".to_string(),
                    day_name: "Sabtu".to_string(),
                    focus: "Total Body Strength".to_string(),
                    moves: vec![
                        "Curtsy Lunge".to_string(),
                        "Hand-Release Push-Up".to_string(),
                        "Tabletop Bridge".to_string(),
                        "Mountain Climber".to_string(),
                        "Forearm Plank".to_string(),
                    ],
                },
                DayRoutine {
                    day_key: "minggu".to_string(),
                    day_name: "Minggu".to_string(),
                    focus: "Mobility & Active Core".to_string(),
                    moves: vec![
                        "Dead Bug".to_string(),
                        "Bird-Dog".to_string(),
                        "Glute Bridge Hold".to_string(),
                        "Child's Pose".to_string(),
                        "Thoracic Stretch".to_string(),
                    ],
                },
            ],
        }
    }
}

impl WorkoutSchedule {
    pub fn get_routine_by_key(&self, key: &str) -> Option<&DayRoutine> {
        self.routines.iter().find(|r| r.day_key.eq_ignore_ascii_case(key))
    }

    pub fn get_routine_for_day_index(&self, day_index: u8) -> &DayRoutine {
        // ponytail: maps JavaScript Date.getDay() (0=Minggu, 1=Senin, ..., 6=Sabtu) to index in routines
        let idx = match day_index {
            1 => 0, // Senin
            2 => 1, // Selasa
            3 => 2, // Rabu
            4 => 3, // Kamis
            5 => 4, // Jumat
            6 => 5, // Sabtu
            _ => 6, // 0 = Minggu
        };
        &self.routines[idx]
    }
}
