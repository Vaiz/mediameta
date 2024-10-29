# MediaMeta

This library offers a simple, user-friendly API for accessing essential metadata, like creation
date and media resolution, from various media files. It prioritizes performance by using Rust
native libraries whenever possible, with an optional fallback to the external mediainfo tool to
cover cases where Rust libraries lack specific functionality. The mediainfo fallback is not
enabled by default and can be activated via a feature flag.

## Future Plans:

- Expand native support to more media types.
- Add functionality to extract additional metadata fields beyond creation date and resolution.

## Links

- [Documentation](https://docs.rs/mediameta/latest/mediameta/)
- [Examples](https://github.com/Vaiz/mediameta/tree/master/examples)
- [media-sync](https://github.com/Vaiz/media-sync/blob/master/src/main.rs) - a media organization
  app based on this library.