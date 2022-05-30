# 基于 ricq 开发的 FF14 的实用性机器人

支持的存储:

- [x] postgresql
- [ ] leveldb
- [ ] mysql

即将支持leveldb方便本地部署

## 配置

- 程序会优先使用当前目录下config.yaml文件的配置
- 如果config.yaml文件不存在则使用环境变量

config.yaml

``` yaml
rsconstr: postgres://postgres:postgres@localhost/xdff14
logskey: appkey
interval: 60
```

- `rsconstr` 数据库配置
- `logskey` fflogs的V1 Client Key
- `interval` 重试间隔，单位是秒

## 指令

目前支持的命令：

- [x] 模糊搜索物品
- [x] 检索物品价格
- [x] 查询玩家logs数据
- [ ] 生产相关功能

### 模糊搜索物品

`@小警察卸坤 6级药 物品`

![模糊搜索物品](README/%24908%603I%24%25STJ%5DKKXK2%5BP%5BPC.png)
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
