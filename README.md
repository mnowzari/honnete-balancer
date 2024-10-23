# Honnete
### Simple load balancer written in Rust.

Honnete is a basic asynchronous load balancer.

For now, it only provides round-robin balancing of incoming requests.
It works well enough for simple stateless REST API calls, but the overall state is _very rough and not production ready._

Honnete requires a YAML config file to run.
The basic YAML configuration requires three items. You can use the provided YAML under the config/ directory as a template. 

    listening-port: <port number to listen to requests from>

    hosts:
      - <host:port>
      - <host:port>
      - <host:port>
    cpu: 8
    env: test


_To build executable from source:_

    cargo build --release

_To start an instance of Honnete load balancer:_

    honnete --conf <absolute-path-to-YAML-conf>
