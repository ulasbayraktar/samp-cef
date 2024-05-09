## Structure
- The file `cef.asi` should be located in the root folder of the game (compiled as `loader.dll`).
- The folder `cef` with all its contents should also be there.
- Additionally, a folder named `CEF` is created at `My Documents/GTA San Andreas User Files/CEF/`, where cache, cookie files, and other necessary things for the correct operation of Chromium are stored.
- `gta_sa.exe`
- `cef.asi`
- `cef/`
    - `client.dll`
    - `libcef.dll`
    - `renderer.exe`
    - etc …

## Tips for Usage and Some Limitations
- Ideally, have one browser with all interfaces. Do not create new ones for different actions, but use the built-in event system.
- If there are client plugins that use relative paths, they will likely break and work incorrectly. Unfortunately, at the moment of initialization, the current directory changes in another thread. As an example: the CLEO library can create its log `cleo.log`, as well as the folders `cleo_text` and `cleo_saves` in the `cef` folder. For correct operation, it is better to determine the path to the current executable file (`gta_sa.exe`).

## Pawn API

`cef_create_browser(player_id, browser_id, const url[], hidden, focused)`

Creates a browser for the specified player.

`cef_destroy_browser(player_id, browser_id)`

Destroys the browser.

`cef_hide_browser(player_id, browser_id, hide)`

Hides the browser.

`cef_emit_event(player_id, const event_name[], args…)`

Triggers an event on the client. Supported argument types: `string`, `integer`, `float`.

`cef_subscribe(const event_name[], const callback[])`

Subscribes to an event from the client. Callback function signature: `Callback(player_id, const arguments[])`

`cef_player_has_plugin(player_id)`

Checks if the client has the plugin.

`cef_create_ext_browser(player_id, browser_id, const texture[], const url[], scale)`

Creates a browser as in the first case, but with a mark that it will be displayed on objects on a specific texture. The `scale` parameter indicates how many times the standard texture should be enlarged. For example, if the standard texture has a size of `250x30`, it will have a size of `1250x150` with the parameter set to 5 units.

`cef_append_to_object(player_id, browser_id, object_id)`

Replaces the texture of the specified object with the browser image on the client side. The browser must be created using `cef_create_ext_browser`, and the texture specified during creation must be present for correct display.

`cef_remove_from_object(player_id, browser_id, object_id)`

Restores the original texture of the object.

`cef_toggle_dev_tools(player_id, browser_id, enabled)`

Enables/disables developer tools.

`native cef_set_audio_settings(player_id, browser_id, Float:max_distance, Float:reference_distance)`

Sets the maximum audible distance for the browser on the object. `reference_distance` is the distance at which maximum volume will be reached, and afterwards, the volume will decrease from `max_distance` to 0.

`cef_focus_browser(player_id, browser_id, focused)`

Makes the browser focused. It comes to the foreground, receives all keyboard and mouse events. Same as passing the argument `focused = true` when creating the browser.

`cef_always_listen_keys(player_id, browser_id, listen)`

Allows the browser to receive keyboard input in the background, even if the browser is not focused or hidden. This allows using functions in JavaScript code to subscribe to keyboard events all the time. For example, you can open/close the interface by pressing a key (`window.addEventListener("keyup")`).

`cef_load_url(player_id, browser_id, const url[])`

Loads the specified URL for the given browser. Faster than recreating the browser.

### There are also two built-in events in the plugin:

`forward OnCefBrowserCreated(player_id, browser_id, status_code)`

Called when the client creates a browser upon request from the server/plugin. The `status_code` value is either 0 (for unsuccessful creation) or an HTTP code (200, 404, etc).

`forward OnCefInitialize(player_id, success)`

Called after the client connects to the CEF server, or when the timeout expires. Roughly speaking, it replaces the manual check `cef_player_has_plugin`.

## Browser API

Browsers also have their own API for managing them.

`cef.set_focus(focused)`

Focuses on the browser, allowing it to be on top of all other windows and to receive keyboard and mouse input.

`cef.on(event_name, callback)`

Subscribes to an event from the browser/other plugins.

`cef.off(event_name, callback)`

NOT IMPLEMENTED AT THE MOMENT
Unsubscribes from an event. To use this function, you need to pass a variable that contains the callback function specified when subscribing to the event.

`cef.hide(hide)`

Hides the browser and mutes its sound.

`cef.emit(event_name, args…)`

Triggers an event on the server/in external plugins with the specified arguments. Supports all types except objects with fields and functions. Note: in plugins, all types can be used normally, but on the server, all arguments are converted into a single string separated by spaces.

## C API

Not working, sorry.

Working example in `cef-interface`, as well as API in `cef-api` and `client/external.rs`

```C++
    #include <cstdint>
    
    // Interrupt the continuation of event callbacks. Also, do not send it to the server.
    static const int EXTERNAL_BREAK = 1;
    // Continue execution. If nobody interrupted it, it will be sent to the server.
    static const int EXTERNAL_CONTINUE = 0;
    
    using BrowserReadyCallback = void(*)(uint32_t);
    using EventCallback = int(*)(const char*, cef_list_value_t*);
    
    extern "C" {
        // Check if a browser exists in the game.
        bool cef_browser_exists(uint32_t browser);
        // Whether the browser has been created and the website has been loaded.
        bool cef_browser_ready(uint32_t browser);
        // Create a browser with the specified parameters. This function is asynchronous, the browser is not created immediately.
        void cef_create_browser(uint32_t id, const char *url, bool hidden, bool focused);
        // Create a CefListValue inside the client.
        cef_list_value_t *cef_create_list();
        // Destroy the browser on the client side.
        void cef_destroy_browser(uint32_t id);
        // Trigger an event on the browser.
        void cef_emit_event(const char *event, cef_list_value_t *list);
        // Focus input on the browser and bring it to the top, as well as receive input from the keyboard and mouse.
        void cef_focus_browser(uint32_t id, bool focus);
        // Whether the game window is active.
        bool cef_gta_window_active();
        // Hide the browser.
        void cef_hide_browser(uint32_t id, bool hide);
        // Check if input is available for a specific browser.
        bool cef_input_available(uint32_t browser);
        // Subscribe to the event of full browser creation.
        void cef_on_browser_ready(uint32_t browser, BrowserReadyCallback callback);
        bool cef_ready();
        // Subscribe to browser events.
        void cef_subscribe(const char *event, EventCallback callback);
        // Attempt to focus on the browser. Similar to a pair of `cef_input_available` + `cef_focus_browser`,
        // but with one significant condition, between the execution of these two functions someone else may take focus.
        // And this function is atomic, which allows you to check and immediately focus, ensuring
        // that no one else can get focus at that time.
        bool cef_try_focus_browser(uint32_t browser);
    }
```


## Instructions for Use

### Description
A browser can be created from two places: from the game mod and plugins.

The browser has two additional states: `hidden` and `focused`. The first state controls whether the browser is displayed on the player's screen. The second state is more complex: if the browser is focused (`focused = true`), the player's camera freezes, a cursor appears, and all their input (from the keyboard and mouse) goes directly to the browser, bypassing GTA and SA:MP (except for some functions like taking a screenshot with F8). The player will never be able to exit this state on their own; you must facilitate this in the browser interface code. For example, you can listen for the ESC key press and when pressed, call `cef.set_focus(false)`.

In other words, once you open a website like youtube.com in it, you can never exit it without closing the game or setting a timer to delete the browser in the mod.

These two states are completely independent of each other, meaning the browser can be `hidden = false` but at the same time `focused = false`. In this case, the browser will be displayed, but it won't have input access, and the player can perform actions in the game freely.

Interaction from the game mod
In short: the game mod should use only a few native functions (creating/deleting browsers, triggering events in the browser, as well as subscribing to them).
