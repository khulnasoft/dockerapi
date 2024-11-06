# dockerapi

[![Build Status](https://github.com/KhulnaSoft/dockerapi/workflows/build/badge.svg?branch=master&event=push)](https://github.com/KhulnaSoft/dockerapi/actions?query=workflow%3Abuild+branch%3Amaster)

**WARNING**: This library was written for the explicit use of 
[BWPlugins](https://github.com/khulnasoft/BWPlugins) and should
**NOT** be used in any other production setting. Much of `dockerapi` is simply
`todo`s at present, and may never be implemented.

`dockerapi` is a synchronous low-level 
[Docker API](https://docs.docker.com/engine/api/v1.40/) Rust library. It relies
upon the [curl](https://crates.io/crates/curl) Rust library. As such, 
`dockerapi` works on any platform with `libcurl`. 

## License

This project is licensed under the BSD-3-Clause License - see the [LICENSE](LICENSE) file for details
