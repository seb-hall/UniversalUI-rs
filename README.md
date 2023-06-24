# UniversalUI
Cross-Platform Development 2

## Motivation

Cross-platform development often involves chosing between performance and simplicity. 
Popular frameworks such as Electron and Flutter are easier to develop with JavaScript but produce very bloated and inefficient applications.
Alternatives such as Qt and wxWidgets improve on performance and but increase development complexity and often lack good mobile support.

UniversalUI aims to find the best of both worlds - providing extremely high efficiency while using a very simple and lean API.

## Architecture

UniversalUI is built primarily in Rust, and organised into a series of *crates*.

**universalui_core** provides the core functionality and types of UniversalUI. It is used internally by other crates, 
and contains everything you need to design and build basic apps for any platform.

**universalui_vulkan** implements Vulkan rendering for Windows, Linux and Android.

**universalui_metal** implements Metal rendering for macOS, iPadOS and iOS.

**universalui_win32** is a crate specifically designed for the Windows platform, not intended for end users.

**universalui_linux** is a crate specifically designed for the Linux platform, not intended for end users.

**universalui_macOS** is a crate specifically designed for the macOS platform, not intended for end users.

**universalui_iOS** is a crate specifically designed for the iOS/iPadOS platforms, not intended for end users.

**universalui_android** is a crate specifically designed for the Android platform, not intended for end users.

<!---
Central to UniversalUI is the **UniversalUI-Server**. 
This manages communication between UniversalUI modules, and is key to the flexibility of UniversalUI.
It can be installed as a shared framework (UNIX platforms only), or bundled within the application bundle (all plaforms).

**UniversalUI Clients** contain application code, and are entirely source-code compatible between different platforms.
These contain the entry point of any app that runs on UniversalUI, and support many different development flows.

Clients can be written with a variety of languages, although support for each language depends on the platform:

- Rust (macOS, iOS, Windows, Linux, Android)
- C++ (macOS, iOS, Windows, Linux)
- Python (macOS, Windows, Linux)
- Swift (macOS, iOS)
- JavaScript (requires NodeJS)

**UniversalUI Modules** provide services and functions to the framework. 
The core framework includes a variety of modules that provide key functionality such as windowing and rendering.
Custom and 3rd party modules can be designed, allowing for even more flexibility and modularity.
Modules can be installed system-wide (UNIX platforms only) or locally within the application bundle (all platforms).

-->

