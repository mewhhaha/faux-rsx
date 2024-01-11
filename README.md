# faux-rsx (WIP)

Repository to learn about [Bevy](https://bevyengine.org/) and procedural macros. Will probably not be used once they release the new scene system.

Since I do a lot of web development and use tailwind I want to be able to do the following:

```rust
rsx! {
    <div class="flex flex-col p-4">
        "hello"
        <div>
            "world"
        </div>
    </div>
}
```
