cd files
tar xvzf nauty2_8_8.tar.gz
mv nauty2_8_8/ ..
cd ../nauty2_8_8
./configure --enable-tls
make
cd ..