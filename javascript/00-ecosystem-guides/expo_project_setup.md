
_______________________________________________________________________________

```sh
mkdir name-of-project
cd name-of-project
```

```sh
bunx create-expo-app@latest .
```

_______________________________________________________________________________

Use this the fix postinstall issues.

```sh
bun pm untrusted
```

```sh
bun pm trust --all
```

_______________________________________________________________________________

```sh
bun run reset-project
```

```
Do you want to move existing files to /app-example instead of deleting them?
(Y/n): n
```

_______________________________________________________________________________

Open a separate terminal and run this command to open your mobile project 
as a web app.

I'm using this because `Android Studio` Emulation is painfully slow
on my device, and I don't have a Mac device to test on.

```sh
bun run web
```

## The web view is NOT a solution. Many of the React Native components 
## do not behave normally.

_______________________________________________________________________________

### The instructions below are only if you want to use Android studio 

_______________________________________________________________________________


https://docs.expo.dev/get-started/set-up-your-environment/

_______________________________________________________________________________

Where would you like to develop?

Select `Android Emulator`

_______________________________________________________________________________
How would you like to develop?

Select `Development Build`

_______________________________________________________________________________

Set up an Android Emulator with a development build

Make sure this is `NOT` checked:

Build with Expo Application Services (EAS)
_______________________________________________________________________________

Install Watchman and JDK

Select `Linux`

Install dependencies

1

Follow instructions from the Watchman documentation to compile and install it from the source.

2

Install Java SE Development Kit (JDK):

You can download and install OpenJDK@17 from AdoptOpenJDK or your system packager.

_______________________________________________________________________________

Don't install

```sh
paru -S --needed watchman-bin
```

```sh
sudo pacman -S --needed jdk-openjdk
```
_______________________________________________________________________________
