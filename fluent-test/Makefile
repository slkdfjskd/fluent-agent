FILES=$(shell find . -not -path "./vendor/*" -type f -name "*.proto")
PROJECT_PATH=$(shell pwd)

test:
	echo $(FILES)

.PHONY: protoc
protoc:
	/usr/local/bin/docker run --rm -v $(PROJECT_PATH):/workspace -w /workspace rvolosatovs/protoc:4.0.0 \
		--proto_path=. \
		--proto_path=vendor \
		--go_out=.  --go_opt=paths=source_relative \
		--go-grpc_out=. --go-grpc_opt=paths=source_relative \
		--validate_out="lang=go,paths=source_relative:." \
		$(FILES)