use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlaybackProfile {
    Performance,
    Balanced,
    Quality,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProxyPreset {
    Eco,
    Balanced,
    Ultra,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GpuTransport {
    Mp4Proxy,
    NativeRealtime,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AudioBarsSource {
    Pulse,
    Synth,
}

#[derive(Parser)]
#[command(name = "kitsune-livewallpaper")]
#[command(about = "Kitsune custom wallpaper engine MVP")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Instala dependencias del sistema requeridas por kitsune-livewallpaper")]
    InstallDependencies,
    #[command(about = "Gestiona la configuracion por monitor en config.json")]
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    #[command(about = "Aplica configuraciones guardadas (solo monitores con cambios)")]
    StartConfig {
        #[arg(long, default_value_os_t = default_config_path())]
        config: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Instala/habilita/deshabilita el servicio de autostart de usuario")]
    ServiceAutostart {
        #[command(subcommand)]
        command: ServiceAutostartCommands,
    },
    #[command(about = "Inspecciona metadatos de un wallpaper de Wallpaper Engine")]
    Inspect {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
    },
    #[command(about = "Vuelca el JSON crudo/normalizado de una escena")]
    SceneDump {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        full: bool,
    },
    #[command(about = "Genera un plan de render/reproduccion para una escena")]
    ScenePlan {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
    },
    #[command(about = "Genera plan de audio para una escena")]
    SceneAudioPlan {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
    },
    #[command(about = "Escanea libreria local y resume contenido de wallpapers")]
    LibraryScan {
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long, default_value_t = 20)]
        top_effects: usize,
        #[arg(long)]
        summary_only: bool,
    },
    #[command(about = "Genera roadmap/prioridades de procesamiento para la libreria")]
    LibraryRoadmap {
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long, default_value_t = 15)]
        top_n: usize,
    },
    #[command(about = "Simula runtime de escena y extrae telemetria basica")]
    SceneRuntime {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 4)]
        seconds: u64,
        #[arg(long, default_value_t = 50)]
        frame_ms: u64,
        #[arg(long)]
        extract_music: bool,
    },
    #[command(about = "Renderiza una escena a salida intermedia")]
    SceneRender {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 4)]
        seconds: u64,
        #[arg(long, default_value_t = 50)]
        frame_ms: u64,
    },
    #[command(about = "Inspecciona/visualiza grafo GPU de una escena")]
    SceneGpuGraph {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
    },
    #[command(about = "Planifica ruta de reproduccion nativa (sin proxy)")]
    SceneNativePlan {
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
    },
    #[command(about = "Reproduce escena con pipeline GPU experimental")]
    SceneGpuPlay {
        wallpaper: String,
        #[arg(long)]
        monitor: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        keep_services: bool,
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 4)]
        seconds: u64,
        #[arg(long, default_value_t = 50)]
        frame_ms: u64,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Performance)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long, default_value_t = true)]
        clock_overlay: bool,
        #[arg(long, default_value_t = true)]
        apply_kitsune_overlay: bool,
        #[arg(long, value_enum, default_value_t = GpuTransport::Mp4Proxy)]
        transport: GpuTransport,
        #[arg(long)]
        require_native: bool,
        #[arg(long, value_enum, default_value_t = AudioBarsSource::Pulse)]
        audio_bars_source: AudioBarsSource,
        #[arg(long, default_value_t = 2560)]
        proxy_width: u32,
        #[arg(long, default_value_t = 60)]
        proxy_fps: u32,
        #[arg(long, default_value_t = 20)]
        proxy_crf: u8,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Actualiza overlays de texto (song/artist/clock)")]
    TextRefresh {
        #[arg(long)]
        spec: PathBuf,
        #[arg(long = "loop", default_value_t = false)]
        loop_mode: bool,
        #[arg(long, default_value_t = 1)]
        interval_seconds: u64,
    },
    #[command(about = "Reproduce una escena de Wallpaper Engine como live wallpaper")]
    ScenePlay {
        wallpaper: String,
        #[arg(long)]
        monitor: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        keep_services: bool,
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 4)]
        seconds: u64,
        #[arg(long, default_value_t = 50)]
        frame_ms: u64,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Performance)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long, default_value_t = true)]
        clock_overlay: bool,
        #[arg(long, value_enum, default_value_t = ProxyPreset::Balanced)]
        proxy_preset: ProxyPreset,
        #[arg(long)]
        auto_tune: bool,
        #[arg(long)]
        proxy_width: Option<u32>,
        #[arg(long)]
        proxy_fps: Option<u32>,
        #[arg(long)]
        proxy_crf: Option<u8>,
        #[arg(long)]
        no_proxy_optimize: bool,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Reproduce un archivo de video como live wallpaper")]
    VideoPlay {
        video: String,
        #[arg(long)]
        monitor: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        keep_services: bool,
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Quality)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long, default_value_t = true)]
        seamless_loop: bool,
        #[arg(long, default_value_t = false)]
        loop_crossfade: bool,
        #[arg(long, default_value_t = 0.35)]
        loop_crossfade_seconds: f32,
        #[arg(long, default_value_t = true)]
        optimize: bool,
        #[arg(long, default_value_t = 3840)]
        proxy_width: u32,
        #[arg(long, default_value_t = 60)]
        proxy_fps: u32,
        #[arg(long, default_value_t = 16)]
        proxy_crf: u8,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Prueba captura de audio desde fuente seleccionada")]
    AudioProbe {
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 2)]
        seconds: u64,
    },
    #[command(about = "Lee stream de audio en tiempo real para analisis/debug")]
    AudioStream {
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = 2)]
        seconds: u64,
        #[arg(long, default_value_t = 50)]
        frame_ms: u64,
    },
    #[command(about = "Detiene servicios en conflicto antes de iniciar wallpaper")]
    StopServices {
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Inicia servicios de wallpaper hasta confirmar que quedan activos")]
    StartServices {
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Aplica un wallpaper (auto: escena o video segun entrada)")]
    Apply {
        wallpaper: String,
        #[arg(long)]
        monitor: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long)]
        keep_services: bool,
        #[arg(long = "service")]
        services: Vec<String>,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Balanced)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long)]
        allow_scene_preview_fallback: bool,
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Agrega o actualiza config de monitor para video-play
    #[command(about = "Agrega o actualiza config de monitor para video-play")]
    SetVideo {
        #[arg(long)]
        monitor: String,
        #[arg(long)]
        video: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        keep_services: bool,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Performance)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long, default_value_t = true)]
        seamless_loop: bool,
        #[arg(long, default_value_t = false)]
        loop_crossfade: bool,
        #[arg(long, default_value_t = 0.35)]
        loop_crossfade_seconds: f32,
        #[arg(long, default_value_t = true)]
        optimize: bool,
        #[arg(long, default_value_t = 2560)]
        proxy_width: u32,
        #[arg(long, default_value_t = 30)]
        proxy_fps: u32,
        #[arg(long, default_value_t = 24)]
        proxy_crf: u8,
        #[arg(long, default_value_os_t = default_config_path())]
        config: PathBuf,
    },
    /// Agrega o actualiza config de monitor para apply
    #[command(about = "Agrega o actualiza config de monitor para apply")]
    SetApply {
        #[arg(long)]
        monitor: String,
        #[arg(long)]
        wallpaper: String,
        #[arg(long, default_value_os_t = default_downloads_root())]
        downloads_root: PathBuf,
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        keep_services: bool,
        #[arg(long)]
        mute_audio: bool,
        #[arg(long, value_enum, default_value_t = PlaybackProfile::Balanced)]
        profile: PlaybackProfile,
        #[arg(long)]
        display_fps: Option<u32>,
        #[arg(long, default_value_t = true)]
        allow_scene_preview_fallback: bool,
        #[arg(long, default_value_os_t = default_config_path())]
        config: PathBuf,
    },
    /// Elimina la configuracion de un monitor
    #[command(about = "Elimina la configuracion de un monitor")]
    Remove {
        #[arg(long)]
        monitor: String,
        #[arg(long, default_value_os_t = default_config_path())]
        config: PathBuf,
    },
    /// Lista la configuracion actual por monitor
    #[command(about = "Lista la configuracion actual por monitor")]
    List {
        #[arg(long, default_value_os_t = default_config_path())]
        config: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum ServiceAutostartCommands {
    /// Instala unit file de systemd --user para autostart
    #[command(about = "Instala unit file de systemd --user para autostart")]
    Install {
        #[arg(long)]
        overwrite: bool,
        #[arg(long)]
        dry_run: bool,
    },
    /// Habilita e inicia el servicio de autostart
    #[command(about = "Habilita e inicia el servicio de autostart")]
    Enable {
        #[arg(long)]
        dry_run: bool,
    },
    /// Deshabilita y detiene el servicio de autostart
    #[command(about = "Deshabilita y detiene el servicio de autostart")]
    Disable {
        #[arg(long)]
        dry_run: bool,
    },
    /// Elimina el unit file de autostart del usuario
    #[command(about = "Elimina el unit file de autostart del usuario")]
    Remove {
        #[arg(long)]
        dry_run: bool,
    },
    /// Muestra estado del servicio/autostart
    #[command(about = "Muestra estado del servicio/autostart")]
    Status,
}

fn default_downloads_root() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".local/share/kitsune/we/downloads");
    }
    PathBuf::from(".")
}

fn default_config_path() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".config/kitsune-livewallpaper/config.json");
    }
    PathBuf::from("config.json")
}
