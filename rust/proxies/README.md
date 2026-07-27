# Proxies

Opens simple servers on 3 ports.

```bash
> curl 0.0.0.0:3000
Hello from print-server
> curl 0.0.0.0:3001
Hello from log-server
> curl 0.0.0.0:3009
Hello from email-server
```

Each server accepts a json message at `/print`, `/log` or `/email`, respectively. The message must be `{"text" : "<your text here>"}`.

```bash
# Client terminal
> curl 0.0.0.0:3000/print --json '{"text": "Hello World"}'
> curl 0.0.0.0:3001/log   --json '{"text": "Hello World"}'
> curl 0.0.0.0:3009/email --json '{"text": "Hello World"}'

# Server terminal
2026-07-27T02:06:02.473300Z  INFO servers: Printing text="Hello World"
2026-07-27T02:06:02.474002Z  INFO servers: Logging text="Hello World"
2026-07-27T02:06:02.474738Z  INFO servers: Emailing text="Hello World"
```

The proxy allows the client to send a Message without needing to know all of the ports and endpoints. It can send a single message and the proxy will distribute to the servers.

```bash
# Client terminal
> curl 0.0.0.0:4000
Hello from the proxy

> curl 0.0.0.0:4000/send --json '{"text": "Hello World"}'

# Proxy terminal
Proxying text="Hello World"

# Server terminal
2026-07-27T02:06:02.473300Z  INFO servers: Printing text="Hello World"
2026-07-27T02:06:02.474002Z  INFO servers: Logging text="Hello World"
2026-07-27T02:06:02.474738Z  INFO servers: Emailing text="Hello World"
```

