# Configuration

Feature not available yet.

## How to configure?

You can change ```rURL``` default values and create new configs you can reuse easily.

To see a configuration, the syntax is:

```bash
> rurl config [config-name]
```

To update a configuration, the syntax is:

```bash
> rurl config [config-name] [options]
```

Please note that ```config-name``` is optional. If not specified it's ```default```. ```config-name``` must be lower-case.

```options``` can be any ```rURL``` options.

## Default config

The ```default``` config is used if no config is specified. For example, if you want to show the response headers (```--header``` or ```-h```):

```bash
> rurl config -h
```

You can select multiple options at the same time:

```bash
> rurl config --header --compact
```

Or the same but shorter:

```bash
> rurl config -hc
```

## Custom config

You can create a config to show the ```-u```RL and method + to show the response ```-h```eaders + to show a ```-c```ompact response:

```bash
> rurl config my-config -uhc
```

## How to use a config

You can use the "my-config" config to show the URL, method, response headers, and compact the response body:

```bash
> rurl my-config https://pie.dev/image/jpeg
```

## Default config options

The config feature is not available yet. Once available, new options will be available in ```rURL``` in order for the config to be more flexible:

```
--hostname=localhost
--port=80
--secure-port=443
--method=GET
--method-if-body=POST
--method-if-pipe=POST
```