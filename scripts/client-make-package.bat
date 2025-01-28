@echo off

pip install pipreqs

cd ..
cd client

pipreqs --force
pip install -r requirements.txt

cd ..
cd scripts
