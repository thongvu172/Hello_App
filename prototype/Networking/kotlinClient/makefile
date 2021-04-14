
all: build

build/libs/kotlinClient.jar: src/main/kotlin/kotlinClient/App.kt
	gradlew jar build 

.PHONY: build run
build: 
	gradlew jar build 

run: build/libs/kotlinClient.jar
	java -jar 'build/libs/kotlinClient.jar' $(args)
