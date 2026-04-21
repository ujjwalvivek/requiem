//? Procedural WAV generation

use engine::{AudioManager, AudioTrack, StaticSoundData, load_sound_data};

const SAMPLE_RATE: u32 = 44100;

//? Encode raw i16 samples as a WAV byte buffer and leak it to 'static.
fn encode_wav(samples: &[i16]) -> &'static [u8] {
    let data_size = (samples.len() * 2) as u32;
    let file_size = 36 + data_size;
    let mut buf = Vec::with_capacity(44 + data_size as usize);

    //* RIFF header
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");

    //* fmt chunk
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); //* subchunk size
    buf.extend_from_slice(&1u16.to_le_bytes()); //* PCM format
    buf.extend_from_slice(&1u16.to_le_bytes()); //* mono
    buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes()); //* sample rate
    buf.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); //* byte rate
    buf.extend_from_slice(&2u16.to_le_bytes()); //* block align
    buf.extend_from_slice(&16u16.to_le_bytes()); //* bits per sample

    //* data chunk
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());
    for &s in samples {
        buf.extend_from_slice(&s.to_le_bytes());
    }

    Box::leak(buf.into_boxed_slice())
}

//? Simple LCG noise source for audio synthesis.
fn noise(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
    ((*seed >> 33) as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn to_sample(val: f32) -> i16 {
    (val.clamp(-1.0, 1.0) * 32000.0) as i16
}

fn samples_for(duration_secs: f32) -> usize {
    (SAMPLE_RATE as f32 * duration_secs) as usize
}

//? Sound generators

//? Shoot: sharp, percussive "tick", square wave + noise transient
fn gen_shoot() -> Vec<i16> {
    let len = samples_for(0.06);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xBAD_CAFE_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 60.0).exp(); //* fast decay

        //* Noise transient in first 5ms
        let noise_env = if t < 0.005 { 1.0 - t / 0.005 } else { 0.0 };
        let noise_val = noise(&mut seed) * noise_env * 0.4;

        //* Square wave at 880Hz
        let phase = (880.0 * t * std::f32::consts::TAU).sin();
        let square = if phase > 0.0 { 0.6 } else { -0.6 };

        let val = (square + noise_val) * env * 0.7;
        out.push(to_sample(val));
    }
    out
}

//? Kill: satisfying crunch, descending noise burst with sine
fn gen_kill() -> Vec<i16> {
    let len = samples_for(0.10);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xDEAD_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 25.0).exp();

        //* Descending sine: 500Hz → 120Hz
        let freq = 500.0 - t * 3800.0;
        let sine = (freq.max(80.0) * t * std::f32::consts::TAU).sin() * 0.5;

        //* Noise crunch
        let n = noise(&mut seed) * 0.5;

        let val = (sine + n) * env * 0.65;
        out.push(to_sample(val));
    }
    out
}

//? Player hit: heavy bass thud
fn gen_hit() -> Vec<i16> {
    let len = samples_for(0.18);
    let mut out = Vec::with_capacity(len);
    let mut seed = 0xBEEF_u64;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 12.0).exp();

        //* Low sine thud at 65Hz
        let sine = (65.0 * t * std::f32::consts::TAU).sin() * 0.7;

        //* Impact noise in first 15ms
        let noise_env = if t < 0.015 {
            1.0
        } else {
            (-((t - 0.015) * 40.0)).exp()
        };
        let n = noise(&mut seed) * noise_env * 0.5;

        let val = (sine + n) * env * 0.8;
        out.push(to_sample(val));
    }
    out
}

//? Level up: triumphant ascending arpeggio, three quick tones
fn gen_levelup() -> Vec<i16> {
    let len = samples_for(0.35);
    let mut out = Vec::with_capacity(len);

    //* C5 → E5 → G5 (major chord arpeggio)
    let notes = [523.25_f32, 659.25, 783.99];
    let note_dur = 0.09;
    let gap = 0.025;

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let mut val = 0.0_f32;

        for (idx, &freq) in notes.iter().enumerate() {
            let start = idx as f32 * (note_dur + gap);
            let local_t = t - start;
            if local_t >= 0.0 && local_t < note_dur {
                let env = (-(local_t / note_dur) * 4.0).exp();
                //* Sine + slight square for brightness
                let sine = (freq * local_t * std::f32::consts::TAU).sin();
                let harmonic = (freq * 2.0 * local_t * std::f32::consts::TAU).sin() * 0.2;
                val += (sine + harmonic) * env * 0.5;
            }
        }

        out.push(to_sample(val * 0.7));
    }
    out
}

//? XP pickup: tiny satisfying blip
fn gen_xp() -> Vec<i16> {
    let len = samples_for(0.035);
    let mut out = Vec::with_capacity(len);

    for i in 0..len {
        let t = i as f32 / SAMPLE_RATE as f32;
        let env = (-t * 80.0).exp();
        let sine = (1600.0 * t * std::f32::consts::TAU).sin();
        let val = sine * env * 0.35;
        out.push(to_sample(val));
    }
    out
}

//? Audio assets
pub struct AudioAssets {
    pub shoot: Option<StaticSoundData>,
    pub kill: Option<StaticSoundData>,
    pub hit: Option<StaticSoundData>,
    pub levelup: Option<StaticSoundData>,
    pub xp: Option<StaticSoundData>,
}

impl AudioAssets {
    pub fn generate() -> Self {
        Self {
            shoot: load_sound_data(encode_wav(&gen_shoot())),
            kill: load_sound_data(encode_wav(&gen_kill())),
            hit: load_sound_data(encode_wav(&gen_hit())),
            levelup: load_sound_data(encode_wav(&gen_levelup())),
            xp: load_sound_data(encode_wav(&gen_xp())),
        }
    }
}

//? SFX event queue (produced in fixed_update, consumed in update)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfxEvent {
    Shoot,
    Kill,
    Hit,
    LevelUp,
    XpPickup,
}

pub fn dispatch_sfx(events: &[SfxEvent], assets: &AudioAssets, audio: &mut AudioManager) {
    for event in events {
        let data = match event {
            SfxEvent::Shoot => &assets.shoot,
            SfxEvent::Kill => &assets.kill,
            SfxEvent::Hit => &assets.hit,
            SfxEvent::LevelUp => &assets.levelup,
            SfxEvent::XpPickup => &assets.xp,
        };
        if let Some(d) = data {
            audio.play_oneshot(d, AudioTrack::Sfx);
        }
    }
}
