# 基于 ricq 开发的 FF14 的实用性机器人

支持的存储:

- [x] postgresql
- [ ] leveldb
- [ ] mysql

即将支持leveldb方便本地部署

## 配置

- 程序会优先使用当前目录下config.yaml里的配置
- 如果config.yaml文件不存在则使用环境变量

config.yaml

``` yaml
rsconstr: postgres://postgres:postgres@localhost/xdff14
logskey: {fflogs的app key,仅支持v1}
```

## 实时发送logs数据到群

- 从<https://www.fflogs.com/>获取数据
- 如果发送的玩家在群会自动at
- 不建议非亲友群使用，注意保护玩家隐私

``` yaml
- qq: {qq号，是0则永远不会at}
  name: {角色名}
  server: {服务器}
  group: 
    - id: {群号1}
    - id: {群号2}
- qq: 694638502
  name: Iker
  server: 琥珀原
  group: 
   - id: 136610715
```
