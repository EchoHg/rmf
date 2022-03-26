# rmf

## 什么是`rmf`?

`rmf` 是一个快速的删除指定路径下面的文件的工具，可以设定保留距今指定天数的文件。

主要使用在删除过期的日志文件，过期的数据文件

```bash
$ rmf -h
rmf 1.0.0
删除指定路径下面的所有文件, 帮助输入 `-h`

USAGE:
    rmf [OPTIONS] --path <PATH>

OPTIONS:
    -d, --days <DAYS>    保留最近文件的天数 [default: 1]
    -h, --help           Print help information
    -p, --path <PATH>    要删除的文件上级路径
    -V, --version        Print version information
```

## 实例

1、删除`/home/user/logs`目录下的所有日志文件，但是保留两天的日志

```bash
$ rmf -p /home/user/logs -d 2
```

2、删除`/home/user/logs`目录下的所有日志文件，但是保留今天的日志

```bash
$ rmf -p /home/user/logs -d 1
// 或者不添加`-d`参数，因为默认值为1
$ rmf -p /home/user/logs
```

3、删除`/home/user/logs`目录下的所有日志文件

```bash
$ rmf -p /home/user/logs -d 0
```

## Author

* EchoAI <Echo_AI#foxmail.com>