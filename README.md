# Grindin' ⏱️⚡

**Grindin'** adalah aplikasi web interval timer harian berbasis **High-Intensity Circuit Training (HICT)** yang dirancang untuk latihan fungsional 10–15 menit setiap hari dengan panduan audio suara dan visual raksasa secara **100% hands-free**.

Dibangun menggunakan **Rust (WebAssembly)** dan Web APIs native (SpeechSynthesis & Web Audio).

---

## 🌟 Fitur Utama

- **Otomatis Deteksi Hari (Senin – Minggu):** Membuka latihan harian spesifik secara otomatis dengan 5 variasi gerakan fungsional.
- **Audio Voice & Synthesizer Cues Lengkap:**
  - 📢 Menyebutkan nama gerakan saat interval kerja mulai (*"Curtsy Lunge, mulai!"*).
  - 🔔 Pemberitahuan saat waktu tinggal setengah (*"Setengah waktu!"* / 20 detik pada interval 40s).
  - ⏳ Hitung mundur audio beep pendek di 3 detik terakhir (*3, 2, 1*).
  - ☕ Pengumuman istirahat beserta preview gerakan berikutnya (*"Istirahat 20 detik. Berikutnya: Hand-Release Push-Up"*).
  - 🎉 Chime perayaan saat seluruh putaran selesai.
- **Giant High-Contrast UI:** Angka hitung mundur berukuran masif dengan kode warna dinamis (Hijau = Kerja, Kuning/Oranye = Rehat, Biru = Pemanasan/Pendinginan). Terbaca jelas dari kejauhan saat posisi plank/push-up di lantai.
- **Customizer Fleksibel:** Tombol **⚙️ Edit** memungkinkan mengubah durasi kerja, durasi istirahat, rehat antar putaran, jumlah putaran, dan nama-nama gerakan yang tersimpan otomatis di `localStorage`.
- **Screen Wake Lock:** Mencegah layar smartphone/laptop meredup atau terkunci saat latihan berlangsung.
- **Zero Asset Bloat:** Menggunakan audio oscillator sintetis dan browser speech engine bawaan.

---

## 📅 Jadwal Gerakan Mingguan (HICT 40s Kerja / 20s Rehat / 2 Putaran)

| Hari | Fokus | Daftar Gerakan |
| :--- | :--- | :--- |
| **Senin** | Lower Body & Core | Squat, Reverse Lunge, Glute Bridge, Plank Shoulder Tap, Bicycle Crunch |
| **Selasa** | Upper Body & Cardio | Push-Up, Mountain Climber, Pike Push-Up / Downward Dog, Skater Steps, Dead Bug |
| **Rabu** | Full-Body Functional | Sumo Squat, Inchworm, Bear Crawl Hold, Lunge to Knee Drive, High Plank |
| **Kamis** | Core Stability & Posture | Bird-Dog, Side Plank, Glute Bridge March, Superman Hold, Cat-Cow |
| **Jumat** | HIIT Fat Burn | Squat to Calf Raise, Speed Skaters, Push-Up to T-Rotation, High Marching, Hollow Tuck |
| **Sabtu** | Total Body Strength | Curtsy Lunge, Hand-Release Push-Up, Tabletop Bridge, Mountain Climber, Forearm Plank |
| **Minggu** | Mobility & Active Core | Dead Bug, Bird-Dog, Glute Bridge Hold, Child's Pose, Thoracic Stretch |

---

## 🚀 Cara Menjalankan Secara Lokal

1. **Jalankan Langsung di Browser (Instan):**
   Cukup klik dua kali file `index.html` atau buka via browser Chrome/Firefox/Edge di laptop Anda.

2. **Kompilasi Rust WASM (Jika Rust & wasm-pack terpasang):**
   ```bash
   wasm-pack build --target web --release
   ```

---

## 🌐 Cara Deploy ke GitHub Pages

1. Buat repositori baru di GitHub bernama **`grindin`**.
2. Masukkan seluruh file project ini ke git:
   ```bash
   git init
   git add .
   git commit -m "feat(grindin): initial commit for interval timer webapp"
   git branch -M main
   git remote add origin https://github.com/<username-github-anda>/grindin.git
   git push -u origin main
   ```
3. Di halaman GitHub repositori Anda:
   - Buka tab **Settings** → **Pages**.
   - Pada bagian **Build and deployment** → **Source**, pilih **GitHub Actions**.
4. GitHub Actions akan otomatis mengompilasi Rust WASM dan mempublikasikan webapp ke:
   `https://<username-github-anda>.github.io/grindin/`
