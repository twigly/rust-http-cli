[![CI](https://github.com/twigly/rh/actions/workflows/ci.yml/badge.svg)](https://github.com/twigly/rh/actions/workflows/ci.yml)
[![CD](https://github.com/twigly/rh/actions/workflows/cd.yml/badge.svg)](https://github.com/twigly/rh/actions/workflows/cd.yml)

# rh: user-friendly command-line HTTP client

```rh``` is a user-friendly, lightweight and performant command-line tool to request HTTP APis. You can debug, test and verify any HTTP APi with ```rh``` in a simple and efficient way. ```rh``` is focused on performance and stability. You don't need OpenSSL because ```rh``` is based on Rustls, a modern TLS library alternative to OpenSSL.

```rh``` is a standalone application with no runtime or garbage collector, so it doesn't require Python or Java installed on your machine for example. ```rh``` is based on [Rust](https://www.rust-lang.org) that is a blazing fast and memory-efficient language.

The name ```rh``` stands for Rust HTTP.

<img width="600" src="../../doc/rh-screencast.svg">

# Getting started

→ [Installation guide](../../doc/install.md)

→ [Contributing guide](../../doc/contributing.md)

# Features

You can already use ```rh```, some features are not available yet. And new features will come based on your requests (please [file an issue](https://github.com/twigly/rh/issues) to do so).

- [X] Simple syntax to be more intuitive
- [X] Easy file download & upload
- [X] JSON made simple for command-line
- [X] JSON-friendly
- [X] Headers made simple for command-line
- [X] Self-signed SSL certificates
- [X] Don't repeat yourself with [aliases](../../doc/alias.md)
- [ ] Package manager
- [ ] Multi URLs
- [ ] Better help & version ([help & version](../../doc/help-and-version.md))
- More [to do](../../doc/todo.md)


# Don't repeat yourself

If you're used to execute very often the same requests, you can save time. An **alias** helps to change default values or create shortcuts. You can predefine what you like, it could be only the headers for example, or everything.

For example, someone could create an alias ```mp1-status``` (that would stand for "my-project-1" for example). Let's say you want to execute the following command very often:

```bash
> rh http://local-dev-mp1/status -UHhc X-Custom-Header:My-app
```

```-UHhc``` to show the ```-U```RL and the method + to show the request ```-H```eaders + to show the response ```-h```eaders + to show a ```-c```ompact response

```bash
> rh alias @mp1-status http://local-dev-mp1/status -UHhc X-Custom-Header:My-app
```

So now, you can reuse this config:

```bash
> rh @mp1-status
```

→ [See more about aliases](../../doc/alias.md)

# Examples

Who doesn't like "Hello, World!":

```bash
> rh httpbin.org/get
```

Change the method:

```bash
> rh HEAD https://httpbin.org/anything
```

Localhost with a particular port:

```bash
> rh :9200
```

You can POST data as JSON (it's the default format, see [more about it](../../doc/json.md)):

```bash
> rh https://httpbin.org/anything X-App:Super1 item1=Hello item2=World
```

You can POST data using the URL encoded format:

```bash
> rh https://httpbin.org/anything key1=1 --form
```

You can POST raw data:

```bash
> rh https://httpbin.org/anything --raw=hello
```

You can download a file and save it:

```bash
> rh https://httpbin.org/image/jpeg > image.jpeg
```

→ [More examples](../../doc/examples.md)

# License

```rh``` is distributed under the terms of the MIT license. See [LICENSE](/LICENSE) for details.

# Contributing

If you are interested in contributing to the ```rh``` project, please take a look at the [contributing guide](../../doc/contributing.md). If you'd like to request a feature or report a bug, please create a [GitHub issue](https://github.com/twigly/rh/issues).

Thanks to the people developing the third party libraries used in this project.
