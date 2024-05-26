#!/usr/bin/env bash
# https://www.linode.com/community/questions/19219/how-do-i-open-a-port-in-my-linodes-firewall
# https://www.cyberciti.biz/faq/ufw-allow-incoming-ssh-connections-from-a-specific-ip-address-subnet-on-ubuntu-debian/

function add_port {
    local port=$1
    local protocol=$2
    sudo ufw allow "$port/$protocol"
}


sudo iptables --policy INPUT DROP
sudo iptables --policy OUTPUT ACCEPT
sudo iptables --policy FORWARD DROP
sudo iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT
sudo iptables -A INPUT -i lo -m comment --comment "Allow loopback connections" -j ACCEPT
sudo iptables -A INPUT -p icmp -m comment --comment "Allow Ping to work as expected" -j ACCEPT
sudo ip6tables --policy INPUT DROP
sudo ip6tables --policy OUTPUT ACCEPT
sudo ip6tables --policy FORWARD DROP
sudo ip6tables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT
sudo ip6tables -A INPUT -i lo -m comment --comment "Allow loopback connections" -j ACCEPT
sudo ip6tables -A INPUT -p icmpv6 -m comment --comment "Allow Ping to work as expected" -j ACCEPT

sudo ufw allow ssh
sudo ufw allow 3000
add_port 80 tcp
add_port 443 tcp
add_port 80 tcp
add_port 443 tcp
add_port 3000 tcp

