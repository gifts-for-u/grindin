use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub muscle_group: String,
    pub form_cue: String,
    pub position_cue: String,
}

impl Exercise {
    pub fn new(name: &str, muscle_group: &str, form_cue: &str, position_cue: &str) -> Self {
        Self {
            name: name.to_string(),
            muscle_group: muscle_group.to_string(),
            form_cue: form_cue.to_string(),
            position_cue: position_cue.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayRoutine {
    pub day_key: String,                  // "senin", "selasa", etc.
    pub day_name: String,                 // "Monday", "Tuesday", etc.
    pub focus: String,                    // "Lower Body & Core", etc.
    pub moves: Vec<Exercise>,             // 5 canonical exercises
    pub warmup_movements: Vec<String>,    // 3 warmup movements
    pub cooldown_movements: Vec<String>,  // 2 cooldown movements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSchedule {
    pub routines: Vec<DayRoutine>,
}

impl Default for WorkoutSchedule {
    fn default() -> Self {
        let warmup_default = vec![
            "Putar lengan (arm circles) ke depan & belakang (30s)".to_string(),
            "Putar pinggul (hip circles) melingkar (30s)".to_string(),
            "Jalan santai di tempat (marching in place) (30s)".to_string(),
        ];

        let cooldown_default = vec![
            "Child's pose dekompresi tulang belakang (30s)".to_string(),
            "Peregangan dada dan paha depan perlahan (30s)".to_string(),
        ];

        Self {
            routines: vec![
                DayRoutine {
                    day_key: "senin".to_string(),
                    day_name: "Monday".to_string(),
                    focus: "Lower Body & Core".to_string(),
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Squat", "Paha depan (quads), bokong (glutes) & core", "Turunkan pinggul seperti duduk di kursi, lutut sejajar ujung jari, dada tetap tegap.", "Posisi berdiri"),
                        Exercise::new("Reverse Lunge", "Paha depan, hamstring, glutes & stabilitas satu kaki", "Langkahkan kaki ke belakang, tekuk kedua lutut 90 derajat tanpa membentur lantai.", "Posisi berdiri"),
                        Exercise::new("Glute Bridge", "Rantai posterior, gluteus maximus & hamstring", "Tekan tumit ke lantai, dorong pinggul ke atas hingga lurus dengan paha, tahan 1 detik.", "Telentang di matras"),
                        Exercise::new("Plank Shoulder Tap", "Stabilitas anti-rotasi inti, bahu & pektoralis", "Tepuk bahu berlawanan perlahan, kunci panggul agar tidak bergoyang ke samping.", "Posisi high plank"),
                        Exercise::new("Bicycle Crunch", "Otot obliq samping perut & rektus abdominis", "Putar bahu ke arah lutut yang ditarik, jangan menarik leher dengan tangan.", "Telentang di matras"),
                    ],
                },
                DayRoutine {
                    day_key: "selasa".to_string(),
                    day_name: "Tuesday".to_string(),
                    focus: "Upper Body & Cardio".to_string(),
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Push-Up", "Dada (pectoralis), bahu anterior & trisep", "Turunkan dada terkontrol, siku membentuk sudut 45 derajat, badan lurus seperti papan.", "Posisi plank di lantai"),
                        Exercise::new("Mountain Climber", "Pemacu kardio, fleksor panggul & perut bawah", "Tarik lutut bergantian ke dada dengan ritme stabil tanpa mengangkat pinggul terlalu tinggi.", "Posisi high plank"),
                        Exercise::new("Pike Push-Up / Downward Dog", "Deltoid bahu atas, trisep & mobilitas hamstring", "Angkat pinggul membentuk huruf V terbalik, tekuk siku dorong kepala ke arah lantai.", "Posisi V terbalik di lantai"),
                        Exercise::new("Skater Steps", "Daya ledak lateral, gluteus medius & stabilitas lutut", "Lompat/langkah menyamping dari satu kaki ke kaki lain, tekuk lutut untuk meredam pendaratan.", "Posisi berdiri"),
                        Exercise::new("Dead Bug", "Stabilitas anterior inti dalam & kontrol pelvis", "Turunkan lengan dan kaki berlawanan perlahan sambil menempelkan pinggang bawah rata ke lantai.", "Telentang di matras"),
                    ],
                },
                DayRoutine {
                    day_key: "rabu".to_string(),
                    day_name: "Wednesday".to_string(),
                    focus: "Full-Body Functional".to_string(),
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Sumo Squat", "Paha dalam (adductor), glutes & quads", "Buka kaki lebih lebar dari bahu dengan ujung jari menyerong 45°, turunkan pinggul dalam.", "Posisi berdiri lebar"),
                        Exercise::new("Inchworm", "Mobilitas hamstring, kekuatan bahu & core", "Bungkukkan badan, jalankan telapak tangan ke depan sampai posisi plank lalu jalan mundur.", "Berdiri lalu merangkak"),
                        Exercise::new("Bear Crawl Hold", "Otot inti dalam, bahu & fleksor panggul", "Posisi merangkak dengan lutut melayang 2-3 cm dari lantai, jaga punggung tetap rata.", "Posisi merangkak (quadrupedal)"),
                        Exercise::new("Lunge to Knee Drive", "Glutes, paha, keseimbangan dinamis & kardio", "Lunge ke belakang lalu dorong lutut yang sama ke arah dada secara bertenaga.", "Posisi berdiri"),
                        Exercise::new("High Plank", "Total core isometric, bahu & scapular stability", "Tumpu pada telapak tangan lurus, kunci perut dan glutes, dorong lantai menjauh.", "Posisi high plank"),
                    ],
                },
                DayRoutine {
                    day_key: "kamis".to_string(),
                    day_name: "Thursday".to_string(),
                    focus: "Core Stability & Posture",
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Bird-Dog", "Erector spinae, glutes & kestabilan silang tubuh", "Rentangkan tangan dan kaki berlawanan sejajar lantai tanpa melengkungkan pinggang bawah.", "Posisi merangkak di matras"),
                        Exercise::new("Side Plank (Kanan/Kiri)", "Obliq samping, quadratus lumborum & gluteus medius", "Tumpu pada satu siku, angkat pinggul hingga badan membentuk satu garis lurus diagonal.", "Posisi menyamping di matras"),
                        Exercise::new("Glute Bridge March", "Gluteus, hamstring & stabilitas panggul dinamis", "Tahan posisi bridge, angkat lutut bergantian perlahan tanpa membiarkan pinggul miring.", "Telentang di matras"),
                        Exercise::new("Superman Hold", "Punggung atas, erector spinae & glutes", "Tengkurap, angkat dada dan paha secara bersamaan dari lantai, tahan dengan napas teratur.", "Tengkurap di matras"),
                        Exercise::new("Cat-Cow", "Mobilitas artikulasi tulang belakang & relaksasi leher", "Lengkungkan punggung ke atas sambil buang napas, lalu turunkan perut sambil tarik napas.", "Posisi merangkak di matras"),
                    ],
                },
                DayRoutine {
                    day_key: "jumat".to_string(),
                    day_name: "Friday".to_string(),
                    focus: "HIIT Fat Burn",
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Squat to Calf Raise", "Quads, glutes & betis (gastrocnemius)", "Lakukan squat penuh, saat berdiri dorong bertenaga hingga berjinjit di ujung jari kaki.", "Posisi berdiri"),
                        Exercise::new("Speed Skaters", "Pembakar kalori, gluteus medius & kelincahan", "Lompat menyamping dengan ritme cepat, silangkan kaki belakang dengan dada tetap tegap.", "Posisi berdiri dinamis"),
                        Exercise::new("Push-Up to T-Rotation", "Dada, bahu, trisep & rotasi obliq", "Lakukan 1 push-up, lalu putar badan ke satu sisi angkat tangan ke atas membentuk huruf T.", "Posisi push-up di lantai"),
                        Exercise::new("High Marching", "Kardio aerobik, fleksor panggul & core", "Angkat lutut setinggi pinggang secara bergantian dengan ayunan lengan yang bertenaga.", "Posisi berdiri di tempat"),
                        Exercise::new("Hollow Tuck", "Rektus abdominis & kekuatan dinding perut", "Duduk seimbang di tulang ekor, tarik lutut ke dada lalu rentangkan badan terkontrol.", "Duduk di matras"),
                    ],
                },
                DayRoutine {
                    day_key: "sabtu".to_string(),
                    day_name: "Saturday".to_string(),
                    focus: "Total Body Strength",
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Curtsy Lunge", "Gluteus medius, paha luar & stabilitas panggul", "Silangkan satu kaki ke belakang menyerong, tekuk kedua lutut stabil, dada tetap tegap.", "Posisi berdiri"),
                        Exercise::new("Hand-Release Push-Up", "Dada, bahu, rentang gerak penuh & postur", "Turunkan badan sampai dada menyentuh lantai, angkat telapak sesaat, lalu dorong ke atas.", "Tengkurap di lantai"),
                        Exercise::new("Tabletop Bridge", "Rantai otot belakang, glutes & pembuka bahu depan", "Duduk lutut ditekuk tangan di belakang, dorong pinggul ke atas hingga sejajar meja, tahan 1 detik.", "Duduk di matras"),
                        Exercise::new("Mountain Climber", "Otot perut bawah, bahu & pacu kardio", "High plank, tarik lutut bergantian cepat dan stabil tanpa membiarkan pinggul naik.", "Posisi high plank"),
                        Exercise::new("Forearm Plank", "Core isometrik & pelindung pinggang bawah", "Tumpu pada lengan bawah, kunci perut dan bokong, pandangan ke ujung jari tangan.", "Posisi forearm plank di lantai"),
                    ],
                },
                DayRoutine {
                    day_key: "minggu".to_string(),
                    day_name: "Sunday".to_string(),
                    focus: "Mobility & Active Core",
                    warmup_movements: warmup_default.clone(),
                    cooldown_movements: cooldown_default.clone(),
                    moves: vec![
                        Exercise::new("Dead Bug", "Stabilitas inti lumbopelvic anterior", "Tekan punggung bawah rata ke matras, gerakkan tangan dan kaki berlawanan perlahan.", "Telentang di matras"),
                        Exercise::new("Bird-Dog", "Keseimbangan diagonal & otot punggung", "Rentangkan tangan dan kaki berlawanan, tahan 2 detik di puncak dengan tubuh sejajar.", "Posisi merangkak di matras"),
                        Exercise::new("Glute Bridge Hold", "Gluteus maximus & rantai posterior", "Dorong pinggul tinggi dan tahan di puncak, remas otot bokong tanpa membebani leher.", "Telentang di matras"),
                        Exercise::new("Child's Pose", "Dekompresi pinggang bawah & relaksasi panggul", "Duduk di atas tumit, julurkan lengan ke depan lantai, tarik napas dalam dan rileks.", "Bersujud di matras"),
                        Exercise::new("Thoracic Stretch", "Mobilitas punggung tengah (toraks) & pembuka dada", "Tangan di belakang kepala, putar siku ke atas membuka dada sambil buang napas perlahan.", "Duduk atau merangkak"),
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
