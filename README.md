# fudu_rust
这是一个对接go-cqhttp，websocket的复读机模块

现在只支持群聊

不同人复读会触发复读功能，同一人复读会回复：¿

# 用法

go-cqhttp需要打开websocket功能

docker: 
```
docker run -e WS="ws://10.243.159.138:30010" -d --name=fudu_rust varitia/fudu_rust

```

docker compose:

```
version: '3'

services:
    go_cqhttp:  #这里是示例，端口挂载这些根据具体镜像进行设置
        image: xxx/go-cqhttp:latest  
        ports:
            - 30009:80 # change ip if required
            - 30010:81
        volumes:
            - ./go-cqhttp-config/config.hjson:/mirai/config.hjson
            - ./go-cqhttp-config/device.json:/mirai/device.json 
    
    fudu:
        image: varitia/fudu_rust
        environment:
            WS: ws://go_cqhttp:81
        depends_on: 
            - go_cqhttp
        links:
            - go_cqhttp
        restart: always
```

现在不支持arm

ps：代码写的比较烂，欢迎pr