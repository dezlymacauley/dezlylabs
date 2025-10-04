# OBS Studio - Recording Settings
_______________________________________________________________________________
## Settings -> Video

### General

- Base (Canvas) Resolution: `1920 x 1080`
- Output(Scaled) Resolution: `1920 x 1080`
- Common FPS Values: `30`
_______________________________________________________________________________
## Settings -> Ouput

Set `Output Mode` to `Advanced`

Click on the `Recording` Tab
_______________________________________________________________________________
### Recording Settings

- Recording Format: `MPEG-4 (.mp4)`
- Video Encoder: `QuickSync H.264` 

Note: If you don't have QuickSync H.264 option, you need to install the 
intel media driver and restart OBS Studio:

```sh
sudo pacman -S --needed intel-media-driver
```

- Audio Encoder: `FFmpeg AAC`
_______________________________________________________________________________
### Encoder Settings

- Rate Control: `VBR`
- Bitrate: `10000 Kbps`
- Max Bitrate: `20000 Kbps`
- Keyframe Interval: `2s`
_______________________________________________________________________________
