# 基于 ricq 开发的 FF14 的实用性机器人

支持的存储:

- [x] postgresql
- [x] sled
- [ ] mysql
- [ ] ~~leveldb~~

开发环境：rust nightly

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

## 实时发送logs数据到群

- 从<https://www.fflogs.com/>获取数据
- 如果发送的玩家在群会自动at
- 不建议非亲友群使用，注意保护玩家隐私
- 使用时注意logs的请求限制，可以通过调整interval和捐赠logs来支持更多玩家

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

![发送logs到群](README/report.png)

## 指令

指令格式为：`@机器人 {参数} {命令}`

目前支持的命令：

- [x] `@机器人` 不带参数命令At机器人，查询玩家logs数据
- [x] `物品` 模糊搜索物品
- [x] `价格` 检索物品价格
- [ ] 生产相关功能

### 查询自己的logs

`@小警察卸坤`

![查询logs](README/high.png)

### 模糊搜索物品

`@小警察卸坤 6级药 物品`

![模糊搜索物品](README/wupin.png)

### 搜索物品价格

`@小警察卸坤 6级耐力之幻药 价格`

![搜索物品价格](README/jiage.png)
