# kitsune-livewallpaper - Commands

Este documento separa:
- `video-play` como flujo estable/recomendado.
- El resto de comandos como demo o en desarrollo.

## 1) Comando estable: `video-play`

Uso base:

```bash
./target/debug/kitsune-livewallpaper video-play [OPTIONS] --monitor <MONITOR> <VIDEO>
```

### Opciones disponibles en `video-play`

- `--monitor <MONITOR>` (requerido)
- `--downloads-root <DOWNLOADS_ROOT>`
- `--keep-services`
- `--service <SERVICES>` (repetible)
- `--mute-audio`
- `--profile <performance|balanced|quality>`
- `--display-fps <DISPLAY_FPS>`
- `--seamless-loop`
- `--loop-crossfade`
- `--loop-crossfade-seconds <SECONDS>` (default `0.35`)
- `--optimize`
- `--proxy-width <WIDTH>` (default `3840`)
- `--proxy-fps <FPS>` (default `60`)
- `--proxy-crf <CRF>` (default `16`)
- `--dry-run`

### Ejemplos `video-play`

Ejemplo recomendado (buen balance rendimiento/calidad):

```bash
./target/debug/kitsune-livewallpaper video-play /ruta/video.mp4 \
  --monitor DP-1 \
  --profile performance \
  --seamless-loop \
  --optimize \
  --proxy-width 2560 \
  --proxy-fps 30 \
  --proxy-crf 24
```

Con crossfade:

```bash
./target/debug/kitsune-livewallpaper video-play /ruta/video.mp4 \
  --monitor DP-1 \
  --profile performance \
  --seamless-loop \
  --loop-crossfade \
  --loop-crossfade-seconds 0.35 \
  --optimize \
  --proxy-width 2560 \
  --proxy-fps 30 \
  --proxy-crf 24
```

Sin audio:

```bash
./target/debug/kitsune-livewallpaper video-play /ruta/video.mp4 \
  --monitor DP-1 \
  --profile performance \
  --mute-audio \
  --seamless-loop \
  --optimize \
  --proxy-width 2560 \
  --proxy-fps 30 \
  --proxy-crf 24
```

Simulación (sin aplicar cambios):

```bash
./target/debug/kitsune-livewallpaper video-play /ruta/video.mp4 \
  --monitor DP-1 \
  --profile performance \
  --seamless-loop \
  --optimize \
  --proxy-width 2560 \
  --proxy-fps 30 \
  --proxy-crf 24 \
  --dry-run
```

### Aplicar `video-play` a todos los monitores (Hyprland)

```bash
VIDEO="/home/kitotsu/Videos/LiveWallpapers/motionbgs/2b-midnight-bloom/2b-midnight-bloom__4k.mp4"
hyprctl -j monitors | jq -r '.[].name' | while IFS= read -r m; do
  ./target/debug/kitsune-livewallpaper video-play "$VIDEO" \
    --monitor "$m" \
    --profile performance \
    --seamless-loop \
    --optimize \
    --proxy-width 2560 \
    --proxy-fps 30 \
    --proxy-crf 24
done
```

## 1.1) Utilidad estable: `install-dependencies`

Uso:

```bash
./target/debug/kitsune-livewallpaper install-dependencies
```

Este comando ejecuta el instalador de dependencias del proyecto. Busca el script en:
- `scripts/install-deps.sh` (repo local)
- `/usr/share/kitsune-livewallpaper/install-deps.sh` (instalacion por paquete)

## 1.2) Config por monitor (`config`)

Archivo por defecto:

```bash
~/.config/kitsune-livewallpaper/config.json
```

Agregar/actualizar monitor con `video-play`:

```bash
./target/debug/kitsune-livewallpaper config set-video \
  --monitor DP-1 \
  --video /ruta/video.mp4 \
  --profile performance \
  --seamless-loop \
  --optimize \
  --proxy-width 2560 \
  --proxy-fps 30 \
  --proxy-crf 24 \
  --keep-services
```

Agregar/actualizar monitor con `apply`:

```bash
./target/debug/kitsune-livewallpaper config set-apply \
  --monitor HDMI-A-1 \
  --wallpaper 3299228616 \
  --profile balanced \
  --allow-scene-preview-fallback \
  --keep-services
```

Quitar monitor:

```bash
./target/debug/kitsune-livewallpaper config remove --monitor HDMI-A-1
```

Listar config:

```bash
./target/debug/kitsune-livewallpaper config list
```

## 1.3) Arranque por config (`start-config`)

Ejecuta solo monitores cuya configuración cambió (incremental):

```bash
./target/debug/kitsune-livewallpaper start-config
```

Dry-run:

```bash
./target/debug/kitsune-livewallpaper start-config --dry-run
```

## 1.4) Servicios (`start-services` / `stop-services`)

Iniciar servicios requeridos:

```bash
./target/debug/kitsune-livewallpaper start-services
```

Detener servicios:

```bash
./target/debug/kitsune-livewallpaper stop-services
```

Con servicios personalizados:

```bash
./target/debug/kitsune-livewallpaper start-services --service swww-daemon.service --service kitowall-watch.service
```

## 1.5) Servicio de autostart (`service-autostart`)

Instalar unit file de usuario:

```bash
./target/debug/kitsune-livewallpaper service-autostart install
```

Habilitar e iniciar:

```bash
./target/debug/kitsune-livewallpaper service-autostart enable
```

Deshabilitar/parar:

```bash
./target/debug/kitsune-livewallpaper service-autostart disable
```

Eliminar del sistema de usuario:

```bash
./target/debug/kitsune-livewallpaper service-autostart remove
```

Estado:

```bash
./target/debug/kitsune-livewallpaper service-autostart status
```

## 2) Comandos disponibles (demo/en desarrollo)

Los siguientes comandos existen en el binario, pero se consideran de demo/proceso de desarrollo:

- `inspect`
- `scene-dump`
- `scene-plan`
- `scene-audio-plan`
- `library-scan`
- `library-roadmap`
- `scene-runtime`
- `scene-render`
- `scene-gpu-graph`
- `scene-native-plan`
- `scene-gpu-play`
- `text-refresh`
- `scene-play`
- `audio-probe`
- `audio-stream`
- `apply`

Ayuda general:

```bash
./target/debug/kitsune-livewallpaper --help
```

Ayuda de un subcomando específico (ejemplo):

```bash
./target/debug/kitsune-livewallpaper scene-gpu-play --help
```
