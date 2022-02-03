# JSON

Items are converted to JSON by default.

## Key/value

Items are a list of key/value. Each key/value is specified as ```key=value```.

## Number and string

If you want to force a number to be as a string, you can use ```/=``` instead of ```=```

```bash
> rurl httpbin.org/post number1=123 number2/=456 text=hello
```

The JSON object will be:

```json
{
  "number1": 123,
  "number2": "456",
  "text": "hello",
}
```

## Boolean and string

If you want to force a boolean to be as a string, you can use ```/=``` instead of ```=```

```bash
> rurl httpbin.org/post b1=true b2=false b3=y b4=n b5/=true b6/=false b7/=y b8/=n
```

The JSON object will be:

```json
{
  "b1": true,
  "b2": false,
  "b3": true,
  "b4": false,
  "b5": "true",
  "b6": "false",
  "b7": "y",
  "b8": "n",
}
```