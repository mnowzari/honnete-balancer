# Honnete
### Simple load balancer written in Rust.

Honnete is a (very) simple asynchronous load balancer. For now, it only provides round-robin balancing of incoming requests.

Honnete requires a YAML config file to run.
The basic YAML configuration requires three items. You can use the provided YAML under the config/ directory as a template. 

    listening-port: <port number to listen to requests from>

    hosts:
        - <host:port>
        - <host:port>
        - <host:port>
        ...
    
    env: test # Choice of test/stage/prod


_To build the executable from source:_

    cargo build --release

_To start a Honnete load balancer instance:_

    honnete --conf <absolute-path-to-YAML-conf>
