FROM --platform=linux/amd64 ubuntu:22.04
#RUN apt -y install sudo
RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl
RUN apt install -y libfuse2
RUN apt install -y libharfbuzz0b
RUN apt install -y libfontconfig
RUN apt install -y libgbm-dev
RUN apt install -y libthai0
RUN apt install -y libglu1-mesa-dev
RUN apt install -y libegl1
RUN apt install -y sudo
RUN apt install -y tmux
RUN #apt install -y protoc
RUN apt install -y musl-dev
RUN apt install -y gzip
RUN apt install -y git
RUN apt install -y jq
RUN apt install -y xvfb

RUN nohup Xvfb :99 -screen 0 1024x768x16 &
ENV DISPLAY=:99
RUN mkdir -p /tmp
#RUN sed -i -e 's/X11Forwarding.*/X11Forwarding yes/g' /etc/ssh/sshd_config
#RUN sed -i -e 's/X11Forwarding.*/X11Forwarding yes/g' /etc/ssh/ssh_config
#RUN systemctl restart sshd
RUN curl -sL0 \
    "https://cloudflare-worker-ip-data.blockmesh.workers.dev" | \
    jq -r '.platforms["linux-x86_64"].url' > /tmp/blockmesh.url
RUN curl -sLO $(cat /tmp/blockmesh.url) \
  && tar -xvf $(cat /tmp/blockmesh.url | sed -e 's,^.*/,,') \
  && chmod +x $(cat /tmp/blockmesh.url | sed -e 's,^.*/,,' | sed -e 's,.tar.gz,,')
RUN mv $(cat /tmp/blockmesh.url | sed -e 's,^.*/,,') /bin/blockmesh

ENTRYPOINT ["/bin/blockmesh"]