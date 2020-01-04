#!/bin/bash
rm -r hoppinworld-export-linux
mkdir hoppinworld-export-linux
cp target/release/hoppinworld hoppinworld-export-linux/
rsync -avp client/assets/* hoppinworld-export-linux/assets
rsync -avp client/maps/* hoppinworld-export-linux/maps
zip -r hoppinworld-linux.zip hoppinworld-export-linux
