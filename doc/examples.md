# Examples

In the following examples, you can add the flags ```-uhH``` to show more details.

- ```-u``` to show the URL and method
- ```-h``` to show the response headers
- ```-H``` to show the request headers

More information with:

```bash
> rurl --help
```

## Basics

Let's start with "Hello, World!":

```bash
> rurl httpbin.org/get
```

You can POST a request (```rURL``` will default to POST because there is a body):

```bash
> rurl httpbin.org/post id=rurl
```

A POST request with headers but no body:

```bash
> rurl POST httpbin.org/post X-key1:true X-key2:true
```

## Headers and items

The separator ```:``` is used to create headers:

```bash
> rurl httpbin.org/get key:Value
```

The separator ```=``` is used to create items to POST (if there are items then the method is POST):

```bash
> rurl httpbin.org/post key=Value
```

## Localhost

To run the examples of this "localhost" section you need a local server. In the following examples, if you don't specify a host, ```localhost``` will be the default host. Once the config feature is available, you'll be able to change the default host.

Basic:

```bash
> rurl http://localhost/test
```

Don't be bothered with the localhost domain:

```bash
> rurl /test
```

Or :

```bash
> rurl :
```

Localhost with a particular port:

```bash
> rurl :9200
```

```bash
> rurl :9200/_cluster/health
```

## Config (not available yet)

You can create a config named ```dev``` (this config says to POST the body ```id=rurl``` to ```httpbin.org/post```):

```bash
> rurl dev httpbin.org/post id=rurl
```

Let's say you have Elasticsearch running on the ```elasticsearch``` domain, you can define the following config ```ei``` (that would stand for Elasticsearch Indices):

```bash
> rurl config ei elasticsearch:9200/_cat/indices/*,-.*?v&s=index
```

Then you can just run the following command to show the Elasticsearch indices:

```bash
> rurl ei
```

## Data

You can POST data using pipes:

```bash
> echo "Hello, World!" | rurl httpbin.org/post
```

You can POST JSON (JSON is the default format):

```bash
> rurl https://httpbin.org/anything key1=1
```

You can POST data using the URL encoded format:

```bash
> rurl https://httpbin.org/anything key1=1 --form
```

Or using the raw flag:

```bash
> rurl https://httpbin.org/anything --raw='{"key1":1}' Content-Type:application/json
```

Or just plain text:

```bash
> rurl https://httpbin.org/anything --raw=hello
```

Or multi-lines:

```bash
> rurl https://httpbin.org/anything --raw='
{
  "inner-planets": ["Mercury", "Venus", "Earth", "Mars"],
  "sun": {
    "temp": 5778,
    "bigger-than-earth": true
  }
}
'
```

## Files

You can download a file and save it:

```bash
> rurl https://httpbin.org/image/jpeg > image.jpeg
```

If you love ```cat``` 🐱, you can upload a file:

```bash
> cat info.txt | rurl httpbin.org/post
```

The following commmand is not available yet, you can upload a file using the symbol ```@``` and the path:

```bash
> rurl httpbin.org/post @info.txt
```

## More or Less

If the response is output to another program there is no colours:

```bash
> rurl :9200/_nodes | more
```

But you can preserve the colours with the ```--pretty=color``` option and ```less -R```:

```bash
> rurl :9200/_nodes --pretty=color | less -R
```

## Some options

Show the URL and method:

```bash
> rurl httpbin.org/get -u
```

Show the headers (request and response):

```bash
> rurl httpbin.org/get -hH
```

Show the URL, method, headers and the response body as a compact form:

```bash
> rurl httpbin.org/get -uhHc
```

More options:

```bash
> rurl --help
```
