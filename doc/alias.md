# Alias

## How to configure?

You can change ```rh``` default behaviour and create aliases you can reuse easily.

To see all the aliases available, the syntax is:

```bash
> rh alias --list
```

To update a configuration, the syntax is:

```bash
> rh alias [@alias] <options>
```

Please note that ```@alias``` is optional. If not specified it's the ```default``` alias. ```@alias``` must be lower-case.

```options``` can be any ```rh``` options.

## Default alias

The ```default``` alias is used if no alias is specified. For example, if you want to show the response headers (```--header``` or ```-h```):

```bash
> rh alias -h
```

You can select multiple options at the same time:

```bash
> rh alias --header --compact
```

Or the same but shorter:

```bash
> rh alias -hc
```

## Custom alias

You can create an alias to show the ```-U```RL and method + to show the response ```-h```eaders + to show a ```-c```ompact response:

```bash
> rh alias @my-alias -Uhc
```

## How to use an alias

You can use the "my-alias" alias created above to show the URL, method, response headers, and compact the response body:

```bash
> rh @my-alias https://httpbin.org/image/jpeg
```

You can use also the previous default alias that was built with the options ```-hc```:

```bash
> rh https://httpbin.org/image/jpeg
```

## Delete an alias

You can delete any alias you created, including the default alias. To delete the default alias:

```bash
> rh alias --delete
```

To delete the alias "my-alias":

```bash
> rh alias --delete @my-alias
```

## List all aliases

As simple as:

```bash
> rh alias --list
```

## More options in the future

The following default options are not available yet. Once available, these options will be available in ```rh``` in order for the aliases to be more flexible:

```
--hostname=localhost
--port=80
--secure-port=443
--method=GET
--method-if-body=POST
--method-if-pipe=POST
```
