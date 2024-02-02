
# command data flow

## legend

* S - server
* P - probe
* B - MQTT broker
* M - measurement (command response)

## commands

* `<region>/command/check` - S requests measurement readiness acknowledgement from the P
* `<probe_id>/command/check/ack` - P reports measurement readiness acknowledgement
* `<probe_id>/command/request` - S requests the measurement
* `<probe_id>/command/response` - P responds with M

### check

The server (S) emits an event to all probes within specific region.

The probe (P) checks its current load and verifies whether its ready to accept new measurement.

#### params:

* `region` - region ISO code

#### body:

* `request_id` - measurement id
* `command` - measurement type

### check/ack

The probe (P) confirms its readiness status.

The server (S) assigns the measurement to the first probe.

#### params:

* `probe_id` - probe id

#### body:

* `id` - measurement id

### command/request

The server (S) encrypts and emits a measurement request.

The probe (P) decrypts the request payload and starts the measurement.

#### params:

* `probe_id` - probe id

#### body:

* `request_id` - measurement id
* `command` - measurement type
* `options` - measurement specific options

### command/response

The probe (P) encrypts and emits the measurement result.

The server (S) verifies the probe ownership of the measurement, decrypts it and saves it in datastorage.

#### params:

* `probe_id` - probe id

#### body:

* `id` - measurement id
* `result` - measurement result
* `result.posix` - raw measurement result
* `result.values` - parsed measurement result

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


#### Line

```
  {
    hostname: string
    address: string
    ttl: number
    time: number
  }
```


#### Ping schema

```
  {
    header: {
      hostname: string
      address: string
    }
    lines: Line[]
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
