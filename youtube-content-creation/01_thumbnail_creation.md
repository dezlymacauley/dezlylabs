# YouTube Thumbnail Creation
_______________________________________________________________________________
### Tools

- GIMP
- 7zip
- https://claude.ai/
- https://imgupscaler.com/
- https://www.remove.bg/upload
- https://www.iloveimg.com/remove-background
_______________________________________________________________________________
### Step 1: Create a directory for YouTube Content Creation

```sh
mkdir -p $HOME/local-storage/youtube-content-creation/first-thumbnail
```
_______________________________________________________________________________
### Step 2: Open Gimp and create a canvas with the correct height and width

`File` => `New`

- Width: 1920
- Height: 1080

Use the `BucketFill Tool` to set the background to bl

`File` => `Save`

Then save the project to:
`local-storage/youtube-content-creation/first-thumbnail`

I saved it as `first_thumbnail.xcf`

_______________________________________________________________________________
### Choose a thumbnail font

Find a thumbnail on YouTube with a font that makes you want to click on the
video.

Take a screenshot of the thumbnail and upload it to an AI platform like
Claude AI, and ask the AI to give you the closet Google Font that matches.

E.g.
```
https://fonts.google.com/specimen/Bebas+Neue
```

Then click `Get Font` and `Download all`

You'll get a zip file like this:
```
Bebas_Neue.zip
```

Move the file to:
`local-storage/youtube-content-creation/first-thumbnail`
_______________________________________________________________________________

Unzip the file:
```
7z x Bebas_Neue.zip
```

Delete the `.zip` file and any other file extension that are NOT `.ttf`

The only file you need to keep is the `.ttf` files

This font only has one `.ttf`
```
BebasNeue-Regular.ttf
```

_______________________________________________________________________________

Make sure you are in the directory:
`local-storage/youtube-content-creation/first-thumbnail`

Then run these commands, to add the font to your local font directory:
```sh
mkdir -p $HOME/.fonts 
cp *.ttf $HOME/.fonts/
```

NOTE: `$HOME/.fonts` is where GIMP expects fonts to be.

Close GIMP, and update your font cache:
```sh
fc-cache -fv
```


To check that you have the font, by running this command:
```sh
fc-list :family:style | grep -i bebas | awk -F': ' '{print $2}' | sort | uniq
```

You should get a result like this back:
```
Bebas Neue:style=Regular
```

Open GIMP and click on the `TextTool`

Search for `Bebas Neue Regular`, it should automatically be there.

Set the foreground colour to white and start using it.

You can use the `Move Tool` to move things around
_______________________________________________________________________________
