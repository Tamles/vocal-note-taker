---
stepsCompleted: ['step-01-validate-prerequisites', 'step-02-design-epics', 'step-03-create-stories', 'step-04-final-validation']
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/planning-artifacts/architecture.md'
requirementsExtracted: true
totalFunctionalRequirements: 48
totalNonFunctionalRequirements: 25
extractionComplete: true
epicsDesigned: true
totalEpics: 6
epicsApproved: true
storiesCreated: true
totalStories: 25
validationComplete: true
workflowComplete: true
readyForDevelopment: true
---

# vocal-note-taker - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for vocal-note-taker, decomposing the requirements from the PRD, UX Design if it exists, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

**Audio Recording (FR1-FR8):**

- **FR1:** User can initiate audio recording via button click
- **FR2:** User can initiate audio recording via global keyboard shortcut
- **FR3:** User can stop audio recording via button click or keyboard shortcut release
- **FR4:** System can capture audio from system microphone input
- **FR5:** System can display recording timer showing elapsed time in seconds
- **FR6:** System can display visual recording indicator (REC icon) during active recording
- **FR7:** System can display real-time audio waveform visualization during recording
- **FR8:** System can save recorded audio as temporary WAV file (16kHz mono)

**Transcription Processing (FR9-FR14):**

- **FR9:** System can transcribe recorded audio using local whisper.cpp model (large)
- **FR10:** System can process transcription entirely offline without network dependency
- **FR11:** System can display transcription progress indicator to user
- **FR12:** System can complete transcription and display results
- **FR13:** System can handle transcription errors gracefully with clear error messages
- **FR14:** System can clean up temporary audio files after successful transcription

**Text Display & Management (FR15-FR19):**

- **FR15:** User can view complete transcribed text in readable format
- **FR16:** System can display transcribed text without truncation or scrolling when text fits viewport
- **FR17:** User can visually scan transcribed text for accuracy verification
- **FR18:** System can automatically clear previous transcription when starting new recording
- **FR19:** System can maintain simple linear workflow (no history management)

**Clipboard Integration (FR20-FR25):**

- **FR20:** User can copy transcribed text to system clipboard via button click
- **FR21:** User can copy transcribed text to system clipboard via Enter keyboard shortcut
- **FR22:** System can automatically focus copy button after transcription completes
- **FR23:** System can display visual confirmation feedback when text is copied ("✓ Copié!")
- **FR24:** System can copy plain text format (no rich formatting)
- **FR25:** User controls when clipboard copy occurs (manual trigger, not automatic)

**System Integration (FR26-FR32):**

- **FR26:** System can register and respond to global keyboard shortcuts while in background
- **FR27:** System can continue running in background after window closure (ghost mode)
- **FR28:** System can minimize to background without terminating processes
- **FR29:** System can display system notification when transcription is complete
- **FR30:** User can click notification to bring application to foreground
- **FR31:** System can integrate with Ubuntu notification system (libnotify)
- **FR32:** System can maintain process persistence in memory (Rust backend + Tauri)

**Configuration Management (FR33-FR37):**

- **FR33:** User can configure global keyboard shortcuts via configuration file
- **FR34:** System can load configuration from local file (TOML format)
- **FR35:** System can store configuration in user config directory (~/.config/vocal-note-taker/)
- **FR36:** System can apply configuration changes on application restart
- **FR37:** System can use default configuration if custom config not found

**Application Lifecycle (FR38-FR43):**

- **FR38:** User can install application via .deb package on Ubuntu
- **FR39:** System can function entirely offline without internet connection
- **FR40:** System can start application from Ubuntu applications menu or command line
- **FR41:** System can display application version in UI
- **FR42:** User can quit application completely via menu or shortcut
- **FR43:** System can maintain RAM consumption under 100MB when idle

**Error Handling & Recovery (FR44-FR48):**

- **FR44:** System can detect and report microphone access errors
- **FR45:** System can detect and report whisper.cpp processing failures
- **FR46:** System can recover gracefully from recording interruptions
- **FR47:** System can provide clear actionable error messages to user
- **FR48:** System can continue operating after non-critical errors

### NonFunctional Requirements

**Performance (NFR-PERF-1 to NFR-PERF-5):**

- **NFR-PERF-1:** Workflow Total Response Time - Complete workflow (raccourci clavier → transcription → copie) must complete in less than 15 seconds for 60 seconds of audio input
- **NFR-PERF-2:** Transcription Latency - Audio transcription must complete within 30 seconds for 60 seconds of recorded audio (quality prioritized over speed)
- **NFR-PERF-3:** UI Responsiveness - User interface must respond to interactions within 100ms. No perceptible freeze or lag during recording or transcription
- **NFR-PERF-4:** Memory Consumption - Application idle memory consumption target: <200MB RAM
- **NFR-PERF-5:** Application Startup - Startup time not critical (launched once and remains in background). Fast recovery from background to foreground (<500ms)

**Usability (NFR-USA-1 to NFR-USA-5):**

- **NFR-USA-1:** Cognitive Load Minimization - Interface must be instantaneously readable without mental effort. Visual hierarchy clear with no ambiguity on available actions
- **NFR-USA-2:** Quick Quality Verification - User can visually scan transcribed text for accuracy in 2-3 seconds
- **NFR-USA-3:** Friction-Free Workflow - Maximum 3 user actions required for complete workflow (shortcut → speak → copy). No unnecessary confirmation dialogs
- **NFR-USA-4:** Keyboard-First Interaction - All critical actions accessible via keyboard shortcuts. No mouse required for primary workflow
- **NFR-USA-5:** Feedback Clarity - Continuous visual feedback during recording (waveform, timer, REC indicator). Immediate confirmation feedback for actions

**Reliability (NFR-REL-1 to NFR-REL-5):**

- **NFR-REL-1:** Crash Tolerance - Application crash rate must be less than 1 occurrence per week of daily usage. Crashes acceptable but not frequent
- **NFR-REL-2:** Uptime & Restart Requirements - Application should support multiple days of continuous operation without restart
- **NFR-REL-3:** Data Loss Tolerance - Loss of in-progress audio recording acceptable if application crashes. No persistent data beyond current session
- **NFR-REL-4:** Error Recovery - Application must recover gracefully from non-critical errors with clear, actionable error messages
- **NFR-REL-5:** System Stability - Zero conflicts with other applications. No interference with system clipboard or shared resources

**Security & Privacy (NFR-SEC-1 to NFR-SEC-5):**

- **NFR-SEC-1:** Network Isolation - Zero network calls during normal operation. All transcription processing occurs locally on-device
- **NFR-SEC-2:** Data Privacy - Voice data never leaves local machine. Audio recordings never uploaded or transmitted
- **NFR-SEC-3:** Temporary Data Cleanup - Temporary audio files (WAV) deleted immediately after successful transcription
- **NFR-SEC-4:** Minimal Permissions - Only microphone access permission required. No root/sudo privileges needed
- **NFR-SEC-5:** Configuration Security - Configuration files stored in user-specific directories with plain text format for transparency

**Maintainability (NFR-MAINT-1 to NFR-MAINT-5):**

- **NFR-MAINT-1:** Code Clarity - Code must be sufficiently clear for future modifications with meaningful naming and logical structure
- **NFR-MAINT-2:** Modular Architecture - Clear separation between Tauri frontend, Rust backend, and whisper-rs integration
- **NFR-MAINT-3:** Documentation Minimum - Basic README with setup and usage instructions, architecture overview, troubleshooting guide
- **NFR-MAINT-4:** Maintenance Time Budget - Post-MVP maintenance effort: maximum 2-4 hours per month
- **NFR-MAINT-5:** Dependency Management - Minimal external dependencies, stable well-maintained libraries, Cargo.toml lockfile for reproducibility

### Additional Requirements

**Starter Template & Technology Stack:**

- **Initialization:** Project must be initialized using `create-tauri-app --template svelte-ts` command
- **Framework:** Tauri 2.x for desktop application framework with native webview
- **Frontend:** Svelte 4.x/5.x + TypeScript with Vite 5.x build tooling
- **Backend:** Rust (stable) with unified backend architecture
- **Async Runtime:** Tokio 1.x for all async operations and long-running tasks
- **Audio Capture:** cpal 0.15 for cross-platform audio capture (ALSA/PulseAudio on Ubuntu, CoreAudio on macOS)
- **Audio Processing:** hound 3.x for WAV file writing (16kHz mono format)
- **Transcription Engine:** whisper-rs bindings for local whisper.cpp model (large, ~3GB)
- **Tauri Plugins:** global-shortcut 2.x, clipboard 2.x, notification 2.x
- **Configuration:** TOML format with serde for config parsing/validation
- **Error Handling:** thiserror crate for AppError enum implementation

**Backend Architecture Requirements:**

- **Module Organization:** Separate modules for audio/ (capture.rs, buffer.rs), transcription/ (whisper.rs), config/ (loader.rs), system/ (hotkeys.rs, clipboard.rs)
- **Commands Layer:** commands.rs for thin IPC orchestration layer that delegates to modules
- **Error System:** Centralized error.rs with AppError enum covering all error cases
- **Double Buffer Strategy:** WAV file writing + waveform samples via tokio::sync::mpsc channel for real-time visualization
- **Async Processing:** All long-running tasks (transcription, audio streaming) in tokio::spawn tasks
- **IPC Commands:** start_recording, stop_recording, load_config, copy_to_clipboard (all returning Result<T, AppError>)
- **IPC Events:** waveform-data (30-60 FPS), transcription-progress (0-100%), transcription-complete, error, recording-started, recording-stopped

**Frontend Architecture Requirements:**

- **Svelte Stores:** Separate stores by domain: recordingState ('idle'|'recording'|'transcribing'), audioData (Float32Array), transcriptionText, transcriptionProgress, configStore, errorStore
- **Derived Stores:** isRecording, canTranscribe, isTranscribing for UI conditional logic
- **Components:** RecordButton.svelte, WaveformDisplay.svelte, Timer.svelte, TranscriptionDisplay.svelte, ProgressBar.svelte, ErrorNotification.svelte
- **Waveform Visualization:** Canvas API native (no library), 30-60 FPS rendering with requestAnimationFrame
- **Component Structure:** Flat structure in /src/components/, stores in /src/stores/, utilities in /src/lib/
- **Event Listeners:** Centralized in main.ts or App.svelte onMount, update stores on backend events

**Configuration Management Requirements:**

- **Config File Location:** ~/.config/vocal-note-taker/config.toml (Linux), ~/Library/Application Support/vocal-note-taker/config.toml (macOS)
- **Config Schema Sections:** [hotkeys] (start_recording, stop_recording, toggle_window), [preferences] (auto_copy_clipboard, show_waveform, whisper_model), [audio] (sample_rate=16000, channels=1)
- **Loading Strategy:** Load at startup via config/loader.rs, validate with serde, use defaults if missing/invalid
- **Hot Reload:** Deferred post-MVP (requires restart for config changes in MVP)

**File System Requirements:**

- **Temporary Audio Storage:** ~/.local/share/vocal-note-taker/temp/recording.wav (Linux), ~/Library/Application Support/vocal-note-taker/temp/recording.wav (macOS)
- **Whisper Models:** ~/.local/share/vocal-note-taker/models/ggml-large.bin (~3GB model file)
- **Model Acquisition:** Download script (scripts/download-models.sh) OR bundled in .deb package
- **Cleanup Policy:** Delete temp WAV immediately after transcription complete (NFR-SEC-3 compliance)
- **Permissions:** User-only read/write for all application data directories

**Implementation Patterns & Standards:**

- **Naming Conventions:** snake_case (Rust functions/modules), camelCase (TypeScript), PascalCase (Rust structs/enums, Svelte components), kebab-case (IPC events), SCREAMING_SNAKE_CASE (constants)
- **Error Propagation:** Result<T, AppError> everywhere in Rust with ? operator, never panic!() or .unwrap() in production
- **IPC Communication:** Commands for user-initiated actions (invoke from frontend), Events for async updates (emit from backend)
- **State Management:** Backend as source of truth, frontend stores reactive to backend events
- **Async Patterns:** tokio::spawn for long tasks, channels for streaming data, async/await throughout
- **Testing Strategy:** ≥70% coverage for critical backend paths (audio, config, transcription, error handling), unit tests co-located with modules

**Build & Deployment Requirements:**

- **Development Command:** `pnpm tauri dev` for dev server with HMR and Rust auto-recompile
- **Build Command:** `pnpm tauri build` for production optimized release
- **Build Outputs:** .deb package (Linux), .dmg/.app (macOS Phase 2), standalone binary
- **Build Optimizations:** Cargo profile with opt-level="z", lto=true, strip=true for size optimization
- **System Dependencies:** libasound2-dev, libwebkit2gtk-4.0-dev (Ubuntu), system audio libraries
- **Installation Structure:** Binary in /usr/bin/, desktop entry in /usr/share/applications/, config template in /usr/share/vocal-note-taker/

**Desktop Integration Requirements:**

- **Desktop Entry:** .desktop file for Ubuntu applications menu with Name, Icon, Exec paths
- **System Tray:** Tauri tray API for ghost mode with Show/Hide and Quit menu items
- **Notifications:** Native desktop notifications via tauri-plugin-notification (libnotify on Linux)
- **Global Hotkeys:** Cross-platform via tauri-plugin-global-shortcut (X11/Wayland on Linux, Cocoa on macOS)
- **Clipboard:** Native clipboard integration via tauri-plugin-clipboard
- **Background Persistence:** Process continues after window close, responds to global hotkeys

### FR Coverage Map

**Epic 1: Project Foundation & Core Infrastructure**
- FR38: User can install application via .deb package on Ubuntu
- FR39: System can function entirely offline without internet connection
- FR40: System can start application from Ubuntu applications menu or command line
- FR41: System can display application version in UI
- FR42: User can quit application completely via menu or shortcut
- FR43: System can maintain RAM consumption under 100MB when idle
- FR44: System can detect and report microphone access errors
- FR45: System can detect and report whisper.cpp processing failures
- FR46: System can recover gracefully from recording interruptions
- FR47: System can provide clear actionable error messages to user
- FR48: System can continue operating after non-critical errors

**Epic 2: Audio Capture & Feedback**
- FR1: User can initiate audio recording via button click
- FR2: User can initiate audio recording via global keyboard shortcut
- FR3: User can stop audio recording via button click or keyboard shortcut release
- FR4: System can capture audio from system microphone input
- FR5: System can display recording timer showing elapsed time in seconds
- FR6: System can display visual recording indicator (REC icon) during active recording
- FR7: System can display real-time audio waveform visualization during recording
- FR8: System can save recorded audio as temporary WAV file (16kHz mono)

**Epic 3: Local Transcription**
- FR9: System can transcribe recorded audio using local whisper.cpp model (large)
- FR10: System can process transcription entirely offline without network dependency
- FR11: System can display transcription progress indicator to user
- FR12: System can complete transcription and display results
- FR13: System can handle transcription errors gracefully with clear error messages
- FR14: System can clean up temporary audio files after successful transcription

**Epic 4: Text Display & Copy Integration**
- FR15: User can view complete transcribed text in readable format
- FR16: System can display transcribed text without truncation or scrolling when text fits viewport
- FR17: User can visually scan transcribed text for accuracy verification
- FR18: System can automatically clear previous transcription when starting new recording
- FR19: System can maintain simple linear workflow (no history management)
- FR20: User can copy transcribed text to system clipboard via button click
- FR21: User can copy transcribed text to system clipboard via Enter keyboard shortcut
- FR22: System can automatically focus copy button after transcription completes
- FR23: System can display visual confirmation feedback when text is copied ("✓ Copié!")
- FR24: System can copy plain text format (no rich formatting)
- FR25: User controls when clipboard copy occurs (manual trigger, not automatic)

**Epic 5: Ghost Mode & Quick Access**
- FR26: System can register and respond to global keyboard shortcuts while in background
- FR27: System can continue running in background after window closure (ghost mode)
- FR28: System can minimize to background without terminating processes
- FR29: System can display system notification when transcription is complete
- FR30: User can click notification to bring application to foreground
- FR31: System can integrate with Ubuntu notification system (libnotify)
- FR32: System can maintain process persistence in memory (Rust backend + Tauri)

**Epic 6: Configuration & Personalization**
- FR33: User can configure global keyboard shortcuts via configuration file
- FR34: System can load configuration from local file (TOML format)
- FR35: System can store configuration in user config directory (~/.config/vocal-note-taker/)
- FR36: System can apply configuration changes on application restart
- FR37: System can use default configuration if custom config not found

**Total FR Coverage: 48/48 (100%)**

## Epic List

### Epic 1: Project Foundation & Core Infrastructure

**Epic Goal:** Establish a solid technical foundation with proper error handling, module structure, and deployment pipeline so that the user can install and run the application with a robust and consistent architecture.

**User Outcome:** User can install vocal-note-taker via .deb package, launch it from the applications menu, and the application runs reliably with graceful error handling for all future features.

**FRs Covered:** FR38-FR48 (11 FRs)
- Installation & lifecycle (FR38, FR40, FR41, FR42, FR43)
- Offline capability (FR39)
- Complete error handling system (FR44-FR48)

**Implementation Notes:**
- Initialize project with `create-tauri-app --template svelte-ts`
- Establish error.rs with AppError enum using thiserror
- Create backend module structure (audio/, transcription/, config/, system/)
- Setup Result<T, AppError> pattern throughout codebase
- Build .deb package with Tauri CLI
- Implement error recovery patterns and tests (≥70% coverage)

---

### Epic 2: Audio Capture & Feedback

**Epic Goal:** Enable users to record their voice with continuous visual feedback (waveform, timer, REC indicator) providing confidence that audio is being captured correctly.

**User Outcome:** User can click a button or use a shortcut to start recording, see real-time waveform and timer, and stop recording with visual confirmation.

**FRs Covered:** FR1-FR8 (8 FRs)
- Recording initiation & control (FR1-FR3)
- Audio capture & WAV saving (FR4, FR8)
- Visual feedback (FR5-FR7)

**Implementation Notes:**
- Implement audio/capture.rs with cpal integration (ALSA/PulseAudio)
- Create double buffer strategy (WAV file + waveform samples via tokio channel)
- Build RecordButton.svelte, WaveformDisplay.svelte (Canvas API), Timer.svelte
- Setup Svelte stores: recordingState, audioData, recordingDuration
- Implement IPC: start_recording, stop_recording commands
- Emit waveform-data events at 30-60 FPS

---

### Epic 3: Local Transcription

**Epic Goal:** Transform recorded audio into accurate text (≥90% quality) using local whisper.cpp processing, ensuring complete privacy and offline capability.

**User Outcome:** User obtains high-quality transcription of their voice recording without any data leaving their machine, with progress feedback during processing.

**FRs Covered:** FR9-FR14 (6 FRs)
- Whisper.cpp integration (FR9, FR10)
- Progress tracking (FR11, FR12)
- Error handling (FR13)
- Cleanup (FR14)

**Implementation Notes:**
- Integrate whisper-rs bindings in transcription/whisper.rs
- Download/bundle ggml-large.bin model (~3GB)
- Implement async transcription with tokio::spawn
- Emit transcription-progress events (0-100%)
- Create ProgressBar.svelte component
- Implement temp file cleanup after transcription
- Handle transcription errors via AppError::TranscriptionFailed

---

### Epic 4: Text Display & Copy Integration

**Epic Goal:** Allow users to verify transcription accuracy and copy the text to their system clipboard for use in other applications (ChatGPT, Teams, etc.).

**User Outcome:** User can quickly scan the transcribed text, press Enter to copy it, and immediately paste it into their target application with a <1 second workflow.

**FRs Covered:** FR15-FR25 (11 FRs)
- Text display (FR15-FR17)
- Linear workflow (FR18, FR19)
- Clipboard operations (FR20-FR25)

**Implementation Notes:**
- Create TranscriptionDisplay.svelte component
- Implement auto-focus on copy button (FR22)
- Setup Enter keyboard shortcut for copy
- Integrate tauri-plugin-clipboard
- Display "✓ Copié!" confirmation feedback
- Implement auto-clear on new recording
- Ensure manual control (no auto-copy unless configured)

---

### Epic 5: Ghost Mode & Quick Access

**Epic Goal:** Make the application "invisible" until needed, allowing users to access it instantly via global shortcuts without interrupting their workflow.

**User Outcome:** User can invoke vocal-note-taker from anywhere using Ctrl+Alt+R (or custom shortcut), record while the app stays in background, and receive a notification when transcription is ready.

**FRs Covered:** FR26-FR32 (7 FRs)
- Global keyboard shortcuts (FR26)
- Ghost mode & background (FR27, FR28, FR32)
- System notifications (FR29-FR31)

**Implementation Notes:**
- Integrate tauri-plugin-global-shortcut 2.x
- Implement system tray icon with Show/Hide/Quit menu
- Setup ghost mode (app persists after window close)
- Integrate tauri-plugin-notification (libnotify on Ubuntu)
- Handle Wayland limitations with fallback messages
- Implement notification click → bring app to foreground

---

### Epic 6: Configuration & Personalization

**Epic Goal:** Enable users to customize the application behavior (keyboard shortcuts, preferences) via a simple configuration file.

**User Outcome:** User can edit ~/.config/vocal-note-taker/config.toml to personalize hotkeys, enable/disable features, and adjust audio settings to match their workflow.

**FRs Covered:** FR33-FR37 (5 FRs)
- Configuration file management (FR33-FR37)

**Implementation Notes:**
- Implement config/loader.rs with TOML parsing (serde)
- Define config schema: [hotkeys], [preferences], [audio] sections
- Load config at startup with validation
- Provide defaults if config missing/invalid
- Create config.example.toml template
- Document all config options in config-schema.md
- Note: Hot reload deferred post-MVP (restart required)

## Epic 1: Project Foundation & Core Infrastructure

**Epic Goal:** Établir une base technique solide avec une gestion d'erreurs appropriée, une structure de modules, et un pipeline de déploiement pour que l'utilisateur puisse installer et exécuter l'application avec une architecture robuste.

**User Outcome:** L'utilisateur peut installer vocal-note-taker via un package .deb, le lancer depuis le menu applications, et l'application fonctionne de manière fiable avec une gestion d'erreurs gracieuse pour toutes les futures fonctionnalités.

**FRs Covered:** FR38-FR48 (11 FRs)

---

### Story 1.1: Initialisation du projet Tauri avec structure modulaire

As a développeur,
I want un projet Tauri initialisé avec la structure de modules définie,
So that je dispose d'une base solide pour implémenter toutes les fonctionnalités.

**Acceptance Criteria:**

**Given** le projet n'existe pas encore
**When** j'exécute `create-tauri-app --template svelte-ts`
**Then** le projet est créé avec Svelte + TypeScript + Vite

**Given** le projet est créé
**When** j'examine la structure backend Rust
**Then** les modules audio/, transcription/, config/, system/ existent
**And** commands.rs existe comme couche d'orchestration IPC
**And** error.rs existe avec un AppError enum vide (thiserror)

**Given** le projet est configuré
**When** j'exécute `pnpm tauri dev`
**Then** l'application démarre sans erreur
**And** aucune connexion réseau n'est établie (FR39)

---

### Story 1.2: Système centralisé de gestion d'erreurs

As a utilisateur,
I want que l'application gère les erreurs de manière cohérente et claire,
So that je comprenne toujours ce qui s'est passé et comment réagir.

**Acceptance Criteria:**

**Given** le module error.rs existe
**When** j'implémente AppError avec thiserror
**Then** les variantes suivantes sont définies:
- MicrophoneAccessDenied (FR44)
- MicrophoneNotFound (FR44)
- TranscriptionFailed(String) (FR45)
- RecordingInterrupted (FR46)
- ConfigurationError(String)
- ClipboardError
**And** chaque variante a un message d'erreur clair et actionnable (FR47)

**Given** une erreur non-critique se produit
**When** l'erreur est propagée au frontend
**Then** l'application affiche le message d'erreur
**And** l'application reste fonctionnelle (FR48)

**Given** une fonction Rust retourne une erreur
**When** l'erreur est de type AppError
**Then** elle est sérialisée correctement pour le frontend via IPC

---

### Story 1.3: Interface utilisateur minimale avec version

As a utilisateur,
I want voir l'interface de base avec le numéro de version,
So that je sache que l'application fonctionne et quelle version j'utilise.

**Acceptance Criteria:**

**Given** l'application est lancée
**When** la fenêtre principale s'affiche
**Then** le numéro de version est visible dans l'interface (FR41)
**And** l'interface utilise la structure de composants Svelte définie

**Given** l'application est au repos (idle)
**When** je mesure la consommation mémoire
**Then** elle est inférieure à 100MB RAM (FR43)

**Given** les stores Svelte sont créés
**When** j'examine src/stores/
**Then** recordingState, errorStore existent
**And** les stores sont réactifs aux événements backend

---

### Story 1.4: Build et packaging .deb pour Ubuntu

As a utilisateur Ubuntu,
I want installer l'application via un package .deb,
So that je puisse l'utiliser comme n'importe quelle application native.

**Acceptance Criteria:**

**Given** le code source est prêt
**When** j'exécute `pnpm tauri build`
**Then** un fichier .deb est généré dans target/release/bundle/deb/

**Given** le .deb est généré
**When** je l'installe avec `sudo dpkg -i vocal-note-taker.deb`
**Then** l'installation réussit sans erreur (FR38)
**And** le binaire est placé dans /usr/bin/
**And** un fichier .desktop est créé dans /usr/share/applications/

**Given** l'application est installée
**When** je cherche "vocal-note-taker" dans le menu Ubuntu
**Then** l'application apparaît et peut être lancée (FR40)

**Given** le build est configuré
**When** j'examine Cargo.toml
**Then** opt-level="z", lto=true, strip=true sont configurés

---

### Story 1.5: Fermeture propre de l'application

As a utilisateur,
I want pouvoir quitter complètement l'application,
So that je libère les ressources système quand je n'en ai plus besoin.

**Acceptance Criteria:**

**Given** l'application est en cours d'exécution
**When** je sélectionne "Quitter" dans le menu ou utilise le raccourci
**Then** l'application se ferme complètement (FR42)
**And** tous les processus sont terminés
**And** aucun processus orphelin ne reste en mémoire

**Given** l'application est en cours d'enregistrement
**When** je tente de quitter
**Then** l'enregistrement en cours est arrêté proprement
**And** les fichiers temporaires sont nettoyés
**And** l'application se ferme

---

## Epic 2: Audio Capture & Feedback

**Epic Goal:** Permettre aux utilisateurs d'enregistrer leur voix avec un feedback visuel continu (waveform, timer, indicateur REC) donnant confiance que l'audio est capturé correctement.

**User Outcome:** L'utilisateur peut cliquer sur un bouton ou utiliser un raccourci pour démarrer l'enregistrement, voir la waveform et le timer en temps réel, et arrêter l'enregistrement avec confirmation visuelle.

**FRs Covered:** FR1-FR8 (8 FRs)

---

### Story 2.1: Capture audio via microphone système

As a utilisateur,
I want que l'application capture l'audio de mon microphone,
So that ma voix soit enregistrée pour la transcription.

**Acceptance Criteria:**

**Given** l'application est lancée
**When** le module audio/capture.rs est initialisé
**Then** cpal détecte le microphone système (ALSA/PulseAudio)

**Given** le microphone est disponible
**When** j'initie un enregistrement
**Then** l'audio est capturé en format 16kHz mono
**And** les échantillons sont envoyés via tokio::sync::mpsc channel

**Given** l'enregistrement est arrêté
**When** le buffer audio est traité
**Then** un fichier WAV est sauvegardé dans ~/.local/share/vocal-note-taker/temp/recording.wav (FR8)
**And** le format est 16kHz mono via hound

**Given** le microphone n'est pas accessible
**When** j'initie un enregistrement
**Then** AppError::MicrophoneAccessDenied est retourné (FR44)

---

### Story 2.2: Bouton d'enregistrement avec indicateur REC

As a utilisateur,
I want un bouton pour démarrer/arrêter l'enregistrement avec un indicateur visuel,
So that je sache clairement quand j'enregistre.

**Acceptance Criteria:**

**Given** l'application est en état idle
**When** je clique sur le bouton d'enregistrement
**Then** l'enregistrement démarre (FR1)
**And** l'événement IPC recording-started est émis
**And** le store recordingState passe à 'recording'

**Given** l'enregistrement est actif
**When** je regarde l'interface
**Then** un indicateur REC rouge est visible (FR6)
**And** le bouton change d'apparence pour indiquer l'état actif

**Given** l'enregistrement est actif
**When** je clique à nouveau sur le bouton
**Then** l'enregistrement s'arrête (FR3)
**And** l'événement IPC recording-stopped est émis
**And** le store recordingState passe à 'idle' ou 'transcribing'

**Given** le composant RecordButton.svelte existe
**When** j'examine son implémentation
**Then** il utilise le store recordingState pour son état
**And** il appelle les commandes IPC start_recording / stop_recording

---

### Story 2.3: Timer d'enregistrement en temps réel

As a utilisateur,
I want voir le temps d'enregistrement écoulé,
So that je sache depuis combien de temps j'enregistre.

**Acceptance Criteria:**

**Given** l'enregistrement est actif
**When** le temps passe
**Then** un timer affiche les secondes écoulées (FR5)
**And** le format est MM:SS ou SS selon la durée

**Given** l'enregistrement démarre
**When** le timer s'initialise
**Then** il commence à 00:00

**Given** l'enregistrement s'arrête
**When** le timer est visible
**Then** il affiche la durée finale de l'enregistrement
**And** il se réinitialise au prochain enregistrement

**Given** le composant Timer.svelte existe
**When** j'examine son implémentation
**Then** il utilise un store recordingDuration
**And** il se met à jour chaque seconde via setInterval

---

### Story 2.4: Visualisation waveform en temps réel

As a utilisateur,
I want voir une visualisation de ma voix pendant l'enregistrement,
So that j'aie un feedback immédiat que le microphone capte bien.

**Acceptance Criteria:**

**Given** l'enregistrement est actif
**When** je parle dans le microphone
**Then** une waveform s'affiche en temps réel (FR7)
**And** la visualisation réagit à l'amplitude de ma voix

**Given** le backend capture l'audio
**When** les échantillons sont traités
**Then** l'événement waveform-data est émis à 30-60 FPS
**And** les données sont de type Float32Array

**Given** le composant WaveformDisplay.svelte existe
**When** j'examine son implémentation
**Then** il utilise Canvas API natif (pas de librairie externe)
**And** il utilise requestAnimationFrame pour le rendu
**And** il consomme le store audioData

**Given** l'enregistrement s'arrête
**When** la waveform est visible
**Then** elle s'arrête de s'animer
**And** affiche un état statique ou se vide

---

### Story 2.5: Raccourci clavier global pour enregistrement

As a utilisateur,
I want utiliser un raccourci clavier pour enregistrer,
So that je puisse démarrer rapidement sans utiliser la souris.

**Acceptance Criteria:**

**Given** l'application est lancée
**When** tauri-plugin-global-shortcut est configuré
**Then** le raccourci par défaut (ex: Ctrl+Alt+R) est enregistré

**Given** le raccourci est configuré
**When** j'appuie sur le raccourci clavier
**Then** l'enregistrement démarre (FR2)
**And** le comportement est identique au clic sur le bouton

**Given** l'enregistrement est actif via raccourci
**When** je relâche le raccourci ou appuie à nouveau
**Then** l'enregistrement s'arrête (FR3)

**Given** le raccourci est utilisé
**When** l'application est en arrière-plan
**Then** le raccourci fonctionne toujours (préparation pour Epic 5)

---

## Epic 3: Local Transcription

**Epic Goal:** Transformer l'audio enregistré en texte précis (≥90% qualité) via whisper.cpp local, garantissant confidentialité totale et fonctionnement hors-ligne.

**User Outcome:** L'utilisateur obtient une transcription de haute qualité de son enregistrement vocal sans qu'aucune donnée ne quitte sa machine, avec un feedback de progression pendant le traitement.

**FRs Covered:** FR9-FR14 (6 FRs)

---

### Story 3.1: Intégration whisper-rs et chargement du modèle

As a utilisateur,
I want que l'application utilise un modèle de transcription local,
So that mes données vocales restent privées et je puisse travailler hors-ligne.

**Acceptance Criteria:**

**Given** le module transcription/whisper.rs existe
**When** whisper-rs est intégré
**Then** le binding vers whisper.cpp est fonctionnel

**Given** le modèle ggml-large.bin (~3GB) est présent
**When** l'application démarre
**Then** le modèle est chargé depuis ~/.local/share/vocal-note-taker/models/
**And** aucune connexion réseau n'est établie (FR10)

**Given** le modèle n'est pas présent
**When** l'application tente de le charger
**Then** un message d'erreur clair indique comment obtenir le modèle
**And** un script download-models.sh est disponible

**Given** la transcription est lancée
**When** le processus s'exécute
**Then** tout le traitement se fait localement (FR9)
**And** aucune donnée n'est envoyée sur le réseau

---

### Story 3.2: Transcription asynchrone avec progression

As a utilisateur,
I want voir la progression de la transcription,
So that je sache que le traitement est en cours et combien de temps il reste.

**Acceptance Criteria:**

**Given** un enregistrement audio existe
**When** la transcription démarre
**Then** elle s'exécute dans un tokio::spawn task (async)
**And** l'interface reste réactive

**Given** la transcription est en cours
**When** le traitement progresse
**Then** l'événement transcription-progress est émis (0-100%) (FR11)
**And** le composant ProgressBar.svelte affiche la progression

**Given** la transcription se termine
**When** le texte est prêt
**Then** l'événement transcription-complete est émis avec le texte (FR12)
**And** le store transcriptionText est mis à jour
**And** le store recordingState passe à 'idle'

**Given** 60 secondes d'audio
**When** la transcription s'exécute
**Then** elle se termine en moins de 30 secondes (NFR-PERF-2)

---

### Story 3.3: Gestion des erreurs de transcription

As a utilisateur,
I want être informé clairement si la transcription échoue,
So that je puisse comprendre le problème et réessayer.

**Acceptance Criteria:**

**Given** le fichier audio est corrompu ou invalide
**When** la transcription est tentée
**Then** AppError::TranscriptionFailed est retourné (FR13)
**And** un message d'erreur clair est affiché
**And** l'application reste fonctionnelle

**Given** le modèle whisper n'est pas chargé correctement
**When** la transcription est tentée
**Then** une erreur explicite indique le problème du modèle
**And** des instructions de résolution sont fournies

**Given** une erreur de transcription se produit
**When** l'erreur est propagée au frontend
**Then** le composant ErrorNotification.svelte l'affiche
**And** l'utilisateur peut relancer un enregistrement

**Given** la transcription échoue
**When** l'état est mis à jour
**Then** recordingState revient à 'idle'
**And** l'interface permet de recommencer

---

### Story 3.4: Nettoyage automatique des fichiers temporaires

As a utilisateur soucieux de ma vie privée,
I want que les fichiers audio temporaires soient supprimés après transcription,
So that mes enregistrements vocaux ne persistent pas sur le disque.

**Acceptance Criteria:**

**Given** la transcription est terminée avec succès
**When** le texte est retourné
**Then** le fichier recording.wav est supprimé immédiatement (FR14)
**And** aucune trace audio ne reste dans ~/.local/share/vocal-note-taker/temp/

**Given** la transcription échoue
**When** l'erreur est gérée
**Then** le fichier audio temporaire est également supprimé
**And** l'utilisateur est informé que l'audio n'a pas été conservé

**Given** l'application se ferme pendant une transcription
**When** le nettoyage est effectué
**Then** les fichiers temporaires orphelins sont supprimés au prochain démarrage

**Given** le dossier temp/ est examiné
**When** l'application est au repos
**Then** aucun fichier .wav n'est présent (NFR-SEC-3)

---

## Epic 4: Text Display & Copy Integration

**Epic Goal:** Permettre aux utilisateurs de vérifier la précision de la transcription et copier le texte dans leur presse-papiers système pour l'utiliser dans d'autres applications (ChatGPT, Teams, etc.).

**User Outcome:** L'utilisateur peut rapidement scanner le texte transcrit, appuyer sur Enter pour le copier, et immédiatement le coller dans son application cible avec un workflow <1 seconde.

**FRs Covered:** FR15-FR25 (11 FRs)

---

### Story 4.1: Affichage du texte transcrit

As a utilisateur,
I want voir le texte transcrit de manière lisible,
So that je puisse vérifier rapidement sa précision.

**Acceptance Criteria:**

**Given** la transcription est terminée
**When** le texte est affiché
**Then** il apparaît dans un format lisible et clair (FR15)
**And** la police est suffisamment grande pour une lecture confortable

**Given** le texte transcrit tient dans le viewport
**When** je regarde l'interface
**Then** le texte s'affiche sans troncature ni scrolling (FR16)
**And** tout le contenu est visible d'un coup d'œil

**Given** le texte est affiché
**When** je le parcours visuellement
**Then** je peux vérifier la précision en 2-3 secondes (FR17, NFR-USA-2)
**And** la hiérarchie visuelle est claire

**Given** le composant TranscriptionDisplay.svelte existe
**When** j'examine son implémentation
**Then** il consomme le store transcriptionText
**And** il gère les cas de texte vide ou en attente

---

### Story 4.2: Copie vers le presse-papiers avec confirmation

As a utilisateur,
I want copier le texte transcrit vers mon presse-papiers,
So that je puisse le coller dans d'autres applications.

**Acceptance Criteria:**

**Given** du texte transcrit est affiché
**When** je clique sur le bouton "Copier"
**Then** le texte est copié dans le presse-papiers système (FR20)
**And** tauri-plugin-clipboard est utilisé

**Given** le texte est copié
**When** l'action réussit
**Then** un feedback visuel "✓ Copié!" s'affiche (FR23)
**And** le feedback disparaît après quelques secondes

**Given** le texte est copié
**When** je colle dans une autre application
**Then** le texte est en format plain text, sans formatage riche (FR24)

**Given** le presse-papiers système a un problème
**When** la copie échoue
**Then** AppError::ClipboardError est retourné
**And** un message d'erreur clair est affiché

---

### Story 4.3: Raccourci Enter pour copier avec auto-focus

As a utilisateur,
I want copier le texte en appuyant sur Enter,
So that mon workflow soit le plus rapide possible (<1 seconde).

**Acceptance Criteria:**

**Given** la transcription vient de se terminer
**When** le texte s'affiche
**Then** le bouton "Copier" reçoit automatiquement le focus (FR22)
**And** il est visuellement mis en évidence

**Given** le bouton "Copier" a le focus
**When** j'appuie sur la touche Enter
**Then** le texte est copié dans le presse-papiers (FR21)
**And** le feedback "✓ Copié!" s'affiche

**Given** le texte est prêt à être copié
**When** j'utilise le workflow complet
**Then** maximum 3 actions utilisateur sont requises (NFR-USA-3)
**And** le workflow raccourci → parler → Enter est possible

**Given** l'utilisateur n'a pas encore appuyé sur Enter
**When** le texte est affiché
**Then** la copie n'est PAS automatique (FR25)
**And** l'utilisateur contrôle quand la copie se fait

---

### Story 4.4: Workflow linéaire et réinitialisation

As a utilisateur,
I want un workflow simple sans historique,
So that l'interface reste épurée et je me concentre sur la tâche actuelle.

**Acceptance Criteria:**

**Given** du texte transcrit est affiché
**When** je démarre un nouvel enregistrement
**Then** le texte précédent est automatiquement effacé (FR18)
**And** l'interface revient à l'état d'enregistrement

**Given** l'application fonctionne
**When** j'examine les fonctionnalités
**Then** il n'y a pas de gestion d'historique (FR19)
**And** pas de liste de transcriptions passées
**And** pas de bouton "précédent/suivant"

**Given** l'utilisateur termine une session
**When** il veut recommencer
**Then** un simple clic/raccourci réinitialise tout
**And** le workflow reste linéaire: enregistrer → transcrire → copier → répéter

---

## Epic 5: Ghost Mode & Quick Access

**Epic Goal:** Rendre l'application "invisible" jusqu'à ce qu'elle soit nécessaire, permettant aux utilisateurs d'y accéder instantanément via des raccourcis globaux sans interrompre leur workflow.

**User Outcome:** L'utilisateur peut invoquer vocal-note-taker depuis n'importe où via Ctrl+Alt+R (ou raccourci personnalisé), enregistrer pendant que l'app reste en arrière-plan, et recevoir une notification quand la transcription est prête.

**FRs Covered:** FR26-FR32 (7 FRs)

---

### Story 5.1: Raccourcis globaux en arrière-plan

As a utilisateur,
I want utiliser les raccourcis clavier même quand l'application est en arrière-plan,
So that je puisse enregistrer instantanément depuis n'importe quelle application.

**Acceptance Criteria:**

**Given** l'application est minimisée ou en arrière-plan
**When** j'appuie sur le raccourci global (ex: Ctrl+Alt+R)
**Then** le raccourci est détecté et l'enregistrement démarre (FR26)
**And** l'application peut rester en arrière-plan pendant l'enregistrement

**Given** tauri-plugin-global-shortcut 2.x est configuré
**When** l'application démarre
**Then** les raccourcis globaux sont enregistrés auprès du système
**And** ils fonctionnent sur X11 et Wayland (avec limitations connues)

**Given** je travaille dans une autre application
**When** j'utilise le raccourci global
**Then** mon focus actuel n'est pas interrompu
**And** l'enregistrement se fait silencieusement

**Given** Wayland est utilisé
**When** certaines limitations existent
**Then** un message informatif est affiché si le raccourci ne fonctionne pas
**And** des alternatives sont suggérées

---

### Story 5.2: Persistance en arrière-plan (Ghost Mode)

As a utilisateur,
I want que l'application continue de fonctionner après fermeture de la fenêtre,
So that je puisse l'invoquer rapidement sans la relancer.

**Acceptance Criteria:**

**Given** l'application est ouverte
**When** je ferme la fenêtre (bouton X)
**Then** l'application ne se termine PAS (FR27)
**And** elle continue de fonctionner en arrière-plan
**And** les raccourcis globaux restent actifs

**Given** l'application est en arrière-plan
**When** je vérifie les processus système
**Then** le processus Rust backend reste en mémoire (FR32)
**And** la consommation mémoire reste sous contrôle (<200MB)

**Given** l'application est minimisée
**When** elle passe en arrière-plan
**Then** les processus ne sont pas terminés (FR28)
**And** l'application peut être restaurée instantanément

**Given** l'application est en ghost mode
**When** j'utilise un raccourci global
**Then** la fenêtre peut réapparaître si nécessaire
**And** l'enregistrement fonctionne normalement

---

### Story 5.3: System tray avec menu contextuel

As a utilisateur,
I want une icône dans le system tray,
So that je puisse contrôler l'application même quand elle est invisible.

**Acceptance Criteria:**

**Given** l'application est lancée
**When** elle s'exécute
**Then** une icône apparaît dans le system tray Ubuntu

**Given** l'icône est dans le tray
**When** je clique dessus
**Then** un menu contextuel s'affiche avec:
- "Afficher/Masquer" pour toggle la fenêtre
- "Quitter" pour fermer complètement l'application

**Given** l'application est en arrière-plan
**When** je sélectionne "Afficher"
**Then** la fenêtre principale réapparaît au premier plan

**Given** l'application est visible
**When** je sélectionne "Masquer"
**Then** la fenêtre se cache mais l'application reste active

**Given** je sélectionne "Quitter"
**When** l'action est confirmée
**Then** l'application se ferme complètement (FR42)
**And** tous les processus sont terminés

---

### Story 5.4: Notifications système et rappel au premier plan

As a utilisateur,
I want recevoir une notification quand la transcription est prête,
So that je sache que je peux copier le texte même si je travaille ailleurs.

**Acceptance Criteria:**

**Given** la transcription est en cours en arrière-plan
**When** elle se termine
**Then** une notification système s'affiche (FR29)
**And** elle utilise libnotify sur Ubuntu (FR31)

**Given** la notification est affichée
**When** elle apparaît
**Then** elle indique que la transcription est prête
**And** elle contient un aperçu ou le début du texte

**Given** la notification est visible
**When** je clique dessus
**Then** l'application revient au premier plan (FR30)
**And** la fenêtre s'affiche avec le texte transcrit
**And** le bouton "Copier" a le focus

**Given** tauri-plugin-notification 2.x est configuré
**When** une notification est émise
**Then** elle s'intègre nativement avec le système Ubuntu
**And** elle respecte les préférences de notification de l'utilisateur

---

## Epic 6: Configuration & Personalization

**Epic Goal:** Permettre aux utilisateurs de personnaliser le comportement de l'application (raccourcis clavier, préférences) via un fichier de configuration simple.

**User Outcome:** L'utilisateur peut éditer ~/.config/vocal-note-taker/config.toml pour personnaliser les raccourcis, activer/désactiver des fonctionnalités, et ajuster les paramètres selon son workflow.

**FRs Covered:** FR33-FR37 (5 FRs)

---

### Story 6.1: Chargement de la configuration TOML

As a utilisateur,
I want que l'application charge ma configuration depuis un fichier local,
So that mes préférences soient persistantes entre les sessions.

**Acceptance Criteria:**

**Given** le module config/loader.rs existe
**When** l'application démarre
**Then** elle cherche config.toml dans ~/.config/vocal-note-taker/ (FR35)
**And** le parsing utilise serde avec le format TOML (FR34)

**Given** le fichier config.toml existe et est valide
**When** il est chargé
**Then** toutes les sections sont parsées: [hotkeys], [preferences], [audio]
**And** les valeurs sont stockées dans configStore

**Given** le fichier config.toml n'existe pas
**When** l'application démarre
**Then** des valeurs par défaut sont utilisées (FR37)
**And** l'application fonctionne normalement

**Given** le fichier config.toml contient des erreurs
**When** le parsing échoue
**Then** AppError::ConfigurationError est retourné
**And** un message indique la ligne/erreur problématique
**And** les valeurs par défaut sont utilisées comme fallback

**Given** un fichier config.example.toml existe
**When** l'utilisateur veut personnaliser
**Then** il peut le copier et le modifier comme référence

---

### Story 6.2: Configuration des raccourcis clavier

As a utilisateur,
I want personnaliser mes raccourcis clavier,
So that j'utilise des combinaisons qui conviennent à mon workflow.

**Acceptance Criteria:**

**Given** la section [hotkeys] existe dans config.toml
**When** j'examine les options disponibles
**Then** je peux configurer: start_recording, stop_recording, toggle_window (FR33)

**Given** j'ai défini start_recording = "Ctrl+Shift+R"
**When** l'application charge la configuration
**Then** ce raccourci remplace la valeur par défaut
**And** tauri-plugin-global-shortcut utilise cette combinaison

**Given** j'ai défini un raccourci invalide ou en conflit
**When** l'application charge la configuration
**Then** un avertissement est affiché
**And** le raccourci par défaut est utilisé

**Given** la documentation config-schema.md existe
**When** je veux connaître les options
**Then** toutes les clés de [hotkeys] sont documentées
**And** les formats acceptés sont expliqués (ex: "Ctrl+Alt+R", "Super+R")

---

### Story 6.3: Application de la configuration au redémarrage

As a utilisateur,
I want que mes changements de configuration prennent effet au redémarrage,
So that je puisse ajuster l'application selon mes besoins.

**Acceptance Criteria:**

**Given** j'ai modifié config.toml
**When** je redémarre l'application
**Then** les nouveaux paramètres sont appliqués (FR36)
**And** l'interface reflète les changements

**Given** l'application est en cours d'exécution
**When** je modifie config.toml
**Then** les changements ne sont PAS appliqués immédiatement (MVP)
**And** un redémarrage est nécessaire

**Given** la section [preferences] existe
**When** j'examine les options
**Then** je peux configurer: auto_copy_clipboard, show_waveform, whisper_model
**And** chaque option a une valeur par défaut documentée

**Given** la section [audio] existe
**When** j'examine les options
**Then** je peux voir: sample_rate=16000, channels=1
**And** ces valeurs sont fixes pour le MVP (pas de personnalisation audio)

**Given** le hot-reload est mentionné
**When** j'examine la documentation
**Then** il est clairement indiqué comme "post-MVP"
**And** le redémarrage est la méthode actuelle
