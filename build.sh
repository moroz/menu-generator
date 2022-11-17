#!/bin/sh

SOURCE=${1:-../source.csv}

cargo run $SOURCE | xelatex --jobname=menu --output-directory=..
