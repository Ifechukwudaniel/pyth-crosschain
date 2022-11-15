# Wormhole SDK Proto Node

NodeJS client protobuf for interacting with Wormhole

> Note: This code is generated by `tsproto` for `env=node` and `outputServices=grpc-js`

### Updating protos

To generate new proto files from the [protobuf source](/proto), at the root of the repo run:

    DOCKER_BUILDKIT=1 docker build --target node-export -f Dockerfile.proto -o type=local,dest=. .