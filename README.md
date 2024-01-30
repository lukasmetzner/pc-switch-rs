# PC Switch Rust
- Takes raw milliseconds as input on port 8000/TCP
- **Current Issue**: GPIO pin is discarded after every call, because `set_low()` does not turn off the relay
- Developed on a Raspberry Pi 3B+
- SRD-05vdc-sl-c relay connected to, 5v, GND and GPIO 21

## Example usage
Connects to application and switches relay on for 1000ms and 500ms.
``` bash
nc 127.0.0.1 8000
1000
500
```
