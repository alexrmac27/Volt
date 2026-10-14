use std::sync::Arc;

use blerp::{Beats, SAMPLE_RATE};
use gpui::{Context, InteractiveElement, IntoElement, Render, Styled, Window, div, prelude::*, px, rgb};

use crate::views::playlist::PlaylistView;

const STEPS: usize = 16;
const NOTE_ROWS: [(u8, &str); 8] = [
    (72, "C5"),
    (71, "B4"),
    (69, "A4"),
    (67, "G4"),
    (65, "F4"),
    (64, "E4"),
    (62, "D4"),
    (60, "C4"),
];
const DRUM_ROWS: [&str; 4] = ["Kick", "Snare", "Closed hat", "Open hat"];

pub struct PianoMachine {
    playlist: gpui::Entity<PlaylistView>,
    notes: [[bool; STEPS]; NOTE_ROWS.len()],
    drums: [[bool; STEPS]; DRUM_ROWS.len()],
    generated_clips: Vec<usize>,
}

impl PianoMachine {
    pub fn new(playlist: gpui::Entity<PlaylistView>) -> Self {
        let mut drums = [[false; STEPS]; DRUM_ROWS.len()];
        for step in [0, 8] {
            drums[0][step] = true;
        }
        for step in [4, 12] {
            drums[1][step] = true;
        }
        for step in (0..STEPS).step_by(2) {
            drums[2][step] = true;
        }
        Self {
            playlist,
            notes: [[false; STEPS]; NOTE_ROWS.len()],
            drums,
            generated_clips: Vec::new(),
        }
    }

    fn rebuild(&mut self, cx: &mut Context<Self>) {
        let notes = self.notes;
        let drums = self.drums;
        let previous = std::mem::take(&mut self.generated_clips);
        let mut generated_clips = Vec::new();
        self.playlist.update(cx, |view, _| {
            view.audio.delete_clips(&previous);
            let tempo = view.audio.playlist().tempo;
            for (row, (midi_note, _)) in NOTE_ROWS.iter().enumerate() {
                for step in 0..STEPS {
                    if notes[row][step] {
                        let start = step as f64 / 4.0;
                        let end = start + 0.24;
                        let samples = synth_note(*midi_note, tempo.bpm(), end - start);
                        let id = view.audio.add_audio_clip(
                            8,
                            format!("{} step {}", NOTE_ROWS[row].1, step + 1),
                            samples,
                            1,
                            Beats::new(start),
                            Beats::new(end),
                        );
                        generated_clips.push(id);
                    }
                }
            }
            for (row, name) in DRUM_ROWS.iter().enumerate() {
                for step in 0..STEPS {
                    if drums[row][step] {
                        let start = step as f64 / 4.0;
                        let end = start + 0.24;
                        let samples = synth_drum(row, tempo.bpm(), end - start);
                        let id = view.audio.add_audio_clip(
                            9,
                            format!("{name} step {}", step + 1),
                            samples,
                            1,
                            Beats::new(start),
                            Beats::new(end),
                        );
                        generated_clips.push(id);
                    }
                }
            }
        });
        self.generated_clips = generated_clips;
        cx.notify();
    }

    fn audition(&mut self, note: u8, cx: &mut Context<Self>) {
        let mut generated_clip = None;
        self.playlist.update(cx, |view, _| {
            let tempo = view.audio.playlist().tempo;
            let start = view.audio.playhead().beats(tempo).f64() + 0.1;
            let end = start + 0.5;
            let samples = synth_note(note, tempo.bpm(), end - start);
            let id = view.audio.add_audio_clip(
                8,
                format!("Piano note {note}"),
                samples,
                1,
                Beats::new(start),
                Beats::new(end),
            );
            generated_clip = Some(id);
            if !view.audio.playing() {
                view.audio.play();
            }
        });
        if let Some(id) = generated_clip {
            self.generated_clips.push(id);
        }
        cx.notify();
    }
}

impl Render for PianoMachine {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Arc::clone(&self.playlist.read(cx).theme);
        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .h(px(260.))
            .gap_1()
            .px_2()
            .py_1()
            .bg(theme.central_background)
            .child(div().text_sm().child("Piano roll · click a key to audition; click a cell to sequence a note"))
            .children(NOTE_ROWS.iter().enumerate().map(|(row, (note, label))| {
                div()
                    .flex()
                    .h(px(18.))
                    .gap(px(2.))
                    .child(
                        div()
                            .w(px(44.))
                            .flex_shrink_0()
                            .text_xs()
                            .child(*label)
                            .id(format!("virtual-key-{note}"))
                            .cursor_pointer()
                            .on_click(cx.listener(move |view, _, _, cx| view.audition(*note, cx))),
                    )
                    .children((0..STEPS).map(|step| {
                        let active = self.notes[row][step];
                        div()
                            .flex_1()
                            .rounded_sm()
                            .bg(if active { rgb(0x68a9d4) } else { rgb(0x252b33) })
                            .cursor_pointer()
                            .id(format!("piano-step-{row}-{step}"))
                            .on_click(cx.listener(move |view, _, _, cx| {
                                view.notes[row][step] = !view.notes[row][step];
                                view.rebuild(cx);
                            }))
                    }))
            }))
            .child(div().mt_1().text_sm().child("Beat machine · 16-step pattern"))
            .children(DRUM_ROWS.iter().enumerate().map(|(row, label)| {
                div()
                    .flex()
                    .h(px(22.))
                    .gap(px(2.))
                    .child(div().w(px(68.)).flex_shrink_0().text_xs().child(*label))
                    .children((0..STEPS).map(|step| {
                        let active = self.drums[row][step];
                        div()
                            .flex_1()
                            .rounded_sm()
                            .bg(if active { rgb(0xd18a55) } else { rgb(0x252b33) })
                            .cursor_pointer()
                            .id(format!("drum-step-{row}-{step}"))
                            .on_click(cx.listener(move |view, _, _, cx| {
                                view.drums[row][step] = !view.drums[row][step];
                                view.rebuild(cx);
                            }))
                    }))
            }))
    }
}

fn synth_note(midi_note: u8, bpm: f64, duration_beats: f64) -> Arc<[f32]> {
    let frames = (SAMPLE_RATE * 60.0 / bpm * duration_beats).ceil() as usize;
    let frequency = 440.0 * 2_f64.powf((f64::from(midi_note) - 69.0) / 12.0);
    let mut samples = Vec::with_capacity(frames);
    for frame in 0..frames {
        let time = frame as f64 / SAMPLE_RATE;
        let envelope = (-5.0 * time).exp() * (1.0 - (-90.0 * time).exp());
        let fundamental = (std::f64::consts::TAU * frequency * time).sin();
        let second = (std::f64::consts::TAU * frequency * 2.01 * time).sin() * 0.22;
        let third = (std::f64::consts::TAU * frequency * 3.98 * time).sin() * 0.08;
        samples.push(((fundamental + second + third) * envelope * 0.35) as f32);
    }
    Arc::from(samples)
}

fn synth_drum(row: usize, bpm: f64, duration_beats: f64) -> Arc<[f32]> {
    let frames = (SAMPLE_RATE * 60.0 / bpm * duration_beats).ceil() as usize;
    let mut samples = Vec::with_capacity(frames);
    let mut noise = 0x1234_5678_u32;
    for frame in 0..frames {
        let time = frame as f64 / SAMPLE_RATE;
        let t = time / (duration_beats * 60.0 / bpm);
        let envelope = (-if row == 0 { 9.0 } else { 15.0 } * t).exp();
        noise ^= noise << 13;
        noise ^= noise >> 17;
        noise ^= noise << 5;
        let hiss = (noise as i32 as f64) / f64::from(i32::MAX);
        let sample = match row {
            0 => (std::f64::consts::TAU * (48.0 + 100.0 * (-24.0 * t).exp()) * time).sin() * envelope,
            1 => (hiss * 0.7 + (std::f64::consts::TAU * 180.0 * time).sin() * 0.3) * envelope,
            2 => hiss * envelope * 0.45,
            _ => hiss * envelope * 0.65,
        };
        samples.push(sample as f32);
    }
    Arc::from(samples)
}
