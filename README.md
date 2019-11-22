Prints keys from `.aws/cli/cache` contents in console.

## Usage

1. Clear the cache contents

```
rm -rf ~/.aws/cli/cache
```

2. Log in to aws with your profile

```
aws s3 ls --profile "???" > /dev/null
```

3. Extract credentials, prints keys from the first cache entry

`extract-credentials`

Outputs:

```
AWS_ACCESS_KEY_ID=...
AWS_SECRET_ACCESS_KEY=...
AWS_SESSION_TOKEN=...
```
