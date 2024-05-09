# Geektime Rust 第二周 simple-redis

## 一、单元测试
```
cargo nextest run
```

## 二、程序运行
```
cargo run
```

## 三、使用客户端测试

```
redis-cli
```

### 3.1 echo 指令测试
```
echo "hello world"
```

可以看到数据原样返回

### 3.2 map 相关指令测试
```
set name kaka # 返回 OK
get name      # 可以正常返回名字 kaka
get age       # 访问不存在的 key 会返回 (nil)
```

### 3.3 hmap 相关指令测试
```
# HSET 返回一个整型，字段是新添加的返回 1，如果是已经存在的返回 0
HSET myhash field1 "Hello"  # (integer) 1
HSET myhash field2 "World"  # (integer) 1
HSET myhash field2 "World"  # (integer) 0
hget myhash field1 # 返回 "Hello"
```

---

```
hgetall myhash # 以数组形式显示所有 key, value
```

返回结果：
```
1) "field2"
2) "World"
3) "field1"
4) "Hello"
```

---

对于没有的 field 会返回 `nil`
```
HMGET myhash field1 field2 nofield
```

返回结果：
```
1) "Hello"
2) "World"
3) (nil)
```

### 3.4 set 相关指令测试

和 hset 有点像，新添加返回 1，已经存在的返回 0
```
SADD myset "one"      # 返回 (integer) 1
SADD myset "one"      # 返回 (integer) 0 # 因为已经存在
SISMEMBER myset "one" # 返回 (integer) 1
SISMEMBER myset "two" # 返回 (integer) 0
```
