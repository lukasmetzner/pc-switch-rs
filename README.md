# PC Switch Rust
- Takes raw milliseconds as integers on port 8000/TCP
- Is intentionally designed to have one client at a time
- Developed on a Raspberry Pi 3B+
- SRD-05vdc-sl-c relay connected to, 3.3v (ups), GND and GPIO 21

## Example usage
Connects to application and switches relay on for 1000ms and 500ms.
``` bash
nc 127.0.0.1 8000
1000
500
```
