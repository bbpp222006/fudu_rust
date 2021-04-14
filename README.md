# fudu_rust
这是一个对接go-cqhttp，websocket的复读机模块

现在只支持群聊

不同人复读会触发复读功能，同一人复读会回复：¿

# 用法

go-cqhttp需要打开websocket功能

docker: 
```
docker run -e WS="ws://10.243.159.138:30010" -d --name=fudu fudu
```

docker compose:

```
还没写好……
```

ps：代码写的比较烂，欢迎pr