FROM amazonlinux:2

# gcc is required to build ring, one of aws-config dependencies
RUN yum update -y \
    && yum install -y gcc \
    && yum clean all

# https://github.com/docker/docs/blob/main/develop/develop-images/dockerfile_best-practices.md#using-pipes
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
# https://rust-lang.github.io/rustup/concepts/profiles.html
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/sh -s -- --profile minimal -y

WORKDIR /volume
