# gRPC Setup for Go
_______________________________________________________________________________

1. Protocol Buffer
```
sudo pacman -S --needed protobuf
```

2. Install the protocol compiler plugins for Go using the following commands:
```sh
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

_______________________________________________________________________________

To confirm that you have everything, run these commands.

_______________________________________________________________________________

```sh
protoc --version
```

libprotoc 32.0

_______________________________________________________________________________

```sh
protoc-gen-go --version
```

protoc-gen-go v1.36.9

_______________________________________________________________________________

```sh
protoc-gen-go-grpc --version
```

protoc-gen-go-grpc 1.5.1

_______________________________________________________________________________
### Create a new project

```sh
mkdir name-of-project
cd name-of-project
```

Intialize the module
```sh
go mod init github.com/your-github-username/projectname
```

This will create a `go.mod` file that looks like this:

```gomod
module go-lab/your-github-username/projectname

go 1.25.1
```

_______________________________________________________________________________

Add a main.go file to the module

```sh
touch main.go
```

Add this to the file:
```sh
cat > main.go << 'EOF'
package main

func main() {

}
EOF
```

_______________________________________________________________________________

Create a proto directory
```sh
mkdir -p proto
```

```sh
touch proto/hello.proto
```

_______________________________________________________________________________
