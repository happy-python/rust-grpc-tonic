# rust + grpc + tonic

## gRPC 四类服务方法
- 单项 RPC
- 服务端流式 RPC
- 客户端流式 RPC
- 双向流式 RPC

## 认证
- Token-based authentication
- TLS based authentication

JWT: https://crates.io/crates/jsonwebtoken

Tonic support TLS

## tonic 编译proto文件自动提示问题
<img src="http://qiniu.rocbj.com/1611046352901-min.png" width="300" height="500"/>


## 客户端测试工具
![](https://github.com/uw-labs/bloomrpc/blob/master/resources/blue/256x256.png)

[BloomRPC](https://github.com/uw-labs/bloomrpc)

## 参考
https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o
