# Template for micro-service in Rust #
[![Build Status](https://dev.azure.com/butzist/DevOpsDemo/_apis/build/status/DevOpsDemoTF.DevOpsDemo-template-Rust?branchName=master)](https://dev.azure.com/butzist/DevOpsDemo/_build/latest?definitionId=6&branchName=master)

### Description ###
Micro-service template to use with my [DevOpsDemo](https://github.com/DevOpsDemoTF/DevOpsDemo)

### Features ###
* Build in multi-stage Docker container
* Configuration via environment variables
* Logging in JSON
* State passed to API handlers
* Health-check endpoint
* Prometheus metrics
* Unit tests with JUnit-compatible output
* API/integration tests with docker-compose

### Links ###
* [Tide Docs](https://docs.rs/tide/0.2.0/tide/struct.App.html)
* [Web programming in Rust](https://github.com/gruberb/web-programming-in-rust)

### To-Do ###
* Use stable Rust and Tide (when it becomes available)
* Switch to rust:stable-alpine base image
* Make code more rusty
* Improve module structure
