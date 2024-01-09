
# command data flow

## legend

* S - server
* P - probe
* B - MQTT broker


## commands

`<region>/command/request` - 
`<request_id>/command/ack` - 
`<request_id>/command/response` - 

## response schema

### general schema

```
  {
    id: string // (request_id)
    status: complete | failed
    result: {
      posix: string
      values: Ping
    }
  }
```

### ping values schema

```
  {
    header: {
      hostname: string
      address: string
    }
    lines: {
      hostname: string
      address: string
      ttl: number
      time: number
    }[]
    rtt: {
      min: number
      avg: number
      max: number
      mdev: number
    }
    packets: {
      total: number
      loss: number
      rcv: number
      drop: number
    }
  }
```
