[![Integration Tests](https://github.com/lele-maxwell/fibbo-project/actions/workflows/intergrated.yml/badge.svg)](https://github.com/lele-maxwell/fibbo-project/actions/workflows/intergrated.yml)

[![Integration Tests](https://github.com/lele-maxwell/fibbo-project/actions/workflows/intergrated.yml/badge.svg)](https://github.com/lele-maxwell/fibbo-project/actions/workflows/intergrated.yml)





# FibBot GitHub Action

## Description:

This program uses a GitHub action in Rust that filters out and numbers i.e numerical values in the contents of a pull request, calculate the Fibonacci of each number extracted provided its below the max_threshold value and the program is also responsible for posting a comment with the result .

So the program action is able to take parameters like the max_threshold and another parameter enable_fib which is like and option to enable the calculation of fib of a particular number

Ths program was develop with rust programming language along site with github actions api and workflow and executed using a Dockerfile

### The rust programming part is responsible:

- For fetching pull request content
- extracting numerical values form pull request contents
- fibonacci calculator which performed the fib of valid input gotten from pull request contents
- Also responsible for posting comments to calculated value

The rust program is executed in a Docker file using a rust base image to run and test the program

The rust program with use of the github actions, a reusable action action.yml was implemented which input from it where passed as parameter to the rust program which defines the threshold limit and option enable_fib with a true or false
The action file using a docker and dockerfile enabled the rust program to interact

An integrated workflow is also present which uses the the action file to automated the process where if any push or pull request is done to the repos the flow is triggered

### The workflow part is responsible:

With the use of the yml files the   they are responsible for setting the repos special permissions which will enable posting and also setting environmental variables that are used by the rust code and also responsible for taking the required parameters 


### USE:
To use tun the program you simply 
- Fork the repository
-  Create a new branch  
-  Add and modify contents 
-  Create a  Pull request
This will triger the workflow and post the the result of the fibo calculator
as comment in the pull





