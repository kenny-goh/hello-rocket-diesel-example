# Hello Rocket Diesel API example
An toy example of Rust API webserver built with Rocket (https://rocket.rs/) and Diesel (https://diesel.rs/).

## Description
This is a toy project to demonstrate how to build an API server (or microservice or whatever you want to call it) 
in Rust using the awesome Rocket web-server library and the Diesel object-relational-mapper library.

Why bother developing microservices in Rust instead of going with the NodeJS or even SpringBoot? 
Its incredibly lightweight, has small footprint and is blazingly fast! 

You can google 'Rust performance benchmark' or you can visit the following links:
- https://www.figma.com/blog/rust-in-production-at-figma/
- https://medium.com/@lholznagel/comparing-nodejs-and-rust-http-frameworks-response-times-5738dfa1843d

A word of caution, Rust is relatively new and will not have as many libraries as other established programming languages.
However, if you are using a microservice centric approach and you want massive performance boost for specific part of your application, 
using Rust is a viable strategy.

## Getting Started
- Make sure you have Rust installed. Please refer to https://www.rust-lang.org/
- Open your terminal
- Check out this project by using the command: git clone git@github.com:kenny-goh/hello-diesel-rocket.git
- Navigate to the project folder
- Type the following commands
```shell

Cargo build #This will build the project

diesel migration run #This will setup the sqlite db.

Cargo run --release # This will spin up the example diesel rocket server
```

# Curl commands to interact with the APIs

```shell
# Create a todo item
curl --request POST \
--url http://localhost:8000/api/todos \
--header 'Content-Type: application/json' \
--data '{
"name": "Hello world!!"
}'

# Find a todo item by id
curl --request GET \
  --url http://localhost:8000/api/todos/11 \
  --header 'Content-Type: application/json'

# Update todo item
curl --request PUT \
  --url http://localhost:8000/api/todos \
  --header 'Content-Type: application/json' \
  --data '
	{
		"id": 11,
		"name": "Hello world ZZZ"
	}
'

# List all todo items
curl --request GET \
  --url http://localhost:8000/api/todos/list

# Query todo item by text
curl --request GET \
  --url 'http://localhost:8000/api/todos/query?text=Hello%20world'

# Delete a specific todo by id
curl --request DELETE \
  --url http://localhost:8000/api/todos/1
```


## Dependencies
* Diesel (1.4.8)
* Rocket (0.5.0-rc.2)

## Roadmap
* Will probably add an authentication (security) layer

## Authors
Kenny Goh

## Version History
* 0.1
    * Initial Release


## License
This project is licensed under the MIT License

