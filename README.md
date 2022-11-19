tunm_truncate_log
==========
file truncate by Rust, auto truncate log 

## Config file
```yaml
rotate: 5
dateext: "%Y-%m-%d"
all_config:
  D:/log/*:
    rotate: 5
    period: 1m
    size: 1000k
  /var/lib/docker/containers/*/*-json.log:
    rotate: 1
    period: 1m
    size: 1m
```

## How to run
run in background
```shell
./tunm_truncate_log --FromDaemon -c config.yaml
```