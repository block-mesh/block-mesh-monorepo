#!/usr/bin/env bash
# IPv4
curl --proxy '192.155.93.107:3000' https://example.com

# IPv6
curl -6 --proxy '[2600:3c02::f03c:94ff:fef5:39d5]:3000' https://example.com