## Future Desktop and Web applications for TutorLoL

- **Check out** [TutorLoLv2 backend repository](https://github.com/LuizGomes56/tutorlolv2)

- **NEW** - Game overlay coming soon!
- **NEW** - Access to in-game formulas
- This is meant to replace the current `C#` made Webview application that runs TutorLoL on production
- A complete new, and stable version is being built such that updates are performed automatically, and avoided if not necessary

### Planned release date

- **This project is planned to be released on November 2025**

- Most of backend's essential sections are completed. The one that will take most of the time is related to champions and items automatic update, that require manual adjustments to make it work. So far not even 5% of the total was completed.

### Focus of the project

- **Performance** - It was taken very seriously, most parts that are blocking are implemented using unsafe blocks to ensure that it will run the closest to its physical limit

    - Be the fastest loading website among competitors. No matter how hard something is to be implemented, one main purpose of this project is to be the best, regardless of the time it took.

- **Overlay feature** - planned about 2 years ago, but my experience wasn't enough to get it done. The same can be said to technology - It was much harder to create such thing in the past

- **Contentful website** - First version of TutorLoL failed to implement dynamically created pages that would help Google's SEO to know that the website has lots of useful contents to share. The new version will expose all of its formulas, images, icons, and provide open API routes for anyone willing to use. Formulas section might replace the need of scraping Wiki's website, and might be interesting for people that are actively trying to create more apps related to the game.

### Improvements

- **Big performance enhancement** on server side - Average time per calculation dropped from `18ms` to `800ns`, meaning that on average, it got 22,500 times faster.
    - In heavy loads, this can be up to 45,000 times faster

- **TutorLoLv2 compares the main 118 legendary items** instead of just two at a time (If we consider this, then v2 is 2,610,000 times faster)

- **New draggable overlay Window** will be created everytime a new game is detected (aka something is on port 2999). This way, the user can control where he want the data to be displayed

- **Shortcuts** will be created to help interact with the application during the game, especially to manipulate data displayed in overlays (Such as hide/show enemies in the overlayed tables)

- **Compile time data validation** - Most of the data that will be used will be available at compile time. It means that part of the application does not require internet or server connection to work!
    - This also prevents many errors from appearing since frontend will require less data sent by the server. This is also great to reduce total payload size; Redundant information is not sent back

- **Binary Protocol** - Everything uses bincode to achieve small payload sizes and fast serialization / deserializations. Crate `rkyv` was considered, but having to maintain the payload alive, with borrowed references violates Yew's requirements for static lifetimes in their components

### WebAssembly - Yew

- All formulas are compressed using `brotli` level 11, which yield a byte array of about 90Kb. This data is decompressed at runtime using the fastest techniques available to make the lesser impact on page startup as possible. The total data expanded is of about 6MB, and to load its slices, JS has to ask WASM for the pointer where such data is located, and transform it into a string type.

- Most components were reviewed and optimized to avoid unnecessary re-renderings. Benchmarks show that page takes about `200ms` to load, meaning that about 16MB of data was created in just this amount of time, and is great for the web.

- Heavy components are created dynamically. When user is not using a feature, such as formula-hovering, nothing is created to prevent DOM from growing too large and making the application slower. Instead, only when the key `SHIFT` is pressed, that an event is created to listen for changes that should display the formula box.

- Some calls that WASM require JS to perform are simply written in JS. The WebSocket connection was written in JS such that the memory usage and copy is lowered as much as possible. This operation is very dangerous and has caused many runtime issues so far, that I'm working on it. Many tests will be performed before it comes to its definite release.

### Tauri and Windows 

- **Native keyboard** - Many games, such as League of Legends overwrite Tauri's keyboard events when it is in full screen. A native keyboard was implemented using Windows library to ensure that this issue will never happen.

- **Realtime Data** - All that Tauri backend has to do is to manipulate the Webview's position, and retreive the data on port 2999, and send back to JS with the least latency possible. This is done through `#[command]` and `InvokeResponseBody` to prevent that data from being serialized into JSON before being sent through the IPC call to the Webview. *No benefits were observed when using a custom protocol* 

### How to run

- It is necessary to download the backend service, run in debug mode and call `buildscript.js`. This way, the `tutorlolv2_imports` crate will receive its data that will be compiled with the final WASM binary. This includes many `enum`, `struct`, and `static` definitions that are essential for the program to run.

- The necessary steps to run the backend service are listed in its README file
