# Volt DAW
![Volt DAW Banner](https://fs.infra.thered.sh/branding/volt/volt-banner.png)

Volt is a custom DAW (digital audio workstation) that is primarily aimed at Linux (with Windows and macOS support too) and is fully open-source. Built with gpui-ce and cpal.

Feel free to make improvements and additions to the DAW, and submit a PR!
Additionally, you can also open issues with the DAW.

## Our community
You can join our official Volt Community Discord server at https://discord.gg/QtYXsnu7jQ!
Feel free to discuss anything related to Volt there, as well as music production.

## Current state

The DAW at the moment is highly unfinished and is currently primarily in the testing phase.
If anyone can volunteer to improve the DAW, please do! PRs are super welcome.

Here's a screenshot:
![Volt DAW current state](./images/volt-preview-7.png)

## Platform and feature status

Volt currently targets desktop systems. Its GPUI windowing and desktop audio-device setup do not provide an Android app, so Android phone support needs a mobile-capable UI and audio backend (or a separate companion app); it is not achieved by resizing the desktop window. The file browser now uses the Windows system drive as its starting location, but a Windows build and audio-device test are still needed before calling Windows 11 support verified. Windows VST3 plugins will also need a native plugin host and compatible 64-bit plugin binaries.

What is present today:
- Browse files and import supported audio, including WAV, MP3, FLAC, Ogg, and Opus.
- Preview audio and place audio clips on a playlist with waveform rendering, playback, tempo/time-signature controls, and basic clip/selection editing.
- Use the built-in 16-step piano-note grid and drum sequencer, with a virtual octave keyboard and synthesized placeholder sounds.
- A WIP mixing/audio engine and theme support.

What is not present yet: a finished project format, MIDI device/file support, Salamander piano samples, plugin/VST hosting, and an AI assistant. The piano and drum sounds are synthesized placeholders, not sampled instruments. Volt remains a desktop application; Android is not currently supported. Mixer, effects, and several playlist workflows are also unfinished; see the roadmap below.

### VST and Windows requirements

Using VST instruments and effects requires more than recognizing `.vst3` files. Volt needs a VST3 host that can scan and load plugins, process audio/MIDI in real time, expose plugin controls and GUIs, route tracks, and save/restore plugin state with projects. A reliable host also needs plugin validation and crash isolation. Windows builds need compatible 64-bit Windows plugins and dependable low-latency audio-device support; ASIO is a common option, while the current audio layer is based on CPAL. VST3 SDK terms and redistribution requirements must be followed. None of this plugin hosting is implemented yet.

### Requested features

- Beat machine and step sequencer: drum/sample lanes, editable steps, tempo-synced playback, velocity, swing, pattern chaining, and project save/load. MIDI export and piano-roll integration can follow.
- Piano workflow: a capable piano roll, an on-screen multi-octave keyboard for mouse/touch input, and live MIDI input from external keyboards. Planned editor tools include draw/select/move/resize notes, snap and quantize, velocity/expression lanes, scale highlighting, and undo/redo.
- Yamaha keyboard support: assuming “PM45” means the Yamaha P-45, support it as a USB MIDI input with device selection, hot-plug handling, low-latency monitoring, and sustain-pedal input. USB MIDI sends note/control data; to record the keyboard's own audio, connect its audio output to an audio interface/input.
- Piano sounds: provide a built-in sampler instrument with velocity layers and sustain-pedal behavior, and investigate an optional Salamander Grand Piano sample-library download or bundle after verifying its license and attribution requirements. Sample playback does not require VST hosting; users' VST instruments do.
- AI assistant: an optional, user-invoked helper for DAW guidance and tasks such as generating/editing patterns, BPM detection, or audio-to-MIDI suggestions. Audio or project data should not be sent to an online service without clear consent; local models and cloud services should be selectable rather than assumed.
- Android phone workflow: first confirm a mobile UI/audio framework that supports Android, then build touch-sized transport and sequencer views, Android audio I/O, file-picker/storage access, and installable packaging. A remote-control companion is a smaller alternative, but would not make the full DAW run on the phone.

## Our Roadmap:

We have a list of things we want to achieve, and here it is for easy editing and viewing:
- [ ] Project file format
    - [ ] Define a spec
    - [ ] Implementation
        - [ ] Store arrangements (and the playlist)
        - [ ] Save project BPM and time signature; persist tempo-map changes when tempo automation is added
        - [ ] Store MIDI
        - [ ] DAW file autosearching if audio file not found
        - [ ] Store references to audio (DO NOT STORE THE ENTIRE AUDIO)
        - [ ] Store mixer information, applied effects
- [ ] Browser functionality
    - [x] Being able to browse files on the system
    - [x] Being able to import audio files
    - [x] Playback of audio
    - [ ] Being able to import devices (that is, plugins, effects, VSTs or similar)
- [ ] Playlist functionality
    - [x] Scrolling, scaling and similar UI functionality
    - [x] Time signatures, BPM and related
    - [x] Playback of audio
    - [x] Fast waveform rendering
    - [ ] Time stretching
    - [ ] Clip configuration and editing
    - [ ] Track configuration
        - [ ] Routing
        - [ ] Effects
    - [ ] Shortcuts
    - [ ] Arrangements
    - [ ] Importing other arrangements into an existing arrangement
- [ ] Piano Roll and MIDI workflow
    - [x] Basic one-octave virtual keyboard, 16-step piano-note grid, and drum pattern grid with synthesized placeholder sounds
    - [ ] Piano roll note draw, select, move, resize, snap/grid, and quantize
    - [ ] Velocity and expression lanes, scale highlighting, undo/redo, and keyboard shortcuts
    - [ ] Multi-octave virtual piano keyboard with polyphonic mouse/touch input and sustain
    - [ ] MIDI device selection, hot-plug, live input/monitoring, and sustain-pedal support
    - [ ] Project save/load for MIDI notes, instruments, and controller data
    - [ ] Test USB MIDI input with Yamaha P-45; support audio capture through an audio interface separately
    - [ ] Built-in sampler instrument with velocity layers and sustain behavior
    - [ ] Evaluate optional Salamander Grand Piano sample availability and comply with its license/attribution
- [ ] UI & UX
    - [x] Theming system
        - This is WIP
    - [ ] Settings UI
    - [ ] About UI
    - [ ] Primary workflow elements (such as editing modes/tools, like a pencil/brush for drawing in clips, or a selection tool for selecting elements)
    - [ ] Command Palette for easy search and execution of tasks
    - [ ] System for dialog boxes (with an option to make them movable)
    - [ ] Intuitive audio track and midi track design
    - [ ] Shortcuts configuration
    - [ ] Welcome screen
    - [ ] Tutorials/lessons system for newcomers
- [ ] Audio Processing
    - [ ] Plugin Support
        - [ ] Built-in plugins
        - [ ] LV2 support
        - [ ] VST3 support
        - [ ] CLAP support
        - [ ] VST2 support
    - [x] Audio playback
    - [ ] Audio formats (support can be provided either through custom implementations or libraries)
        - [x] .wav support
        - [ ] .ogg support
        - [x] .mp3 support
        - [ ] .m4a support
        - [ ] .opus support
        - [x] .flac support
        - [ ] .alac support
        - [ ] .aac support
        - [ ] .wma support
        - [ ] .aiff support
    - [ ] Effects pipeline
        - **Note:** this is work-in-progress!
    - [ ] Fast live audio processing (this means being able to take in an unpredictable input and processing it, like microphone input)
    - [ ] Support for ALSA, JACK, PulseAudio and PipeWire if possible
    - [x] Mixing pipeline
        - This is WIP
    - [ ] ASIO support (this is an issue primarily on Windows)
    - [ ] Audio stretching and squeezing with little to no noticable artifacts, while preserving or modifying formants
    - [ ] Potentially related to machine learning:
        - [ ] Converting audio to MIDI using Machine Learning tools
        - [ ] Audio declipping using Machine Learning tools
        - [ ] Audio BPM detection
- [ ] Beat machine and step sequencer
    - [ ] Drum/sample lanes and step editing
    - [ ] Velocity, swing, and pattern chaining
    - [ ] Save patterns in project files and sync playback to the playlist
- [ ] AI assistance
    - [ ] DAW help and opt-in pattern generation/editing
    - [ ] BPM detection and audio-to-MIDI assistance
    - [ ] Document privacy boundaries and provide local/cloud choices
- [ ] Android phone support
    - [ ] Validate a mobile-capable UI and audio stack (current GPUI desktop app is not an Android app)
    - [ ] Touch-first controls, Android audio I/O, file access, and packaging

The roadmap is still very work-in-progress, we're structuring it as we go, not everything may be mentioned.

There are a number of important design decisions and routes we want to take for our DAW. Here are our core principles:
- Uncluttered design. This means that we should stick to one primary way of achieving a task, rather than having many ways to do the same thing. This point is where a lot of open-source DAWs (and other software) fail.
- Clear language. This means that our design language should follow industry standard and use easily understandable icons or text to imply actions.
- Don't be an "everything" software. We don't want to have every feature under the sun. We want to have a reasonable amount of features that we think DAWs should have, and keep improving on them.
- Quality over quantity. We want to make our existing features as close to good enough or perfect as possible, not have a ridiculous amount of features most of which you'll never use. This means we prioritize improving core features over tiny features which barely affect workflow.
- Stay out of the way. We don't want to be like what happens to VSCode when a few extensions lead you to notification hell and clutter the whole notifcations menu with messages you do not care about. The design should be clean and the software should stay out of your way.
- Speed. We want for our DAW to function efficiently and we aim to prioritize performance over overly flashy looks.
- Reliability. The DAW shouldn't crash and shouldn't freeze whatsoever (there's a reason we chose Rust over languages like C++).
- Never freeze. It's better to show the user the DAW is actually doing something (i.e. show a loading icon) rather than freezing the whole DAW. This means putting things off to other threads, and reserving the main thread for UI. We are in the multi-threaded era, and we can make use of this. This way, the DAW can not only *be* fast, but *feel* fast.
- Power-efficiency. The DAW should consume minimal power while idle, as being able to make music on the go is important for many music producers.
- Compatibility. Volt aims to function on Windows, macOS and Linux without major restrictions or downsides.
- Support. Volt aims to maximize support for things like MIDI controllers, audio hardware or similar. This is not as high of a priority as all the other points, however, it is still important to us, as is to a lot of music producers.
