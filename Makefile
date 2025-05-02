files:
	mkdir -p files
	echo "Hello" > files/1.txt
	echo "Hello" > files/2.txt
	echo "Hello World" > files/3.txt
	echo ".env test" > files/.env.test
	mkdir -p files/more_files
	echo "This is a unique file" > files/more_files/4.txt
	echo "Hello" > files/more_files/5.txt
	echo "This is another unique file" > files/more_files/6.txt
	mkdir -p files/more_files/more_more_files
	echo "Hello" > files/more_files/more_more_files/7.txt
	echo "Last one" > files/more_files/more_more_files/8.txt

clean:
	rm -rf files

tests:
	make clean
	make files
	cargo test
