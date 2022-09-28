#!/bin/sh

cargo run ../source.csv | xelatex --jobname=menu --output-directory=..
