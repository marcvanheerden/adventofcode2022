#!/bin/bash

for value in {1..10}
do
cat input >> input10
done

for value in {1..10}
do
cat input10 >> input100
done

for value in {1..10}
do
cat input100 >> input1000
done

for value in {1..10}
do
cat input1000 >> input10000
done

