test workflow

```bash
docker build -t noderust .
act -P ubuntu-latest=noderust:latest --pull=false
```


