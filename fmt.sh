#!/bin/bash

cp $HOME/miniquad/java/MainActivity.java .
gjf MainActivity.java >ma.java
diff MainActivity.java ma.java
