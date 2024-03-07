# Cloudflare Worker IP Data

This is a simple worker that returns the IP data of a request.

`curl https://cloudflare-worker-ip-data.ohaddahan.workers.dev`

```json
{
  "cf_connecting_ip": "XX.XXX.XX.XX",
  "x_real_ip": "XX.XXX.XX.XX",
  "x_forwarded_for": "XX.XXX.XX.XX",
  "cf_ipcountry": "XX"
}
```