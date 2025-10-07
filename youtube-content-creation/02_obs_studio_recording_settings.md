# OBS Studio - Recording Settings
_______________________________________________________________________________
## Settings -> Video

### General

- Base (Canvas) Resolution: `2560x1440`
- Output(Scaled) Resolution: `2560x1440`
- Common FPS Values: `30`
_______________________________________________________________________________
## Settings -> Ouput

Set `Output Mode` to `Advanced`

Click on the `Recording` Tab
_______________________________________________________________________________
### Recording Settings

- Recording Format: `MPEG-4 (.mp4)`
- Video Encoder: `FFmpeg VAAPI H.264` 

Note: If you don't have QuickSync H.264 option, you need to install the 
intel media driver and restart OBS Studio:

```sh
sudo pacman -S --needed intel-media-driver
```

- Audio Encoder: `FFmpeg AAC`
_______________________________________________________________________________
### Encoder Settings

- VAAPI Device: `WhiskeyLake-U GT2 [UHD Graphics 620]`
- Profile: `High`
- Level: `Auto`
- Rate Control: `CBR`
- Bitrate: `60000 Kbps`
- Keyframe Interval: `2 s`
_______________________________________________________________________________
## Bottom Tray of OBS Studio
_______________________________________________________________________________
### Scenes

Create a scene and call it `Face`
_______________________________________________________________________________
### Sources

- Video Capture Device (V4L2)
- Audio Input Capture (PulseAudio)
_______________________________________________________________________________
### Audio Mixer

- Audio Input Capture (PulseAudio) = -2.5 dB
- Desktop Audio = 0.0 dB
- Mic/Aux = Mute
_______________________________________________________________________________
### Mirror the cam

Click `Video Capture Device`

Then right click the area in OBS Studio where the recording is being shown.

Click `Transform`, `Flip Horizontal`
_______________________________________________________________________________
