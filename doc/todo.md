# Tasks to do

You're welcome to help on any of the following tasks for example.

Everything that improves performance is very welcome.

## Package manager

- [ ] Homebrew (Mac)
- [ ] MacPorts (Mac)
- [ ] Debian (Linux)
- [ ] Fedora (Linux)
- [ ] Ubuntu (Linux)
- [ ] Alpine (Linux)
- [ ] Arch (Linux)
- [ ] nixOS (Linux)
- [ ] openSUSE (Linux)
- [ ] Void Linux (Linux)
- [ ] Gentoo (Linux)
- [ ] Android
- [ ] Chocolatey (Windows)
- [ ] Others...

## Benchmark

- [ ] Comparison between ```cURL``` and ```rh```

## Features

### Authentication / proxy

- [ ] Deal with authentications [see authentication](authentication.md)
- [ ] Deal with proxies

### Items / headers

- [ ] Recognise arrays in data items (ex: ```array=item1,item2,item3```)
- [ ] Recognise files in data items (ex: ```file_content=@/path/file```)
- [ ] Remove headers with ```key:``` and set an empty value with ```"key: "```
- [ ] Read file content using the symbol ```@``` (for example ```--raw=@/path/file``` or ```key=@/path/file```)
- [ ] Append URL parameters via items
- [ ] Option to sort header and JSON keys (for example ```--sort``` to sort both of them, ```--sort=h``` to sort headers, ```--sort=j``` to sort JSON keys)

### Content encoding

- [ ] Read ```content-encoding=gzip``` (https://httpbin.org/gzip)
- [ ] Read ```content-encoding=brotli``` (https://httpbin.org/brotli)
- [ ] Read ```content-encoding=deflate``` (https://httpbin.org/deflate)

### Timeout / redirect

- [ ] Set a max redirects
- [ ] Set a timeout
- [ ] Show redirects if --verbose

### Misc

- [ ] Multi URLs
- [ ] Add an option ```--pretty=format``` to format without colouring
- [ ] Specify cookies without using the ```cookies``` header (and avoid using ```"``` to escape the ```;``` separator) - maybe not worth (low priority)
- [ ] Completion on available platforms

## Performance

- [ ] ```rh``` performance is very good but it would be nice to review the code and try to optimise
- [ ] the current binary size is acceptable but there are certainly ways to decrease it (without sacrificing performance)
