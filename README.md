# rURL: user-friendly command-line HTTP client

```rURL``` is a user-friendly command-line tool to request HTTP APis. You can debug, test and verify any HTTP APi with ```rURL``` in a simple and efficient way. The name and the tool is inspired by ```cURL```. ```rURL``` is a standalone application that doesn't require Python or Java installed on your machine. ```rURL``` is based on [Rust](https://www.rust-lang.org) to provide speed and memory-safety.

<img width="600" src="doc/rurl-screencast.svg">

# Getting started

→ [Installation guide](doc/install.md)

→ [Contributing guide](doc/contributing.md)

# Features

You can already use ```rURL```, some features are not available yet. And new features will come based on your requests (please [file an issue](/twigly/rurl/issues) to do so).

- [X] Simple syntax to be more intuitive
- [X] Easy file download & upload
- [X] JSON made simple for command-line
- [X] JSON-friendly
- [X] Headers made simple for command-line
- [ ] Don't repeat yourself with [configurations](doc/configuration.md)
- [ ] Package manager
- [ ] More options with SSL
- [ ] Multi URLs

For now, the priority is:

- configurations
- package manager
- SSL options

# Don't repeat yourself

This feature is in progress and not available yet.

If you're used to execute very often the same requests, you can save time. A config helps to change default values or create shortcuts. You can predefine what you like, it could be only the headers for example, or everything.

For example, someone could create a configuration ```mp1-status``` (that would stand for "my-project-1" for example). Let's say you want to execute the following command very often:

```bash
> rurl http://local-dev-mp1/status -uHhc X-Custom-Header:My-app
```

```-uHhc``` to show the ```-u```RL and the method + to show the request ```-H```eaders + to show the response ```-h```eaders + to show a ```-c```ompact response

```bash
> rurl config mp1-status http://local-dev-mp1/status -chuH X-App:My-app
```

So now, you can reuse this config:

```bash
> rurl mp1-status
```

→ [See more about configurations](doc/configuration)

# Examples

Who doesn't like "Hello, World!":

```bash
> rurl httpbin.org/get
```

Change the method:

```bash
> rurl HEAD https://httpbin.org/anything
```

Localhost with a particular port:

```bash
> rurl :9200
```

You can POST data as JSON (it's the default format, see [more about it](doc/json.md)):

```bash
> rurl https://httpbin.org/anything X-App:Super1 item1=Hello item2=World
```

You can POST data using the URL encoded format:

```bash
> rurl https://httpbin.org/anything key1=1 --form
```

You can POST raw data:

```bash
> rurl https://httpbin.org/anything --raw=hello
```

You can download a file and save it:

```bash
> rurl https://httpbin.org/image/jpeg > image.jpeg
```

→ [More examples](doc/examples.md)

# Benchmarks

Some benchmarks have been carried out. But before publishing anything they've been reviewed since the aim is to be accurate and not misleading.

# License

```rURL``` is distributed under the terms of the MIT license. See [LICENSE](/LICENSE) for details.

# Contributing

If you are interested in contributing to the ```rURL``` project, please take a look at the [contributing guide](doc/contributing.md). If you'd like to request a feature or report a bug, please create a [GitHub issue](/twigly/rurl/issues).